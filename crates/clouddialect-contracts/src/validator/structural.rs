use std::collections::BTreeSet;

use boon::{Compiler, ErrorKind, InstanceToken, Schemas, ValidationError};
use serde_json::{Map, Value, json};

use super::version::{VersionCheckResult, check_version_compatibility};
use crate::documents::RawFields;
use crate::{
    CatalogSnapshot, CatalogUrlLoader, DocumentKind, ErrorCategory, StructuralOutcome,
    StructuralResult, ValidatorError, ValidatorWarning,
};

const COMPATIBILITY: &str =
    ".kiro/specs/contracts-evidence-foundation/requirements.md#requirement-7-compatibility-policy";

/// Owns explicitly supplied catalog bytes. Construction is not provenance,
/// semantic validation, behavioral verification or acceptance authorization.
pub struct Validator {
    snapshot: CatalogSnapshot,
}

impl Validator {
    pub fn new(snapshot: CatalogSnapshot) -> Self {
        Self { snapshot }
    }

    pub fn validate_structural(&self, kind: DocumentKind, document: &[u8]) -> StructuralResult {
        let mut result = StructuralResult {
            document_id: None,
            format_version: None,
            outcome: StructuralOutcome::Pass,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        let parsed: Value = match serde_json::from_slice(document) {
            Ok(value) => value,
            Err(error) => return parse_failure(result, document, error),
        };
        if !parsed.is_object() {
            return fail(
                result,
                ErrorCategory::Structural,
                "$",
                "Document must be a JSON object",
            );
        }
        // Check original core objects before Value can hide duplicate members.
        // Opaque extension values and embedded schema fragments stay untouched.
        if let Err(error) = check_core_duplicates(kind, document) {
            return parse_failure(result, document, error);
        }
        result.document_id = parsed.get("id").and_then(Value::as_str).map(str::to_owned);
        result.format_version = parsed
            .get("format_version")
            .and_then(Value::as_str)
            .map(str::to_owned);
        let Some(declared) = result.format_version.clone() else {
            return fail(
                result,
                ErrorCategory::Structural,
                "format_version",
                "A string document-format version is required",
            );
        };
        let entries: Vec<_> = self
            .snapshot
            .entries()
            .iter()
            .filter(|e| e.document_type == kind)
            .collect();
        let versions: Vec<_> = entries.iter().map(|e| e.format_version.as_str()).collect();
        let (index, minor_newer) = match check_version_compatibility(&declared, &versions) {
            VersionCheckResult::Supported(index) => (index, false),
            VersionCheckResult::MinorNewer(index) => (index, true),
            VersionCheckResult::InvalidFormat => {
                return fail(
                    result,
                    ErrorCategory::Version,
                    "format_version",
                    "Document-format version is not valid SemVer 2.0.0",
                );
            }
            VersionCheckResult::UnsupportedMajor => {
                return fail(
                    result,
                    ErrorCategory::UnsupportedMajorVersion,
                    "format_version",
                    &format!("unsupported-major-version {declared:?}; see {COMPATIBILITY}"),
                );
            }
            VersionCheckResult::UnsupportedVersion => {
                return fail(
                    result,
                    ErrorCategory::UnsupportedVersion,
                    "format_version",
                    &format!("unsupported-version {declared:?}; see {COMPATIBILITY}"),
                );
            }
        };
        let selected = entries[index];
        if let Some(uri) = parsed.get("$schema")
            && uri.as_str() != Some(selected.schema_id.as_str())
        {
            return fail(
                result,
                ErrorCategory::UnresolvableReference,
                "$schema",
                "Instance $schema must exactly match the selected admitted kind/version URI",
            );
        }
        let mut compiler = Compiler::new();
        compiler.use_loader(Box::new(CatalogUrlLoader::new(self.snapshot.clone())));
        compiler.enable_format_assertions();
        let mut schemas = Schemas::new();
        let root = match compiler.compile(&selected.schema_id, &mut schemas) {
            Ok(root) => root,
            Err(_) => {
                return fail(
                    result,
                    ErrorCategory::UnresolvableReference,
                    "$schema",
                    "Selected admitted schema could not compile or resolve its references",
                );
            }
        };
        let mut view = parsed.clone();
        if minor_newer {
            let base: Value = serde_json::from_slice(
                self.snapshot
                    .schema_bytes(&selected.schema_id)
                    .expect("immutable admitted entry"),
            )
            .expect("inventory checked schema JSON");
            let Some(key_schema) = root_key_schema(&base) else {
                return fail(
                    result,
                    ErrorCategory::Unsupported,
                    "format_version",
                    "Base schema root field vocabulary cannot be safely projected; no compatibility fallback performed",
                );
            };
            // A separate trusted key-only probe uses the pinned engine's own
            // pattern semantics. It never replaces the actual base validator.
            let probe_uri = "urn:clouddialect:root-field-projection";
            let mut probe = Compiler::new();
            probe.use_loader(Box::new(CatalogUrlLoader::new(self.snapshot.clone())));
            let mut probe_schemas = Schemas::new();
            let probe_root = probe
                .add_resource(probe_uri, key_schema)
                .map_err(|_| ())
                .and_then(|()| probe.compile(probe_uri, &mut probe_schemas).map_err(|_| ()));
            let Ok(probe_root) = probe_root else {
                return fail(
                    result,
                    ErrorCategory::Structural,
                    "$schema",
                    "Base schema field projection could not compile",
                );
            };
            result.warnings.push(ValidatorWarning {
                field_name_verbatim: "format_version".to_owned(),
                description: format!("Newer MINOR {declared:?} checked using admitted base {:?}; root additions are reported separately", selected.format_version),
            });
            let object = view.as_object_mut().expect("checked object");
            for key in parsed.as_object().expect("checked object").keys() {
                let candidate = Value::Object(Map::from_iter([(key.clone(), Value::Null)]));
                if probe_schemas.validate(&candidate, probe_root).is_err() {
                    object.remove(key);
                    result.warnings.push(ValidatorWarning {
                        field_name_verbatim: key.clone(),
                        description: format!(
                            "Root field at {} was not validated by the base schema",
                            property_path("", key)
                        ),
                    });
                }
            }
            object.insert(
                "format_version".to_owned(),
                Value::String(selected.format_version.clone()),
            );
        }
        if let Err(error) = schemas.validate(&view, root) {
            collect_errors(&error, &mut result);
        }
        let (warnings, errors) = super::extensions::scan_extension_fields(&parsed);
        result.warnings.extend(warnings);
        result.errors.extend(errors);
        result
            .errors
            .sort_by(|a, b| (&a.field_path, &a.description).cmp(&(&b.field_path, &b.description)));
        result.errors.dedup();
        result.outcome = if !result.errors.is_empty() {
            StructuralOutcome::Fail
        } else if minor_newer {
            StructuralOutcome::Warn
        } else {
            StructuralOutcome::Pass
        };
        result
    }
}

// These edges follow the document DTOs, not JSON Schema keywords or arbitrary
// payload keys. A leaf checks that object's member names, but never interprets
// its opaque values. This is deliberately not a second schema/type validator.
fn check_core_duplicates(kind: DocumentKind, document: &[u8]) -> serde_json::Result<()> {
    use serde_json::value::RawValue;

    #[derive(Clone, Copy)]
    enum Shape {
        Object(&'static [(&'static str, Shape)]),
        Array(&'static [(&'static str, Shape)]),
    }
    use Shape::{Array, Object};
    const LEAF: Shape = Object(&[]);
    const ARTIFACT: &[(&str, Shape)] = &[("content_hash", LEAF)];
    const CONTRACT: &[(&str, Shape)] = &[
        ("operation_identity", LEAF),
        ("side_effects", Array(&[])),
        (
            "resource_dependencies",
            Array(&[
                ("known_behavior", LEAF),
                ("unresolved_assumptions", Array(&[])),
            ]),
        ),
        ("obligations", Array(&[("unresolved_assumption", LEAF)])),
        ("behavioral_notes", Array(&[])),
    ];
    const EVIDENCE: &[(&str, Shape)] =
        &[("provenance", LEAF), ("unresolved_assumptions", Array(&[]))];
    const VERIFICATION: &[(&str, Shape)] = &[
        ("candidate", Object(ARTIFACT)),
        ("pack", Object(ARTIFACT)),
        ("toolchain", Object(ARTIFACT)),
        ("configuration", Object(ARTIFACT)),
        (
            "binding",
            Object(&[
                ("input", Object(ARTIFACT)),
                ("dependencies", Object(ARTIFACT)),
                ("suite", Object(ARTIFACT)),
                ("oracle", Object(ARTIFACT)),
                ("policy", Object(ARTIFACT)),
            ]),
        ),
        ("tests", Array(&[("failure_description", LEAF)])),
        (
            "acceptance",
            Object(&[
                ("verification_result", LEAF),
                ("approval_evidence", Object(ARTIFACT)),
            ]),
        ),
    ];

    fn check(raw: &str, shape: Shape) -> serde_json::Result<()> {
        match shape {
            Object(children) if raw.starts_with('{') => {
                let fields: RawFields = serde_json::from_str(raw)?;
                for (name, shape) in children {
                    if let Some(value) = fields.0.get(*name) {
                        check(value.get(), *shape)?;
                    }
                }
            }
            Array(children) if raw.starts_with('[') => {
                for value in serde_json::from_str::<Vec<Box<RawValue>>>(raw)? {
                    check(value.get(), Object(children))?;
                }
            }
            _ => {} // The actual active schema reports incorrect field types.
        }
        Ok(())
    }

    let children = match kind {
        DocumentKind::BehavioralContract => CONTRACT,
        DocumentKind::EvidenceRecord => EVIDENCE,
        DocumentKind::VerificationResult => VERIFICATION,
        DocumentKind::CatalogIndex => {
            &[("schemas", Array(&[])), ("resolver_resources", Array(&[]))]
        }
        DocumentKind::ExtensionRegistry => &[("extensions", Array(&[]))],
    };
    let raw: Box<RawValue> = serde_json::from_slice(document)?;
    check(raw.get(), Object(children))
}

