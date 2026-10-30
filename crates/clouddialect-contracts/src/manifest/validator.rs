//! Local and scoped semantic TOML validation, never execution admission.

use std::{collections::BTreeSet, sync::OnceLock};

use regex::Regex;
use semver::{Version, VersionReq};

use super::{ManifestFidelity, ManifestOutcome, ManifestValidationResult, PackManifest};
use crate::{
    CatalogSnapshot, DocumentKind, DocumentStore, ErrorCategory, SemanticOutcome, SemanticResult,
    Validator, ValidatorError, check_fidelity_mapping,
};

/// Owns the explicitly supplied catalog for validation phases. The local phase
/// checks declaration syntax, not catalog compatibility or external documents.
pub struct ManifestValidator {
    snapshot: CatalogSnapshot,
}

impl ManifestValidator {
    pub fn new(snapshot: CatalogSnapshot) -> Self {
        Self { snapshot }
    }

    /// Local validity precedes exact project-store references and fidelity.
    /// Semantic success authenticates neither observations nor approval.
    pub fn validate_manifest_semantic(
        &self,
        toml_bytes: &[u8],
        store: &dyn DocumentStore,
    ) -> SemanticResult {
        let local = self.validate_manifest(toml_bytes);
        if local.outcome == ManifestOutcome::Fail {
            let errors: Vec<_> = local
                .structural_errors
                .into_iter()
                .chain(local.semantic_errors)
                .collect();
            return SemanticResult {
                document_id: errors.iter().find_map(|e| e.document_id.clone()),
                outcome: SemanticOutcome::Fail,
                errors,
            };
        }
        let manifest: PackManifest = toml::from_str(
            std::str::from_utf8(toml_bytes).expect("local validation decoded identical bytes"),
        )
        .expect("local validation decoded identical manifest");
        let validator = Validator::new(self.snapshot.clone());
        let mut errors = check_fidelity_mapping(&manifest, store, &validator);
        // Full/partial bindings have already been resolved by the fidelity
        // checker. Explicit unsupported/orphan references still must resolve.
        for (key, reference) in &manifest.contracts {
            if manifest
                .operations
                .iter()
                .any(|op| op.id == *key && op.behavioral_fidelity != ManifestFidelity::Unsupported)
            {
                continue;
            }
            let record = match crate::validator::document_resolution::resolve_document(
                &validator,
                store,
                &reference.id,
                Some(&reference.format_version),
                DocumentKind::BehavioralContract,
            ) {
                Ok(record) => record,
                Err(diagnostics) => {
                    let path = property_path("contracts", key);
                    errors.extend(diagnostics.into_iter().map(|mut error| {
                        // Preserve the referenced document's original location.
                        // Only locationless store failures belong at the binding.
                        if error.field_path.is_none() {
                            error.description = format!(
                                "{} (referenced document {:?}, format version {:?})",
                                error.description, reference.id, reference.format_version
                            );
                            error.document_id = Some(manifest.pack.id.clone());
                            error.format_version = None;
                            error.field_path = Some(format!("{path}.id"));
                        } else {
                            error
                                .document_id
                                .get_or_insert_with(|| reference.id.clone());
                            error
                                .format_version
                                .get_or_insert_with(|| reference.format_version.clone());
                        }
                        error
                    }));
                    continue;
                }
            };
            errors.extend(
                validator
                    .validate_semantic(DocumentKind::BehavioralContract, &record.bytes, store)
                    .errors,
            );
        }
        errors
            .sort_by(|a, b| (&a.field_path, &a.description).cmp(&(&b.field_path, &b.description)));
        SemanticResult {
            document_id: Some(manifest.pack.id),
            outcome: if errors.is_empty() {
                SemanticOutcome::Pass
            } else {
                SemanticOutcome::Fail
            },
            errors,
        }
    }

