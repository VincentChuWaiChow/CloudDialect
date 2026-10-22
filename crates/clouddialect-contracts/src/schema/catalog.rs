//! Catalog inventory/identity checks. This is neither provenance nor a schema resolver.

use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet},
    error::Error,
    io,
    rc::Rc,
};

use boon::{CompileError, Compiler, Schemas, UrlLoader};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    DocumentKind, ErrorCategory, RawObject, ValidatorError,
    documents::raw_objects::{RawFields, raw_object_model},
};

const INDEX_URI: &str = "https://schemas.clouddialect.io/catalog-index/1.0.0/schema.json";
const INDEX_SCHEMA: &[u8] =
    include_bytes!("../../../../catalog/schemas/catalog-index/1.0/schema.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct CatalogIndex {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub format_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_extensions: Option<Vec<String>>,
    pub schemas: Vec<CatalogEntry>,
    pub resolver_resources: Vec<ResolverResource>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

raw_object_model!(
    CatalogIndex,
    [format_version, schemas, resolver_resources],
    [schema => "$schema", required_extensions => "required_extensions"]
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub schema_id: String,
    pub document_type: DocumentKind,
    pub format_version: String,
    pub catalog_path: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_life_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverResource {
    pub schema_id: String,
    pub catalog_path: String,
}

// Bootstrap trusts the compiled-in index schema, never a selector in input bytes.
struct IndexSchemaLoader;

impl UrlLoader for IndexSchemaLoader {
    fn load(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        if url != INDEX_URI {
            return Err(
                io::Error::new(io::ErrorKind::NotFound, "unlisted bootstrap schema").into(),
            );
        }
        Ok(serde_json::from_slice(INDEX_SCHEMA)?)
    }
}

impl CatalogIndex {
    /// Checks index shape and exact URI/path/material inventory from caller-owned bytes.
    /// Does not acquire files, authenticate material or freeze/digest it. The
    /// pinned engine probes resource identities, not schema validity.
    pub fn from_parts(
        index: &[u8],
        material: &BTreeMap<String, Vec<u8>>,
    ) -> Result<Self, ValidatorError> {
        let value = parse_json(index)?;
        let mut compiler = Compiler::new();
        compiler.use_loader(Box::new(IndexSchemaLoader));
        compiler.enable_format_assertions();
        let mut schemas = Schemas::new();
        let root = compiler
            .compile(INDEX_URI, &mut schemas)
            .map_err(|_| invalid("Bundled catalog-index schema could not compile"))?;
        schemas
            .validate(&value, root)
            .map_err(|_| invalid("Catalog index does not match its schema"))?;
        // Decode original members so duplicate core declarations cannot be
        // erased by the schema-checking Value before typed conversion.
        let parsed: Self = serde_json::from_slice(index)
            .map_err(|_| invalid("Catalog index fields could not decode"))?;
        if parsed.format_version != "1.0.0" {
            let mut error = invalid("Unsupported catalog index version");
            error.category = ErrorCategory::UnsupportedVersion;
            return Err(error);
        }
        if parsed.schema.as_deref().is_some_and(|uri| uri != INDEX_URI) {
            return Err(invalid(
                "Catalog index schema selector does not match its format",
            ));
        }
        for path in material.keys() {
            check_path(path)?;
        }
        let mut uris = BTreeSet::new();
        let mut paths = BTreeSet::new();
        let mut kinds = BTreeSet::new();
        for entry in &parsed.schemas {
            let kind = kind_name(entry.document_type);
            let mut components = entry.format_version.split('.');
            // The trusted JSON Schema has already checked the full SemVer grammar.
            let directory = format!(
                "{}.{}",
                components.next().unwrap(),
                components.next().unwrap()
            );
            if entry.schema_id
                != format!(
                    "https://schemas.clouddialect.io/{kind}/{}/schema.json",
                    entry.format_version
                )
                || entry.catalog_path != format!("schemas/{kind}/{directory}/schema.json")
            {
                return Err(invalid(
                    "Application schema identity, kind, version and path disagree",
                ));
            }
            check_resource(
                &entry.schema_id,
                &entry.catalog_path,
                material,
                &mut uris,
                &mut paths,
                true,
            )?;
            kinds.insert(kind);
        }
        if kinds.len() != 5 {
            return Err(invalid(
                "Catalog must represent all five application document kinds",
            ));
        }
        for entry in &parsed.resolver_resources {
            check_resource(
                &entry.schema_id,
                &entry.catalog_path,
                material,
                &mut uris,
                &mut paths,
                false,
            )?;
            check_external_resource_uri(&entry.schema_id)?;
        }
        check_resource_identities(&parsed, material)?;
        Ok(parsed)
    }
}

struct IdentityProbeLoader(Rc<BTreeMap<String, Value>>);

impl UrlLoader for IdentityProbeLoader {
    fn load(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        self.0.get(url).cloned().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Unlisted identity probe resource").into()
        })
    }
}