fn fail(
    mut result: StructuralResult,
    category: ErrorCategory,
    path: &str,
    description: &str,
) -> StructuralResult {
    result.outcome = StructuralOutcome::Fail;
    result.errors.push(ValidatorError {
        document_id: result.document_id.clone(),
        format_version: result.format_version.clone(),
        category,
        field_path: Some(path.to_owned()),
        description: description.to_owned(),
        byte_offset: None,
    });
    result
}

fn parse_failure(
    result: StructuralResult,
    bytes: &[u8],
    error: serde_json::Error,
) -> StructuralResult {
    let line_start: usize = bytes
        .split_inclusive(|b| *b == b'\n')
        .take(error.line().saturating_sub(1))
        .map(<[u8]>::len)
        .sum();
    let mut result = fail(
        result,
        ErrorCategory::Unparseable,
        "$",
        "Document JSON could not parse unambiguously",
    );
    result.errors[0].byte_offset =
        Some((line_start + error.column().saturating_sub(1)).min(bytes.len()));
    result
}

pub(super) fn property_path(parent: &str, key: &str) -> String {
    let simple = !key.is_empty()
        && key.bytes().enumerate().all(|(i, b)| {
            b.is_ascii_alphabetic()
                || b == b'_'
                || b == b'$'
                || (i > 0 && (b.is_ascii_digit() || b == b'-'))
        });
    if simple {
        if parent.is_empty() {
            key.to_owned()
        } else {
            format!("{parent}.{key}")
        }
    } else {
        format!(
            "{parent}[{}]",
            serde_json::to_string(key).expect("string encoding")
        )
    }
}

