use std::{collections::BTreeMap, process::Command};

use boon::{Compiler, Schemas, UrlLoader};
use clouddialect_contracts::{CatalogSnapshot, CatalogUrlLoader, schema::get_embedded_schema};
use serde_json::{Value, json};

const EVIDENCE: &str = "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json";
const SUPPORT: &str = "urn:clouddialect:test:support";
const META: &str = "https://json-schema.org/draft/2020-12/schema";
const INDEX: &[u8] = include_bytes!("../../../catalog/index.json");

fn parts() -> (Vec<u8>, BTreeMap<String, Vec<u8>>) {
    let files: [(&str, &[u8]); 6] = [
        (
            "extension-registry.json",
            include_bytes!("../../../catalog/extension-registry.json"),
        ),
        (
            "schemas/behavioral-contract/1.0/schema.json",
            include_bytes!("../../../catalog/schemas/behavioral-contract/1.0/schema.json"),
        ),
        (
            "schemas/evidence-record/1.0/schema.json",
            include_bytes!("../../../catalog/schemas/evidence-record/1.0/schema.json"),
        ),
        (
            "schemas/verification-result/1.0/schema.json",
            include_bytes!("../../../catalog/schemas/verification-result/1.0/schema.json"),
        ),
        (
            "schemas/catalog-index/1.0/schema.json",
            include_bytes!("../../../catalog/schemas/catalog-index/1.0/schema.json"),
        ),
        (
            "schemas/extension-registry/1.0/schema.json",
            include_bytes!("../../../catalog/schemas/extension-registry/1.0/schema.json"),
        ),
    ];
    (
        INDEX.to_vec(),
        files
            .into_iter()
            .map(|(p, b)| (p.to_owned(), b.to_vec()))
            .collect(),
    )
}

fn with_evidence_schema(schema: Value) -> CatalogSnapshot {
    let (index, mut material) = parts();
    material.insert(
        "schemas/evidence-record/1.0/schema.json".to_owned(),
        serde_json::to_vec(&schema).unwrap(),
    );
    CatalogSnapshot::from_parts(index, material).unwrap()
}

fn compile(
    snapshot: CatalogSnapshot,
    uri: &str,
    formats: bool,
) -> Result<(Schemas, boon::SchemaIndex), Box<boon::CompileError>> {
    let mut compiler = Compiler::new();
    compiler.use_loader(Box::new(CatalogUrlLoader::new(snapshot)));
    if formats {
        compiler.enable_format_assertions();
    }
    let mut schemas = Schemas::new();
    let root = compiler.compile(uri, &mut schemas)?;
    Ok((schemas, root))
}

#[test]
fn embedded_inventory_bytes_and_digest_match_all_explicit_material() {
    let (index, material) = parts();
    let explicit = CatalogSnapshot::from_parts(index, material.clone()).unwrap();
    let embedded = CatalogSnapshot::embedded().unwrap();
    assert_eq!(embedded.digest(), explicit.digest());
    let index: Value = serde_json::from_slice(INDEX).unwrap();
    for entry in index["schemas"].as_array().unwrap() {
        let uri = entry["schema_id"].as_str().unwrap();
        let expected = material[entry["catalog_path"].as_str().unwrap()].as_slice();
        assert!(!expected.is_empty());
        assert_eq!(embedded.schema_bytes(uri), Some(expected));
        let static_bytes: &'static [u8] = get_embedded_schema(uri).unwrap();
        assert_eq!(static_bytes, expected);
    }
    assert!(embedded.schema_bytes("extension-registry.json").is_none());
}

