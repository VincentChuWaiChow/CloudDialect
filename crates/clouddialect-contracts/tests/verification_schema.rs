mod support;

use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use support::SchemaHarness;

fn harness() -> SchemaHarness {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/schemas/verification-result/1.0/schema.json");
    assert!(
        path.is_file(),
        "required verification-result schema is not delivered"
    );
    let bytes = std::fs::read(path).unwrap();
    let schema: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        schema["$id"],
        "https://schemas.clouddialect.io/verification-result/1.0.0/schema.json"
    );
    SchemaHarness::new(
        BTreeMap::from([(schema["$id"].as_str().unwrap().into(), bytes)]),
        schema["$id"].as_str().unwrap(),
        true,
    )
    .unwrap()
}
fn valid(h: &SchemaHarness, v: &Value) -> bool {
    h.validate_json(&serde_json::to_vec(v).unwrap()).unwrap()
}
fn artifact() -> Value {
    json!({"identity":"immutable-artifact","content_hash":{"algorithm":"sha-256","value":"a".repeat(64)}})
}
fn verification() -> Value {
    json!({
    "id":"550e8400-e29b-41d4-a716-446655440000","format_version":"1.0.0","candidate":artifact(),"configuration":artifact(),
    "pack":{"id":"io.example.pack","version":"1.2.3-rc.1","content_hash":artifact()["content_hash"]},
    "toolchain":{"id":"toolchain","version":"1.2.3+build.1","content_hash":artifact()["content_hash"]},
    "binding":{"tenant_id":"tenant","project_id":"project","run_id":"run","attempt_id":"attempt","input":artifact(),"dependencies":artifact(),"suite":artifact(),"oracle":artifact(),"policy":artifact(),"obligation_ids":["o1"]},
    "tests":[{"test_id":"t1","obligation_ids":["o1"],"outcome":"pass","origin_identity":"untrusted-origin-claim"}]})
}
#[test]
fn complete_record_has_structure_not_authenticated_authority() {
    let h = harness();
    assert!(valid(&h, &verification()));
}
#[test]
fn required_configuration_context_and_artifact_identities_are_enforced() {
    let h = harness();
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
        let mut v = verification();
        v.as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v), "{key}");
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
        let mut v = verification();
        v["binding"].as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v), "{key}");
    }
}
#[test]
fn tests_are_nonempty_with_explicit_identity_outcome_and_obligations() {
    let h = harness();
    let mut v = verification();
    v["tests"] = json!([]);
    assert!(!valid(&h, &v));
    for key in ["test_id", "obligation_ids", "outcome", "origin_identity"] {
        let mut v = verification();
        v["tests"][0].as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v));
    }
    for ids in [json!([]), json!(["o1", "o1"])] {
        let mut v = verification();
        v["tests"][0]["obligation_ids"] = ids;
        assert!(!valid(&h, &v));
    }
    let mut v = verification();
    v["tests"][0]["outcome"] = json!("accepted");
    assert!(!valid(&h, &v));
}
#[test]
fn every_nonpass_requires_a_bounded_structured_unicode_failure() {
    let h = harness();
    for outcome in ["fail", "skip", "error"] {
        let mut v = verification();
        v["tests"][0]["outcome"] = json!(outcome);
        assert!(!valid(&h, &v));
        for description in [
            json!("legacy"),
            json!({"detail":""}),
            json!({"detail":"safe","raw_stdout":"not permitted"}),
            json!({"detail":"é".repeat(2001)}),
        ] {
            let mut bad = v.clone();
            bad["tests"][0]["failure_description"] = description;
            assert!(!valid(&h, &bad));
        }
        v["tests"][0]["failure_description"] = json!({"detail":"é".repeat(2000)});
        assert!(valid(&h, &v));
    }
}
#[test]
fn algorithm_names_and_digest_shapes_are_closed_but_hashes_still_need_verification() {
    let h = harness();
    for (algorithm, length) in [
        ("sha-256", 64),
        ("sha-384", 96),
        ("sha-512", 128),
        ("sha3-256", 64),
        ("sha3-512", 128),
    ] {
        let mut v = verification();
        v["candidate"]["content_hash"] = json!({"algorithm":algorithm,"value":"a".repeat(length)});
        assert!(valid(&h, &v));
    }
    for hash in [
        json!({"algorithm":"md5","value":"a".repeat(32)}),
        json!({"algorithm":"sha-256","value":"AB"}),
        json!({"algorithm":"sha-256"}),
    ] {
        let mut v = verification();
        v["candidate"]["content_hash"] = hash;
        assert!(!valid(&h, &v));
    }
    let mut v = verification();
    v["candidate"]["content_hash"]["value"] = json!("a");
    assert!(valid(&h, &v)); // Exact length remains a required semantic check.
}
#[test]
fn pack_toolchain_and_document_versions_use_full_semver() {
    let h = harness();
    for key in ["pack", "toolchain"] {
        for version in ["1.2.3-rc.1", "1.2.3+build.1", "1.2.3-0"] {
            let mut v = verification();
            v[key]["version"] = json!(version);
            assert!(valid(&h, &v));
        }
        for version in ["01.2.3", "1.2.3-01", "1.2.3-", "1.2.3+", "1.2.3\n"] {
            let mut v = verification();
            v[key]["version"] = json!(version);
            assert!(!valid(&h, &v));
        }
    }
}
#[test]
fn approval_requires_all_fields_and_utc_z_but_never_authenticates_an_approver() {
    let h = harness();
    let mut v = verification();
    v["acceptance"] = json!({"approver_identities":["claimed-owner"],"timestamp":"2026-10-22T00:00:00Z","verification_result":{"id":v["id"],"format_version":"1.0.0"},"approval_evidence":artifact()});
    assert!(valid(&h, &v));
    for key in [
        "approver_identities",
        "timestamp",
        "verification_result",
        "approval_evidence",
    ] {
        let mut bad = v.clone();
        bad["acceptance"].as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &bad));
    }
    for time in [
        "2026-10-22T01:00:00+01:00",
        "not-a-dateZ",
        "2026-02-30T00:00:00Z",
    ] {
        let mut bad = v.clone();
        bad["acceptance"]["timestamp"] = json!(time);
        assert!(!valid(&h, &bad));
    }
}
#[test]
fn valid_community_extensions_do_not_replace_required_core_fields() {
    let h = harness();
    let mut v = verification();
    v["acme:diagnostic"] = json!({"safe":"opaque"});
    assert!(valid(&h, &v));
    v.as_object_mut().unwrap().remove("binding");
    assert!(!valid(&h, &v));
}
