use std::collections::BTreeMap;

use clouddialect_contracts::{
    CatalogSnapshot, DocumentKind as K, ErrorCategory as E, StructuralOutcome as O, Validate,
    Validator,
};
use serde_json::{Value, json};

const ID: &str = "550e8400-e29b-41d4-a716-446655440000";

fn documents() -> [(K, Value); 5] {
    let artifact = json!({"identity":"artifact","content_hash":{"algorithm":"sha-256","value":"a".repeat(64)}});
    [
        (
            K::BehavioralContract,
            json!({"id":ID,"format_version":"1.0.0","operation_identity":{"call_site":"pkg.call"},"inputs":{},"outputs":{},"side_effects":[],"resource_dependencies":[],"obligations":[{"obligation_id":"o1","behavioral_fidelity":"unknown","unresolved_assumption":{"assumption_id":"a1","description":"unknown","reason":"not observed"}}]}),
        ),
        (
            K::EvidenceRecord,
            json!({"id":ID,"format_version":"1.0.0","claim":"claim","classification":"inferred","reasoning_chain":["explicit reason"]}),
        ),
        (
            K::VerificationResult,
            json!({"id":ID,"format_version":"1.0.0","candidate":artifact,"configuration":artifact,"pack":{"id":"pack","version":"1.0.0","content_hash":artifact["content_hash"]},"toolchain":{"id":"tool","version":"1.0.0","content_hash":artifact["content_hash"]},"binding":{"tenant_id":"tenant","project_id":"project","run_id":"run","attempt_id":"attempt","input":artifact,"dependencies":artifact,"suite":artifact,"oracle":artifact,"policy":artifact,"obligation_ids":["o1"]},"tests":[{"test_id":"t1","obligation_ids":["o1"],"outcome":"pass","origin_identity":"untrusted-claim"}]}),
        ),
        (
            K::CatalogIndex,
            serde_json::from_slice(include_bytes!("../../../catalog/index.json")).unwrap(),
        ),
        (
            K::ExtensionRegistry,
            serde_json::from_slice(include_bytes!("../../../catalog/extension-registry.json"))
                .unwrap(),
        ),
    ]
}

fn validator() -> Validator {
    Validator::new(CatalogSnapshot::embedded().unwrap())
}

fn validate(v: &dyn Validate, kind: K, value: &Value) -> clouddialect_contracts::StructuralResult {
    v.validate_structural(kind, &serde_json::to_vec(value).unwrap())
}

fn parts() -> (Value, BTreeMap<String, Vec<u8>>) {
    let index: Value =
        serde_json::from_slice(include_bytes!("../../../catalog/index.json")).unwrap();
    let material = [
        (
            "behavioral-contract",
            include_bytes!("../../../catalog/schemas/behavioral-contract/1.0/schema.json")
                .as_slice(),
        ),
        (
            "evidence-record",
            include_bytes!("../../../catalog/schemas/evidence-record/1.0/schema.json").as_slice(),
        ),
        (
            "verification-result",
            include_bytes!("../../../catalog/schemas/verification-result/1.0/schema.json")
                .as_slice(),
        ),
        (
            "catalog-index",
            include_bytes!("../../../catalog/schemas/catalog-index/1.0/schema.json").as_slice(),
        ),
        (
            "extension-registry",
            include_bytes!("../../../catalog/schemas/extension-registry/1.0/schema.json")
                .as_slice(),
        ),
    ]
    .into_iter()
    .map(|(kind, bytes)| (format!("schemas/{kind}/1.0/schema.json"), bytes.to_vec()))
    .collect();
    (index, material)
}

fn from_parts(index: Value, material: BTreeMap<String, Vec<u8>>) -> Validator {
    Validator::new(
        CatalogSnapshot::from_parts(serde_json::to_vec(&index).unwrap(), material).unwrap(),
    )
}