#[test]
fn actual_boon_adapter_returns_only_the_owned_snapshot_values() {
    let schema = json!({"$id":EVIDENCE,"type":"null","title":"explicit caller bytes"});
    let snapshot = with_evidence_schema(schema.clone());
    let loader = CatalogUrlLoader::new(snapshot.clone());
    assert_eq!(loader.load(EVIDENCE).unwrap(), schema);
    assert_eq!(
        snapshot.schema_bytes(EVIDENCE).unwrap(),
        serde_json::to_vec(&schema).unwrap()
    );
    assert_ne!(
        loader.load(EVIDENCE).unwrap(),
        serde_json::from_slice::<Value>(get_embedded_schema(EVIDENCE).unwrap()).unwrap()
    );
    let (schemas, root) = compile(snapshot, EVIDENCE, true).unwrap();
    assert!(schemas.validate(&Value::Null, root).is_ok());
    assert!(schemas.validate(&json!({}), root).is_err());
}

#[test]
fn unknown_uri_errors_include_exact_uri_without_prefix_or_file_fallback() {
    let loader = CatalogUrlLoader::new(CatalogSnapshot::embedded().unwrap());
    for uri in [
        "file:///tmp/unlisted-schema.json",
        "http://127.0.0.1:9/unlisted-schema",
        "https://schemas.clouddialect.io/unknown/1.0.0/schema.json",
        "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json#field",
        "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json?query",
        "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json/extra",
        "schemas/evidence-record/1.0/schema.json",
        "../catalog/index.json",
        META,
    ] {
        let error = loader.load(uri).unwrap_err().to_string();
        assert!(error.contains(uri), "{error}");
        assert!(get_embedded_schema(uri).is_none(), "{uri}");
    }
    // Standard meta-schemas are the pinned engine's built-ins, not a catalog allowlist exception.
}

#[test]
fn pinned_engine_compiles_all_embedded_schemas_and_validates_real_catalog_documents() {
    let index: Value = serde_json::from_slice(INDEX).unwrap();
    for entry in index["schemas"].as_array().unwrap() {
        let uri = entry["schema_id"].as_str().unwrap();
        let (schemas, root) = compile(CatalogSnapshot::embedded().unwrap(), uri, true).unwrap();
        if entry["document_type"] == "catalog-index" {
            assert!(schemas.validate(&index, root).is_ok());
            assert!(schemas.validate(&json!({}), root).is_err());
        } else if entry["document_type"] == "extension-registry" {
            let registry: Value =
                serde_json::from_slice(include_bytes!("../../../catalog/extension-registry.json"))
                    .unwrap();
            assert!(schemas.validate(&registry, root).is_ok());
            assert!(schemas.validate(&json!({}), root).is_err());
        }
    }
    for uri in [
        META,
        "https://json-schema.org/draft/2020-12/meta/core",
        "https://json-schema.org/draft/2020-12/meta/format-assertion",
    ] {
        compile(CatalogSnapshot::embedded().unwrap(), uri, true).unwrap();
    }
}

#[test]
fn indexed_support_resources_and_local_fragments_use_actual_adapter() {
    let (index, mut material) = parts();
    let mut index: Value = serde_json::from_slice(&index).unwrap();
    index["resolver_resources"] =
        json!([{"schema_id":SUPPORT,"catalog_path":"support/string.json"}]);
    material.insert("support/string.json".to_owned(), serde_json::to_vec(&json!({"$id":SUPPORT,"$schema":META,"$defs":{"inner":{"type":"string","minLength":2}},"$ref":"#/$defs/inner"})).unwrap());
    material.insert(
        "schemas/evidence-record/1.0/schema.json".to_owned(),
        serde_json::to_vec(&json!({"$id":EVIDENCE,"$schema":META,"$ref":SUPPORT})).unwrap(),
    );
    // Even schema-looking unindexed material cannot acquire a resolver mapping.
    material.insert(
        "support/unlisted.json".to_owned(),
        br#"{"$id":"urn:unlisted","type":"null"}"#.to_vec(),
    );
    let snapshot =
        CatalogSnapshot::from_parts(serde_json::to_vec(&index).unwrap(), material).unwrap();
    let loader = CatalogUrlLoader::new(snapshot.clone());
    assert!(loader.load(SUPPORT).is_ok());
    assert!(loader.load("urn:unlisted").is_err());
    assert!(loader.load("support/unlisted.json").is_err());
    let (schemas, root) = compile(snapshot, EVIDENCE, true).unwrap();
    assert!(schemas.validate(&json!("ok"), root).is_ok());
    assert!(schemas.validate(&json!("x"), root).is_err());
    assert!(schemas.validate(&json!(1), root).is_err());
}