fn collect_errors(error: &ValidationError<'_, '_>, result: &mut StructuralResult) {
    if !error.causes.is_empty() {
        for cause in &error.causes {
            collect_errors(cause, result);
        }
        return;
    }
    let mut path = String::new();
    for token in &error.instance_location.tokens {
        path = match token {
            InstanceToken::Prop(key) => property_path(&path, key),
            InstanceToken::Item(index) => format!("{path}[{index}]"),
        };
    }
    let mut push = |path: String, description: &str| {
        result.errors.push(ValidatorError {
            document_id: result.document_id.clone(),
            format_version: result.format_version.clone(),
            category: ErrorCategory::Structural,
            field_path: Some(if path.is_empty() {
                "$".to_owned()
            } else {
                path
            }),
            description: description.to_owned(),
            byte_offset: None,
        });
    };
    match &error.kind {
        ErrorKind::Required { want } => {
            for key in want {
                push(property_path(&path, key), "Required field is missing");
            }
        }
        ErrorKind::DependentRequired { missing, .. } | ErrorKind::Dependency { missing, .. } => {
            for key in missing {
                push(
                    property_path(&path, key),
                    "Required dependent field is missing",
                );
            }
        }
        ErrorKind::AdditionalProperties { got } => {
            for key in got {
                push(
                    property_path(&path, key),
                    "Field is not permitted by the selected schema",
                );
            }
        }
        ErrorKind::PropertyName { prop } => push(
            property_path(&path, prop),
            "Field name does not satisfy the selected schema",
        ),
        ErrorKind::Format { .. } => push(path, "Field does not satisfy its declared format"),
        ErrorKind::Pattern { .. } => push(path, "Field does not match the required grammar"),
        ErrorKind::FalseSchema => push(path, "Field is not permitted by the selected schema"),
        _ => push(
            path,
            "Field does not satisfy the selected schema constraint",
        ),
    }
}

