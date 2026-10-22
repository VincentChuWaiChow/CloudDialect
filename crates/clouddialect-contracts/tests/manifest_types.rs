use clouddialect_contracts::manifest::{
    CapabilityAction, CapabilityDeclaration, CapabilityRequirement, ManifestFidelity,
    ManifestOutcome, ManifestValidationResult, PackManifest,
};
use clouddialect_contracts::{ErrorCategory, ValidatorError, ValidatorWarning};
use serde_json::json;

const MANIFEST: &str = r#"
[pack]
id = "io.clouddialect.python.oss-to-s3"
version = "0.1.0-rc.1+build.1"
name = "OSS to S3"
description = "Example pack"
[source]
cloud = "alibaba"
service = "oss"
language = "python"
language_version_range = ">=3.8.0,<4.0.0"
[target]
cloud = "aws"
service = "s3"
[schema_refs]
contract_schema_version = "1.0.0"
evidence_record_schema_version = "1.0.0"
verification_result_schema_version = "1.0.0"
[compatibility]
catalog_min = "1.0.0"
catalog_max = "1.0.0"
[capabilities]
format_version = "1.0.0"
requirements = []
[[operations]]
id = "upload_object"
behavioral_fidelity = "partial"
assumptions = ["bucket exists"]
limitations = []
[[operations]]
id = "delete_object"
behavioral_fidelity = "unsupported"
[[toolchain]]
name = "boto3"
version_range = ">=1.28.0"
[contracts.upload_object]
id = "cef-contract-upload-001"
format_version = "1.0.0"
"#;

fn value() -> toml::Value {
    toml::from_str(MANIFEST).unwrap()
}

#[test]
fn complete_manifest_retains_all_sections_and_unsupported_operation() {
    let manifest: PackManifest = toml::from_str(MANIFEST).unwrap();
    assert_eq!(manifest.pack.version, "0.1.0-rc.1+build.1");
    assert_eq!(manifest.source.language_version_range, ">=3.8.0,<4.0.0");
    assert_eq!(manifest.target.service, "s3");
    assert_eq!(manifest.schema_refs.contract_schema_version, "1.0.0");
    assert_eq!(manifest.compatibility.catalog_min, "1.0.0");
    assert_eq!(manifest.capabilities.format_version, "1.0.0");
    assert!(manifest.capabilities.requirements.is_empty());
    assert_eq!(manifest.operations.len(), 2);
    assert_eq!(manifest.operations[1].id, "delete_object");
    assert_eq!(
        manifest.operations[1].behavioral_fidelity,
        ManifestFidelity::Unsupported
    );
    assert_eq!(
        manifest.operations[0].assumptions.as_deref(),
        Some(["bucket exists".into()].as_slice())
    );
    assert_eq!(manifest.operations[0].limitations, Some(vec![]));
    assert_eq!(manifest.operations[1].assumptions, None);
    assert_eq!(manifest.toolchain[0].name, "boto3");
    assert_eq!(manifest.contracts["upload_object"].format_version, "1.0.0");
    assert!(manifest.x.is_none());
}

#[test]
fn every_unconditional_core_section_is_required() {
    for section in [
        "pack",
        "source",
        "target",
        "schema_refs",
        "compatibility",
        "capabilities",
        "operations",
        "contracts",
    ] {
        let mut input = value();
        input.as_table_mut().unwrap().remove(section);
        assert!(
            input.try_into::<PackManifest>().is_err(),
            "missing {section}"
        );
    }
}

#[test]
fn required_nested_core_fields_are_not_defaulted() {
    for (section, fields) in [
        ("pack", &["id", "version", "name", "description"][..]),
        (
            "source",
            &["cloud", "service", "language", "language_version_range"][..],
        ),
        ("target", &["cloud", "service"][..]),
        (
            "schema_refs",
            &[
                "contract_schema_version",
                "evidence_record_schema_version",
                "verification_result_schema_version",
            ][..],
        ),
        ("compatibility", &["catalog_min", "catalog_max"][..]),
    ] {
        for field in fields {
            let mut input = value();
            input[section].as_table_mut().unwrap().remove(*field);
            assert!(
                input.try_into::<PackManifest>().is_err(),
                "missing {section}.{field}"
            );
        }
    }
}