#[test]
fn format_assertions_are_an_explicit_compiler_setting_not_a_loader_claim() {
    let snapshot = with_evidence_schema(
        json!({"$id":EVIDENCE,"$schema":META,"type":"string","format":"date"}),
    );
    let (schemas, root) = compile(snapshot.clone(), EVIDENCE, false).unwrap();
    assert!(schemas.validate(&json!("not-a-date"), root).is_ok());
    let (schemas, root) = compile(snapshot, EVIDENCE, true).unwrap();
    assert!(schemas.validate(&json!("2024-02-29"), root).is_ok());
    for invalid in ["not-a-date", "2023-02-29", "2024-13-01"] {
        assert!(
            schemas.validate(&json!(invalid), root).is_err(),
            "{invalid}"
        );
    }
}

#[test]
fn unresolved_references_and_unknown_meta_schemas_fail_closed_in_real_compiler() {
    for uri in [
        "file:///tmp/never-open-schema.json",
        "http://127.0.0.1:9/no-schema",
        "https://schemas.clouddialect.io/unknown/schema.json",
    ] {
        for keyword in ["$ref", "$schema"] {
            let mut schema = json!({"$id":EVIDENCE,"$schema":META});
            schema[keyword] = json!(uri);
            let error = compile(with_evidence_schema(schema), EVIDENCE, true)
                .err()
                .expect("unlisted URI must fail");
            assert!(format!("{error:#}").contains(uri), "{error:#}");
        }
    }
}

#[test]
fn malformed_material_and_invalid_schema_are_not_successful_validation() {
    let (index, mut material) = parts();
    material.insert(
        "schemas/evidence-record/1.0/schema.json".to_owned(),
        b"{secret-invalid-json".to_vec(),
    );
    let error = CatalogSnapshot::from_parts(index, material).unwrap_err();
    assert!(!error.description.contains("secret-invalid-json"));
    // Inventory-valid JSON is not proof that the schema itself compiles.
    let snapshot = with_evidence_schema(json!({"$id":EVIDENCE,"type":42}));
    assert!(compile(snapshot, EVIDENCE, true).is_err());
}

#[test]
fn hostile_ambient_catalog_overrides_do_not_select_any_runtime_material() {
    const CHILD: &str = "CLOUDDIALECT_TEST_CATALOG_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let (index, material) = parts();
        let expected = CatalogSnapshot::from_parts(index, material).unwrap();
        assert_eq!(
            CatalogSnapshot::embedded().unwrap().digest(),
            expected.digest()
        );
        let loader = CatalogUrlLoader::new(expected);
        assert_eq!(
            loader.load(EVIDENCE).unwrap(),
            serde_json::from_slice::<Value>(get_embedded_schema(EVIDENCE).unwrap()).unwrap()
        );
        compile(CatalogSnapshot::embedded().unwrap(), EVIDENCE, true).unwrap();
        return;
    }
    // Fresh processes avoid unsafe process-global environment mutation in Rust 2024.
    for value in [
        "/tmp/no-approved-catalog",
        "https://untrusted.invalid/catalog",
    ] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "hostile_ambient_catalog_overrides_do_not_select_any_runtime_material",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .env("CLOUDDIALECT_SCHEMA_CATALOG", value)
            .env("CLOUDDIALECT_CATALOG_DIR", value)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
    }
    // This is deterministic library evidence, not approved native zero-network qualification.
}
