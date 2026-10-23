use clouddialect_contracts::{
    ErrorCategory, SemanticOutcome, SemanticResult, StructuralOutcome, StructuralResult,
    ValidatorError, ValidatorWarning,
};
use serde_json::json;

fn diagnostic(category: ErrorCategory) -> ValidatorError {
    ValidatorError {
        document_id: None,
        format_version: None,
        category,
        field_path: None,
        description: "input cannot be parsed".into(),
        byte_offset: Some(0),
    }
}

#[test]
fn error_preserves_unknown_context_and_zero_offset() {
    let error = diagnostic(ErrorCategory::Unparseable);
    let value = json!({
        "document_id": null,
        "format_version": null,
        "category": "unparseable",
        "field_path": null,
        "description": "input cannot be parsed",
        "byte_offset": 0,
    });
    assert_eq!(serde_json::to_value(&error).unwrap(), value);
    assert_eq!(
        serde_json::from_value::<ValidatorError>(value).unwrap(),
        error
    );
}

#[test]
fn category_labels_match_the_closed_error_taxonomy() {
    let cases = [
        (ErrorCategory::Structural, "structural"),
        (ErrorCategory::Semantic, "semantic"),
        (ErrorCategory::Version, "version"),
        (
            ErrorCategory::UnresolvableReference,
            "unresolvable-reference",
        ),
        (ErrorCategory::Unparseable, "unparseable"),
        (ErrorCategory::UnsupportedVersion, "unsupported-version"),
        (
            ErrorCategory::UnsupportedMajorVersion,
            "unsupported-major-version",
        ),
        (ErrorCategory::Unsupported, "unsupported"),
        (ErrorCategory::BlockingGap, "blocking-gap"),
    ];
    for (category, label) in cases {
        assert_eq!(serde_json::to_value(category).unwrap(), json!(label));
        assert_eq!(
            serde_json::from_value::<ErrorCategory>(json!(label)).unwrap(),
            category
        );
    }
    for unknown in [
        "accepted",
        "execution",
        "Structural",
        "",
        "unresolvable_reference",
    ] {
        assert!(serde_json::from_value::<ErrorCategory>(json!(unknown)).is_err());
    }
}

#[test]
fn warning_keeps_the_verbatim_field_name_and_description() {
    let warning = ValidatorWarning {
        field_name_verbatim: "CoRe-Example:évidence".into(),
        description: "prefix 'CoRe-Example' is reserved for core use".into(),
    };
    let value = json!({
        "field_name_verbatim": "CoRe-Example:évidence",
        "description": "prefix 'CoRe-Example' is reserved for core use",
    });
    assert_eq!(serde_json::to_value(&warning).unwrap(), value);
    assert_eq!(
        serde_json::from_value::<ValidatorWarning>(value).unwrap(),
        warning
    );
}

#[test]
fn structural_outcomes_use_only_the_three_phase_labels() {
    for (outcome, label) in [
        (StructuralOutcome::Pass, "pass"),
        (StructuralOutcome::Warn, "warn"),
        (StructuralOutcome::Fail, "fail"),
    ] {
        assert_eq!(serde_json::to_value(outcome).unwrap(), json!(label));
        assert_eq!(
            serde_json::from_value::<StructuralOutcome>(json!(label)).unwrap(),
            outcome
        );
    }
    for label in ["accepted", "evidence-complete", "deferred", "Pass"] {
        assert!(serde_json::from_value::<StructuralOutcome>(json!(label)).is_err());
    }
}

#[test]
fn semantic_outcomes_do_not_gain_warnings_or_acceptance() {
    for (outcome, label) in [
        (SemanticOutcome::Pass, "pass"),
        (SemanticOutcome::Fail, "fail"),
    ] {
        assert_eq!(serde_json::to_value(outcome).unwrap(), json!(label));
        assert_eq!(
            serde_json::from_value::<SemanticOutcome>(json!(label)).unwrap(),
            outcome
        );
    }
    for label in ["warn", "accepted", "evidence-complete", "deferred", "Pass"] {
        assert!(serde_json::from_value::<SemanticOutcome>(json!(label)).is_err());
    }
}

#[test]
fn structural_result_has_only_its_canonical_diagnostic_fields() {
    let result = StructuralResult {
        document_id: Some("contract-1".into()),
        format_version: Some("1.0.0".into()),
        outcome: StructuralOutcome::Warn,
        errors: vec![],
        warnings: vec![ValidatorWarning {
            field_name_verbatim: "CORE:extra".into(),
            description: "reserved prefix".into(),
        }],
    };
    let value = json!({
        "document_id": "contract-1", "format_version": "1.0.0", "outcome": "warn",
        "errors": [],
        "warnings": [{"field_name_verbatim": "CORE:extra", "description": "reserved prefix"}],
    });
    assert_eq!(serde_json::to_value(&result).unwrap(), value);
    assert_eq!(
        serde_json::from_value::<StructuralResult>(value).unwrap(),
        result
    );

    let failed = StructuralResult {
        document_id: None,
        format_version: None,
        outcome: StructuralOutcome::Fail,
        errors: vec![diagnostic(ErrorCategory::Unparseable)],
        warnings: vec![],
    };
    let value = serde_json::to_value(&failed).unwrap();
    assert_eq!(
        serde_json::from_value::<StructuralResult>(value).unwrap(),
        failed
    );
}

#[test]
fn semantic_result_remains_distinct_from_structural_result() {
    let error = ValidatorError {
        document_id: Some("manifest-1".into()),
        format_version: Some("1.0.0".into()),
        category: ErrorCategory::Semantic,
        field_path: Some("contracts.upload.id".into()),
        description: "exact contract reference is absent".into(),
        byte_offset: None,
    };
    let result = SemanticResult {
        document_id: Some("manifest-1".into()),
        outcome: SemanticOutcome::Fail,
        errors: vec![error.clone()],
    };
    let value = json!({"document_id": "manifest-1", "outcome": "fail", "errors": [error]});
    assert_eq!(serde_json::to_value(&result).unwrap(), value);
    assert_eq!(
        serde_json::from_value::<SemanticResult>(value).unwrap(),
        result
    );
    assert_ne!(
        std::any::TypeId::of::<StructuralResult>(),
        std::any::TypeId::of::<SemanticResult>()
    );
    let _: clouddialect_contracts::errors::SemanticResult = result;
    let _: clouddialect_contracts::errors::StructuralOutcome = StructuralOutcome::Pass;
}

#[test]
fn malformed_diagnostics_and_missing_phase_data_are_not_silently_defaulted() {
    for value in [
        json!({"category": "structural"}),
        json!({"description": "missing category"}),
        json!({"category": "unparseable", "description": "bad offset", "byte_offset": -1}),
    ] {
        assert!(serde_json::from_value::<ValidatorError>(value).is_err());
    }
    assert!(
        serde_json::from_value::<ValidatorWarning>(json!({"description": "missing field"}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<StructuralResult>(json!({"outcome": "pass", "errors": []}))
            .is_err()
    );
    assert!(serde_json::from_value::<SemanticResult>(json!({"outcome": "pass"})).is_err());
}
