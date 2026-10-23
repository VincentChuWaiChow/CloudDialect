mod support;

use clouddialect_contracts::schema::CatalogIndex;
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use support::SchemaHarness;

fn parts() -> (Value, BTreeMap<String, Vec<u8>>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog");
    let index: Value =
        serde_json::from_slice(&std::fs::read(root.join("index.json")).unwrap()).unwrap();
    let material = index["schemas"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            let path = entry["catalog_path"].as_str().unwrap().to_owned();
            let bytes = std::fs::read(root.join(&path)).unwrap();
            (path, bytes)
        })
        .collect();
    (index, material)
}

fn admit(
    index: &Value,
    material: &BTreeMap<String, Vec<u8>>,
) -> Result<CatalogIndex, clouddialect_contracts::ValidatorError> {
    CatalogIndex::from_parts(&serde_json::to_vec(index).unwrap(), material)
}

#[test]
fn delivered_inventory_contains_all_five_kinds_and_its_own_schema() {
    let (index, material) = parts();
    let admitted = admit(&index, &material).unwrap();
    assert_eq!(admitted.format_version, "1.0.0");
    assert_eq!(admitted.schemas.len(), 5);
    assert!(
        admitted
            .schemas
            .iter()
            .any(|e| e.schema_id
                == "https://schemas.clouddialect.io/catalog-index/1.0.0/schema.json")
    );
    assert!(admitted.resolver_resources.is_empty());
}

#[test]
fn malformed_and_shape_invalid_index_bytes_are_rejected() {
    let (mut index, material) = parts();
    let error = CatalogIndex::from_parts(b"{", &material).unwrap_err();
    assert_eq!(
        error.category,
        clouddialect_contracts::ErrorCategory::Unparseable
    );
    assert!(error.byte_offset.is_some());
    index.as_object_mut().unwrap().remove("resolver_resources");
    assert!(admit(&index, &material).is_err());
    let (mut index, material) = parts();
    index["schemas"][0]["status"] = json!("trusted");
    assert!(admit(&index, &material).is_err());
    index["schemas"][0]["status"] = json!("current");
    index["format_version"] = json!("2.0.0");
    assert_eq!(
        admit(&index, &material).unwrap_err().category,
        clouddialect_contracts::ErrorCategory::UnsupportedVersion
    );
    index["format_version"] = json!("1.0.0");
    index["$schema"] = json!("https://attacker.test/index");
    assert!(admit(&index, &material).is_err());
}

#[test]
fn exact_uri_uniqueness_and_path_aliasing_span_both_lists() {
    let (index, material) = parts();
    let mut duplicate = index.clone();
    let first = duplicate["schemas"][0].clone();
    duplicate["schemas"].as_array_mut().unwrap().push(first);
    assert!(admit(&duplicate, &material).is_err());
    for (uri, path) in [
        (
            index["schemas"][0]["schema_id"].as_str().unwrap(),
            "support/other.json",
        ),
        (
            "https://example.test/support",
            index["schemas"][0]["catalog_path"].as_str().unwrap(),
        ),
    ] {
        let mut duplicate = index.clone();
        duplicate["resolver_resources"] = json!([{"schema_id":uri,"catalog_path":path}]);
        assert!(admit(&duplicate, &material).is_err());
    }
}

#[test]
fn missing_material_and_mismatched_schema_id_fail() {
    let (index, mut material) = parts();
    let path = index["schemas"][0]["catalog_path"].as_str().unwrap();
    let original = material.remove(path).unwrap();
    assert!(admit(&index, &material).is_err());
    let mut schema: Value = serde_json::from_slice(&original).unwrap();
    schema["$id"] = json!("https://attacker.test/schema");
    material.insert(path.to_owned(), serde_json::to_vec(&schema).unwrap());
    assert!(admit(&index, &material).is_err());
    material.insert(path.to_owned(), b"{".to_vec());
    assert!(admit(&index, &material).is_err());
    material.insert(path.to_owned(), b"true".to_vec());
    assert!(admit(&index, &material).is_err());
}

#[test]
fn noncanonical_paths_and_reserved_index_material_are_rejected() {
    let (index, material) = parts();
    for path in [
        "../outside.json",
        "/tmp/schema.json",
        "support/../schema.json",
        "support//schema.json",
        "./schema.json",
        "support\\schema.json",
        "support/",
        "index.json",
        "C:/schema.json",
    ] {
        let mut bad = index.clone();
        bad["resolver_resources"] =
            json!([{"schema_id":"https://example.test/meta","catalog_path":path}]);
        let mut bytes = material.clone();
        bytes.insert(path.to_owned(), b"true".to_vec());
        assert!(admit(&bad, &bytes).is_err(), "{path}");
    }
}

#[test]
fn application_identity_kind_version_and_directory_must_agree() {
    let (index, material) = parts();
    for (field, value) in [
        ("document_type", "pack-manifest"),
        ("format_version", "1.1.0"),
        ("schema_id", "https://attacker.test/schema"),
        ("catalog_path", "schemas/behavioral-contract/1.0/other.json"),
    ] {
        let mut bad = index.clone();
        bad["schemas"][0][field] = json!(value);
        assert!(admit(&bad, &material).is_err(), "{field}");
    }
    let mut missing = index;
    missing["schemas"].as_array_mut().unwrap().pop();
    assert!(admit(&missing, &material).is_err());
}

#[test]
fn supplied_support_material_and_relocated_bytes_need_no_live_paths() {
    let (mut index, mut material) = parts();
    index["resolver_resources"] =
        json!([{"schema_id":"https://example.test/meta","catalog_path":"support/meta.json"}]);
    material.insert("support/meta.json".to_owned(), b"true".to_vec());
    assert!(admit(&index, &material).is_ok());
    material.insert("support/meta.json".to_owned(), b"42".to_vec());
    assert!(admit(&index, &material).is_err());
    material.insert("support/meta.json".to_owned(), b"true".to_vec());
    let frozen_index = serde_json::to_vec(&index).unwrap();
    let copied_material = material.clone();
    material.clear();
    let admitted = CatalogIndex::from_parts(&frozen_index, &copied_material).unwrap();
    assert_eq!(
        admitted.resolver_resources[0].catalog_path,
        "support/meta.json"
    );
    let mut mismatch = copied_material;
    mismatch.insert(
        "support/meta.json".to_owned(),
        br#"{"$id":"https://wrong.test/meta"}"#.to_vec(),
    );
    assert!(admit(&index, &mismatch).is_err());
}

#[test]
fn pinned_engine_compiles_all_bundled_schemas_with_deny_unknown_loader() {
    let (index, material) = parts();
    let admitted = admit(&index, &material).unwrap();
    let resources = admitted
        .schemas
        .iter()
        .map(|entry| {
            (
                entry.schema_id.clone(),
                material[&entry.catalog_path].clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for entry in &admitted.schemas {
        let harness = SchemaHarness::new(resources.clone(), &entry.schema_id, true).unwrap();
        assert!(!harness.validate_json(b"null").unwrap());
    }
    // This demonstrates Boon's bundled meta-resources, not native zero-network qualification.
    assert!(SchemaHarness::new(resources, "https://example.test/unlisted", true).is_err());
}
