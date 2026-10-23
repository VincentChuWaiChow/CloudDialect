use clouddialect_contracts::{
    AcceptanceRecord, BehavioralOutcome, BehavioralResult, BlockingGap, DocumentRef,
    FailureDescription, TestOutcome, TestResult, VerificationResult,
};
use serde_json::{Value, json};

fn artifact(identity: &str) -> Value {
    json!({"identity":identity,"content_hash":{"algorithm":"sha-256","value":"a".repeat(64)}})
}

fn verification() -> Value {
    json!({
        "id":"verification-1", "format_version":"1.0.0", "candidate":artifact("candidate"),
        "pack":{"id":"pack", "version":"1.2.3-rc.1+build.2", "content_hash":{"algorithm":"sha-256","value":"b".repeat(64)}},
        "toolchain":{"id":"toolchain", "version":"1.2.3+build.1", "content_hash":{"algorithm":"sha-256","value":"c".repeat(64)}},
        "configuration":artifact("configuration"),
        "binding":{"tenant_id":"tenant", "project_id":"project", "run_id":"run", "attempt_id":"attempt",
            "input":artifact("input"), "dependencies":artifact("dependencies"), "suite":artifact("suite"),
            "oracle":artifact("oracle"), "policy":artifact("policy"), "obligation_ids":["o1"]},
        "tests":[{"test_id":"test-1", "obligation_ids":["o1"], "outcome":"pass", "origin_identity":"verifier-claim"}]
    })
}

#[test]
fn verification_model_retains_every_binding_and_immutable_reference_field() {
    let value = verification();
    let result: VerificationResult = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(result.configuration.identity, "configuration");
    assert_eq!(result.binding.attempt_id, "attempt");
    assert_eq!(result.pack.version, "1.2.3-rc.1+build.2");
    assert_eq!(serde_json::to_value(result).unwrap(), value);
}

#[test]
fn required_verification_fields_are_not_silently_defaulted() {
    for key in [
        "id",
        "format_version",
        "candidate",
        "pack",
        "toolchain",
        "configuration",
        "binding",
        "tests",
    ] {
        let mut value = verification();
        value.as_object_mut().unwrap().remove(key);
        assert!(
            serde_json::from_value::<VerificationResult>(value).is_err(),
            "{key}"
        );
    }
    for key in [
        "tenant_id",
        "project_id",
        "run_id",
        "attempt_id",
        "input",
        "dependencies",
        "suite",
        "oracle",
        "policy",
        "obligation_ids",
    ] {
        let mut value = verification();
        value["binding"].as_object_mut().unwrap().remove(key);
        assert!(
            serde_json::from_value::<VerificationResult>(value).is_err(),
            "{key}"
        );
    }
    let mut value = verification();
    value["candidate"]
        .as_object_mut()
        .unwrap()
        .remove("content_hash");
    assert!(serde_json::from_value::<VerificationResult>(value).is_err());
}

#[test]
fn test_outcomes_and_failure_object_shape_match_the_proposed_contract() {
    for (outcome, label) in [
        (TestOutcome::Pass, "pass"),
        (TestOutcome::Fail, "fail"),
        (TestOutcome::Skip, "skip"),
        (TestOutcome::Error, "error"),
    ] {
        assert_eq!(serde_json::to_value(outcome).unwrap(), json!(label));
        assert_eq!(
            serde_json::from_value::<TestOutcome>(json!(label)).unwrap(),
            outcome
        );
    }
    assert!(serde_json::from_value::<TestOutcome>(json!("accepted")).is_err());
    for invalid in [
        json!("legacy string"),
        json!({"detail":"safe", "raw_stdout":"untrusted"}),
        json!({}),
    ] {
        assert!(serde_json::from_value::<FailureDescription>(invalid).is_err());
    }
    let test = TestResult {
        test_id: "failed".into(),
        obligation_ids: vec!["o1".into()],
        outcome: TestOutcome::Fail,
        origin_identity: "unverified".into(),
        failure_description: Some(FailureDescription {
            detail: "é".repeat(2000),
        }),
    };
    let value = serde_json::to_value(&test).unwrap();
    assert_eq!(serde_json::from_value::<TestResult>(value).unwrap(), test);
}