#[test]
fn all_five_trusted_kinds_pass_and_round_trip_without_other_phase_claims() {
    let v = validator();
    for (kind, value) in documents() {
        let r = validate(&v, kind, &value);
        assert_eq!(r.outcome, O::Pass, "{kind:?}: {:?}", r.errors);
        assert!(r.errors.is_empty() && r.warnings.is_empty());
        assert_eq!(r.format_version.as_deref(), Some("1.0.0"));
        let roundtrip: Value =
            serde_json::from_slice(&serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(validate(&v, kind, &roundtrip), r);
        let fields = serde_json::to_value(r).unwrap();
        assert!(fields.get("accepted").is_none());
        assert!(fields.get("semantic").is_none());
        assert!(fields.get("behavioral").is_none());
    }
}

#[test]
fn instance_schema_is_optional_but_must_equal_the_selected_admitted_uri() {
    let v = validator();
    for (kind, mut value) in documents() {
        value.as_object_mut().unwrap().remove("$schema");
        assert_eq!(validate(&v, kind, &value).outcome, O::Pass);
        let index: Value =
            serde_json::from_slice(include_bytes!("../../../catalog/index.json")).unwrap();
        let name = serde_json::to_value(kind).unwrap();
        let selected = index["schemas"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["document_type"] == name)
            .unwrap()["schema_id"]
            .as_str()
            .unwrap();
        value["$schema"] = json!(selected);
        assert_eq!(validate(&v, kind, &value).outcome, O::Pass);
        for bad in [
            json!("https://unlisted.invalid/schema"),
            json!(format!("{selected}#")),
            json!("https://json-schema.org/draft/2020-12/schema"),
            json!("file:///tmp/unlisted.json"),
            json!(17),
            json!(null),
            json!(""),
        ] {
            value["$schema"] = bad;
            let r = validate(&v, kind, &value);
            assert_eq!(r.outcome, O::Fail);
            assert!(
                r.errors
                    .iter()
                    .any(|e| e.field_path.as_deref() == Some("$schema"))
            );
        }
    }
    let evidence = &documents()[1].1;
    assert_eq!(
        validate(&v, K::BehavioralContract, evidence).outcome,
        O::Fail
    );
}

#[test]
fn missing_or_non_string_versions_never_guess_a_schema() {
    let v = validator();
    for bad in [None, Some(json!(null)), Some(json!(1)), Some(json!({}))] {
        let mut value = documents()[1].1.clone();
        value.as_object_mut().unwrap().remove("format_version");
        if let Some(bad) = bad {
            value["format_version"] = bad;
        }
        let r = validate(&v, K::EvidenceRecord, &value);
        assert_eq!(r.outcome, O::Fail);
        assert_eq!(r.errors.len(), 1);
        assert_eq!(r.errors[0].category, E::Structural);
        assert_eq!(r.errors[0].field_path.as_deref(), Some("format_version"));
    }
}

#[test]
fn actual_production_dispatch_enforces_every_version_disposition() {
    let v = validator();
    for (declared, category) in [
        ("1.0.1", E::UnsupportedVersion),
        ("1.0.0-alpha+build", E::UnsupportedVersion),
        ("1.0.18446744073709551616", E::UnsupportedVersion),
        ("2.0.0", E::UnsupportedMajorVersion),
        ("0.1.0", E::UnsupportedMajorVersion),
        ("01.0.0", E::Version),
        ("1.0.0\n", E::Version),
    ] {
        let value = json!({"id":ID,"format_version":declared,"claim":"PRIVATE-SENTINEL"});
        let r = validate(&v, K::EvidenceRecord, &value);
        assert_eq!(r.outcome, O::Fail);
        assert_eq!(
            r.errors.len(),
            1,
            "no partial schema diagnostics: {declared}"
        );
        let e = &r.errors[0];
        assert_eq!(e.category, category);
        assert_eq!(e.document_id.as_deref(), Some(ID));
        assert_eq!(e.format_version.as_deref(), Some(declared));
        assert_eq!(e.field_path.as_deref(), Some("format_version"));
        assert!(!e.description.contains("PRIVATE-SENTINEL"));
        if matches!(category, E::UnsupportedVersion | E::UnsupportedMajorVersion) {
            assert!(e.description.contains(declared));
            assert!(e.description.contains("compatibility-policy"));
            assert!(
                e.description
                    .contains(if category == E::UnsupportedVersion {
                        "unsupported-version"
                    } else {
                        "unsupported-major-version"
                    })
            );
        }
        assert!(r.warnings.is_empty());
    }
}

#[test]
fn build_metadata_does_not_create_a_new_schema_uri_or_change_reported_version() {
    let v = validator();
    for (kind, mut value) in documents() {
        value.as_object_mut().unwrap().remove("$schema");
        value["format_version"] = json!("1.0.0+build.001");
        let r = validate(&v, kind, &value);
        assert_eq!(r.outcome, O::Pass, "{kind:?}: {:?}", r.errors);
        assert_eq!(r.format_version.as_deref(), Some("1.0.0+build.001"));
        value["$schema"] =
            json!("https://schemas.clouddialect.io/evidence-record/1.0.0+build.001/schema.json");
        assert_eq!(validate(&v, kind, &value).outcome, O::Fail);
    }
}

#[test]
fn newer_minor_projects_only_root_additions_and_warns_with_unambiguous_paths() {
    let v = validator();
    for (kind, mut value) in documents() {
        value.as_object_mut().unwrap().remove("$schema");
        value["format_version"] = json!("1.1.7+future");
        value["future_field"] = json!({"opaque":[1,2,3]});
        value["future.field"] = json!(true);
        value["quote\"\nfield"] = json!(null);
        let bytes = serde_json::to_vec(&value).unwrap();
        let original = bytes.clone();
        let r = v.validate_structural(kind, &bytes);
        assert_eq!(r.outcome, O::Warn, "{kind:?}: {:?}", r.errors);
        assert!(r.errors.is_empty());
        assert_eq!(r.format_version.as_deref(), Some("1.1.7+future"));
        assert_eq!(bytes, original);
        assert!(
            r.warnings
                .iter()
                .any(|w| w.description.contains("1.1.7+future") && w.description.contains("1.0.0"))
        );
        for (key, path) in [
            ("future_field", "future_field"),
            ("future.field", "[\"future.field\"]"),
            ("quote\"\nfield", "[\"quote\\\"\\nfield\"]"),
        ] {
            assert!(
                r.warnings
                    .iter()
                    .any(|w| w.field_name_verbatim == key && w.description.contains(path)),
                "missing exact warning {key:?}"
            );
        }
        assert!(
            !r.warnings
                .iter()
                .any(|w| w.field_name_verbatim == "reasoning_chain")
        );
        let kind_name = serde_json::to_value(kind).unwrap();
        value["$schema"] = json!(format!(
            "https://schemas.clouddialect.io/{}/1.0.0/schema.json",
            kind_name.as_str().unwrap()
        ));
        assert_eq!(validate(&v, kind, &value).outcome, O::Warn);
        value["$schema"] =
            json!("https://schemas.clouddialect.io/evidence-record/1.1.7/schema.json");
        assert_eq!(validate(&v, kind, &value).outcome, O::Fail);
    }
}

#[test]
fn newer_minor_still_checks_known_conditional_fields_and_required_children() {
    let v = validator();
    let mut value = documents()[1].1.clone();
    value["format_version"] = json!("1.1.0");
    for bad in [json!([]), json!(false), json!([17])] {
        value["reasoning_chain"] = bad;
        assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Fail);
    }
    value.as_object_mut().unwrap().remove("reasoning_chain");
    assert!(
        validate(&v, K::EvidenceRecord, &value)
            .errors
            .iter()
            .any(|e| e.field_path.as_deref() == Some("reasoning_chain"))
    );
    value["classification"] = json!("observed");
    value["provenance"] =
        json!({"source_artifact_identity":"source","analysis_tool_identity":"tool"});
    assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Warn);
    value["provenance"]
        .as_object_mut()
        .unwrap()
        .remove("analysis_tool_identity");
    assert!(
        validate(&v, K::EvidenceRecord, &value)
            .errors
            .iter()
            .any(|e| e.field_path.as_deref() == Some("provenance.analysis_tool_identity"))
    );
    value["provenance"]["analysis_tool_identity"] = json!("tool");
    value["reasoning_chain"] = json!(["inactive branch field must not be stripped"]);
    assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Fail);
}

