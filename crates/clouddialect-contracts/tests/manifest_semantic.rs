use std::cell::Cell;
use clouddialect_contracts::*;

struct Missing(Cell<usize>);
impl DocumentStore for Missing {
    fn get(&self,_:&str,_:Option<&str>)->Result<Option<StoredDocument>,ValidatorError> {
        self.0.set(self.0.get()+1); Ok(None)
    }
}

fn input()->String {
    // Reuse the actual valid fixture literal, not a second divergent manifest.
    let source=include_str!("manifest_validation.rs");
    source.split("const MANIFEST: &str = r#\"").nth(1).unwrap().split("\"#;").next().unwrap().to_owned()
}

#[test]
fn local_pass_missing_contract_semantic_fail_and_trait_dispatch_remain_separate() {
    let v=ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    let data=input();let store=Missing(Cell::new(0));
    assert_eq!(v.validate_manifest(data.as_bytes()).outcome,ManifestOutcome::Pass);
    let checker:&dyn ValidateManifest=&v;
    let semantic=checker.validate_manifest_semantic(data.as_bytes(),&store);
    assert_eq!(semantic.outcome,SemanticOutcome::Fail);
    assert!(semantic.errors.iter().any(|e|e.category==ErrorCategory::UnresolvableReference && e.field_path.as_deref()==Some("contracts.upload_object.id")));
    assert_eq!(store.0.get(),1);
    assert!(serde_json::to_value(semantic).unwrap().get("accepted").is_none());
}

#[test]
fn invalid_local_manifest_never_queries_store() {
    let v=ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    let store=Missing(Cell::new(0));
    assert_eq!(v.validate_manifest_semantic(b"not toml",&store).outcome,SemanticOutcome::Fail);
    assert_eq!(store.0.get(),0);
}

#[test]
fn unsupported_without_contracts_is_semantic_pass_not_authority() {
    let v=ManifestValidator::new(CatalogSnapshot::embedded().unwrap());let store=Missing(Cell::new(0));
    let mut doc:toml::Value=toml::from_str(&input()).unwrap();
    doc["operations"][0]["behavioral_fidelity"]="unsupported".into();
    doc["contracts"]=toml::Value::Table(toml::map::Map::new());
    let bytes=toml::to_string(&doc).unwrap();
    let result=v.validate_manifest_semantic(bytes.as_bytes(),&store);
    assert_eq!(result.outcome,SemanticOutcome::Pass);
    assert!(result.errors.is_empty());assert_eq!(store.0.get(),0);
}
