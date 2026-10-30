use std::cell::RefCell;

use clouddialect_contracts::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[path = "support/documents.rs"]
mod documents;

const EVIDENCE_ID: &str = "550e8400-e29b-41d4-a716-446655440001";

struct Store {
    contract: StoredDocument,
    evidence: Option<StoredDocument>,
    calls: RefCell<Vec<(String, Option<String>)>>,
}

impl DocumentStore for Store {
    fn get(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<Option<StoredDocument>, ValidatorError> {
        self.calls
            .borrow_mut()
            .push((id.to_owned(), version.map(str::to_owned)));
        if id == self.contract.reference.id {
            assert_eq!(
                version,
                Some(self.contract.reference.format_version.as_str())
            );
            Ok(Some(self.contract.clone()))
        } else {
            assert_eq!(id, EVIDENCE_ID);
            assert_eq!(version, None);
            Ok(self.evidence.clone())
        }
    }
}

fn stored(value: &Value) -> StoredDocument {
    let bytes = serde_json::to_vec(value).unwrap();
    StoredDocument {
        reference: DocumentRef {
            id: value["id"].as_str().unwrap().into(),
            format_version: value["format_version"].as_str().unwrap().into(),
        },
        content_hash: ContentHash {
            algorithm: "sha-256".into(),
            value: Sha256::digest(&bytes)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
        },
        bytes,
    }
}

fn store() -> Store {
    let mut contract = documents::minimal_documents()[0].1.clone();
    contract["obligations"] = json!([{
        "obligation_id":"o1", "behavioral_fidelity":"equivalent-within-contract",
        "evidence_refs":[EVIDENCE_ID]
    }]);
    let mut evidence = documents::minimal_documents()[1].1.clone();
    evidence["id"] = json!(EVIDENCE_ID);
    Store {
        contract: stored(&contract),
        evidence: Some(stored(&evidence)),
        calls: RefCell::new(Vec::new()),
    }
}

fn manifest(mode: &str) -> Vec<u8> {
    let source = include_str!("manifest_validation.rs");
    let literal = source
        .split("const MANIFEST: &str = r#\"")
        .nth(1)
        .unwrap()
        .split("\"#;")
        .next()
        .unwrap();
    let mut value: toml::Value = toml::from_str(literal).unwrap();
    value["contracts"]["upload_object"]["id"] = store().contract.reference.id.into();
    if mode == "orphan" {
        value["operations"] = toml::Value::Array(Vec::new());
    } else {
        value["operations"][0]["behavioral_fidelity"] = mode.into();
    }
    toml::to_string(&value).unwrap().into_bytes()
}

#[test]
fn every_manifest_binding_checks_evidence_without_refetching_the_contract() {
    let validator = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    for mode in ["full", "partial", "unsupported", "orphan"] {
        let bytes = manifest(mode);
        assert_eq!(
            validator.validate_manifest(&bytes).outcome,
            ManifestOutcome::Pass
        );
        let mut store = store();
        store.evidence = None;
        let result = validator.validate_manifest_semantic(&bytes, &store);
        assert_eq!(result.outcome, SemanticOutcome::Fail, "{mode}: {result:?}");
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.document_id.as_deref()
                    == Some(store.contract.reference.id.as_str())
                    && error.field_path.as_deref() == Some("obligations[0].evidence_refs[0]")),
            "{result:?}"
        );
        assert_eq!(
            *store.calls.borrow(),
            vec![
                (
                    store.contract.reference.id.clone(),
                    Some("1.0.0".to_owned())
                ),
                (EVIDENCE_ID.to_owned(), None)
            ]
        );
    }
}

#[test]
fn fidelity_helper_and_manifest_consume_the_same_validated_contract_evidence_boundary() {
    let snapshot = CatalogSnapshot::embedded().unwrap();
    let validator = Validator::new(snapshot.clone());
    let manifest_validator = ManifestValidator::new(snapshot);
    let bytes = manifest("full");
    let parsed: PackManifest = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
    let mut store = store();
    assert!(check_fidelity_mapping(&parsed, &store, &validator).is_empty());
    assert_eq!(
        manifest_validator
            .validate_manifest_semantic(&bytes, &store)
            .outcome,
        SemanticOutcome::Pass
    );
    store.evidence = None;
    assert!(!check_fidelity_mapping(&parsed, &store, &validator).is_empty());
}

#[test]
fn corrupt_or_mandatory_evidence_blocks_but_unknown_classification_is_not_a_reference_failure() {
    let validator = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    for mode in ["full", "partial", "unsupported", "orphan"] {
        for defect in ["hash", "mandatory", "unknown"] {
            let mut store = store();
            let evidence = store.evidence.as_mut().unwrap();
            if defect == "hash" {
                evidence.content_hash.value = "0".repeat(64);
            } else {
                let mut value: Value = serde_json::from_slice(&evidence.bytes).unwrap();
                if defect == "mandatory" {
                    value["required_extensions"] = json!(["acme"]);
                } else {
                    value["classification"] = json!("unknown");
                    value.as_object_mut().unwrap().remove("reasoning_chain");
                    value["unresolved_assumptions"] = json!([{"assumption_id":"a1","description":"unknown","reason":"not observed"}]);
                }
                *evidence = stored(&value);
            }
            let result = validator.validate_manifest_semantic(&manifest(mode), &store);
            assert_eq!(
                result.outcome,
                if defect == "unknown" {
                    SemanticOutcome::Pass
                } else {
                    SemanticOutcome::Fail
                },
                "{mode}/{defect}: {result:?}"
            );
        }
    }
}
