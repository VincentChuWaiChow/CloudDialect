use clouddialect_contracts::{
    BehavioralContract, CatalogSnapshot, DocumentKind, ErrorCategory, StructuralOutcome, Validator,
};
use serde_json::{Value, json};

#[path = "support/documents.rs"]
mod documents;

fn validate(kind: DocumentKind, bytes: &[u8]) -> clouddialect_contracts::StructuralResult {
    Validator::new(CatalogSnapshot::embedded().unwrap()).validate_structural(kind, bytes)
}

fn duplicate(value: &Value, field: &str, encoded_value: &str) -> Vec<u8> {
    let original = serde_json::to_string(value).unwrap();
    let member = format!("\"{field}\":{encoded_value}");
    assert!(original.contains(&member), "fixture member must exist");
    original
        .replacen(&member, &format!("{member},{member}"), 1)
        .into_bytes()
}

#[test]
fn nested_core_duplicates_are_rejected_for_all_five_document_kinds() {
    let mut cases = documents::minimal_documents();
    cases[1].1["classification"] = json!("observed");
    cases[1]
        .1
        .as_object_mut()
        .unwrap()
        .remove("reasoning_chain");
    cases[1].1["provenance"] =
        json!({"source_artifact_identity":"source","analysis_tool_identity":"tool"});
    cases[4].1["extensions"] = json!([{"namespace_prefix":"acme","owner":"owner","purpose":"purpose","registration_date":"2026-10-22"}]);
    for (index, field, value) in [
        (0, "call_site", "\"pkg.call\""),
        (1, "source_artifact_identity", "\"source\""),
        (2, "algorithm", "\"sha-256\""),
        (3, "document_type", "\"behavioral-contract\""),
        (4, "owner", "\"owner\""),
    ] {
        let (kind, document) = &cases[index];
        assert_eq!(
            validate(*kind, &serde_json::to_vec(document).unwrap()).outcome,
            StructuralOutcome::Pass
        );
        let bytes = duplicate(document, field, value);
        let result = validate(*kind, &bytes);
        assert_eq!(
            result.outcome,
            StructuralOutcome::Fail,
            "{kind:?}: ambiguous nested member passed"
        );
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.category == ErrorCategory::Unparseable)
        );
    }
}

#[test]
fn escaped_duplicate_names_and_duplicates_in_resource_and_test_arrays_fail() {
    let mut contract = documents::minimal_documents()[0].1.clone();
    contract["resource_dependencies"] = json!([{"identity":"bucket","permissions":[],"configuration":{},"known_behavior":{"ordering":"known","consistency":"known","retry":"known","failure":"known"},"unresolved_assumptions":[{"behavior":"ordering","assumption_id":"a1","description":"unknown","reason":"unobserved"}]}]);
    let bytes = serde_json::to_string(&contract).unwrap().replace(
        "\"call_site\":\"pkg.call\"",
        "\"call_site\":\"first\",\"call_\\u0073ite\":\"pkg.call\"",
    );
    assert!(serde_json::from_slice::<BehavioralContract>(bytes.as_bytes()).is_err());
    assert_eq!(
        validate(DocumentKind::BehavioralContract, bytes.as_bytes()).outcome,
        StructuralOutcome::Fail
    );
    for (field, value) in [
        ("behavior", "\"ordering\""),
        ("ordering", "\"known\""),
        ("assumption_id", "\"a1\""),
    ] {
        assert_eq!(
            validate(
                DocumentKind::BehavioralContract,
                &duplicate(&contract, field, value)
            )
            .outcome,
            StructuralOutcome::Fail
        );
    }
    let verification = &documents::minimal_documents()[2].1;
    for (field, value) in [("outcome", "\"pass\""), ("tenant_id", "\"tenant\"")] {
        assert_eq!(
            validate(
                DocumentKind::VerificationResult,
                &duplicate(verification, field, value)
            )
            .outcome,
            StructuralOutcome::Fail
        );
    }
}

#[test]
fn opaque_extension_values_and_schema_fragments_are_not_reinterpreted() {
    let mut contract = serde_json::to_string(&documents::minimal_documents()[0].1).unwrap();
    contract.pop();
    let opaque = r#"{ "same":1, "same":2, "nested": {"id":1,"id":2} }"#;
    contract.push_str(&format!(",\"acme:opaque\":{opaque}}}"));
    let decoded: BehavioralContract = serde_json::from_str(&contract).unwrap();
    assert_eq!(decoded.extensions["acme:opaque"].get(), opaque);
    assert_eq!(
        validate(DocumentKind::BehavioralContract, contract.as_bytes()).outcome,
        StructuralOutcome::Pass
    );
    assert_eq!(decoded.extensions["acme:opaque"].get(), opaque);
    let document = contract.replace("\"inputs\":{}", &format!("\"inputs\":{opaque}"));
    assert_eq!(
        validate(DocumentKind::BehavioralContract, document.as_bytes()).outcome,
        StructuralOutcome::Pass
    );
    let mut evidence = serde_json::to_string(&documents::minimal_documents()[1].1).unwrap();
    evidence.pop();
    evidence.push_str(&format!(",\"format_addition\":{opaque}}}"));
    evidence = evidence.replace("\"1.0.0\"", "\"1.1.0\"");
    assert_eq!(
        validate(DocumentKind::EvidenceRecord, evidence.as_bytes()).outcome,
        StructuralOutcome::Warn
    );
}
