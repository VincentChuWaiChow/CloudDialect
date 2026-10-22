mod support;

use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use support::SchemaHarness;

fn harness(kind: &str) -> SchemaHarness {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../catalog/schemas/{kind}/1.0/schema.json"));
    assert!(path.is_file(), "required {kind} schema is not delivered");
    let bytes = std::fs::read(path).unwrap();
    let schema: Value = serde_json::from_slice(&bytes).unwrap();
    let uri = format!("https://schemas.clouddialect.io/{kind}/1.0.0/schema.json");
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["$id"], uri);
    SchemaHarness::new(BTreeMap::from([(uri.clone(), bytes)]), &uri, true).unwrap()
}
fn valid(h: &SchemaHarness, v: &Value) -> bool {
    h.validate_json(&serde_json::to_vec(v).unwrap()).unwrap()
}
fn index() -> Value {
    json!({"format_version":"1.0.0","schemas":[{"schema_id":"https://schemas.clouddialect.io/catalog-index/1.0.0/schema.json","document_type":"catalog-index","format_version":"1.0.0","catalog_path":"schemas/catalog-index/1.0/schema.json","status":"current"}],"resolver_resources":[]})
}
fn registry() -> Value {
    json!({"format_version":"1.0.0","extensions":[{"namespace_prefix":"acme-tools","owner":"claimed-owner","purpose":"test metadata","registration_date":"2026-10-22"}]})
}
#[test]
fn empty_registry_seed_is_valid_but_not_identity_authentication() {
    let h = harness("extension-registry");
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog/extension-registry.json");
    assert!(path.is_file(), "registry seed missing");
    let seed: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    assert_eq!(seed, json!({"format_version":"1.0.0","extensions":[]}));
    assert!(valid(&h, &seed));
    assert!(valid(&h, &registry()));
}
#[test]
fn registry_entries_require_all_metadata_and_real_dates() {
    let h = harness("extension-registry");
    for key in ["namespace_prefix", "owner", "purpose", "registration_date"] {
        let mut v = registry();
        v["extensions"][0].as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v), "{key}");
    }
    for date in ["not-a-date", "2026-02-30"] {
        let mut v = registry();
        v["extensions"][0]["registration_date"] = json!(date);
        assert!(!valid(&h, &v));
    }
}
#[test]
fn registry_has_required_version_list_and_namespace_grammar() {
    let h = harness("extension-registry");
    for key in ["format_version", "extensions"] {
        let mut v = registry();
        v.as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v));
    }
    for prefix in ["a", "with:colon", "acme/tools", "соre", "acme\n"] {
        let mut v = registry();
        v["extensions"][0]["namespace_prefix"] = json!(prefix);
        assert!(!valid(&h, &v));
    }
}
#[test]
fn index_requires_nonempty_application_list_and_explicit_resolver_resources() {
    let h = harness("catalog-index");
    assert!(valid(&h, &index()));
    for key in ["format_version", "schemas", "resolver_resources"] {
        let mut v = index();
        v.as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v));
    }
    let mut v = index();
    v["schemas"] = json!([]);
    assert!(!valid(&h, &v));
}
#[test]
fn application_kinds_exclude_toml_manifests_and_private_records() {
    let h = harness("catalog-index");
    for kind in [
        "behavioral-contract",
        "evidence-record",
        "verification-result",
        "catalog-index",
        "extension-registry",
    ] {
        let mut v = index();
        v["schemas"][0]["document_type"] = json!(kind);
        assert!(valid(&h, &v));
    }
    for kind in ["pack-manifest", "manifest", "stored-document"] {
        let mut v = index();
        v["schemas"][0]["document_type"] = json!(kind);
        assert!(!valid(&h, &v));
    }
}
#[test]
fn end_of_life_status_requires_a_valid_date() {
    let h = harness("catalog-index");
    let mut v = index();
    v["schemas"][0]["status"] = json!("end-of-life-declared");
    assert!(!valid(&h, &v));
    v["schemas"][0]["end_of_life_date"] = json!("2027-10-22");
    assert!(valid(&h, &v));
    v["schemas"][0]["end_of_life_date"] = json!("2027-02-30");
    assert!(!valid(&h, &v));
}
#[test]
fn resolver_support_mappings_are_separate_exact_shapes() {
    let h = harness("catalog-index");
    let mut v = index();
    v["resolver_resources"] =
        json!([{"schema_id":"https://example.test/meta","catalog_path":"support/meta.json"}]);
    assert!(valid(&h, &v));
    for key in ["schema_id", "catalog_path"] {
        let mut bad = v.clone();
        bad["resolver_resources"][0]
            .as_object_mut()
            .unwrap()
            .remove(key);
        assert!(!valid(&h, &bad));
    }
    v["resolver_resources"][0]["document_type"] = json!("private-data");
    assert!(!valid(&h, &v));
}
#[test]
fn version_syntax_and_entry_metadata_do_not_prove_path_or_uri_admission() {
    let h = harness("catalog-index");
    let mut v = index();
    v["schemas"][0]["format_version"] = json!("1.2.3-rc.1");
    assert!(valid(&h, &v));
    v["schemas"][0]["format_version"] = json!("not-semver");
    assert!(!valid(&h, &v));
    let mut v = index();
    v["schemas"][0]["catalog_path"] = json!("../outside.json");
    assert!(valid(&h, &v)); // Later catalog admission must reject this shape-valid path.
}
