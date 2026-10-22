use clouddialect_contracts::CatalogSnapshot;
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn parts() -> (Vec<u8>, BTreeMap<String, Vec<u8>>) {
    let kinds = ["behavioral-contract", "evidence-record", "verification-result", "catalog-index", "extension-registry"];
    let entries = kinds.map(|kind| json!({
        "schema_id":format!("https://schemas.clouddialect.io/{kind}/1.0.0/schema.json"),
        "document_type":kind,"format_version":"1.0.0",
        "catalog_path":format!("schemas/{kind}/1.0/schema.json"),"status":"current"
    }));
    let index = serde_json::to_vec(&json!({"format_version":"1.0.0","schemas":entries,"resolver_resources":[]})).unwrap();
    let material = entries.iter().map(|entry| {
        (entry["catalog_path"].as_str().unwrap().to_owned(), serde_json::to_vec(&json!({"$id":entry["schema_id"],"type":"object"})).unwrap())
    }).collect();
    (index, material)
}

fn changed_index(index: &[u8], change: impl FnOnce(&mut Value)) -> Vec<u8> {
    let mut value = serde_json::from_slice(index).unwrap();
    change(&mut value);
    serde_json::to_vec(&value).unwrap()
}

#[test]
fn exact_p0_binary_encoding_matches_independent_sha256_vector() {
    let (index, material) = parts();
    let snapshot = CatalogSnapshot::from_parts(index, material).unwrap();
    assert_eq!(snapshot.digest().algorithm, "sha-256");
    assert_eq!(snapshot.digest().value, "92994dedad35826e2a35528febaf6f8f4f5308aee3f5e6da1b1a234d305f1fc1");
}

#[test]
fn utf8_byte_lengths_and_empty_payloads_match_independent_vector() {
    let (index, mut material) = parts();
    material.insert("meta/é.json".to_owned(), vec![0, 255]);
    material.insert("z/empty".to_owned(), vec![]);
    let snapshot = CatalogSnapshot::from_parts(index, material).unwrap();
    assert_eq!(snapshot.digest().value, "46cd72941288e88ef6d24ba7953da7e708947dd30dd828c8f16c6e514e478e45");
    // Digest-bound unindexed bytes never become resolvable schema resources.
    assert!(snapshot.schema_bytes("meta/é.json").is_none());
    assert!(snapshot.schema_bytes("index.json").is_none());
}

#[test]
fn insertion_order_is_irrelevant_but_length_prefixes_prevent_concatenation_aliases() {
    let (index, material) = parts();
    let reversed = material.iter().rev().map(|(k, v)| (k.clone(), v.clone())).collect();
    assert_eq!(CatalogSnapshot::from_parts(index.clone(), material.clone()).unwrap().digest(), CatalogSnapshot::from_parts(index.clone(), reversed).unwrap().digest());
    let mut a = material.clone();
    a.insert("a".to_owned(), b"bc".to_vec());
    let mut b = material;
    b.insert("ab".to_owned(), b"c".to_vec());
    let a = CatalogSnapshot::from_parts(index.clone(), a).unwrap();
    let b = CatalogSnapshot::from_parts(index, b).unwrap();
    assert_eq!(a.digest().value, "aede7eb989c8a41ec8b4289635d4ddc17bdb72e38c4dfa3ff25745faefc3690c");
    assert_eq!(b.digest().value, "358e20ef80fc9f86ee801eff1acbc21ddd8e0dc19c7f66afcfaecf22ab6eddc8");
    assert_ne!(a.digest(), b.digest());
}

#[test]
fn every_exact_index_byte_is_bound_even_when_json_values_are_equal() {
    let (index, material) = parts();
    let mut whitespace = index.clone();
    whitespace.push(b'\n');
    assert_eq!(serde_json::from_slice::<Value>(&index).unwrap(), serde_json::from_slice::<Value>(&whitespace).unwrap());
    assert_ne!(CatalogSnapshot::from_parts(index, material.clone()).unwrap().digest(), CatalogSnapshot::from_parts(whitespace, material).unwrap().digest());
}

#[test]
fn frozen_bytes_and_identity_do_not_follow_later_caller_mutation() {
    let (mut index, mut material) = parts();
    let snapshot = CatalogSnapshot::from_parts(index.clone(), material.clone()).unwrap();
    let uri = "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json";
    let expected = material["schemas/evidence-record/1.0/schema.json"].clone();
    index.clear();
    material.values_mut().for_each(Vec::clear);
    assert_eq!(snapshot.schema_bytes(uri), Some(expected.as_slice()));
    assert_eq!(snapshot.digest().value, "92994dedad35826e2a35528febaf6f8f4f5308aee3f5e6da1b1a234d305f1fc1");
    assert_eq!(snapshot.clone().digest(), snapshot.digest());
}

#[test]
fn unresolved_and_malformed_parts_do_not_construct_a_snapshot() {
    let (index, mut material) = parts();
    assert!(CatalogSnapshot::from_parts(b"{".to_vec(), material.clone()).is_err());
    material.remove("schemas/evidence-record/1.0/schema.json");
    assert!(CatalogSnapshot::from_parts(index.clone(), material).is_err());
    let (_, mut material) = parts();
    material.insert("schemas/evidence-record/1.0/schema.json".to_owned(), b"{".to_vec());
    assert!(CatalogSnapshot::from_parts(index, material).is_err());
}

#[test]
fn duplicate_mappings_and_index_aliases_cannot_enter_frozen_material() {
    let (index, material) = parts();
    let duplicate = changed_index(&index, |value| {
        let first = value["schemas"][0].clone();
        value["schemas"].as_array_mut().unwrap().push(first);
    });
    assert!(CatalogSnapshot::from_parts(duplicate, material.clone()).is_err());
    for path in ["index.json", "../outside.json", "/tmp/schema.json", "x\\schema.json"] {
        let mut bad = material.clone();
        bad.insert(path.to_owned(), b"true".to_vec());
        assert!(CatalogSnapshot::from_parts(index.clone(), bad).is_err(), "{path}");
    }
}

#[test]
fn lookup_is_exact_and_constructor_does_not_attest_schema_provenance() {
    let (index, mut material) = parts();
    let uri = "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json";
    let path = "schemas/evidence-record/1.0/schema.json";
    material.insert(path.to_owned(), serde_json::to_vec(&json!({"$id":uri,"type":"null"})).unwrap());
    let snapshot = CatalogSnapshot::from_parts(index, material).unwrap();
    // Inventory-valid caller-supplied schemas are not claimed to be owner-approved.
    assert!(snapshot.schema_bytes(uri).is_some());
    for unknown in ["schemas/evidence-record/1.0/schema.json", "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json#x", "file:///tmp/schema.json", "https://example.test/schema"] {
        assert!(snapshot.schema_bytes(unknown).is_none(), "{unknown}");
    }
}