/// Collect root-context declarations, including inactive conditional branches,
/// but never descend into property values, embedded data or nested item schemas.
fn root_key_schema(base: &Value) -> Option<Value> {
    fn collect(
        schema: &Value,
        base: &Value,
        seen: &mut BTreeSet<String>,
        names: &mut Map<String, Value>,
        patterns: &mut Map<String, Value>,
        covers_all: &mut bool,
    ) -> Option<()> {
        if schema.get("$id").is_some() && schema.get("$id") != base.get("$id") {
            return None;
        }
        // Explicit wildcard schemas constrain/evaluate root keys. Keep even
        // malformed values for the real base check; never classify by value.
        *covers_all |= [
            "propertyNames",
            "const",
            "enum",
            "minProperties",
            "maxProperties",
        ]
        .iter()
        .any(|keyword| schema.get(*keyword).is_some());
        for keyword in ["additionalProperties", "unevaluatedProperties"] {
            if let Some(value) = schema.get(keyword)
                && value != &Value::Bool(false)
            {
                *covers_all = true;
            }
        }
        for keyword in ["properties", "dependentRequired", "dependentSchemas"] {
            if let Some(map) = schema.get(keyword).and_then(Value::as_object) {
                for key in map.keys() {
                    names.insert(key.clone(), Value::Bool(true));
                }
            }
        }
        if let Some(dependencies) = schema.get("dependentRequired").and_then(Value::as_object) {
            for key in dependencies
                .values()
                .filter_map(Value::as_array)
                .flatten()
                .filter_map(Value::as_str)
            {
                names.insert(key.to_owned(), Value::Bool(true));
            }
        }
        if let Some(required) = schema.get("required").and_then(Value::as_array) {
            for key in required.iter().filter_map(Value::as_str) {
                names.insert(key.to_owned(), Value::Bool(true));
            }
        }
        if let Some(map) = schema.get("patternProperties").and_then(Value::as_object) {
            for key in map.keys() {
                patterns.insert(key.clone(), Value::Bool(true));
            }
        }
        for keyword in ["if", "then", "else", "not"] {
            if let Some(child) = schema.get(keyword) {
                collect(child, base, seen, names, patterns, covers_all)?;
            }
        }
        for keyword in ["allOf", "anyOf", "oneOf"] {
            if let Some(children) = schema.get(keyword).and_then(Value::as_array) {
                for child in children {
                    collect(child, base, seen, names, patterns, covers_all)?;
                }
            }
        }
        if let Some(children) = schema.get("dependentSchemas").and_then(Value::as_object) {
            for child in children.values() {
                collect(child, base, seen, names, patterns, covers_all)?;
            }
        }
        // The admitted MVP root vocabulary uses transparent local references.
        // Opaque/dynamic root references must fail compatibility, never cause
        // guessed field removal. Exact-version validation still uses Boon.
        if schema.get("$dynamicRef").is_some() || schema.get("$recursiveRef").is_some() {
            return None;
        }
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            let pointer = reference.strip_prefix('#')?;
            if seen.insert(reference.to_owned()) {
                collect(
                    base.pointer(pointer)?,
                    base,
                    seen,
                    names,
                    patterns,
                    covers_all,
                )?;
            }
        }
        Some(())
    }
    let mut names = Map::new();
    let mut patterns = Map::new();
    let mut covers_all = false;
    collect(
        base,
        base,
        &mut BTreeSet::new(),
        &mut names,
        &mut patterns,
        &mut covers_all,
    )?;
    Some(
        json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":names,"patternProperties":patterns,"additionalProperties":covers_all}),
    )
}
