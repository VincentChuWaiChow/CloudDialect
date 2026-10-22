use clouddialect_contracts::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::cell::Cell;
#[path = "support/documents.rs"]
mod documents;

struct Missing(Cell<usize>);
impl DocumentStore for Missing {
    fn get(&self, _: &str, _: Option<&str>) -> Result<Option<StoredDocument>, ValidatorError> {
        self.0.set(self.0.get() + 1);
        Ok(None)
    }
}

fn input() -> String {
    // Reuse the actual valid fixture literal, not a second divergent manifest.
    let source = include_str!("manifest_validation.rs");
    source
        .split("const MANIFEST: &str = r#\"")
        .nth(1)
        .unwrap()
        .split("\"#;")
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn local_pass_missing_contract_semantic_fail_and_trait_dispatch_remain_separate() {
    let v = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    let data = input();
    let store = Missing(Cell::new(0));
    assert_eq!(
        v.validate_manifest(data.as_bytes()).outcome,
        ManifestOutcome::Pass
    );
    let checker: &dyn ValidateManifest = &v;
    let semantic = checker.validate_manifest_semantic(data.as_bytes(), &store);
    assert_eq!(semantic.outcome, SemanticOutcome::Fail);
    assert!(
        semantic
            .errors
            .iter()
            .any(|e| e.category == ErrorCategory::UnresolvableReference
                && e.field_path.as_deref() == Some("contracts.upload_object.id"))
    );
    assert_eq!(store.0.get(), 1);
    assert!(
        serde_json::to_value(semantic)
            .unwrap()
            .get("accepted")
            .is_none()
    );
}

#[test]
fn invalid_local_manifest_never_queries_store() {
    let v = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    let store = Missing(Cell::new(0));
    assert_eq!(
        v.validate_manifest_semantic(b"not toml", &store).outcome,
        SemanticOutcome::Fail
    );
    assert_eq!(store.0.get(), 0);
}

#[test]
fn unsupported_without_contracts_is_semantic_pass_not_authority() {
    let v = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    let store = Missing(Cell::new(0));
    let mut doc: toml::Value = toml::from_str(&input()).unwrap();
    doc["operations"][0]["behavioral_fidelity"] = "unsupported".into();
    doc["contracts"] = toml::Value::Table(toml::map::Map::new());
    let bytes = toml::to_string(&doc).unwrap();
    let result = v.validate_manifest_semantic(bytes.as_bytes(), &store);
    assert_eq!(result.outcome, SemanticOutcome::Pass);
    assert!(result.errors.is_empty());
    assert_eq!(store.0.get(), 0);
}

struct Record(StoredDocument);
impl DocumentStore for Record {
    fn get(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<Option<StoredDocument>, ValidatorError> {
        assert_eq!(id, self.0.reference.id);
        assert_eq!(version, Some("1.0.0"));
        Ok(Some(self.0.clone()))
    }
}
fn valid_record() -> (String, Record) {
    let mut value = documents::minimal_documents()[0].1.clone();
    value["obligations"] = json!([{"obligation_id":"o1","behavioral_fidelity":"equivalent-within-contract","evidence_refs":[value["id"]]}]);
    let bytes = serde_json::to_vec(&value).unwrap();
    let id = value["id"].as_str().unwrap().to_owned();
    let mut manifest: toml::Value = toml::from_str(&input()).unwrap();
    manifest["contracts"]["upload_object"]["id"] = id.clone().into();
    let record = StoredDocument {
        reference: DocumentRef {
            id,
            format_version: "1.0.0".into(),
        },
        content_hash: ContentHash {
            algorithm: "sha-256".into(),
            value: Sha256::digest(&bytes)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect(),
        },
        bytes,
    };
    (toml::to_string(&manifest).unwrap(), Record(record))
}

#[test]
fn valid_stored_contract_passes_real_semantics_but_explicit_unsupported_refs_still_resolve() {
    let v = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    let (data, store) = valid_record();
    assert_eq!(
        v.validate_manifest_semantic(data.as_bytes(), &store)
            .outcome,
        SemanticOutcome::Pass
    );
    for mode in ["full", "partial"] {
        let mut bad: Value = serde_json::from_slice(&store.0.bytes).unwrap();
        bad["obligations"][0]["behavioral_fidelity"] = json!("architectural-change-required");
        let mut record = store.0.clone();
        record.bytes = serde_json::to_vec(&bad).unwrap();
        record.content_hash.value = Sha256::digest(&record.bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        let input = data.replace(
            "behavioral_fidelity = \"full\"",
            &format!("behavioral_fidelity = \"{mode}\""),
        );
        let result = v.validate_manifest_semantic(input.as_bytes(), &Record(record));
        assert_eq!(result.outcome, SemanticOutcome::Fail);
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.category == ErrorCategory::Semantic)
        );
    }
    let unsupported = data.replace(
        "behavioral_fidelity = \"full\"",
        "behavioral_fidelity = \"unsupported\"",
    );
    let missing = Missing(Cell::new(0));
    assert_eq!(
        v.validate_manifest_semantic(unsupported.as_bytes(), &missing)
            .outcome,
        SemanticOutcome::Fail
    );
    assert_eq!(missing.0.get(), 1);
}

#[test]
fn semantic_entrypoint_uses_its_owned_active_catalog() {
    let base = CatalogSnapshot::embedded().unwrap();
    let mut material: std::collections::BTreeMap<String, Vec<u8>> = [
        "behavioral-contract",
        "evidence-record",
        "verification-result",
        "catalog-index",
        "extension-registry",
    ]
    .into_iter()
    .map(|kind| {
        (
            format!("schemas/{kind}/1.0/schema.json"),
            base.schema_bytes(&format!(
                "https://schemas.clouddialect.io/{kind}/1.0.0/schema.json"
            ))
            .unwrap()
            .to_vec(),
        )
    })
    .collect();
    let path = "schemas/behavioral-contract/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
    schema["properties"]["operation_identity"]["properties"]["call_site"]["const"] =
        json!("active-catalog-only");
    material.insert(path.into(), serde_json::to_vec(&schema).unwrap());
    let strict = CatalogSnapshot::from_parts(
        include_bytes!("../../../catalog/index.json").to_vec(),
        material,
    )
    .unwrap();
    let (data, store) = valid_record();
    assert_eq!(
        ManifestValidator::new(base)
            .validate_manifest_semantic(data.as_bytes(), &store)
            .outcome,
        SemanticOutcome::Pass
    );
    assert_eq!(
        ManifestValidator::new(strict)
            .validate_manifest_semantic(data.as_bytes(), &store)
            .outcome,
        SemanticOutcome::Fail
    );
}
