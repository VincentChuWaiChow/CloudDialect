use std::cell::RefCell;

use clouddialect_contracts::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[path = "support/documents.rs"]
mod documents;

const EVIDENCE_ID: &str = "550e8400-e29b-41d4-a716-446655440001";

#[derive(Default)]
struct Store {
    record: Option<StoredDocument>,
    failure: bool,
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
        if self.failure {
            Err(ValidatorError {
                document_id: None,
                format_version: None,
                category: ErrorCategory::Semantic,
                field_path: None,
                description: "STORE_PRIVATE_DETAIL".into(),
                byte_offset: None,
            })
        } else {
            Ok(self.record.clone())
        }
    }
}

fn validator() -> Validator {
    Validator::new(CatalogSnapshot::embedded().unwrap())
}

fn changed_catalog(kind: &str, change: impl FnOnce(&mut Value)) -> CatalogSnapshot {
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
    let path = format!("schemas/{kind}/1.0/schema.json");
    let mut schema: Value = serde_json::from_slice(&material[&path]).unwrap();
    change(&mut schema);
    material.insert(path, serde_json::to_vec(&schema).unwrap());
    CatalogSnapshot::from_parts(
        include_bytes!("../../../catalog/index.json").to_vec(),
        material,
    )
    .unwrap()
}

fn evaluate(v: &Validator, kind: DocumentKind, value: &Value, store: &Store) -> SemanticResult {
    let api: &dyn Validate = v;
    api.validate_semantic(kind, &serde_json::to_vec(value).unwrap(), store)
}

fn stored(value: &Value) -> StoredDocument {
    let bytes = serde_json::to_vec(value).unwrap();
    StoredDocument {
        reference: DocumentRef {
            id: value["id"].as_str().unwrap().to_owned(),
            format_version: value["format_version"].as_str().unwrap().to_owned(),
        },
        content_hash: digest(&bytes),
        bytes,
    }
}

fn digest(bytes: &[u8]) -> ContentHash {
    ContentHash {
        algorithm: "sha-256".into(),
        value: Sha256::digest(bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
    }
}

fn evidence() -> Value {
    let mut value = documents::minimal_documents()[1].1.clone();
    value["id"] = json!(EVIDENCE_ID);
    value
}

fn contract_with_references() -> Value {
    let mut value = documents::minimal_documents()[0].1.clone();
    value["obligations"] = json!([{
        "obligation_id": "o1", "behavioral_fidelity": "equivalent-within-contract",
        "evidence_refs": [EVIDENCE_ID]
    }]);
    value["side_effects"] = json!([{"description": "writes", "evidence_ref": EVIDENCE_ID}]);
    value["behavioral_notes"] = json!([{
        "note_type": "service-specific", "description": "note", "evidence_ref": EVIDENCE_ID
    }]);
    value
}

#[test]
fn structural_failures_are_non_accepting_and_never_query_the_store() {
    let v = validator();
    let store = Store::default();
    for bytes in [b"not JSON".as_slice(), b"{}", b"null"] {
        let result = v.validate_semantic(DocumentKind::BehavioralContract, bytes, &store);
        assert_eq!(result.outcome, SemanticOutcome::Fail);
        assert!(!result.errors.is_empty());
    }
    let value = contract_with_references();
    assert_eq!(
        evaluate(&v, DocumentKind::EvidenceRecord, &value, &store).outcome,
        SemanticOutcome::Fail
    );
    assert!(store.calls.borrow().is_empty());
    // An explicit active catalog can differ from the bundled one, but a
    // permissive schema must not turn an undecodable core DTO into a pass.
    for (kind, name, field) in [
        (
            DocumentKind::BehavioralContract,
            "behavioral-contract",
            "obligations",
        ),
        (
            DocumentKind::VerificationResult,
            "verification-result",
            "tests",
        ),
    ] {
        let snapshot = changed_catalog(name, |schema| {
            schema["required"]
                .as_array_mut()
                .unwrap()
                .retain(|value| value != field);
        });
        let mut value = documents::minimal_documents()
            .into_iter()
            .find(|(k, _)| *k == kind)
            .unwrap()
            .1;
        value.as_object_mut().unwrap().remove(field);
        let result = evaluate(&Validator::new(snapshot), kind, &value, &store);
        assert_eq!(result.outcome, SemanticOutcome::Fail);
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.category == ErrorCategory::Semantic
                    && error.field_path.as_deref() == Some("$"))
        );
    }
}

