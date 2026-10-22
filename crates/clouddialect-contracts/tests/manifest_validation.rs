use clouddialect_contracts::{
    CatalogSnapshot, ErrorCategory, ManifestFidelity, ManifestOutcome, ManifestValidationResult,
    ManifestValidator, PackManifest,
};

const MANIFEST: &str = r#"
# TOML 1.0-compatible pack declaration, not execution authority.
[pack]
id = "io.clouddialect.python.oss-to-s3"
version = "0.1.0-rc.1+build.1"
name = "OSS to S3"
description = "Local-validation example"
[source]
cloud = "alibaba"
service = "oss"
language = "python"
language_version_range = ">=3.8.0, <4.0.0"
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
behavioral_fidelity = "full"
assumptions = []
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

fn validator() -> ManifestValidator {
    ManifestValidator::new(CatalogSnapshot::embedded().unwrap())
}

fn validate(value: &toml::Value) -> ManifestValidationResult {
    validator().validate_manifest(toml::to_string(value).unwrap().as_bytes())
}

fn fails_at(result: &ManifestValidationResult, path: &str, category: ErrorCategory) {
    assert_eq!(result.outcome, ManifestOutcome::Fail, "{result:?}");
    assert!(result.structural_errors.is_empty(), "{result:?}");
    assert!(
        result.semantic_errors.iter().any(|error| {
            error.field_path.as_deref() == Some(path) && error.category == category
        }),
        "expected {path} ({category:?}): {result:?}"
    );
}

#[test]
fn complete_toml_10_manifest_passes_without_acceptance_or_external_reference_claims() {
    let result = validator().validate_manifest(MANIFEST.as_bytes());
    assert_eq!(result.outcome, ManifestOutcome::Pass, "{result:?}");
    assert!(result.structural_errors.is_empty());
    assert!(result.semantic_errors.is_empty());
    assert!(result.warnings.is_empty());
    let encoded = serde_json::to_value(result).unwrap();
    assert_eq!(encoded.as_object().unwrap().len(), 4);
    assert!(encoded.get("accepted").is_none());
    // Contract existence and catalog compatibility are not this local phase.
    let mut input = value();
    input["contracts"]["upload_object"]["id"] = "not-in-any-instance-store".into();
    input["schema_refs"]["contract_schema_version"] = "99.0.0".into();
    input["compatibility"]["catalog_min"] = "9.0.0".into();
    assert_eq!(validate(&input).outcome, ManifestOutcome::Pass);
}

#[test]
fn reverse_dns_pack_identifier_is_whole_string_ascii_and_dot_separated() {
    for id in ["io.example", "a.b", "a-1.b2-c"] {
        let mut input = value();
        input["pack"]["id"] = id.into();
        assert_eq!(validate(&input).outcome, ManifestOutcome::Pass, "{id}");
    }
    for id in [
        "",
        "single",
        "Io.example",
        "1a.b",
        ".a.b",
        "a..b",
        "a_b.c",
        "a.b\n",
        "é.b",
    ] {
        let mut input = value();
        input["pack"]["id"] = id.into();
        fails_at(&validate(&input), "pack.id", ErrorCategory::Semantic);
    }
}

#[test]
fn all_version_fields_use_full_semver_syntax_without_conflating_spaces() {
    for (section, field) in [
        ("pack", "version"),
        ("compatibility", "catalog_min"),
        ("compatibility", "catalog_max"),
        ("schema_refs", "contract_schema_version"),
        ("schema_refs", "evidence_record_schema_version"),
        ("schema_refs", "verification_result_schema_version"),
    ] {
        for version in [
            "1.2.3-rc.1",
            "1.2.3+build.1",
            "1.2.3-0",
            "1.2.3-999999999999999999999999999999999999999",
        ] {
            let mut input = value();
            input[section][field] = version.into();
            assert_eq!(
                validate(&input).outcome,
                ManifestOutcome::Pass,
                "{section}.{field}: {version}"
            );
        }
        for version in ["1.2", "01.2.3", "1.2.3-01", "1.2.3+", "1.2.3-", "1.2.3\n"] {
            let mut input = value();
            input[section][field] = version.into();
            fails_at(
                &validate(&input),
                &format!("{section}.{field}"),
                ErrorCategory::Version,
            );
        }
        let mut input = value();
        input[section][field] = "18446744073709551616.0.0".into();
        fails_at(
            &validate(&input),
            &format!("{section}.{field}"),
            ErrorCategory::UnsupportedVersion,
        );
    }
}