#[test]
fn operation_toolchain_and_contract_entries_require_their_exact_fields() {
    for (section, fields) in [
        ("operations", &["id", "behavioral_fidelity"][..]),
        ("toolchain", &["name", "version_range"][..]),
        ("contracts", &["id", "format_version"][..]),
    ] {
        for field in fields {
            let mut input = value();
            let entry = if section == "contracts" {
                &mut input[section]["upload_object"]
            } else {
                &mut input[section].as_array_mut().unwrap()[0]
            };
            entry.as_table_mut().unwrap().remove(*field);
            assert!(
                input.try_into::<PackManifest>().is_err(),
                "missing {section}.{field}"
            );
        }
        let mut input = value();
        let entry = if section == "contracts" {
            &mut input[section]["upload_object"]
        } else {
            &mut input[section].as_array_mut().unwrap()[0]
        };
        entry
            .as_table_mut()
            .unwrap()
            .insert("authority".into(), toml::Value::Boolean(true));
        assert!(input.try_into::<PackManifest>().is_err());
    }
}

#[test]
fn absent_toolchain_is_representable_for_unsupported_only_packs() {
    let mut input = value();
    input.as_table_mut().unwrap().remove("toolchain");
    input["operations"].as_array_mut().unwrap().remove(0);
    let manifest: PackManifest = input.try_into().unwrap();
    assert!(manifest.toolchain.is_empty());
    assert_eq!(
        manifest.operations[0].behavioral_fidelity,
        ManifestFidelity::Unsupported
    );
}

#[test]
fn fidelity_is_closed_and_lowercase() {
    for (text, expected) in [
        ("full", ManifestFidelity::Full),
        ("partial", ManifestFidelity::Partial),
        ("unsupported", ManifestFidelity::Unsupported),
    ] {
        assert_eq!(
            serde_json::from_value::<ManifestFidelity>(json!(text)).unwrap(),
            expected
        );
    }
    for text in ["FULL", "equivalent-within-contract", "unknown", "accepted"] {
        assert!(serde_json::from_value::<ManifestFidelity>(json!(text)).is_err());
    }
}

fn requirement() -> serde_json::Value {
    json!({"id":"write-test-store", "operation_id":"upload_object", "action":"write", "resource":"test-object-store", "purpose":"behavioral-test"})
}

#[test]
fn capability_actions_have_exactly_nine_lowercase_variants() {
    for (text, action) in [
        ("read", CapabilityAction::Read),
        ("write", CapabilityAction::Write),
        ("execute", CapabilityAction::Execute),
        ("network", CapabilityAction::Network),
        ("model", CapabilityAction::Model),
        ("propose", CapabilityAction::Propose),
        ("approve", CapabilityAction::Approve),
        ("deploy", CapabilityAction::Deploy),
        ("delete", CapabilityAction::Delete),
    ] {
        assert_eq!(
            serde_json::from_value::<CapabilityAction>(json!(text)).unwrap(),
            action
        );
        assert_eq!(serde_json::to_value(action).unwrap(), json!(text));
    }
    for text in ["Read", "*", "admin", "accepted"] {
        assert!(serde_json::from_value::<CapabilityAction>(json!(text)).is_err());
    }
}

#[test]
fn capabilities_require_both_declared_version_and_explicit_requirement_list() {
    for input in [
        json!({"format_version":"1.0.0"}),
        json!({"requirements":[]}),
    ] {
        assert!(serde_json::from_value::<CapabilityDeclaration>(input).is_err());
    }
    let declaration: CapabilityDeclaration =
        serde_json::from_value(json!({"format_version":"1.0.0", "requirements":[requirement()]}))
            .unwrap();
    assert_eq!(declaration.requirements[0].resource, "test-object-store");
    let manifest_with_requirement = MANIFEST.replace("requirements = []", "[[capabilities.requirements]]\nid = 'write-test-store'\noperation_id = 'upload_object'\naction = 'write'\nresource = 'test-object-store'\npurpose = 'behavioral-test'");
    let manifest: PackManifest = toml::from_str(&manifest_with_requirement).unwrap();
    assert_eq!(
        manifest.capabilities.requirements[0].action,
        CapabilityAction::Write
    );
}

#[test]
fn capability_fields_are_required_and_unknown_fields_are_rejected() {
    for field in ["id", "operation_id", "action", "resource", "purpose"] {
        let mut input = requirement();
        input.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<CapabilityRequirement>(input).is_err(),
            "missing {field}"
        );
    }
    let mut input = requirement();
    input["grant"] = json!(true);
    assert!(serde_json::from_value::<CapabilityRequirement>(input).is_err());
    assert!(
        serde_json::from_value::<CapabilityDeclaration>(
            json!({"format_version":"1.0.0", "requirements":[], "grants":["approve"]})
        )
        .is_err()
    );
}