#[test]
fn declared_core_evidence_references_resolve_uniquely_without_scanning_opaque_payloads() {
    let v = validator();
    let mut contract = contract_with_references();
    contract["acme:references"] = json!({"evidence_refs": ["not-a-real-reference"]});
    contract["inputs"] = json!({"evidence_ref": "opaque-schema-key"});
    let store = Store {
        record: Some(stored(&evidence())),
        ..Store::default()
    };
    let result = evaluate(&v, DocumentKind::BehavioralContract, &contract, &store);
    assert_eq!(result.outcome, SemanticOutcome::Pass, "{result:?}");
    assert!(result.errors.is_empty());
    assert_eq!(
        *store.calls.borrow(),
        vec![(EVIDENCE_ID.to_owned(), None); 3]
    );
    let output = serde_json::to_value(result).unwrap();
    for field in [
        "accepted",
        "verified",
        "test_results",
        "verification_result",
    ] {
        assert!(output.get(field).is_none());
    }
    let strict = Validator::new(changed_catalog("evidence-record", |schema| {
        schema["properties"]["claim"]["const"] = json!("active-catalog-only");
    }));
    let result = evaluate(&strict, DocumentKind::BehavioralContract, &contract, &store);
    assert_eq!(result.outcome, SemanticOutcome::Fail);
    assert!(
        result
            .errors
            .iter()
            .all(|error| error.document_id.as_deref() == Some(EVIDENCE_ID)
                && error.field_path.as_deref() == Some("claim"))
    );
}

#[test]
fn missing_ambiguous_or_unavailable_evidence_reports_every_referring_path_safely() {
    let v = validator();
    let contract = contract_with_references();
    for failure in [false, true] {
        let store = Store {
            failure,
            ..Store::default()
        };
        let result = evaluate(&v, DocumentKind::BehavioralContract, &contract, &store);
        assert_eq!(result.outcome, SemanticOutcome::Fail);
        for path in [
            "obligations[0].evidence_refs[0]",
            "side_effects[0].evidence_ref",
            "behavioral_notes[0].evidence_ref",
        ] {
            assert!(
                result.errors.iter().any(|error| {
                    error.field_path.as_deref() == Some(path)
                        && error.document_id.as_deref() == contract["id"].as_str()
                        && error.format_version.as_deref() == Some("1.0.0")
                        && error.category == ErrorCategory::UnresolvableReference
                        && error.description.contains(EVIDENCE_ID)
                }),
                "{result:?}"
            );
        }
        assert!(!format!("{result:?}").contains("STORE_PRIVATE_DETAIL"));
        assert_eq!(store.calls.borrow().len(), 3);
    }
}