#[test]
fn fidelity_is_closed_and_executable_operations_need_nonempty_toolchain() {
    for fidelity in ["full", "partial"] {
        let mut input = value();
        input["operations"][0]["behavioral_fidelity"] = fidelity.into();
        assert_eq!(validate(&input).outcome, ManifestOutcome::Pass);
        input.as_table_mut().unwrap().remove("toolchain");
        fails_at(&validate(&input), "toolchain", ErrorCategory::Semantic);
        input
            .as_table_mut()
            .unwrap()
            .insert("toolchain".into(), toml::Value::Array(vec![]));
        fails_at(&validate(&input), "toolchain", ErrorCategory::Semantic);
    }
    for fidelity in ["FULL", "equivalent-within-contract", "full-featured"] {
        let mut input = value();
        input["operations"][0]["behavioral_fidelity"] = fidelity.into();
        let result = validate(&input);
        assert_eq!(result.outcome, ManifestOutcome::Fail);
        assert_eq!(
            result.structural_errors[0].category,
            ErrorCategory::Unparseable
        );
    }
}

#[test]
fn contract_references_are_exact_closed_shapes_with_nonempty_id_and_semver() {
    let mut input = value();
    input["contracts"]["upload_object"]["id"] = "".into();
    fails_at(
        &validate(&input),
        "contracts.upload_object.id",
        ErrorCategory::Semantic,
    );
    for version in ["1.2.3-rc.1+build.1", "1.2.3-0"] {
        let mut input = value();
        input["contracts"]["upload_object"]["format_version"] = version.into();
        assert_eq!(validate(&input).outcome, ManifestOutcome::Pass);
    }
    let mut input = value();
    input["contracts"]["upload_object"]["format_version"] = "1.0".into();
    fails_at(
        &validate(&input),
        "contracts.upload_object.format_version",
        ErrorCategory::Version,
    );
    for field in ["id", "format_version"] {
        let mut input = value();
        input["contracts"]["upload_object"]
            .as_table_mut()
            .unwrap()
            .remove(field);
        assert_eq!(
            validate(&input).structural_errors[0].category,
            ErrorCategory::Unparseable
        );
    }
    let mut input = value();
    input["contracts"]["upload_object"]
        .as_table_mut()
        .unwrap()
        .insert("obligations".into(), toml::Value::Array(vec![]));
    assert_eq!(
        validate(&input).structural_errors[0].category,
        ErrorCategory::Unparseable
    );
}

#[test]
fn unsupported_operations_remain_accessible_and_need_no_toolchain() {
    let mut input = value();
    input["operations"].as_array_mut().unwrap().remove(0);
    input.as_table_mut().unwrap().remove("toolchain");
    assert_eq!(validate(&input).outcome, ManifestOutcome::Pass);
    let parsed: PackManifest = input.try_into().unwrap();
    assert_eq!(parsed.operations.len(), 1);
    assert_eq!(parsed.operations[0].id, "delete_object");
    assert_eq!(
        parsed.operations[0].behavioral_fidelity,
        ManifestFidelity::Unsupported
    );
}

#[test]
fn extensions_preserve_toml_values_but_cannot_shadow_top_level_core_tables() {
    let extended = format!(
        "{MANIFEST}\n[x]\n\"vendor:counter\" = 9007199254740993\nobserved_at = 2026-10-22T12:00:00Z\nnested = {{ pack = 'opaque' }}\n"
    );
    assert_eq!(
        validator().validate_manifest(extended.as_bytes()).outcome,
        ManifestOutcome::Pass
    );
    let parsed: PackManifest = toml::from_str(&extended).unwrap();
    assert_eq!(
        parsed.x.as_ref().unwrap()["vendor:counter"].as_integer(),
        Some(9007199254740993)
    );
    assert!(parsed.x.as_ref().unwrap()["observed_at"].is_datetime());
    for key in [
        "pack",
        "source",
        "target",
        "schema_refs",
        "compatibility",
        "capabilities",
        "operations",
        "toolchain",
        "contracts",
        "x",
    ] {
        let mut input = value();
        input
            .as_table_mut()
            .unwrap()
            .insert("x".into(), toml::Value::Table(toml::map::Map::new()));
        input["x"]
            .as_table_mut()
            .unwrap()
            .insert(key.into(), toml::Value::Boolean(true));
        fails_at(
            &validate(&input),
            &format!("x.{key}"),
            ErrorCategory::Semantic,
        );
    }
    let mut input = value();
    input
        .as_table_mut()
        .unwrap()
        .insert("x".into(), "not-a-table".into());
    fails_at(&validate(&input), "x", ErrorCategory::Semantic);
}

