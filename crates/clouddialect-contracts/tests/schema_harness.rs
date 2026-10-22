mod support;

use std::collections::BTreeMap;

use serde_json::{Value, json};
use support::SchemaHarness;

const ROOT: &str = "https://example.test/root.json";

fn resources(schema: Value) -> BTreeMap<String, Vec<u8>> {
    BTreeMap::from([(ROOT.into(), serde_json::to_vec(&schema).unwrap())])
}

#[test]
fn real_pinned_engine_accepts_a_valid_instance() {
    let harness = SchemaHarness::new(
        resources(json!({"type":"object","required":["id"],"properties":{"id":{"type":"string"}}})),
        ROOT,
        true,
    )
    .unwrap();
    assert!(harness.validate_json(br#"{"id":"reference"}"#).unwrap());
}

#[test]
fn deliberately_incorrect_instance_is_not_a_passing_control() {
    let harness = SchemaHarness::new(
        resources(json!({"type":"object","required":["id"],"properties":{"id":{"type":"string"}}})),
        ROOT,
        true,
    )
    .unwrap();
    assert!(!harness.validate_json(br#"{"id":17}"#).unwrap());
    assert!(!harness.validate_json(br#"{}"#).unwrap());
}

#[test]
fn malformed_instance_is_a_parse_error_not_validity_false_or_success() {
    let harness = SchemaHarness::new(resources(json!({"type":"object"})), ROOT, true).unwrap();
    assert!(harness.validate_json(b"{").is_err());
}

#[test]
fn malformed_and_invalid_schema_resources_fail_compilation() {
    let mut invalid_json = resources(json!({"type":"object"}));
    invalid_json.insert(ROOT.into(), b"{".to_vec());
    assert!(SchemaHarness::new(invalid_json, ROOT, true).is_err());
    assert!(SchemaHarness::new(resources(json!({"type":17})), ROOT, true).is_err());
}

#[test]
fn unknown_http_and_file_references_use_the_deny_loader() {
    for uri in [
        "https://not-admitted.invalid/schema.json",
        "file:///not-admitted/schema.json",
    ] {
        let error = SchemaHarness::new(resources(json!({"$ref":uri})), ROOT, true)
            .err()
            .unwrap();
        let detail = format!("{error:#}");
        assert!(detail.contains("test resources deny URI"), "{detail}");
        assert!(detail.contains(uri), "{detail}");
    }
}

#[test]
fn local_refs_and_explicitly_supplied_external_resources_work() {
    let mut schemas =
        resources(json!({"$defs":{"name":{"type":"string","minLength":1}},"$ref":"#/$defs/name"}));
    let local = SchemaHarness::new(schemas.clone(), ROOT, true).unwrap();
    assert!(local.validate_json(br#""name""#).unwrap());
    assert!(!local.validate_json(br#""""#).unwrap());
    schemas.insert(
        ROOT.into(),
        serde_json::to_vec(&json!({"$ref":"https://example.test/name.json"})).unwrap(),
    );
    schemas.insert(
        "https://example.test/name.json".into(),
        serde_json::to_vec(&json!({"type":"string"})).unwrap(),
    );
    let supplied = SchemaHarness::new(schemas, ROOT, true).unwrap();
    assert!(supplied.validate_json(br#""name""#).unwrap());
    assert!(!supplied.validate_json(b"17").unwrap());
}

#[test]
fn format_assertion_switch_has_actual_positive_and_negative_controls() {
    let schemas = resources(
        json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"string","format":"date-time"}),
    );
    let annotation = SchemaHarness::new(schemas.clone(), ROOT, false).unwrap();
    assert!(annotation.validate_json(br#""not-a-date""#).unwrap());
    let asserted = SchemaHarness::new(schemas, ROOT, true).unwrap();
    assert!(!asserted.validate_json(br#""not-a-date""#).unwrap());
    assert!(
        asserted
            .validate_json(br#""2026-10-22T00:00:00Z""#)
            .unwrap()
    );
}

#[test]
fn resource_lookup_is_exact_not_a_host_prefix_permission() {
    let error = SchemaHarness::new(
        resources(json!({"type":"string"})),
        "https://example.test/other.json",
        true,
    )
    .err()
    .unwrap();
    assert!(format!("{error:#}").contains("test resources deny URI"));
}