#[test]
fn store_identity_type_hash_version_and_malformed_bytes_cannot_substitute_evidence() {
    let v = validator();
    let contract = contract_with_references();
    let original = stored(&evidence());
    let mut cases = Vec::new();
    let mut record = original.clone();
    record.reference.id = contract["id"].as_str().unwrap().to_owned();
    cases.push(record);
    let mut record = original.clone();
    record.content_hash.value = "0".repeat(64);
    cases.push(record);
    let mut record = original.clone();
    record.reference.format_version = "1.1.0".into();
    cases.push(record);
    let mut wrong_type = contract.clone();
    wrong_type["id"] = json!(EVIDENCE_ID);
    cases.push(stored(&wrong_type));
    let mut wrong_body = evidence();
    wrong_body["id"] = contract["id"].clone();
    let mut record = stored(&wrong_body);
    record.reference = original.reference.clone();
    cases.push(record);
    let mut mandatory = evidence();
    mandatory["required_extensions"] = json!(["acme"]);
    cases.push(stored(&mandatory));
    let mut record = original.clone();
    record.bytes = b"{malformed".to_vec();
    record.content_hash = digest(&record.bytes);
    cases.push(record);
    for record in cases {
        let result = evaluate(
            &v,
            DocumentKind::BehavioralContract,
            &contract,
            &Store {
                record: Some(record),
                ..Store::default()
            },
        );
        assert_eq!(result.outcome, SemanticOutcome::Fail, "{result:?}");
        assert!(!result.errors.is_empty());
        assert!(
            result
                .errors
                .iter()
                .all(|error| error.document_id.is_some())
        );
        assert!(
            result
                .errors
                .iter()
                .all(|error| error.format_version.is_some())
        );
    }
}

#[test]
fn unknown_evidence_and_explicit_assumptions_are_semantic_data_not_completeness_or_acceptance() {
    let v = validator();
    let mut unknown = evidence();
    unknown["classification"] = json!("unknown");
    unknown.as_object_mut().unwrap().remove("reasoning_chain");
    unknown["unresolved_assumptions"] =
        json!([{"assumption_id":"a1","description":"unknown","reason":"not observed"}]);
    let store = Store {
        record: Some(stored(&unknown)),
        ..Store::default()
    };
    assert_eq!(
        evaluate(
            &v,
            DocumentKind::BehavioralContract,
            &contract_with_references(),
            &store
        )
        .outcome,
        SemanticOutcome::Pass
    );
    for (kind, value) in documents::minimal_documents() {
        let result = evaluate(&v, kind, &value, &Store::default());
        assert_eq!(
            result.outcome,
            SemanticOutcome::Pass,
            "{kind:?}: {result:?}"
        );
    }
}

#[test]
fn unknown_mandatory_extensions_block_all_five_kinds_with_or_without_payloads() {
    let v = validator();
    for (kind, mut value) in documents::minimal_documents() {
        value["required_extensions"] = json!(["acme"]);
        if kind == DocumentKind::BehavioralContract {
            // The schema permits an empty obligation scope. The semantic
            // mandatory blocker must not depend on producing obligation gaps.
            value["obligations"] = json!([]);
        }
        assert_ne!(
            v.validate_structural(kind, &serde_json::to_vec(&value).unwrap())
                .outcome,
            StructuralOutcome::Fail
        );
        for with_payload in [false, true] {
            if with_payload {
                value["acme:accepted"] = json!(true);
                if kind == DocumentKind::ExtensionRegistry {
                    value["extensions"] = json!([{
                        "namespace_prefix":"acme", "owner":"claimed owner", "purpose":"claims trust",
                        "registration_date":"2026-10-23"
                    }]);
                }
            }
            let store = Store::default();
            let result = evaluate(&v, kind, &value, &store);
            assert_eq!(result.outcome, SemanticOutcome::Fail);
            assert!(
                result
                    .errors
                    .iter()
                    .any(|error| error.category == ErrorCategory::BlockingGap
                        && error.field_path.as_deref() == Some("required_extensions[0]"))
            );
            assert!(store.calls.borrow().is_empty());
        }
        value["required_extensions"] = json!([]);
        assert_eq!(
            evaluate(&v, kind, &value, &Store::default()).outcome,
            SemanticOutcome::Pass
        );
    }
}