#[test]
fn projection_never_strips_nested_closed_fields_or_embedded_schema_content() {
    let v = validator();
    let mut contract = documents()[0].1.clone();
    contract["format_version"] = json!("1.1.0");
    contract["inputs"] = json!({"type":"object","properties":{"future":{"type":"string"}},"custom_keyword":{"arbitrary":true}});
    assert_eq!(
        validate(&v, K::BehavioralContract, &contract).outcome,
        O::Warn
    );
    let mut verification = documents()[2].1.clone();
    verification["format_version"] = json!("1.1.0");
    verification["tests"][0]["future_nested"] = json!(true);
    let r = validate(&v, K::VerificationResult, &verification);
    assert_eq!(r.outcome, O::Fail);
    assert!(
        r.errors
            .iter()
            .any(|e| e.field_path.as_deref() == Some("tests[0].future_nested"))
    );
}

#[test]
fn required_errors_have_child_paths_context_and_no_private_input_values() {
    let v = validator();
    let mut value = documents()[2].1.clone();
    value["tests"][0]["outcome"] = json!("PRIVATE-SENTINEL");
    value["tests"][0].as_object_mut().unwrap().remove("test_id");
    value["binding"]
        .as_object_mut()
        .unwrap()
        .remove("tenant_id");
    let r = validate(&v, K::VerificationResult, &value);
    assert_eq!(r.outcome, O::Fail);
    for path in ["tests[0].outcome", "tests[0].test_id", "binding.tenant_id"] {
        assert!(
            r.errors
                .iter()
                .any(|e| e.field_path.as_deref() == Some(path)),
            "{path}: {:?}",
            r.errors
        );
    }
    assert!(r.errors.iter().all(|e| e.category == E::Structural
        && e.document_id.as_deref() == Some(ID)
        && e.format_version.as_deref() == Some("1.0.0")
        && !e.description.contains("PRIVATE-SENTINEL")));
}

