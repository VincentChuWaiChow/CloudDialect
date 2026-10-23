use clouddialect_contracts::{RawObject, schema::CatalogIndex};
use serde_json::Value;
use std::{collections::BTreeMap, path::Path};

fn parts() -> (String, BTreeMap<String, Vec<u8>>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog");
    let index = std::fs::read_to_string(root.join("index.json")).unwrap();
    let value: Value = serde_json::from_str(&index).unwrap();
    let material = value["schemas"]
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

fn append_members(index: &str, members: &str) -> String {
    format!(
        "{}, {members}}}",
        index.trim_end().strip_suffix('}').unwrap()
    )
}

#[test]
fn typed_and_inventory_decoding_preserve_exact_extension_value_bytes() {
    let (index, material) = parts();
    let raw = r#"{ "escaped": "\u0061\/", "number": 1.2300e+04, "array": [true, null] }"#;
    let bytes = append_members(&index, &format!(r#""acme:payload":{raw}"#));
    let typed: CatalogIndex = serde_json::from_str(&bytes).unwrap();
    let admitted = CatalogIndex::from_parts(bytes.as_bytes(), &material).unwrap();
    for document in [typed, admitted] {
        assert_eq!(document.extensions["acme:payload"].get(), raw);
        let encoded = serde_json::to_vec(&document).unwrap();
        let members: RawObject = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(
            members.get("acme:payload").map(|value| value.get()),
            Some(raw)
        );
        assert_eq!(document.schemas.len(), 5);
        assert!(document.resolver_resources.is_empty());
    }
}

#[test]
fn decoded_duplicate_extension_names_are_rejected_before_inventory_admission() {
    let (index, material) = parts();
    for members in [
        r#""acme:payload":1,"acme:payload":2"#,
        r#""acme:payload":1,"\u0061cme:payload":2"#,
    ] {
        let bytes = append_members(&index, members);
        assert!(serde_json::from_str::<CatalogIndex>(&bytes).is_err());
        assert!(CatalogIndex::from_parts(bytes.as_bytes(), &material).is_err());
    }
}

#[test]
fn decoded_duplicate_core_names_remain_rejected() {
    let (index, material) = parts();
    for members in [
        r#""format_version":"1.0.0""#,
        r#""\u0066ormat_version":"1.0.0""#,
        r#""schemas":[]"#,
        r#""resolver_resources":[]"#,
    ] {
        let bytes = append_members(&index, members);
        assert!(serde_json::from_str::<CatalogIndex>(&bytes).is_err());
        assert!(CatalogIndex::from_parts(bytes.as_bytes(), &material).is_err());
    }
}

#[test]
fn mandatory_declaration_remains_a_typed_core_field() {
    let (index, material) = parts();
    let bytes = append_members(
        &index,
        r#""required_extensions":["acme"],"acme:payload":false"#,
    );
    let admitted = CatalogIndex::from_parts(bytes.as_bytes(), &material).unwrap();
    assert_eq!(
        admitted.required_extensions.as_deref(),
        Some(["acme".to_owned()].as_slice())
    );
    assert!(!admitted.extensions.contains_key("required_extensions"));
    let encoded = serde_json::to_vec(&admitted).unwrap();
    let members: RawObject = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(members["required_extensions"].get(), r#"["acme"]"#);
    assert_eq!(
        members.get("acme:payload").map(|value| value.get()),
        Some("false")
    );
    let duplicate = append_members(&bytes, r#""required_extensions":["other"]"#);
    assert!(serde_json::from_str::<CatalogIndex>(&duplicate).is_err());
    assert!(CatalogIndex::from_parts(duplicate.as_bytes(), &material).is_err());
}

#[test]
fn admitted_catalog_serialization_omits_absent_dates_and_revalidates() {
    let (index, material) = parts();
    let mut document = CatalogIndex::from_parts(index.as_bytes(), &material).unwrap();
    assert!(
        document
            .schemas
            .iter()
            .all(|entry| entry.end_of_life_date.is_none())
    );
    document.schemas[0].end_of_life_date = Some("2030-01-01".to_owned());
    let encoded = serde_json::to_vec(&document).unwrap();
    let value: Value = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(value["schemas"][0]["end_of_life_date"], "2030-01-01");
    for entry in value["schemas"].as_array().unwrap().iter().skip(1) {
        assert!(entry.get("end_of_life_date").is_none());
    }
    let reparsed = CatalogIndex::from_parts(&encoded, &material).unwrap();
    assert_eq!(
        reparsed.schemas[0].end_of_life_date.as_deref(),
        Some("2030-01-01")
    );
    assert!(
        reparsed
            .schemas
            .iter()
            .skip(1)
            .all(|entry| entry.end_of_life_date.is_none())
    );
}

fn append_entry_members(index: &str, members: &str) -> String {
    // Insert raw members only after encoding the original extension-free fixture.
    let value: Value = serde_json::from_str(index).unwrap();
    let first = serde_json::to_string(&value["schemas"][0]).unwrap();
    let encoded = serde_json::to_string(&value).unwrap();
    assert_eq!(encoded.matches(&first).count(), 1);
    encoded.replacen(&first, &append_members(&first, members), 1)
}

#[test]
fn nested_entry_values_survive_typed_inventory_and_revalidated_round_trips() {
    let (index, material) = parts();
    for raw in [
        r#"{ "escaped": "\u0061\/", "number": 1.2300e+04, "same": 1, "same": 2 }"#,
        r#"[ true, null, { "spaced": "value" } ]"#,
        r#""\u0061\/""#,
        "-0.00e+0",
        "null",
        "false",
    ] {
        let bytes = append_entry_members(
            &index,
            &format!(r#""acme:payload":{raw},"entry_note":{raw}"#),
        );
        let typed: CatalogIndex = serde_json::from_str(&bytes).unwrap();
        let admitted = CatalogIndex::from_parts(bytes.as_bytes(), &material).unwrap();
        for document in [typed, admitted] {
            let encoded = serde_json::to_vec(&document).unwrap();
            let root: RawObject = serde_json::from_slice(&encoded).unwrap();
            let entries: Vec<Box<serde_json::value::RawValue>> =
                serde_json::from_str(root["schemas"].get()).unwrap();
            let first: RawObject = serde_json::from_str(entries[0].get()).unwrap();
            for name in ["acme:payload", "entry_note"] {
                assert_eq!(
                    first.get(name).map(|value| value.get()),
                    Some(raw),
                    "{name}: entry value lost or rewritten"
                );
            }
            let reparsed = CatalogIndex::from_parts(&encoded, &material).unwrap();
            let second = serde_json::to_vec(&reparsed).unwrap();
            assert_eq!(encoded, second, "second round-trip must remain stable");
        }
    }
}

#[test]
fn decoded_duplicate_entry_extension_names_are_rejected() {
    let (index, material) = parts();
    for members in [
        r#""acme:payload":1,"acme:payload":2"#,
        r#""acme:payload":1,"\u0061cme:payload":2"#,
        r#""entry_note":1,"entry_note":2"#,
    ] {
        let bytes = append_entry_members(&index, members);
        assert!(serde_json::from_str::<CatalogIndex>(&bytes).is_err());
        assert!(CatalogIndex::from_parts(bytes.as_bytes(), &material).is_err());
    }
}

#[test]
fn entry_core_fields_remain_typed_and_duplicate_aware() {
    let (index, material) = parts();
    for members in [
        r#""schema_id":"https://schemas.clouddialect.io/behavioral-contract/1.0.0/schema.json""#,
        r#""\u0073chema_id":"https://schemas.clouddialect.io/behavioral-contract/1.0.0/schema.json""#,
        r#""status":"current""#,
        r#""format_version":"1.0.0""#,
    ] {
        let bytes = append_entry_members(&index, members);
        assert!(serde_json::from_str::<CatalogIndex>(&bytes).is_err());
        assert!(CatalogIndex::from_parts(bytes.as_bytes(), &material).is_err());
    }
    let bytes = append_entry_members(
        &index,
        r#""end_of_life_date":"2030-01-01","acme:payload":null"#,
    );
    let document = CatalogIndex::from_parts(bytes.as_bytes(), &material).unwrap();
    assert_eq!(
        document.schemas[0].end_of_life_date.as_deref(),
        Some("2030-01-01")
    );
    CatalogIndex::from_parts(&serde_json::to_vec(&document).unwrap(), &material).unwrap();
}