fn check_resource_identities(
    index: &CatalogIndex,
    material: &BTreeMap<String, Vec<u8>>,
) -> Result<(), ValidatorError> {
    let resources = Rc::new(
        index
            .schemas
            .iter()
            .map(|entry| (&entry.schema_id, &entry.catalog_path))
            .chain(
                index
                    .resolver_resources
                    .iter()
                    .map(|entry| (&entry.schema_id, &entry.catalog_path)),
            )
            .map(|(uri, path)| Ok((uri.clone(), parse_json(&material[path])?)))
            .collect::<Result<BTreeMap<_, _>, ValidatorError>>()?,
    );
    for (uri, original) in resources.iter() {
        let mut probe = original.clone();
        let Some(object) = probe.as_object_mut() else {
            continue;
        };
        // Temporary siblings make the pinned engine detect indexed/nested ID
        // collisions using its own draft traversal and normalized URL joining.
        // `definitions` is a schema container in every draft Boon supports;
        // IDs in instance data (e.g. const/examples) are not recursively scanned.
        let definitions = object
            .entry("definitions")
            .or_insert_with(|| Value::Object(Default::default()));
        if !definitions.is_object() {
            // The engine finds no resources in a non-object container. Changing
            // only the probe lets it inspect IDs elsewhere even in invalid schemas.
            *definitions = Value::Object(Default::default());
        }
        let definitions = definitions.as_object_mut().unwrap();
        for (position, other_uri) in resources.keys().filter(|id| *id != uri).enumerate() {
            let mut key = format!("__clouddialect_identity_probe_{position}");
            while definitions.contains_key(&key) {
                key.push('_');
            }
            definitions.insert(key, serde_json::json!({"$id": other_uri, "id": other_uri}));
        }
        let mut compiler = Compiler::new();
        compiler.use_loader(Box::new(IdentityProbeLoader(Rc::clone(&resources))));
        compiler
            .add_resource(uri, probe)
            .map_err(|_| invalid("Catalog resource identity could not be probed"))?;
        // Other compilation errors retain the existing lazy schema-validity
        // boundary. Probe copies/compiled output never become snapshot material.
        if matches!(
            compiler.compile(uri, &mut Schemas::new()),
            Err(CompileError::DuplicateId { .. })
        ) {
            return Err(invalid(
                "Catalog contains a conflicting schema resource identity",
            ));
        }
    }
    Ok(())
}

// Boon normalizes resource URLs and resolves its bundled meta-schemas before
// calling UrlLoader. Do not admit an exact mapping it would silently bypass.
struct ExactResourceProbe {
    expected: String,
    reached: Rc<Cell<bool>>,
}

impl UrlLoader for ExactResourceProbe {
    fn load(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        self.reached.set(url == self.expected);
        Err(io::Error::new(io::ErrorKind::NotFound, "Resource probe denies acquisition").into())
    }
}

fn check_external_resource_uri(uri: &str) -> Result<(), ValidatorError> {
    let reached = Rc::new(Cell::new(false));
    let mut compiler = Compiler::new();
    compiler.use_loader(Box::new(ExactResourceProbe {
        expected: uri.to_owned(),
        reached: Rc::clone(&reached),
    }));
    // Only engine-owned built-ins can load here. The probe never returns or
    // compiles caller-supplied content, and never opens files or network URLs.
    let _ = compiler.compile(uri, &mut Schemas::new());
    if !reached.get() {
        return Err(invalid(
            "Resolver URI cannot be dispatched as an exact external resource",
        ));
    }
    Ok(())
}

fn kind_name(kind: DocumentKind) -> &'static str {
    match kind {
        DocumentKind::BehavioralContract => "behavioral-contract",
        DocumentKind::EvidenceRecord => "evidence-record",
        DocumentKind::VerificationResult => "verification-result",
        DocumentKind::CatalogIndex => "catalog-index",
        DocumentKind::ExtensionRegistry => "extension-registry",
    }
}

fn check_resource<'a>(
    uri: &'a str,
    path: &'a str,
    material: &BTreeMap<String, Vec<u8>>,
    uris: &mut BTreeSet<&'a str>,
    paths: &mut BTreeSet<&'a str>,
    application: bool,
) -> Result<(), ValidatorError> {
    check_path(path)?;
    if !uris.insert(uri) || !paths.insert(path) {
        return Err(invalid(
            "Catalog contains a duplicate URI or aliased material path",
        ));
    }
    let bytes = material
        .get(path)
        .ok_or_else(|| invalid("Catalog material is missing"))?;
    let value = parse_json(bytes)?;
    if !(value.is_object() || value.is_boolean()) {
        return Err(invalid(
            "Catalog schema material must be an object or boolean",
        ));
    }
    let id = value.get("$id");
    if (application || id.is_some()) && id.and_then(Value::as_str) != Some(uri) {
        return Err(invalid(
            "Catalog material schema identity does not match its entry",
        ));
    }
    Ok(())
}

fn check_path(path: &str) -> Result<(), ValidatorError> {
    if path == "index.json"
        || path.contains(['\\', ':', '\0'])
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(invalid(
            "Catalog material path is not a canonical relative path",
        ));
    }
    Ok(())
}

fn parse_json(bytes: &[u8]) -> Result<Value, ValidatorError> {
    serde_json::from_slice(bytes).map_err(|error| {
        let line_start = bytes
            .split_inclusive(|byte| *byte == b'\n')
            .take(error.line().saturating_sub(1))
            .map(<[u8]>::len)
            .sum::<usize>();
        ValidatorError {
            document_id: None,
            format_version: None,
            category: ErrorCategory::Unparseable,
            field_path: None,
            description: "Catalog JSON could not parse".to_owned(),
            byte_offset: Some((line_start + error.column().saturating_sub(1)).min(bytes.len())),
        }
    })
}

fn invalid(description: &str) -> ValidatorError {
    ValidatorError {
        document_id: None,
        format_version: None,
        category: ErrorCategory::Structural,
        field_path: None,
        description: description.to_owned(),
        byte_offset: None,
    }
}