#[test]
fn malformed_json_reports_bounded_approximate_offsets_and_scalars_fail_structurally() {
    let v = validator();
    for bytes in [
        b"".as_slice(),
        b"{\n  \"id\": \"unicode\xc3\xa9\",\n  \"format_version\": ",
        b"{\"id\":\xff}",
    ] {
        let r = v.validate_structural(K::EvidenceRecord, bytes);
        assert_eq!(r.outcome, O::Fail);
        assert_eq!(r.errors[0].category, E::Unparseable);
        assert!(r.errors[0].byte_offset.unwrap() <= bytes.len());
        assert!(r.document_id.is_none() && r.format_version.is_none());
    }
    for value in [
        json!(null),
        json!([]),
        json!("valid JSON, wrong root"),
        json!(17),
    ] {
        let r = validate(&v, K::EvidenceRecord, &value);
        assert_eq!(r.outcome, O::Fail);
        assert_eq!(r.errors[0].category, E::Structural);
    }
}

#[test]
fn duplicate_core_members_cannot_change_version_or_required_extensions() {
    let v = validator();
    for bytes in [
        r#"{"format_version":"2.0.0","format_version":"1.0.0"}"#,
        r#"{"required_extensions":["unimplemented"],"required_extensions":[],"format_version":"1.0.0"}"#,
        r#"{"format_version":"2.0.0","\u0066ormat_version":"1.0.0"}"#,
    ] {
        let r = v.validate_structural(K::EvidenceRecord, bytes.as_bytes());
        assert_eq!(r.outcome, O::Fail);
        assert_eq!(r.errors[0].category, E::Unparseable);
    }
}

#[test]
fn explicit_catalog_selection_not_an_embedded_or_same_minor_patch_default() {
    let (mut index, mut material) = parts();
    let old = "schemas/evidence-record/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[old]).unwrap();
    let uri = "https://schemas.clouddialect.io/evidence-record/1.0.1/schema.json";
    schema["$id"] = json!(uri);
    schema["required"]
        .as_array_mut()
        .unwrap()
        .push(json!("trusted_marker"));
    schema["properties"]["trusted_marker"] = json!({"type":"boolean"});
    material.insert(old.to_owned(), serde_json::to_vec(&schema).unwrap());
    index["schemas"][1]["schema_id"] = json!(uri);
    index["schemas"][1]["format_version"] = json!("1.0.1");
    let v = from_parts(index, material);
    let mut value = documents()[1].1.clone();
    assert_eq!(
        validate(&v, K::EvidenceRecord, &value).errors[0].category,
        E::UnsupportedVersion
    );
    value["format_version"] = json!("1.0.1");
    assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Fail);
    value["trusted_marker"] = json!(true);
    value["$schema"] = json!(uri);
    assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Pass);
    value["format_version"] = json!("1.1.0");
    assert_eq!(
        validate(&v, K::EvidenceRecord, &value).errors[0].category,
        E::UnsupportedVersion
    );
}

#[test]
fn missing_external_schema_resource_fails_closed_with_safe_diagnostics() {
    let (index, mut material) = parts();
    let path = "schemas/evidence-record/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
    schema["$ref"] = json!("https://unlisted.invalid/PRIVATE-SCHEMA-SENTINEL");
    material.insert(path.to_owned(), serde_json::to_vec(&schema).unwrap());
    let r = validate(
        &from_parts(index, material),
        K::EvidenceRecord,
        &documents()[1].1,
    );
    assert_eq!(r.outcome, O::Fail);
    assert_eq!(r.errors[0].category, E::UnresolvableReference);
    assert!(!r.errors[0].description.contains("PRIVATE-SCHEMA-SENTINEL"));
}