fn capability() -> toml::Value {
    toml::from_str("id = 'test-store'\noperation_id = 'upload_object'\naction = 'write'\nresource = 'logical-store'\npurpose = 'behavioral-test'\n").unwrap()
}

fn with_capability() -> toml::Value {
    let mut input = value();
    input["capabilities"]["requirements"] = toml::Value::Array(vec![capability()]);
    input
}

#[test]
fn capability_version_fields_actions_and_operation_references_are_checked_locally() {
    for version in ["1.0.0", "1.0.0+build.1"] {
        let mut input = with_capability();
        input["capabilities"]["format_version"] = version.into();
        assert_eq!(validate(&input).outcome, ManifestOutcome::Pass);
    }
    for version in ["99.0.0", "1.0.1", "1.0.0-rc.1"] {
        let mut input = with_capability();
        input["capabilities"]["format_version"] = version.into();
        fails_at(
            &validate(&input),
            "capabilities.format_version",
            ErrorCategory::UnsupportedVersion,
        );
    }
    let mut input = with_capability();
    input["capabilities"]["format_version"] = "1.0".into();
    fails_at(
        &validate(&input),
        "capabilities.format_version",
        ErrorCategory::Version,
    );
    for field in ["id", "operation_id", "resource", "purpose"] {
        let mut input = with_capability();
        input["capabilities"]["requirements"][0][field] = "".into();
        fails_at(
            &validate(&input),
            &format!("capabilities.requirements[0].{field}"),
            ErrorCategory::Semantic,
        );
    }
    let mut input = with_capability();
    input["capabilities"]["requirements"][0]["operation_id"] = "not-declared".into();
    fails_at(
        &validate(&input),
        "capabilities.requirements[0].operation_id",
        ErrorCategory::Semantic,
    );
    let mut input = with_capability();
    input["capabilities"]["requirements"]
        .as_array_mut()
        .unwrap()
        .push(capability());
    fails_at(
        &validate(&input),
        "capabilities.requirements[1].id",
        ErrorCategory::Semantic,
    );
    for action in ["*", "Read", "admin"] {
        let mut input = with_capability();
        input["capabilities"]["requirements"][0]["action"] = action.into();
        assert_eq!(
            validate(&input).structural_errors[0].category,
            ErrorCategory::Unparseable
        );
    }
}

#[test]
fn capability_tables_are_required_closed_and_extensions_never_grant_authority() {
    for field in ["format_version", "requirements"] {
        let mut input = value();
        input["capabilities"].as_table_mut().unwrap().remove(field);
        assert_eq!(
            validate(&input).structural_errors[0].category,
            ErrorCategory::Unparseable
        );
    }
    for field in ["id", "operation_id", "action", "resource", "purpose"] {
        let mut input = with_capability();
        input["capabilities"]["requirements"][0]
            .as_table_mut()
            .unwrap()
            .remove(field);
        assert_eq!(
            validate(&input).structural_errors[0].category,
            ErrorCategory::Unparseable
        );
    }
    let mut input = with_capability();
    input["capabilities"]["requirements"][0]
        .as_table_mut()
        .unwrap()
        .insert("grant".into(), toml::Value::Boolean(true));
    assert_eq!(
        validate(&input).structural_errors[0].category,
        ErrorCategory::Unparseable
    );
    let mut input = value();
    input["capabilities"]
        .as_table_mut()
        .unwrap()
        .insert("grants".into(), toml::Value::Array(vec![]));
    assert_eq!(
        validate(&input).structural_errors[0].category,
        ErrorCategory::Unparseable
    );
    // All closed actions may be requested; Spec2 admission, not parsing, denies
    // prohibited worker actions. Resource strings are never expanded here.
    for action in [
        "read", "write", "execute", "network", "model", "propose", "approve", "deploy", "delete",
    ] {
        let mut input = with_capability();
        input["capabilities"]["requirements"][0]["action"] = action.into();
        input["capabilities"]["requirements"][0]["resource"] = "../unresolved/*".into();
        assert_eq!(validate(&input).outcome, ManifestOutcome::Pass);
    }
    let mut input = with_capability();
    input
        .as_table_mut()
        .unwrap()
        .insert("x".into(), toml::Value::Table(toml::map::Map::new()));
    input["x"]
        .as_table_mut()
        .unwrap()
        .insert("capabilities".into(), capability());
    fails_at(&validate(&input), "x.capabilities", ErrorCategory::Semantic);
    input.as_table_mut().unwrap().remove("capabilities");
    assert_eq!(
        validate(&input).structural_errors[0].category,
        ErrorCategory::Unparseable
    );
}

