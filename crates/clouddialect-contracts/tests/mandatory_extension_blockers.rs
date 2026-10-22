#[path = "support/documents.rs"]
mod documents;
#[path = "../src/validator/mandatory_extensions.rs"]
mod mandatory_extensions;

pub use clouddialect_contracts::{ErrorCategory, ValidatorError};
use clouddialect_contracts::{CatalogSnapshot, StructuralOutcome, Validator};
use mandatory_extensions::check_mandatory_extensions;
use serde_json::{Value, json};

#[test]
fn unknown_declaration_without_payload_is_structurally_valid_but_separately_blocked() {
    let validator = Validator::new(CatalogSnapshot::embedded().unwrap());
    for (kind, mut value) in documents::minimal_documents() {
        value["required_extensions"] = json!(["acme", "CORE", "cdc-security"]);
        let structural = validator.validate_structural(kind, &serde_json::to_vec(&value).unwrap());
        assert_eq!(structural.outcome, StructuralOutcome::Pass, "{kind:?}");
        let blockers = check_mandatory_extensions(&value);
        assert_eq!(blockers.len(), 3);
        for (index, blocker) in blockers.iter().enumerate() {
            assert_eq!(blocker.category, ErrorCategory::BlockingGap);
            assert_eq!(blocker.field_path, Some(format!("required_extensions[{index}]")));
            assert_eq!(blocker.document_id, structural.document_id);
            assert_eq!(blocker.format_version, structural.format_version);
            assert!(blocker.description.contains(value["required_extensions"][index].as_str().unwrap()));
        }
        // Neither result type exposes an acceptance/grant field. Semantic
        // consumption of this private helper remains a separate pending unit.
        assert!(serde_json::to_value(structural).unwrap().get("accepted").is_none());
    }
}

#[test]
fn registry_membership_payloads_and_ownership_claims_cannot_clear_blockers() {
    for (_, mut value) in documents::minimal_documents() {
        value["required_extensions"] = json!(["acme"]);
        let baseline = check_mandatory_extensions(&value);
        value["acme:approved"] = json!({"owner":"trusted-core", "recognized":true, "accepted":true});
        value["namespaces"] = json!([{"prefix":"acme", "owner":"trusted-core"}]);
        assert_eq!(check_mandatory_extensions(&value), baseline);
        assert_eq!(baseline.len(), 1);
        assert_eq!(baseline[0].category, ErrorCategory::BlockingGap);
    }
}

#[test]
fn absent_or_explicit_empty_declaration_has_no_mandatory_blocker() {
    for (_, mut value) in documents::minimal_documents() {
        assert!(check_mandatory_extensions(&value).is_empty());
        value["required_extensions"] = json!([]);
        assert!(check_mandatory_extensions(&value).is_empty());
    }
}

#[test]
fn malformed_direct_helper_inputs_fail_closed_and_do_not_echo_payloads() {
    for declaration in [Value::Null, json!("PRIVATE-SENTINEL"), json!(false), json!({}), json!([false, "acme"])] {
        let value = json!({"required_extensions":declaration});
        let errors = check_mandatory_extensions(&value);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.category == ErrorCategory::Structural));
        assert!(errors.iter().all(|e| !e.description.contains("PRIVATE-SENTINEL")));
        assert!(errors.iter().all(|e| e.document_id.is_none() && e.format_version.is_none()));
    }
}

#[test]
fn diagnostics_escape_declared_names_without_interpreting_them_as_paths() {
    let value = json!({"id":"record", "format_version":"1.0.0", "required_extensions":["acme\nforged: accepted"]});
    let errors = check_mandatory_extensions(&value);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].category, ErrorCategory::BlockingGap);
    assert_eq!(errors[0].field_path.as_deref(), Some("required_extensions[0]"));
    assert!(!errors[0].description.contains('\n'));
    assert!(errors[0].description.contains("\\n"));
}