#[test]
fn schema_formats_are_assertions_not_only_annotations() {
    let v = validator();
    let mut value = documents()[2].1.clone();
    let artifact = value["candidate"].clone();
    value["acceptance"] = json!({"approver_identities":["owner"],"timestamp":"2025-02-30T12:00:00Z","verification_result":{"id":ID,"format_version":"1.0.0"},"approval_evidence":artifact});
    let r = validate(&v, K::VerificationResult, &value);
    assert_eq!(r.outcome, O::Fail);
    assert!(
        r.errors
            .iter()
            .any(|e| e.field_path.as_deref() == Some("acceptance.timestamp"))
    );
}

#[test]
fn newer_minor_does_not_bypass_declared_root_wildcard_constraints() {
    let (index, mut material) = parts();
    let path = "schemas/extension-registry/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
    schema["additionalProperties"] = json!({"type":"integer"});
    material.insert(path.to_owned(), serde_json::to_vec(&schema).unwrap());
    let v = from_parts(index, material);
    let mut value = documents()[4].1.clone();
    value["format_version"] = json!("1.1.0");
    value["wildcard_field"] = json!("PRIVATE-SENTINEL");
    assert_eq!(validate(&v, K::ExtensionRegistry, &value).outcome, O::Fail);
    value["wildcard_field"] = json!(17);
    let r = validate(&v, K::ExtensionRegistry, &value);
    assert_eq!(r.outcome, O::Warn);
    assert!(
        !r.warnings
            .iter()
            .any(|w| w.field_name_verbatim == "wildcard_field")
    );
}

#[test]
fn root_composition_and_local_reference_fields_remain_checked() {
    for reference in [false, true] {
        let (index, mut material) = parts();
        let path = "schemas/evidence-record/1.0/schema.json";
        let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
        let constraint = json!({"properties":{"trusted_marker":{"type":"integer"}},"required":["trusted_marker"]});
        if reference {
            schema["$defs"] = json!({"root_constraint":constraint});
            schema["$ref"] = json!("#/$defs/root_constraint");
        } else {
            schema["allOf"] = json!([constraint]);
        }
        material.insert(path.to_owned(), serde_json::to_vec(&schema).unwrap());
        let v = from_parts(index, material);
        let mut value = documents()[1].1.clone();
        value["format_version"] = json!("1.1.0");
        value["trusted_marker"] = json!("not an integer");
        assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Fail);
        value["trusted_marker"] = json!(17);
        assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Warn);
    }
}

#[test]
fn projection_preserves_root_dependent_required_names() {
    let (index, mut material) = parts();
    let path = "schemas/extension-registry/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
    schema["dependentRequired"] = json!({"trigger":["companion"]});
    material.insert(path.to_owned(), serde_json::to_vec(&schema).unwrap());
    let v = from_parts(index, material);
    let mut value = documents()[4].1.clone();
    value["format_version"] = json!("1.1.0");
    value["trigger"] = json!(true);
    value["companion"] = json!(true);
    assert_eq!(validate(&v, K::ExtensionRegistry, &value).outcome, O::Warn);
    value.as_object_mut().unwrap().remove("companion");
    assert_eq!(validate(&v, K::ExtensionRegistry, &value).outcome, O::Fail);
}

#[test]
fn changed_root_reference_scope_never_causes_guessed_property_removal() {
    let (index, mut material) = parts();
    let path = "schemas/evidence-record/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
    schema["$defs"] = json!({"constraint":{"properties":{"wrong_scope_marker":{}}}});
    schema["allOf"] = json!([{"$id":"urn:clouddialect:scoped-root", "$defs":{"constraint":{"properties":{"scoped_marker":{"type":"integer"}}}}, "$ref":"#/$defs/constraint"}]);
    material.insert(path.to_owned(), serde_json::to_vec(&schema).unwrap());
    let v = from_parts(index, material);
    let mut value = documents()[1].1.clone();
    value["scoped_marker"] = json!("not an integer");
    assert_eq!(validate(&v, K::EvidenceRecord, &value).outcome, O::Fail);
    value["format_version"] = json!("1.1.0");
    let r = validate(&v, K::EvidenceRecord, &value);
    assert_eq!(r.outcome, O::Fail);
    assert_eq!(r.errors[0].category, E::Unsupported);
}
