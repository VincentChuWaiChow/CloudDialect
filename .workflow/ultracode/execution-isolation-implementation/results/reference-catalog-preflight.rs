use std::collections::BTreeMap;
use clouddialect_contracts::{CatalogSnapshot, DocumentKind, Validator};
use clouddialect_contracts::schema::CatalogIndex;
use serde_json::{Value,json};
fn main() {
 let root=std::path::Path::new("/home/raishin/projects/CloudDialect/catalog");
 let index=std::fs::read(root.join("index.json")).unwrap();
 let mut material=BTreeMap::new();
 for kind in ["behavioral-contract","evidence-record","verification-result","catalog-index","extension-registry"] {
  let path=format!("schemas/{kind}/1.0/schema.json");
  material.insert(path.clone(),std::fs::read(root.join(path)).unwrap());
 }
 let mut extended=String::from_utf8(index.clone()).unwrap();
 let end=extended.rfind('}').unwrap();extended.truncate(end);
 let raw=r#"{ "escaped":"\u0061", "number":1.2300e+04 }"#;
 extended.push_str(&format!(",\"acme:payload\":{raw}}}"));
 let checked=CatalogIndex::from_parts(extended.as_bytes(),&material).unwrap();
 let encoded=serde_json::to_string(&checked).unwrap();
 let input:Value=serde_json::from_str(&extended).unwrap();let output:Value=serde_json::from_str(&encoded).unwrap();
 assert!(input.get("acme:payload").is_some());assert!(output.get("acme:payload").is_none());
 println!("CONFIRMED: actual CatalogIndex::from_parts admits optional root extension, but typed serialization drops the entire payload.");
 let evidence=json!({"id":"550e8400-e29b-41d4-a716-446655440000","format_version":"1.0.0","claim":"claim","classification":"inferred","reasoning_chain":["explicit reason"]});
 let bytes=serde_json::to_vec(&evidence).unwrap();
 let bundled=Validator::new(CatalogSnapshot::embedded().unwrap()).validate_structural(DocumentKind::EvidenceRecord,&bytes);
 let path="schemas/evidence-record/1.0/schema.json";
 let mut schema:Value=serde_json::from_slice(&material[path]).unwrap();
 schema["properties"]["explicit_catalog_requirement"]=json!({"const":true});
 schema["required"].as_array_mut().unwrap().push(json!("explicit_catalog_requirement"));
 material.insert(path.to_owned(),serde_json::to_vec(&schema).unwrap());
 let admitted=Validator::new(CatalogSnapshot::from_parts(index,material).unwrap()).validate_structural(DocumentKind::EvidenceRecord,&bytes);
 assert_eq!(bundled.outcome,clouddialect_contracts::StructuralOutcome::Pass);
 assert_eq!(admitted.outcome,clouddialect_contracts::StructuralOutcome::Fail);
 assert!(admitted.errors.iter().any(|e|e.field_path.as_deref()==Some("explicit_catalog_requirement")));
 println!("CONFIRMED: identical evidence bytes pass embedded catalog and fail an explicitly supplied valid snapshot. A standalone helper must receive the active validation context to select the intended policy.");
}