#[test]
fn deserialization_retains_claims_for_later_validation_not_admission() {
    let mut request = requirement();
    request["id"] = json!("");
    request["operation_id"] = json!("not-declared");
    request["action"] = json!("approve");
    request["resource"] = json!("../unresolved/*");
    let declaration: CapabilityDeclaration = serde_json::from_value(json!({
        "format_version": "99.0.0",
        "requirements": [request.clone(), request]
    }))
    .unwrap();
    // Version support, nonempty/unique IDs and operation references belong to
    // local validation; no request here resolves resources or grants authority.
    assert_eq!(declaration.format_version, "99.0.0");
    assert_eq!(declaration.requirements.len(), 2);
    assert_eq!(declaration.requirements[0], declaration.requirements[1]);
    assert_eq!(
        declaration.requirements[0].action,
        CapabilityAction::Approve
    );
    assert_eq!(declaration.requirements[0].resource, "../unresolved/*");
}

#[test]
fn unknown_core_fields_and_inline_contract_bodies_are_rejected() {
    for section in [
        None,
        Some("pack"),
        Some("source"),
        Some("target"),
        Some("schema_refs"),
        Some("compatibility"),
        Some("capabilities"),
    ] {
        let mut input = value();
        let table = match section {
            Some(section) => input[section].as_table_mut().unwrap(),
            None => input.as_table_mut().unwrap(),
        };
        table.insert("grant".into(), toml::Value::Boolean(true));
        assert!(
            input.try_into::<PackManifest>().is_err(),
            "unknown field in {section:?}"
        );
    }
    let inline_body = format!("{MANIFEST}\nobligations = []\n");
    assert!(toml::from_str::<PackManifest>(&inline_body).is_err());
}

#[test]
fn extensions_preserve_toml_values_but_do_not_populate_capabilities() {
    let input = format!(
        "{MANIFEST}\n[x]\ncapabilities = {{ grants = ['approve', 'deploy', 'delete'] }}\n\"vendor:precise\" = 9007199254740993\nobserved_at = 2026-10-22T12:00:00Z\n"
    );
    let manifest: PackManifest = toml::from_str(&input).unwrap();
    assert!(manifest.capabilities.requirements.is_empty());
    let extensions = manifest.x.unwrap();
    assert_eq!(
        extensions["vendor:precise"].as_integer(),
        Some(9007199254740993)
    );
    assert!(extensions["observed_at"].is_datetime());
    assert_eq!(
        extensions["capabilities"]["grants"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    let only_extension = input.replace(
        "[capabilities]\nformat_version = \"1.0.0\"\nrequirements = []\n",
        "",
    );
    assert!(toml::from_str::<PackManifest>(&only_extension).is_err());
}

#[test]
fn manifest_results_have_only_canonical_local_diagnostic_fields() {
    let result = ManifestValidationResult {
        outcome: ManifestOutcome::Fail,
        structural_errors: vec![ValidatorError {
            document_id: None,
            format_version: None,
            category: ErrorCategory::Unparseable,
            field_path: None,
            description: "invalid TOML".into(),
            byte_offset: Some(0),
        }],
        semantic_errors: vec![ValidatorError {
            document_id: Some("pack-1".into()),
            format_version: None,
            category: ErrorCategory::Semantic,
            field_path: Some("capabilities.requirements[0].operation_id".into()),
            description: "unknown operation".into(),
            byte_offset: None,
        }],
        warnings: vec![ValidatorWarning {
            field_name_verbatim: "vendor:preview".into(),
            description: "reserved namespace".into(),
        }],
    };
    let encoded = serde_json::to_value(&result).unwrap();
    assert_eq!(encoded.as_object().unwrap().len(), 4);
    assert_eq!(encoded["outcome"], "fail");
    assert_eq!(encoded["structural_errors"][0]["byte_offset"], 0);
    assert_eq!(encoded["semantic_errors"][0]["category"], "semantic");
    assert_eq!(
        encoded["warnings"][0]["field_name_verbatim"],
        "vendor:preview"
    );
    assert_eq!(
        serde_json::from_value::<ManifestValidationResult>(encoded).unwrap(),
        result
    );
    assert_eq!(serde_json::to_value(ManifestOutcome::Pass).unwrap(), "pass");
    for input in ["warn", "accepted", "evidence-complete"] {
        assert!(serde_json::from_value::<ManifestOutcome>(json!(input)).is_err());
    }
}
