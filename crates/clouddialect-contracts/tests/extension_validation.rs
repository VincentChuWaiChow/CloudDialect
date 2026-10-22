#[path = "support/documents.rs"]
mod documents;

use clouddialect_contracts::{
    CatalogSnapshot, DocumentKind, ErrorCategory, EvidenceRecord, StructuralOutcome, Validator,
};
use documents::minimal_documents;
use serde_json::{Value, json};

fn validator() -> Validator {
    Validator::new(CatalogSnapshot::embedded().unwrap())
}

fn validate(
    v: &Validator,
    kind: DocumentKind,
    value: &Value,
) -> clouddialect_contracts::StructuralResult {
    v.validate_structural(kind, &serde_json::to_vec(value).unwrap())
}

#[test]
fn reserved_exact_case_and_family_variants_warn_verbatim_without_failing_structure() {
    let v = validator();
    for (kind, base) in minimal_documents() {
        for prefix in [
            "core",
            "CORE",
            "Core-Validation",
            "cdc",
            "CDC",
            "Cdc-Evidence",
            "CORE-New-Family-17",
            "core-",
            "cdc--",
        ] {
            let key = format!("{prefix}:field");
            let mut value = base.clone();
            value[&key] = json!({"claimed_owner":"untrusted"});
            let r = validate(&v, kind, &value);
            assert_eq!(r.outcome, StructuralOutcome::Pass, "{kind:?}: {key}");
            assert!(r.errors.is_empty());
            assert_eq!(r.warnings.len(), 1, "{kind:?}: {key}");
            assert_eq!(r.warnings[0].field_name_verbatim, key);
            assert!(
                r.warnings[0]
                    .description
                    .contains(&format!("prefix '{prefix}' is reserved for core use"))
            );
        }
    }
}

#[test]
fn community_prefixes_and_nonreserved_neighbors_are_allowed_without_warnings() {
    let v = validator();
    for (kind, base) in minimal_documents() {
        for key in [
            "acme:custom",
            "ACME:custom",
            "acme-tools:v1:field",
            "acme-:",
            "corex:field",
            "CDC2:field",
        ] {
            let mut value = base.clone();
            value[key] = json!(true);
            let r = validate(&v, kind, &value);
            assert_eq!(
                r.outcome,
                StructuralOutcome::Pass,
                "{kind:?}: {key}: {:?}",
                r.errors
            );
            assert!(r.errors.is_empty() && r.warnings.is_empty());
        }
    }
}

#[test]
fn malformed_prefixes_fail_on_closed_open_and_minor_projection_roots() {
    let v = validator();
    let mut missed = Vec::new();
    for (kind, base) in minimal_documents() {
        for prefix in [
            "",
            "a",
            "1acme",
            "ac me",
            "acme_tools",
            "acme\\tools",
            "acme.",
            "équipe",
            "соre",
            "core-K",
            "acme\n",
            "acme\0",
        ] {
            for version in ["1.0.0", "1.1.0"] {
                let key = format!("{prefix}:field");
                let mut value = base.clone();
                value["format_version"] = json!(version);
                value[&key] = json!("PRIVATE-SENTINEL");
                let r = validate(&v, kind, &value);
                if r.outcome != StructuralOutcome::Fail {
                    missed.push(format!("{kind:?} {version} {key:?}: {:?}", r.outcome));
                } else {
                    assert!(
                        r.errors
                            .iter()
                            .any(|e| e.category == ErrorCategory::Structural)
                    );
                    assert!(
                        r.errors
                            .iter()
                            .all(|e| !e.description.contains("PRIVATE-SENTINEL"))
                    );
                }
            }
        }
    }
    assert!(missed.is_empty(), "{}", missed.join("\n"));
}

#[test]
fn reserved_warnings_coexist_with_minor_warning_without_promoting_failures() {
    let v = validator();
    let mut value = minimal_documents()[1].1.clone();
    value["format_version"] = json!("1.1.0");
    value["CORE:field"] = json!(true);
    let r = validate(&v, DocumentKind::EvidenceRecord, &value);
    assert_eq!(r.outcome, StructuralOutcome::Warn);
    assert_eq!(r.warnings.len(), 2);
    value["reasoning_chain"] = json!([]);
    assert_eq!(
        validate(&v, DocumentKind::EvidenceRecord, &value).outcome,
        StructuralOutcome::Fail
    );
    value["format_version"] = json!("2.0.0");
    let r = validate(&v, DocumentKind::EvidenceRecord, &value);
    assert_eq!(r.errors.len(), 1);
    assert_eq!(r.errors[0].category, ErrorCategory::UnsupportedMajorVersion);
    assert!(r.warnings.is_empty());
}

#[test]
fn nested_embedded_schema_keys_are_not_root_extension_declarations() {
    let v = validator();
    let mut value = minimal_documents()[0].1.clone();
    value["inputs"] = json!({"properties":{"CORE:field":{"type":"string"}},"acme:keyword":true});
    let r = validate(&v, DocumentKind::BehavioralContract, &value);
    assert_eq!(r.outcome, StructuralOutcome::Pass);
    assert!(r.warnings.is_empty());
}

#[test]
fn optional_payloads_cannot_override_core_fields_or_structural_failures() {
    let v = validator();
    for (kind, mut value) in minimal_documents() {
        value["acme:format_version"] = json!("9.0.0");
        value["acme:verification-status"] = json!("accepted");
        let r = validate(&v, kind, &value);
        assert_eq!(r.outcome, StructuralOutcome::Pass);
        assert_eq!(r.format_version.as_deref(), Some("1.0.0"));
        assert!(serde_json::to_value(&r).unwrap().get("accepted").is_none());
        value["format_version"] = json!("not-semver");
        assert_eq!(validate(&v, kind, &value).outcome, StructuralOutcome::Fail);
    }
}

#[test]
fn validation_does_not_rewrite_unknown_optional_encoded_value_bytes() {
    let v = validator();
    let base = serde_json::to_string(&minimal_documents()[1].1).unwrap();
    let raw = r#"{ "escaped":"\u0061", "n":1.2300e+04, "ordered":{"b":2,"a":1} }"#;
    let bytes = format!(
        "{},\"acme:payload\":{raw}}}",
        base.strip_suffix('}').unwrap()
    )
    .into_bytes();
    let original = bytes.clone();
    let r = v.validate_structural(DocumentKind::EvidenceRecord, &bytes);
    assert_eq!(r.outcome, StructuralOutcome::Pass);
    assert_eq!(bytes, original);
    let record: EvidenceRecord = serde_json::from_slice(&bytes).unwrap();
    assert!(serde_json::to_string(&record).unwrap().contains(raw));
}
