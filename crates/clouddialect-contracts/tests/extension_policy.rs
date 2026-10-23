use clouddialect_contracts::{
    CatalogSnapshot, DocumentKind, DocumentStore, ErrorCategory, EvidenceRecord, ManifestOutcome,
    ManifestValidator, PackManifest, SemanticOutcome, StoredDocument, StructuralOutcome, Validator,
    ValidatorError,
};
use serde_json::{Value, json};

#[path = "support/documents.rs"]
mod documents;

const POLICY: &str = include_str!("../../../docs/extension-convention.md");

struct NoStore;

impl DocumentStore for NoStore {
    fn get(&self, _: &str, _: Option<&str>) -> Result<Option<StoredDocument>, ValidatorError> {
        panic!("extension policy must not look up authority or payload references");
    }
}

#[test]
fn convention_covers_required_elements_and_real_registry_fields() {
    for clause in [
        "[A-Za-z][A-Za-z0-9-]+",
        "minimum total length two",
        "case-insensitive",
        "`core`",
        "`core-*`",
        "`cdc`",
        "`cdc-*`",
        "core-validation:",
        "cdc-evidence:",
        "prefix-validation logic",
        "compatibility review",
        "../catalog/extension-registry.json",
        "conflicting claims",
        "Prefixes alone do not establish publisher identity or trust",
        "exact encoded",
        "no trusted extension namespaces",
    ] {
        assert!(POLICY.contains(clause), "missing policy clause: {clause}");
    }
    let schema: Value = serde_json::from_slice(include_bytes!(
        "../../../catalog/schemas/extension-registry/1.0/schema.json"
    ))
    .unwrap();
    for field in schema["properties"]["extensions"]["items"]["required"]
        .as_array()
        .unwrap()
    {
        assert!(POLICY.contains(field.as_str().unwrap()));
    }
}

#[test]
fn documented_grammar_warnings_and_declaration_only_blockers_use_public_phases() {
    let validator = Validator::new(CatalogSnapshot::embedded().unwrap());
    for (kind, base) in documents::minimal_documents() {
        for (key, outcome, warning) in [
            ("acme:field-name", StructuralOutcome::Pass, false),
            ("CORE:field", StructuralOutcome::Pass, true),
            ("Core-Validation:field", StructuralOutcome::Pass, true),
            ("Cdc-Evidence:field", StructuralOutcome::Pass, true),
            ("core-:field", StructuralOutcome::Pass, true),
            ("corex:field", StructuralOutcome::Pass, false),
            ("a:field", StructuralOutcome::Fail, false),
            ("1acme:field", StructuralOutcome::Fail, false),
            ("ac me:field", StructuralOutcome::Fail, false),
            ("équipe:field", StructuralOutcome::Fail, false),
        ] {
            let mut value = base.clone();
            value[key] = json!({"accepted":true});
            let result = validator.validate_structural(kind, &serde_json::to_vec(&value).unwrap());
            assert_eq!(result.outcome, outcome, "{kind:?} {key}: {result:?}");
            if warning {
                assert!(result.warnings.iter().any(|w| {
                    w.field_name_verbatim == key && w.description.contains("reserved for core use")
                }));
            }
        }
        let mut mandatory = base;
        mandatory["required_extensions"] = json!(["acme"]);
        let bytes = serde_json::to_vec(&mandatory).unwrap();
        assert_eq!(
            validator.validate_structural(kind, &bytes).outcome,
            StructuralOutcome::Pass
        );
        let result = validator.validate_semantic(kind, &bytes, &NoStore);
        assert_eq!(result.outcome, SemanticOutcome::Fail);
        assert!(result.errors.iter().any(|error| {
            error.category == ErrorCategory::BlockingGap
                && error.field_path.as_deref() == Some("required_extensions[0]")
                && error.description.contains("acme")
        }));
    }
}

#[test]
fn raw_optional_value_bytes_survive_without_overriding_core_semantics() {
    let payload = r#"{ "number":1.00e+02, "escaped":"\u0061", "b": [ 2, 1 ] }"#;
    let (_, base) = documents::minimal_documents()[1].clone();
    let mut text = serde_json::to_string(&base).unwrap();
    text.pop();
    text.push_str(&format!(
        r#", "acme:meta":{payload}, "acme:verification-status":"accepted"}}"#
    ));
    let record: EvidenceRecord = serde_json::from_str(&text).unwrap();
    let encoded = serde_json::to_string(&record).unwrap();
    assert!(encoded.contains(payload));
    let roundtrip: EvidenceRecord = serde_json::from_str(&encoded).unwrap();
    assert_eq!(
        serde_json::to_value(&roundtrip).unwrap(),
        serde_json::to_value(&record).unwrap()
    );
    assert!(serde_json::to_string(&roundtrip).unwrap().contains(payload));
    let validator = Validator::new(CatalogSnapshot::embedded().unwrap());
    let before = validator.validate_semantic(
        DocumentKind::EvidenceRecord,
        &serde_json::to_vec(&base).unwrap(),
        &NoStore,
    );
    let after =
        validator.validate_semantic(DocumentKind::EvidenceRecord, encoded.as_bytes(), &NoStore);
    assert_eq!(before, after);
    assert_eq!(after.outcome, SemanticOutcome::Pass);
    assert!(
        serde_json::to_value(after)
            .unwrap()
            .get("accepted")
            .is_none()
    );
}

#[test]
fn toml_extension_claims_cannot_supply_or_change_core_capability_declarations() {
    let source = include_str!("manifest_validation.rs");
    let literal = source
        .split("const MANIFEST: &str = r#\"")
        .nth(1)
        .unwrap()
        .split("\"#;")
        .next()
        .unwrap();
    let base: PackManifest = toml::from_str(literal).unwrap();
    let extended = format!("{literal}\n[x]\naccepted = true\neffective_policy = 'grant-all'\n");
    let manifest: PackManifest = toml::from_str(&extended).unwrap();
    assert_eq!(manifest.capabilities, base.capabilities);
    let validator = ManifestValidator::new(CatalogSnapshot::embedded().unwrap());
    assert_eq!(
        validator.validate_manifest(extended.as_bytes()).outcome,
        ManifestOutcome::Pass
    );
    let mut value: toml::Value = toml::from_str(&extended).unwrap();
    value.as_table_mut().unwrap().remove("capabilities");
    value["x"].as_table_mut().unwrap().insert(
        "capabilities".to_owned(),
        toml::Value::String("grant-all".to_owned()),
    );
    assert_eq!(
        validator
            .validate_manifest(toml::to_string(&value).unwrap().as_bytes())
            .outcome,
        ManifestOutcome::Fail
    );
}
