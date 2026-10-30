use clouddialect_contracts::{
    CatalogSnapshot, ErrorCategory, ManifestOutcome, ManifestValidator, PackManifest,
    StructuralOutcome, Validator,
};
use semver::{Version, VersionReq};
use serde_json::json;

#[path = "support/documents.rs"]
mod documents;

const POLICY: &str = include_str!("../../../docs/compatibility-policy.md");

#[test]
fn source_tested_matrix_has_real_version_constraints_and_does_not_certify_pack_runtime() {
    let row = POLICY
        .lines()
        .find(|line| line.starts_with("| `=0.1.0`"))
        .unwrap();
    let constraints: Vec<_> = row
        .split('|')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches('`'))
        .collect();
    assert_eq!(constraints.len(), 4);
    for (constraint, actual) in constraints.iter().zip([
        env!("CARGO_PKG_VERSION"),
        "1.0.0",
        "1.0.0",
        "0.1.0-rc.1+build.1",
    ]) {
        assert!(
            VersionReq::parse(constraint)
                .unwrap()
                .matches(&Version::parse(actual).unwrap())
        );
    }
    assert!(POLICY.contains("declaration-format handling only"));
    assert!(POLICY.contains("No deployed"));
    assert!(POLICY.contains("12 calendar months from the publication date of its successor"));
    for required in [
        "$schema",
        "format_version",
        "pack.version",
        "end-of-life-declared",
        "end_of_life_date",
        "migration guide",
        "MAJOR",
        "MINOR",
        "PATCH",
    ] {
        assert!(POLICY.contains(required), "missing {required}");
    }
}

#[test]
fn documented_document_profile_matches_the_actual_structural_entrypoint() {
    let validator = Validator::new(CatalogSnapshot::embedded().unwrap());
    let (kind, original) = documents::minimal_documents()[1].clone();
    for (version, outcome, category) in [
        ("1.0.0+build", StructuralOutcome::Pass, None),
        ("1.1.0", StructuralOutcome::Warn, None),
        (
            "1.0.1",
            StructuralOutcome::Fail,
            Some(ErrorCategory::UnsupportedVersion),
        ),
        (
            "1.0.0-rc.1",
            StructuralOutcome::Fail,
            Some(ErrorCategory::UnsupportedVersion),
        ),
        (
            "18446744073709551616.0.0",
            StructuralOutcome::Fail,
            Some(ErrorCategory::UnsupportedVersion),
        ),
        (
            "2.0.0",
            StructuralOutcome::Fail,
            Some(ErrorCategory::UnsupportedMajorVersion),
        ),
        (
            "01.0.0",
            StructuralOutcome::Fail,
            Some(ErrorCategory::Version),
        ),
    ] {
        let mut value = original.clone();
        value["format_version"] = json!(version);
        let result = validator.validate_structural(kind, &serde_json::to_vec(&value).unwrap());
        assert_eq!(result.outcome, outcome, "{version}: {result:?}");
        if let Some(category) = category {
            assert!(result.errors.iter().any(|error| error.category == category));
        }
    }
    let mut value = original;
    value["format_version"] = json!("1.1.0");
    value["future_root"] = json!(true);
    let result = validator.validate_structural(kind, &serde_json::to_vec(&value).unwrap());
    assert_eq!(result.outcome, StructuralOutcome::Warn);
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.field_name_verbatim == "future_root")
    );
    // Existing open/freeform objects remain open. A verification test entry is
    // explicitly closed, so it must not acquire a root-only projection fallback.
    let (kind, mut verification) = documents::minimal_documents()[2].clone();
    verification["format_version"] = json!("1.1.0");
    verification["tests"][0]["future_nested"] = json!(true);
    assert_eq!(
        validator
            .validate_structural(kind, &serde_json::to_vec(&verification).unwrap())
            .outcome,
        StructuralOutcome::Fail
    );
}

#[test]
fn legacy_security_fields_do_not_gain_compatibility_through_extensions() {
    let validator = Validator::new(CatalogSnapshot::embedded().unwrap());
    let (kind, mut verification) = documents::minimal_documents()[2].clone();
    verification.as_object_mut().unwrap().remove("binding");
    verification["acme:binding"] = json!({"accepted":true});
    assert_eq!(
        validator
            .validate_structural(kind, &serde_json::to_vec(&verification).unwrap())
            .outcome,
        StructuralOutcome::Fail
    );
    let source = include_str!("manifest_validation.rs");
    let literal = source
        .split("const MANIFEST: &str = r#\"")
        .nth(1)
        .unwrap()
        .split("\"#;")
        .next()
        .unwrap();
    let manifest: PackManifest = toml::from_str(literal).unwrap();
    assert_eq!(manifest.pack.version, "0.1.0-rc.1+build.1");
    let validator = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    assert_eq!(
        validator.validate_manifest(literal.as_bytes()).outcome,
        ManifestOutcome::Pass
    );
    let mut value: toml::Value = toml::from_str(literal).unwrap();
    value.as_table_mut().unwrap().remove("capabilities");
    value
        .as_table_mut()
        .unwrap()
        .insert("x".to_owned(), toml::Value::Table(toml::map::Map::new()));
    assert_eq!(
        validator
            .validate_manifest(toml::to_string(&value).unwrap().as_bytes())
            .outcome,
        ManifestOutcome::Fail
    );
}
