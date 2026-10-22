#[path = "support/documents.rs"]
mod documents;

use std::cell::Cell;

use clouddialect_contracts::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const ID: &str = "550e8400-e29b-41d4-a716-446655440000";

fn manifest(fidelity: &str) -> PackManifest {
    toml::from_str(&format!(r#"
[pack]
id="io.example.pack"
version="0.1.0"
name="test"
description="test"
[source]
cloud="source"
service="store"
language="python"
language_version_range="*"
[target]
cloud="target"
service="store"
[schema_refs]
contract_schema_version="1.0.0"
evidence_record_schema_version="1.0.0"
verification_result_schema_version="1.0.0"
[compatibility]
catalog_min="1.0.0"
catalog_max="1.0.0"
[capabilities]
format_version="1.0.0"
requirements=[]
[[operations]]
id="upload"
behavioral_fidelity="{fidelity}"
[[toolchain]]
name="tool"
version_range="*"
[contracts.upload]
id="{ID}"
format_version="1.0.0"
"#)).unwrap()
}

struct Store {
    record: Option<StoredDocument>,
    calls: Cell<usize>,
    unavailable: bool,
}

impl DocumentStore for Store {
    fn get(&self, id: &str, version: Option<&str>) -> Result<Option<StoredDocument>, ValidatorError> {
        self.calls.set(self.calls.get() + 1);
        assert_eq!(id, ID);
        assert_eq!(version, Some("1.0.0"));
        if self.unavailable {
            return Err(ValidatorError { document_id: None, format_version: None, category: ErrorCategory::UnresolvableReference, field_path: None, description: "PRIVATE-STORE-SECRET".into(), byte_offset: None });
        }
        Ok(self.record.clone())
    }
}

fn contract(fidelity: &str) -> Value {
    let mut value = documents::minimal_documents()[0].1.clone();
    value["obligations"] = json!([{"obligation_id":"o1","behavioral_fidelity":fidelity,"evidence_refs":[ID]}]);
    value
}

fn store(value: &Value) -> Store {
    let bytes = serde_json::to_vec(value).unwrap();
    Store { record: Some(StoredDocument { reference: DocumentRef { id:ID.into(), format_version:"1.0.0".into() }, content_hash:ContentHash { algorithm:"sha-256".into(), value:Sha256::digest(&bytes).iter().map(|b|format!("{b:02x}")).collect() }, bytes }), calls:Cell::new(0), unavailable:false }
}

fn validator() -> Validator { Validator::new(CatalogSnapshot::embedded().unwrap()) }

#[test]
fn full_and_partial_enforce_every_declared_obligation_fidelity() {
    for mode in ["full", "partial"] {
        for fidelity in ["equivalent-within-contract", "adaptation-required", "architectural-change-required", "unsupported", "unknown"] {
            let mut value = contract(fidelity);
            if fidelity=="unknown" { value["obligations"][0].as_object_mut().unwrap().remove("evidence_refs"); value["obligations"][0]["unresolved_assumption"]=json!({"assumption_id":"a1","description":"unknown","reason":"not observed"}); }
            let errors = check_fidelity_mapping(&manifest(mode), &store(&value), &validator());
            let permitted = fidelity=="equivalent-within-contract" || (mode=="partial" && fidelity=="adaptation-required");
            assert_eq!(errors.is_empty(),permitted,"{mode}/{fidelity}: {errors:?}");
            if !permitted { assert!(errors.iter().any(|e| e.category==ErrorCategory::Semantic && e.field_path.as_deref()==Some("operations[\"upload\"].behavioral_fidelity") && e.description.contains("o1"))); }
        }
    }
}

#[test]
fn unsupported_is_retained_and_does_not_require_a_fidelity_lookup() {
    let mut manifest = manifest("unsupported");
    manifest.contracts.clear();
    let source = Store { record:None,calls:Cell::new(0),unavailable:true };
    assert!(check_fidelity_mapping(&manifest,&source,&validator()).is_empty());
    assert_eq!(source.calls.get(),0);
    assert_eq!(manifest.operations.len(),1);
}

#[test]
fn missing_bindings_records_and_outages_never_become_success() {
    let mut manifest=manifest("full");
    manifest.contracts.clear();
    let mut source=store(&contract("equivalent-within-contract"));
    assert!(!check_fidelity_mapping(&manifest,&source,&validator()).is_empty());
    manifest= self::manifest("full");
    source.record=None;
    assert!(!check_fidelity_mapping(&manifest,&source,&validator()).is_empty());
    source.unavailable=true;
    let errors=check_fidelity_mapping(&manifest,&source,&validator());
    assert!(!errors.is_empty());
    assert!(errors.iter().all(|e|!e.description.contains("PRIVATE-STORE-SECRET")));
}

#[test]
fn corrupted_or_schema_invalid_records_are_not_used_for_fidelity() {
    let manifest=manifest("full");
    let mut source=store(&contract("equivalent-within-contract"));
    source.record.as_mut().unwrap().bytes.push(b' ');
    assert!(!check_fidelity_mapping(&manifest,&source,&validator()).is_empty());
    let mut invalid=contract("equivalent-within-contract");
    invalid["obligations"]=json!([]);
    assert!(!check_fidelity_mapping(&manifest,&store(&invalid),&validator()).is_empty());
}

#[test]
fn mandatory_semantics_block_and_all_fidelity_violations_are_reported_without_acceptance() {
    let manifest=manifest("full");
    let mut value=contract("adaptation-required");
    let mut second=value["obligations"][0].clone();second["obligation_id"]=json!("o2");
    value["obligations"].as_array_mut().unwrap().push(second);
    let errors=check_fidelity_mapping(&manifest,&store(&value),&validator());
    assert_eq!(errors.iter().filter(|e| e.category==ErrorCategory::Semantic).count(),2);
    value["required_extensions"]=json!(["acme"]);
    let errors=check_fidelity_mapping(&manifest,&store(&value),&validator());
    assert!(errors.iter().any(|e|e.category==ErrorCategory::BlockingGap));
    assert!(serde_json::to_string(&errors).unwrap().find("accepted").is_none());
}