    /// Validates local rules without granting capabilities or checking fidelity.
    pub fn validate_manifest(&self, toml_bytes: &[u8]) -> ManifestValidationResult {
        let text = match std::str::from_utf8(toml_bytes) {
            Ok(text) => text,
            Err(error) => return unparseable(Some(error.valid_up_to())),
        };
        let manifest: PackManifest = match toml::from_str(text) {
            Ok(manifest) => manifest,
            Err(error) => return unparseable(error.span().map(|span| span.start)),
        };
        let mut errors = Vec::new();
        let mut report = |path: &str, category, description: &str| {
            errors.push(ValidatorError {
                document_id: Some(manifest.pack.id.clone()),
                // A pack release is not a document-format revision.
                format_version: None,
                category,
                field_path: Some(path.into()),
                description: description.into(),
                byte_offset: None,
            });
        };
        static PACK_ID: OnceLock<Regex> = OnceLock::new();
        let pack_id = PACK_ID.get_or_init(|| {
            Regex::new(r"\A[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)+\z")
                .expect("fixed reverse-domain grammar")
        });
        if !pack_id.is_match(&manifest.pack.id) {
            report(
                "pack.id",
                ErrorCategory::Semantic,
                "Pack identifier must use the reverse-domain grammar",
            );
        }
        for (path, version) in [
            ("pack.version", &manifest.pack.version),
            (
                "compatibility.catalog_min",
                &manifest.compatibility.catalog_min,
            ),
            (
                "compatibility.catalog_max",
                &manifest.compatibility.catalog_max,
            ),
            (
                "schema_refs.contract_schema_version",
                &manifest.schema_refs.contract_schema_version,
            ),
            (
                "schema_refs.evidence_record_schema_version",
                &manifest.schema_refs.evidence_record_schema_version,
            ),
            (
                "schema_refs.verification_result_schema_version",
                &manifest.schema_refs.verification_result_schema_version,
            ),
        ] {
            if let Err(category) = parse_version(version) {
                report(path, category, "Expected a supported SemVer 2.0.0 value");
            }
        }
        for (key, reference) in &manifest.contracts {
            let path = property_path("contracts", key);
            if reference.id.is_empty() {
                report(
                    &format!("{path}.id"),
                    ErrorCategory::Semantic,
                    "Contract reference identifier must not be empty",
                );
            }
            if let Err(category) = parse_version(&reference.format_version) {
                report(
                    &format!("{path}.format_version"),
                    category,
                    "Expected a supported SemVer 2.0.0 value",
                );
            }
        }
        if manifest.toolchain.is_empty()
            && manifest.operations.iter().any(|operation| {
                matches!(
                    operation.behavioral_fidelity,
                    ManifestFidelity::Full | ManifestFidelity::Partial
                )
            })
        {
            report(
                "toolchain",
                ErrorCategory::Semantic,
                "Full or partial operations require a nonempty toolchain",
            );
        }
        if VersionReq::parse(&manifest.source.language_version_range).is_err() {
            report(
                "source.language_version_range",
                ErrorCategory::Semantic,
                "Expected a Cargo SemVer version requirement",
            );
        }
        for (index, dependency) in manifest.toolchain.iter().enumerate() {
            if VersionReq::parse(&dependency.version_range).is_err() {
                report(
                    &format!("toolchain[{index}].version_range"),
                    ErrorCategory::Semantic,
                    "Expected a Cargo SemVer version requirement",
                );
            }
        }
        if let Some(extension) = &manifest.x {
            if let Some(table) = extension.as_table() {
                for key in [
                    "pack",
                    "source",
                    "target",
                    "schema_refs",
                    "compatibility",
                    "capabilities",
                    "operations",
                    "toolchain",
                    "contracts",
                    "x",
                ] {
                    if table.contains_key(key) {
                        report(
                            &format!("x.{key}"),
                            ErrorCategory::Semantic,
                            "Extension field must not shadow a core top-level key",
                        );
                    }
                }
            } else {
                report(
                    "x",
                    ErrorCategory::Semantic,
                    "Extension section must be a TOML table",
                );
            }
        }
        match parse_version(&manifest.capabilities.format_version) {
            Ok(version) if version.cmp_precedence(&Version::new(1, 0, 0)).is_eq() => {}
            Ok(_) => report(
                "capabilities.format_version",
                ErrorCategory::UnsupportedVersion,
                "Capability declaration version is not supported",
            ),
            Err(category) => report(
                "capabilities.format_version",
                category,
                "Expected a supported SemVer 2.0.0 value",
            ),
        }
        let mut ids = BTreeSet::new();
        for (index, requirement) in manifest.capabilities.requirements.iter().enumerate() {
            let path = format!("capabilities.requirements[{index}]");
            for (field, value) in [
                ("id", &requirement.id),
                ("operation_id", &requirement.operation_id),
                ("resource", &requirement.resource),
                ("purpose", &requirement.purpose),
            ] {
                if value.is_empty() {
                    report(
                        &format!("{path}.{field}"),
                        ErrorCategory::Semantic,
                        "Capability requirement field must not be empty",
                    );
                }
            }
            if !ids.insert(&requirement.id) {
                report(
                    &format!("{path}.id"),
                    ErrorCategory::Semantic,
                    "Capability requirement identifier must be unique",
                );
            }
            if !requirement.operation_id.is_empty()
                && !manifest
                    .operations
                    .iter()
                    .any(|operation| operation.id == requirement.operation_id)
            {
                report(
                    &format!("{path}.operation_id"),
                    ErrorCategory::Semantic,
                    "Capability requirement must reference a declared operation",
                );
            }
        }
        errors.sort_by(|left, right| {
            left.field_path
                .cmp(&right.field_path)
                .then_with(|| left.description.cmp(&right.description))
        });
        ManifestValidationResult {
            outcome: if errors.is_empty() {
                ManifestOutcome::Pass
            } else {
                ManifestOutcome::Fail
            },
            structural_errors: Vec::new(),
            semantic_errors: errors,
            warnings: Vec::new(),
        }
    }
}

fn unparseable(byte_offset: Option<usize>) -> ManifestValidationResult {
    ManifestValidationResult {
        outcome: ManifestOutcome::Fail,
        structural_errors: vec![ValidatorError {
            document_id: None,
            format_version: None,
            category: ErrorCategory::Unparseable,
            field_path: None,
            // The parser's Display/message may contain attacker-controlled data.
            description: "Manifest could not decode as the required TOML structure".into(),
            byte_offset,
        }],
        semantic_errors: Vec::new(),
        warnings: Vec::new(),
    }
}

fn parse_version(text: &str) -> Result<Version, ErrorCategory> {
    Version::parse(text).map_err(|_| {
        // Full SemVer syntax permits unbounded core integers. The approved
        // implementation profile supports u64; overflow is not malformed syntax.
        static GRAMMAR: OnceLock<Regex> = OnceLock::new();
        let grammar = GRAMMAR.get_or_init(|| {
            Regex::new(r"\A(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*)?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?\z")
                .expect("fixed SemVer grammar")
        });
        if grammar.is_match(text) { ErrorCategory::UnsupportedVersion } else { ErrorCategory::Version }
    })
}

fn property_path(parent: &str, key: &str) -> String {
    if !key.is_empty()
        && key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        format!("{parent}.{key}")
    } else {
        format!(
            "{parent}[{}]",
            serde_json::to_string(key).expect("JSON string encoding")
        )
    }
}
