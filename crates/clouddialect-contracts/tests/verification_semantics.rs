use clouddialect_contracts::*;
use serde_json::{Value, json};

#[path = "support/documents.rs"]
mod documents;
#[path = "../src/validator/verification_semantics.rs"]
mod verification_semantics;

fn document() -> Value {
    let mut value = documents::minimal_documents()[2].1.clone();
    value["acceptance"] = json!({
        "approver_identities": ["untrusted-approval-claim"],
        "timestamp": "2026-10-23T12:34:56Z",
        "verification_result": {"id": value["id"], "format_version": value["format_version"]},
        "approval_evidence": value["candidate"]
    });
    value
}

fn check(value: &Value) -> Vec<ValidatorError> {
    let record: VerificationResult = serde_json::from_value(value.clone()).unwrap();
    verification_semantics::check_verification(&record)
}

fn has_error(value: &Value, path: &str) {
    let errors = check(value);
    assert!(
        errors.iter().any(|error| {
            error.category == ErrorCategory::Semantic
                && error.field_path.as_deref() == Some(path)
                && error.document_id.as_deref() == value["id"].as_str()
                && error.format_version.as_deref() == value["format_version"].as_str()
        }),
        "expected {path}, got {errors:?}"
    );
}

const HASH_PATHS: [&str; 10] = [
    "/candidate/content_hash",
    "/pack/content_hash",
    "/toolchain/content_hash",
    "/configuration/content_hash",
    "/binding/input/content_hash",
    "/binding/dependencies/content_hash",
    "/binding/suite/content_hash",
    "/binding/oracle/content_hash",
    "/binding/policy/content_hash",
    "/acceptance/approval_evidence/content_hash",
];

#[test]
fn all_declared_hash_algorithms_have_exact_canonical_lengths_in_every_binding() {
    for (algorithm, length) in [
        ("sha-256", 64),
        ("sha-384", 96),
        ("sha-512", 128),
        ("sha3-256", 64),
        ("sha3-512", 128),
    ] {
        let mut value = document();
        for path in HASH_PATHS {
            *value.pointer_mut(path).unwrap() =
                json!({"algorithm": algorithm, "value": "a".repeat(length)});
        }
        assert!(check(&value).is_empty(), "{algorithm}");
        for path in HASH_PATHS {
            let mut bad = value.clone();
            bad.pointer_mut(path).unwrap()["value"] = json!("a".repeat(length - 1));
            let field = format!("{}.value", path[1..].replace('/', "."));
            has_error(&bad, &field);
            bad.pointer_mut(path).unwrap()["value"] = json!("a".repeat(length + 1));
            has_error(&bad, &field);
        }
    }
}

#[test]
fn malformed_digest_encodings_and_unknown_algorithms_never_match_by_length_alone() {
    for path in HASH_PATHS {
        for encoding in [
            "".to_owned(),
            "A".repeat(64),
            "g".repeat(64),
            "a".repeat(63) + "\n",
            "é".repeat(32),
        ] {
            let mut value = document();
            value.pointer_mut(path).unwrap()["value"] = json!(encoding);
            has_error(&value, &format!("{}.value", path[1..].replace('/', ".")));
        }
        let mut value = document();
        value.pointer_mut(path).unwrap()["algorithm"] = json!("sha-257");
        has_error(
            &value,
            &format!("{}.algorithm", path[1..].replace('/', ".")),
        );
    }
}

#[test]
fn duplicate_test_ids_and_unmapped_obligations_are_reported_at_exact_indices() {
    let mut value = document();
    value["tests"].as_array_mut().unwrap().push(json!({
        "test_id": "t2", "obligation_ids": ["o1"],
        "outcome": "skip", "origin_identity": "not-authenticated",
        "failure_description": {"detail": "not run"}
    }));
    assert!(check(&value).is_empty());
    value["tests"][1]["test_id"] = json!("t1");
    has_error(&value, "tests[1].test_id");
    value["tests"][1]["obligation_ids"] = json!(["not-in-binding", "o1", "also-unmapped"]);
    has_error(&value, "tests[1].obligation_ids[0]");
    has_error(&value, "tests[1].obligation_ids[2]");
    value["tests"][1]["test_id"] = json!("T1");
    assert!(
        check(&value)
            .iter()
            .all(|error| error.field_path.as_deref() != Some("tests[1].test_id"))
    );
}

#[test]
fn acceptance_self_reference_requires_exact_identity_and_version_without_store_lookup() {
    let mut value = document();
    value["acceptance"]["verification_result"]["id"] =
        json!("550e8400-e29b-41d4-a716-446655440001");
    has_error(&value, "acceptance.verification_result.id");
    value["acceptance"]["verification_result"]["id"] = value["id"].clone();
    value["acceptance"]["verification_result"]["format_version"] = json!("1.0.0+other");
    has_error(&value, "acceptance.verification_result.format_version");
    value["format_version"] = json!("1.0.0+other");
    assert!(check(&value).is_empty());
}

#[test]
fn calendar_and_utc_z_are_required_even_if_the_active_schema_relaxes_timestamp_assertions() {
    for timestamp in [
        "2026-10-23T12:34:56+00:00",
        "2026-10-23T12:34:56-01:00",
        "2026-10-23T12:34:56z",
        "2026-10-23T12:34:56Z\n",
        "2023-02-29T12:34:56Z",
        "2026-04-31T12:34:56Z",
        "not-a-dateZ",
    ] {
        let mut value = document();
        value["acceptance"]["timestamp"] = json!(timestamp);
        has_error(&value, "acceptance.timestamp");
    }
    let mut value = document();
    value["acceptance"]["timestamp"] = json!("2024-02-29T23:59:59.123456789Z");
    assert!(check(&value).is_empty());
}

#[test]
fn valid_nonpassing_test_records_and_untrusted_authority_claims_do_not_change_integrity_rules() {
    for outcome in ["pass", "fail", "skip", "error"] {
        let mut value = document();
        value["tests"][0]["outcome"] = json!(outcome);
        if outcome != "pass" {
            value["tests"][0]["failure_description"] = json!({"detail": "safe detail"});
        }
        value["acme:accepted"] = json!(true);
        value["acme:test-overrides"] = json!({"t1": {"outcome": "pass"}});
        assert!(check(&value).is_empty());
        value["tests"][0]["obligation_ids"] = json!(["unmapped"]);
        has_error(&value, "tests[0].obligation_ids[0]");
    }
    let mut value = document();
    value.as_object_mut().unwrap().remove("acceptance");
    assert!(check(&value).is_empty());
}