#[test]
fn verification_rules_are_enforced_by_the_public_method_and_cannot_be_overridden_by_extensions() {
    let v = validator();
    for outcome in ["pass", "fail", "skip", "error"] {
        let mut value = documents::minimal_documents()[2].1.clone();
        value["tests"][0]["outcome"] = json!(outcome);
        if outcome != "pass" {
            value["tests"][0]["failure_description"] = json!({"detail":"recorded, not accepted"});
        }
        assert_eq!(
            evaluate(
                &v,
                DocumentKind::VerificationResult,
                &value,
                &Store::default()
            )
            .outcome,
            SemanticOutcome::Pass
        );
    }
    let mut value = documents::minimal_documents()[2].1.clone();
    let duplicate = value["tests"][0].clone();
    value["tests"].as_array_mut().unwrap().push(duplicate);
    value["tests"][1]["obligation_ids"] = json!(["unmapped"]);
    value["candidate"]["content_hash"]["value"] = json!("a");
    value["acceptance"] = json!({
        "approver_identities":["claim"], "timestamp":"2026-10-23T12:34:56Z",
        "verification_result":{"id":"550e8400-e29b-41d4-a716-446655440002","format_version":"1.0.0+other"},
        "approval_evidence":value["configuration"]
    });
    value["acme:accepted"] = json!(true);
    assert_ne!(
        v.validate_structural(
            DocumentKind::VerificationResult,
            &serde_json::to_vec(&value).unwrap()
        )
        .outcome,
        StructuralOutcome::Fail
    );
    let result = evaluate(
        &v,
        DocumentKind::VerificationResult,
        &value,
        &Store::default(),
    );
    assert_eq!(result.outcome, SemanticOutcome::Fail);
    for path in [
        "tests[1].test_id",
        "tests[1].obligation_ids[0]",
        "candidate.content_hash.value",
        "acceptance.verification_result.id",
        "acceptance.verification_result.format_version",
    ] {
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.field_path.as_deref() == Some(path)),
            "{result:?}"
        );
    }
}

#[test]
fn semantic_calendar_and_utc_validation_uses_real_schema_assertions_not_only_z_suffix() {
    let v = validator();
    let mut value = documents::minimal_documents()[2].1.clone();
    value["acceptance"] = json!({
        "approver_identities":["claim"], "timestamp":"2024-02-29T12:34:56.123456789Z",
        "verification_result":{"id":value["id"],"format_version":value["format_version"]},
        "approval_evidence":value["configuration"]
    });
    assert_eq!(
        evaluate(
            &v,
            DocumentKind::VerificationResult,
            &value,
            &Store::default()
        )
        .outcome,
        SemanticOutcome::Pass
    );
    for timestamp in [
        "2023-02-29T12:34:56Z",
        "2026-04-31T12:34:56Z",
        "2026-10-23T25:34:56Z",
        "2026-10-23T12:34:56+00:00",
        "not-a-dateZ",
        "2026-10-23T12:34:56Z\n",
    ] {
        value["acceptance"]["timestamp"] = json!(timestamp);
        let result = evaluate(
            &v,
            DocumentKind::VerificationResult,
            &value,
            &Store::default(),
        );
        assert_eq!(
            result.outcome,
            SemanticOutcome::Fail,
            "{timestamp}: {result:?}"
        );
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.field_path.as_deref() == Some("acceptance.timestamp")),
            "{result:?}"
        );
    }
    let relaxed = Validator::new(changed_catalog("verification-result", |schema| {
        schema["properties"]["acceptance"]["properties"]["timestamp"] = json!({"type":"string"});
    }));
    value["acceptance"]["timestamp"] = json!("2026-02-30T12:34:56Z");
    assert_ne!(
        relaxed
            .validate_structural(
                DocumentKind::VerificationResult,
                &serde_json::to_vec(&value).unwrap()
            )
            .outcome,
        StructuralOutcome::Fail
    );
    let result = evaluate(
        &relaxed,
        DocumentKind::VerificationResult,
        &value,
        &Store::default(),
    );
    assert_eq!(result.outcome, SemanticOutcome::Fail);
    assert!(
        result
            .errors
            .iter()
            .any(|error| error.category == ErrorCategory::Semantic
                && error.field_path.as_deref() == Some("acceptance.timestamp"))
    );
}
