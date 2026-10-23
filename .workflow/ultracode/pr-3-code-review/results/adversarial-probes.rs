
use clouddialect_contracts::*;
use clouddialect_contracts::schema::CatalogIndex;
use serde_json::{Value,json};
use std::{collections::BTreeMap,fs};
const ROOT:&str="/home/raishin/projects/CloudDialect";
fn parts()->(Vec<u8>,BTreeMap<String,Vec<u8>>) {
 let index=fs::read(format!("{ROOT}/catalog/index.json")).unwrap();
 let value:Value=serde_json::from_slice(&index).unwrap();
 let mut material=BTreeMap::new();
 for entry in value["schemas"].as_array().unwrap() {
  let path=entry["catalog_path"].as_str().unwrap();
  material.insert(path.to_string(),fs::read(format!("{ROOT}/catalog/{path}")).unwrap());
 }
 (index,material)
}
#[test]
fn nested_duplicate_core_fields_pass_structure_but_typed_decode_rejects() {
 let document=br#"{"id":"550e8400-e29b-41d4-a716-446655440000","format_version":"1.0.0","operation_identity":{"call_site":"first","call_site":"second"},"inputs":{},"outputs":{},"side_effects":[],"resource_dependencies":[],"obligations":[]}"#;
 let result=Validator::new(CatalogSnapshot::embedded().unwrap()).validate_structural(DocumentKind::BehavioralContract,document);
 let decoded=serde_json::from_slice::<BehavioralContract>(document);
 println!("nested duplicate: structural={:?}; typed_error={:?}",result.outcome,decoded.as_ref().err());
 assert_eq!(result.outcome,StructuralOutcome::Pass);
 assert!(decoded.is_err());
}
#[test]
fn admitted_catalog_serialization_fails_its_own_schema() {
 let (index,material)=parts();
 let admitted=CatalogIndex::from_parts(&index,&material).unwrap();
 let encoded=serde_json::to_vec(&admitted).unwrap();
 let value:Value=serde_json::from_slice(&encoded).unwrap();
 let result=CatalogIndex::from_parts(&encoded,&material);
 println!("catalog serialization: end_of_life_date={}; re-admission={:?}",value["schemas"][0]["end_of_life_date"],result.as_ref().err());
 assert!(value["schemas"][0].get("end_of_life_date").unwrap().is_null());
 assert!(result.is_err());
}
#[test]
fn nested_resource_identity_shadows_indexed_bytes() {
 let (index,mut material)=parts();
 let mut index:Value=serde_json::from_slice(&index).unwrap();
 let uri="urn:clouddialect:pr3-review-support";
 index["resolver_resources"]=json!([{"schema_id":uri,"catalog_path":"support/false.json"}]);
 material.insert("support/false.json".into(),b"false".to_vec());
 let path="schemas/evidence-record/1.0/schema.json";
 let mut schema:Value=serde_json::from_slice(&material[path]).unwrap();
 schema["$defs"]["shadow"]=json!({"$id":uri,"type":"object"});
 schema["$ref"]=json!(uri);
 material.insert(path.into(),serde_json::to_vec(&schema).unwrap());
 let snapshot=CatalogSnapshot::from_parts(serde_json::to_vec(&index).unwrap(),material).unwrap();
 assert_eq!(snapshot.schema_bytes(uri),Some(b"false".as_slice()));
 let validator=Validator::new(snapshot);
 let document=json!({"id":"550e8400-e29b-41d4-a716-446655440000","format_version":"1.0.0","claim":"claim","classification":"inferred","reasoning_chain":["reason"]});
 let result=validator.validate_structural(DocumentKind::EvidenceRecord,&serde_json::to_vec(&document).unwrap());
 println!("nested resource: indexed=false, actual outcome={:?}, errors={:?}",result.outcome,result.errors);
 assert_eq!(result.outcome,StructuralOutcome::Pass);
}

struct Store(StoredDocument);
impl DocumentStore for Store {
 fn get(&self,_:&str,_:Option<&str>)->Result<Option<StoredDocument>,ValidatorError>{Ok(Some(self.0.clone()))}
}
fn manifest_case(fidelity:&str,omit_identity:bool)->(String,Store){
 use sha2::{Sha256,Digest};
 let id="550e8400-e29b-41d4-a716-446655440000";
 let mut document=json!({"id":id,"format_version":"1.0.0","operation_identity":{"call_site":"pkg.call"},"inputs":{},"outputs":{},"side_effects":[],"resource_dependencies":[],"obligations":[{"obligation_id":"o1","behavioral_fidelity":fidelity,"evidence_refs":[id]}]});
 if omit_identity {document.as_object_mut().unwrap().remove("operation_identity");}
 let bytes=serde_json::to_vec(&document).unwrap();
 let mut manifest:toml::Value=toml::from_str(include_str!("/tmp/clouddialect-pr3-review-manifest.toml")).unwrap();
 manifest["contracts"]["upload_object"]["id"]=id.into();
 let content_hash=ContentHash{algorithm:"sha-256".into(),value:Sha256::digest(&bytes).iter().map(|b|format!("{b:02x}")).collect()};
 let store=Store(StoredDocument{reference:DocumentRef{id:id.into(),format_version:"1.0.0".into()},content_hash,bytes});
 (toml::to_string(&manifest).unwrap(),store)
}
#[test]
fn fidelity_diagnostic_indexes_an_array_by_string(){
 let (data,store)=manifest_case("adaptation-required",false);
 let result=ManifestValidator::new(CatalogSnapshot::embedded().unwrap()).validate_manifest_semantic(data.as_bytes(),&store);
 println!("fidelity path: {:?}",result.errors);
 assert_eq!(result.outcome,SemanticOutcome::Fail);
 assert_eq!(result.errors[0].field_path.as_deref(),Some("operations[\"upload_object\"].behavioral_fidelity"));
}
#[test]
fn contract_failure_is_reattributed_to_nonexistent_pack_field(){
 let (data,store)=manifest_case("equivalent-within-contract",true);
 let result=ManifestValidator::new(CatalogSnapshot::embedded().unwrap()).validate_manifest_semantic(data.as_bytes(),&store);
 println!("reference failure attribution: {:?}",result.errors);
 assert_eq!(result.outcome,SemanticOutcome::Fail);
 assert_eq!(result.errors[0].field_path.as_deref(),Some("contracts.upload_object.operation_identity"));
 assert_ne!(result.errors[0].document_id.as_deref(),Some("550e8400-e29b-41d4-a716-446655440000"));
}