#[test]
fn source_and_toolchain_ranges_follow_cargo_comparator_syntax() {
    for range in [
        ">=1.2.3, <2.0.0",
        "^1.2",
        "~1.2.3",
        "*",
        "=1.2.3-rc.1",
        "1.2.3+build.1",
    ] {
        let mut input = value();
        input["source"]["language_version_range"] = range.into();
        input["toolchain"][0]["version_range"] = range.into();
        assert_eq!(validate(&input).outcome, ManifestOutcome::Pass, "{range}");
    }
    for range in [
        "",
        ">=1.0 <2.0",
        "1.0.0 || 2.0.0",
        "1.0.0 - 2.0.0",
        "@1.0.0",
        "1.2.3-01",
    ] {
        let mut input = value();
        input["source"]["language_version_range"] = range.into();
        fails_at(
            &validate(&input),
            "source.language_version_range",
            ErrorCategory::Semantic,
        );
        let mut input = value();
        input["toolchain"][0]["version_range"] = range.into();
        fails_at(
            &validate(&input),
            "toolchain[0].version_range",
            ErrorCategory::Semantic,
        );
    }
}

#[test]
fn malformed_utf8_toml_shapes_unknown_fields_and_duplicates_fail_without_input_leaks() {
    let parser = validator();
    let invalid_utf8 = parser.validate_manifest(&[b'#', 0xff]);
    assert_eq!(
        invalid_utf8.structural_errors[0].category,
        ErrorCategory::Unparseable
    );
    assert_eq!(invalid_utf8.structural_errors[0].byte_offset, Some(1));
    for input in [
        "",
        "pack = [",
        "secret = 'DO_NOT_ECHO_SECRET'\n",
        "[pack]\nid = 'DO_NOT_ECHO_SECRET'\nid = 'duplicate'\n",
    ] {
        let result = parser.validate_manifest(input.as_bytes());
        assert_eq!(result.outcome, ManifestOutcome::Fail);
        assert_eq!(result.structural_errors.len(), 1);
        assert_eq!(
            result.structural_errors[0].category,
            ErrorCategory::Unparseable
        );
        assert!(result.semantic_errors.is_empty());
        assert!(
            !serde_json::to_string(&result)
                .unwrap()
                .contains("DO_NOT_ECHO_SECRET")
        );
    }
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
        assert_eq!(
            validate(&input).structural_errors[0].category,
            ErrorCategory::Unparseable
        );
    }
    let mut input = value();
    input
        .as_table_mut()
        .unwrap()
        .insert("authority".into(), "DO_NOT_ECHO_SECRET".into());
    let result = validate(&input);
    assert_eq!(
        result.structural_errors[0].category,
        ErrorCategory::Unparseable
    );
    assert!(
        !serde_json::to_string(&result)
            .unwrap()
            .contains("DO_NOT_ECHO_SECRET")
    );
}

#[test]
fn local_diagnostics_are_deterministic_contextual_and_escape_opaque_contract_keys() {
    let mut input = value();
    input["pack"]["version"] = "1".into();
    let reference = input["contracts"]["upload_object"].clone();
    input["contracts"]
        .as_table_mut()
        .unwrap()
        .insert("z".into(), reference.clone());
    input["contracts"]
        .as_table_mut()
        .unwrap()
        .insert("a".into(), reference.clone());
    input["contracts"]["a"]["id"] = "".into();
    input["contracts"]["z"]["format_version"] = "1".into();
    input["contracts"]
        .as_table_mut()
        .unwrap()
        .insert("unsafe.key\n".into(), reference);
    input["contracts"]["unsafe.key\n"]["id"] = "".into();
    let result = validate(&input);
    assert_eq!(result, validate(&input));
    fails_at(
        &result,
        "contracts[\"unsafe.key\\n\"].id",
        ErrorCategory::Semantic,
    );
    for error in &result.semantic_errors {
        assert_eq!(
            error.document_id.as_deref(),
            Some("io.clouddialect.python.oss-to-s3")
        );
        assert!(
            error.format_version.is_none(),
            "pack version is not a document format version"
        );
        assert!(error.byte_offset.is_none());
    }
}
