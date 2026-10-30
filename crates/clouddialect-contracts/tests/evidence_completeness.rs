use std::cell::Cell;

use clouddialect_contracts::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[path = "support/documents.rs"]
mod documents;

const EVIDENCE_ID: &str = "550e8400-e29b-41d4-a716-446655440001";
const CANDIDATE: &str = "caller-bound-candidate";

struct Store {
    record: Option<StoredDocument>,
    unavailable: bool,
    calls: Cell<usize>,
}

impl DocumentStore for Store {
    fn get(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<Option<StoredDocument>, ValidatorError> {
        self.calls.set(self.calls.get() + 1);
        assert_eq!(id, EVIDENCE_ID);
        assert_eq!(version, None);
        if self.unavailable {
            Err(ValidatorError {
                document_id: None,
                format_version: None,
                category: ErrorCategory::UnresolvableReference,
                field_path: None,
                description: "PRIVATE_STORE_DETAIL".into(),
                byte_offset: None,
            })
        } else {
            Ok(self.record.clone())
        }
    }
}

fn evidence() -> Value {
    let mut value = documents::minimal_documents()[1].1.clone();
    value["id"] = json!(EVIDENCE_ID);
    value
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
    Store {
        record: Some(stored(&evidence())),
        unavailable: false,
        calls: Cell::new(0),
    }
}

fn contract() -> BehavioralContract {
    let mut value = documents::minimal_documents()[0].1.clone();
    value["obligations"] = json!([
        {"obligation_id":"o1","behavioral_fidelity":"equivalent-within-contract","evidence_refs":[EVIDENCE_ID]},
        {"obligation_id":"o2","behavioral_fidelity":"adaptation-required","evidence_refs":[EVIDENCE_ID]}
    ]);
    serde_json::from_value(value).unwrap()
}

fn check(contract: &BehavioralContract, store: &Store) -> BehavioralResult {
    let validator = Validator::new(CatalogSnapshot::embedded().unwrap());
    let result = check_evidence_completeness(contract, CANDIDATE, store, &validator);
    assert_eq!(result.candidate_id, CANDIDATE);
    assert_eq!(result.verification_result, None);
    assert!(result.test_results.is_empty());
    assert!(
        result
            .blocking_gaps
            .iter()
            .all(|gap| gap.candidate_id == CANDIDATE)
    );
    let json = serde_json::to_value(&result).unwrap();
    assert!(json.get("accepted").is_none() && json.get("verified").is_none());
    result
}

#[test]
fn observed_and_inferred_support_can_be_evidence_complete_but_never_verification() {
    for classification in ["observed", "inferred"] {
        let mut value = evidence();
        if classification == "observed" {
            value["classification"] = json!(classification);
            value.as_object_mut().unwrap().remove("reasoning_chain");
            value["provenance"] = json!({"source_artifact_identity":"claim-only", "analysis_tool_identity":"claim-only"});
        }
        let mut store = store();
        store.record = Some(stored(&value));
        let result = check(&contract(), &store);
        assert_eq!(result.outcome, BehavioralOutcome::EvidenceComplete);
        assert!(result.blocking_gaps.is_empty());
        assert_eq!(store.calls.get(), 2);
    }
}

#[test]
fn direct_and_unknown_evidence_assumptions_identify_every_affected_obligation() {
    let mut contract = contract();
    contract.obligations[0].evidence_refs = None;
    contract.obligations[0].unresolved_assumption = Some(serde_json::from_value(json!({"assumption_id":"direct", "description":"ordering unknown", "reason":"not observed"})).unwrap());
    let mut value = evidence();
    value["classification"] = json!("unknown");
    value.as_object_mut().unwrap().remove("reasoning_chain");
    value["unresolved_assumptions"] = json!([
        {"assumption_id":"a1", "description":"retry unknown", "reason":"not measured"},
        {"assumption_id":"a2", "description":"failure unknown", "reason":"no fixture"}
    ]);
    let mut store = store();
    store.record = Some(stored(&value));
    let result = check(&contract, &store);
    assert_eq!(result.outcome, BehavioralOutcome::Blocked);
    assert_eq!(result.blocking_gaps.len(), 3);
    for (id, text) in [
        ("o1", "ordering unknown"),
        ("o2", "retry unknown"),
        ("o2", "failure unknown"),
    ] {
        assert!(
            result
                .blocking_gaps
                .iter()
                .any(|gap| gap.obligation_id == id && gap.assumption_text.contains(text)),
            "{result:?}"
        );
    }
}

#[test]
fn residual_assumptions_block_even_when_the_evidence_classification_is_known() {
    for classification in ["observed", "inferred"] {
        let mut value = evidence();
        if classification == "observed" {
            value["classification"] = json!(classification);
            value.as_object_mut().unwrap().remove("reasoning_chain");
            value["provenance"] =
                json!({"source_artifact_identity":"source", "analysis_tool_identity":"tool"});
        }
        value["unresolved_assumptions"] = json!([{"assumption_id":"residual", "description":"unknown retry", "reason":"partial observation"}]);
        let mut store = store();
        store.record = Some(stored(&value));
        let result = check(&contract(), &store);
        assert_eq!(result.outcome, BehavioralOutcome::Blocked);
        assert_eq!(result.blocking_gaps.len(), 2);
        assert!(
            result
                .blocking_gaps
                .iter()
                .all(|gap| gap.assumption_text.contains("unknown retry"))
        );
    }
}

#[test]
fn missing_corrupt_wrong_type_and_unavailable_records_are_named_blockers_not_completeness() {
    for defect in [
        "missing",
        "outage-or-ambiguous",
        "hash",
        "identity",
        "version",
        "type",
        "malformed",
    ] {
        let mut store = store();
        match defect {
            "missing" => store.record = None,
            "outage-or-ambiguous" => store.unavailable = true,
            "hash" => store.record.as_mut().unwrap().content_hash.value = "0".repeat(64),
            "identity" => store.record.as_mut().unwrap().reference.id = "wrong".into(),
            "version" => store.record.as_mut().unwrap().reference.format_version = "1.1.0".into(),
            "type" => {
                let mut value = serde_json::to_value(contract()).unwrap();
                value["id"] = json!(EVIDENCE_ID);
                store.record = Some(stored(&value));
            }
            "malformed" => {
                let record = store.record.as_mut().unwrap();
                record.bytes = b"{".to_vec();
                record.content_hash.value = Sha256::digest(&record.bytes)
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect();
            }
            _ => unreachable!(),
        }
        let result = check(&contract(), &store);
        assert_eq!(
            result.outcome,
            BehavioralOutcome::Blocked,
            "{defect}: {result:?}"
        );
        for id in ["o1", "o2"] {
            assert!(
                result
                    .blocking_gaps
                    .iter()
                    .any(|gap| gap.obligation_id == id),
                "{result:?}"
            );
        }
        assert!(!format!("{result:?}").contains("PRIVATE_STORE_DETAIL"));
    }
}

#[test]
fn invalid_source_is_deferred_without_invented_obligation_ids_or_store_queries() {
    for defect in ["unknown-version", "bad-id", "empty-evidence"] {
        let mut contract = contract();
        match defect {
            "unknown-version" => contract.format_version = "9.0.0".into(),
            "bad-id" => contract.id = "not-a-document-id".into(),
            "empty-evidence" => contract.obligations[0].evidence_refs = Some(vec![]),
            _ => unreachable!(),
        }
        let store = store();
        let result = check(&contract, &store);
        assert_eq!(
            result.outcome,
            BehavioralOutcome::Deferred,
            "{defect}: {result:?}"
        );
        assert!(result.blocking_gaps.is_empty());
        assert_eq!(store.calls.get(), 0);
    }
}

#[test]
fn valid_empty_scope_is_deferred_while_mandatory_semantics_remain_separate() {
    let mut contract = contract();
    contract.obligations.clear();
    let store = store();
    for required in [None, Some(vec!["acme".into()])] {
        contract.required_extensions = required;
        let result = check(&contract, &store);
        assert_eq!(result.outcome, BehavioralOutcome::Deferred);
        assert!(result.blocking_gaps.is_empty());
        assert_eq!(store.calls.get(), 0);
    }
    let validator = Validator::new(CatalogSnapshot::embedded().unwrap());
    let bytes = serde_json::to_vec(&contract).unwrap();
    assert_eq!(
        validator
            .validate_structural(DocumentKind::BehavioralContract, &bytes)
            .outcome,
        StructuralOutcome::Pass
    );
    let semantic = validator.validate_semantic(DocumentKind::BehavioralContract, &bytes, &store);
    assert_eq!(semantic.outcome, SemanticOutcome::Fail);
    assert!(
        semantic
            .errors
            .iter()
            .any(|e| e.category == ErrorCategory::BlockingGap)
    );
    assert_eq!(store.calls.get(), 0);
}

#[test]
fn each_resource_assumption_blocks_every_declared_obligation_even_with_partial_known_behavior() {
    let mut contract = contract();
    contract.resource_dependencies = serde_json::from_value(json!([{
        "identity":"resource", "permissions":[], "configuration":{},
        "known_behavior":{"ordering":"known", "consistency":"known", "retry":"partly known", "failure":"partly known"},
        "unresolved_assumptions":[
            {"behavior":"retry", "assumption_id":"r1", "description":"resource retry unknown", "reason":"partial"},
            {"behavior":"failure", "assumption_id":"r2", "description":"resource failure unknown", "reason":"partial"}
        ]
    }])).unwrap();
    let result = check(&contract, &store());
    assert_eq!(result.outcome, BehavioralOutcome::Blocked);
    assert_eq!(result.blocking_gaps.len(), 4);
    for id in ["o1", "o2"] {
        for assumption in ["resource retry unknown", "resource failure unknown"] {
            assert!(
                result
                    .blocking_gaps
                    .iter()
                    .any(|gap| gap.obligation_id == id && gap.assumption_text.contains(assumption)),
                "{result:?}"
            );
        }
    }
    contract.required_extensions = Some(vec!["acme".into()]);
    let store = store();
    let result = check(&contract, &store);
    assert_eq!(result.outcome, BehavioralOutcome::Blocked);
    assert_eq!(result.blocking_gaps.len(), 6);
    assert_eq!(store.calls.get(), 0);
}

#[test]
fn mandatory_root_and_evidence_namespaces_block_without_accepting_extension_overrides() {
    let mut contract = contract();
    contract.required_extensions = Some(vec!["acme".into()]);
    let store = store();
    let result = check(&contract, &store);
    assert_eq!(result.outcome, BehavioralOutcome::Blocked);
    assert_eq!(result.blocking_gaps.len(), 2);
    assert_eq!(store.calls.get(), 0);
    contract.required_extensions = None;
    let mut value = evidence();
    value["required_extensions"] = json!(["acme"]);
    value["acme:accepted"] = json!(true);
    let store = Store {
        record: Some(stored(&value)),
        unavailable: false,
        calls: Cell::new(0),
    };
    let result = check(&contract, &store);
    assert_eq!(result.outcome, BehavioralOutcome::Blocked);
    assert!(
        result
            .blocking_gaps
            .iter()
            .all(|gap| gap.assumption_text.contains("acme"))
    );
}

#[test]
fn active_catalog_context_is_required_and_opaque_extension_values_remain_untouched() {
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
    let path = "schemas/evidence-record/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
    schema["properties"]["claim"]["const"] = json!("strict-active-catalog");
    material.insert(path.into(), serde_json::to_vec(&schema).unwrap());
    let strict = Validator::new(
        CatalogSnapshot::from_parts(
            include_bytes!("../../../catalog/index.json").to_vec(),
            material,
        )
        .unwrap(),
    );
    let mut contract = contract();
    let opaque = r#"{ "u":"\u0061", "number":1e+04, "accepted":true }"#;
    contract
        .extensions
        .insert("acme:opaque".into(), serde_json::from_str(opaque).unwrap());
    let result = check_evidence_completeness(&contract, CANDIDATE, &store(), &strict);
    assert_eq!(result.outcome, BehavioralOutcome::Blocked);
    assert_eq!(contract.extensions["acme:opaque"].get(), opaque);
    assert!(result.verification_result.is_none() && result.test_results.is_empty());
}

#[test]
fn direct_assumptions_are_reported_even_when_a_root_mandatory_namespace_blocks() {
    let mut contract = contract();
    contract.required_extensions = Some(vec!["acme".into()]);
    for (index, obligation) in contract.obligations.iter_mut().enumerate() {
        obligation.evidence_refs = None;
        obligation.unresolved_assumption = Some(
            serde_json::from_value(json!({
                "assumption_id":format!("local-{index}"),
                "description":format!("direct uncertainty {index}"),
                "reason":"not observed"
            }))
            .unwrap(),
        );
    }
    let store = store();
    let result = check(&contract, &store);
    assert_eq!(result.outcome, BehavioralOutcome::Blocked);
    assert_eq!(result.blocking_gaps.len(), 4);
    assert_eq!(store.calls.get(), 0);
    for (index, obligation) in contract.obligations.iter().enumerate() {
        assert!(result.blocking_gaps.iter().any(|gap| {
            gap.obligation_id == obligation.obligation_id
                && gap.assumption_text == format!("direct uncertainty {index}")
        }));
    }
}
