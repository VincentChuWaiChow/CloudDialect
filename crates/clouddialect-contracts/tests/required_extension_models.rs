use clouddialect_contracts::{
    BehavioralContract, EvidenceRecord, VerificationResult, schema::CatalogIndex,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn contract() -> Value {
    json!({"id":"c1","format_version":"1.0.0","operation_identity":{"call_site":"pkg.call"},"inputs":{},"outputs":{},"side_effects":[],"resource_dependencies":[],"obligations":[]})
}

fn evidence() -> Value {
    json!({"id":"e1","format_version":"1.0.0","claim":"claim","classification":"inferred","reasoning_chain":["reason"]})
}

fn verification() -> Value {
    let artifact = json!({"identity":"artifact","content_hash":{"algorithm":"sha-256","value":"a".repeat(64)}});
    json!({"id":"v1","format_version":"1.0.0","candidate":artifact,"configuration":artifact,
        "pack":{"id":"pack","version":"1.0.0","content_hash":artifact["content_hash"]},
        "toolchain":{"id":"tool","version":"1.0.0","content_hash":artifact["content_hash"]},
        "binding":{"tenant_id":"tenant","project_id":"project","run_id":"run","attempt_id":"attempt","input":artifact,"dependencies":artifact,"suite":artifact,"oracle":artifact,"policy":artifact,"obligation_ids":["o1"]},
        "tests":[{"test_id":"t1","obligation_ids":["o1"],"outcome":"pass","origin_identity":"claim"}]})
}

fn index() -> Value {
    serde_json::from_slice(include_bytes!("../../../catalog/index.json")).unwrap()
}

fn roundtrip<T: DeserializeOwned + Serialize>(value: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(value).unwrap()).unwrap()
}

#[test]
fn mandatory_declarations_are_explicit_fields_not_unknown_extension_payloads() {
    let mut c = contract();
    c["required_extensions"] = json!(["acme"]);
    let c: BehavioralContract = serde_json::from_value(c).unwrap();
    assert_eq!(
        c.required_extensions.as_deref(),
        Some(["acme".to_owned()].as_slice())
    );
    assert!(!c.extensions.contains_key("required_extensions"));
    let mut e = evidence();
    e["required_extensions"] = json!(["acme"]);
    let e: EvidenceRecord = serde_json::from_value(e).unwrap();
    assert_eq!(
        e.required_extensions.as_deref(),
        c.required_extensions.as_deref()
    );
    assert!(!e.extensions.contains_key("required_extensions"));
    let mut v = verification();
    v["required_extensions"] = json!(["acme"]);
    let v: VerificationResult = serde_json::from_value(v).unwrap();
    assert_eq!(
        v.required_extensions.as_deref(),
        c.required_extensions.as_deref()
    );
    assert!(!v.extensions.contains_key("required_extensions"));
    let mut i = index();
    i["required_extensions"] = json!(["acme"]);
    let i: CatalogIndex = serde_json::from_value(i).unwrap();
    assert_eq!(
        i.required_extensions.as_deref(),
        c.required_extensions.as_deref()
    );
}

fn presence<T: DeserializeOwned + Serialize>(base: Value) {
    assert!(
        roundtrip::<T>(base.clone())
            .get("required_extensions")
            .is_none()
    );
    for prefixes in [json!([]), json!(["acme"]), json!(["acme", "Acme"])] {
        let mut value = base.clone();
        value["required_extensions"] = prefixes.clone();
        assert_eq!(roundtrip::<T>(value)["required_extensions"], prefixes);
    }
    for malformed in [json!("acme"), json!({}), json!([1]), json!([null])] {
        let mut value = base.clone();
        value["required_extensions"] = malformed;
        assert!(serde_json::from_value::<T>(value).is_err());
    }
    let mut value = base;
    value["required_extensions"] = json!(["a", "acme:payload"]);
    assert_eq!(
        roundtrip::<T>(value)["required_extensions"],
        json!(["a", "acme:payload"])
    );
    // DTO string decoding is not namespace grammar or implemented recognition.
}

#[test]
fn absence_empty_lists_and_string_types_round_trip_in_all_existing_dtos() {
    presence::<BehavioralContract>(contract());
    presence::<EvidenceRecord>(evidence());
    presence::<VerificationResult>(verification());
    presence::<CatalogIndex>(index());
}

fn raw_payload<T: DeserializeOwned + Serialize>(mut value: Value) {
    let raw = r#"{ "z" : "\u0061", "numbers" : [1.2300e+04,  2] }"#;
    value["required_extensions"] = json!(["acme"]);
    value["acme:payload"] = json!("__RAW__");
    let input = serde_json::to_string(&value)
        .unwrap()
        .replace("\"__RAW__\"", raw);
    let document: T = serde_json::from_str(&input).unwrap();
    assert!(serde_json::to_string(&document).unwrap().contains(raw));
    let duplicate = input.replacen(
        "{",
        r#"{"required_extensions":[],"required_extensions":["other"],"#,
        1,
    );
    assert!(serde_json::from_str::<T>(&duplicate).is_err());
}

#[test]
fn explicit_declarations_do_not_normalize_payload_bytes_or_weaken_duplicate_rejection() {
    raw_payload::<BehavioralContract>(contract());
    raw_payload::<EvidenceRecord>(evidence());
    raw_payload::<VerificationResult>(verification());
}

#[test]
fn catalog_inventory_retains_unknown_requirements_without_granting_support() {
    let mut value = index();
    value["required_extensions"] = json!(["unimplemented"]);
    let material: BTreeMap<String, Vec<u8>> = value["schemas"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["catalog_path"].as_str().unwrap().to_owned(),
                clouddialect_contracts::schema::get_embedded_schema(
                    e["schema_id"].as_str().unwrap(),
                )
                .unwrap()
                .to_vec(),
            )
        })
        .collect();
    let input = serde_json::to_string(&value).unwrap();
    let parsed = CatalogIndex::from_parts(input.as_bytes(), &material).unwrap();
    assert_eq!(
        parsed.required_extensions.as_deref(),
        Some(["unimplemented".to_owned()].as_slice())
    );
    assert_eq!(
        serde_json::to_value(parsed).unwrap()["required_extensions"],
        json!(["unimplemented"])
    );
    let duplicate = input.replacen(
        "{",
        r#"{"required_extensions":[],"required_extensions":["other"],"#,
        1,
    );
    assert!(CatalogIndex::from_parts(duplicate.as_bytes(), &material).is_err());
    // Inventory is non-accepting; the future extension validator must block unknown semantics.
}