#[test]
fn completeness_result_has_only_the_three_non_accepting_outcomes() {
    for (outcome, label) in [
        (BehavioralOutcome::Deferred, "deferred"),
        (BehavioralOutcome::EvidenceComplete, "evidence-complete"),
        (BehavioralOutcome::Blocked, "blocked"),
    ] {
        let result = BehavioralResult {
            candidate_id: "candidate".into(),
            outcome,
            verification_result: None,
            test_results: vec![],
            blocking_gaps: vec![],
        };
        let value = json!({"candidate_id":"candidate", "outcome":label, "verification_result":null, "test_results":[], "blocking_gaps":[]});
        assert_eq!(serde_json::to_value(&result).unwrap(), value);
        assert_eq!(
            serde_json::from_value::<BehavioralResult>(value).unwrap(),
            result
        );
    }
    for label in ["accepted", "pass", "fail", "warn"] {
        assert!(serde_json::from_value::<BehavioralOutcome>(json!(label)).is_err());
    }
}

#[test]
fn projected_tests_and_gap_diagnostics_remain_exact_non_accepting_data() {
    let verified: VerificationResult = serde_json::from_value(verification()).unwrap();
    let result = BehavioralResult {
        candidate_id: verified.candidate.identity.clone(),
        outcome: BehavioralOutcome::Blocked,
        verification_result: Some(DocumentRef {
            id: verified.id.clone(),
            format_version: verified.format_version.clone(),
        }),
        test_results: verified.tests.clone(),
        blocking_gaps: vec![BlockingGap {
            candidate_id: "candidate".into(),
            obligation_id: "o1".into(),
            assumption_text: "origin not authenticated".into(),
        }],
    };
    assert_eq!(result.test_results, verified.tests);
    let value = serde_json::to_value(&result).unwrap();
    assert!(value.get("accepted").is_none());
    assert_eq!(
        serde_json::from_value::<BehavioralResult>(value).unwrap(),
        result
    );
    let _: clouddialect_contracts::errors::BehavioralResult = result;
}

#[test]
fn schema_selector_and_extension_values_survive_verification_round_trip() {
    let mut value = verification();
    value["$schema"] = json!("https://example.test/proposal/schema.json");
    value["acme:detail"] = json!({"claim":"untrusted", "values":[null,17]});
    let result: VerificationResult = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), value);
}

#[test]
fn approval_strings_and_unvalidated_projection_data_do_not_become_authority() {
    let mut value = verification();
    value["acceptance"] = json!({"approver_identities":["claimed-owner"], "timestamp":"unverified-date", "verification_result":{"id":"different-record","format_version":"1.0.0"},"approval_evidence":artifact("unverified-proof")});
    let result: VerificationResult = serde_json::from_value(value).unwrap();
    let approval: AcceptanceRecord = result.acceptance.unwrap();
    assert_ne!(approval.verification_result.id, result.id); // Later semantic/trust checks must reject.
    let empty: FailureDescription = serde_json::from_value(json!({"detail":""})).unwrap();
    assert!(empty.detail.is_empty()); // DTO parsing does not enforce schema length limits.
    let _: clouddialect_contracts::errors::TestOutcome = TestOutcome::Pass;
}

#[test]
fn verification_extension_values_keep_their_original_json_encoding() {
    let raw = r#"{ "z" : "\u0061", "a" : [1,  2] }"#;
    let mut value = verification();
    value["acme:encoded"] = json!("__RAW__");
    let input = serde_json::to_string(&value)
        .unwrap()
        .replace("\"__RAW__\"", raw);
    let document: VerificationResult = serde_json::from_slice(input.as_bytes()).unwrap();
    assert!(serde_json::to_string(&document).unwrap().contains(raw));
}
