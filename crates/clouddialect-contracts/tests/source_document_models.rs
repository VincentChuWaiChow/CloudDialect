use clouddialect_contracts::{
    BehavioralContract, BehavioralFidelity, EvidenceClassification, EvidenceRecord,
    ResourceBehaviorDimension, ResourceUnresolvedAssumption, UnresolvedAssumption,
};
use serde_json::{Value, json};

fn contract() -> Value {
    json!({
        "id":"contract-1", "format_version":"1.0.0",
        "operation_identity":{"call_site":"pkg.upload", "wrapper_chain":["outer", "inner"]},
        "inputs":{"type":"object", "properties":{"payload":{"type":"string"}}},
        "outputs":{"type":"object"},
        "side_effects":[{"description":"writes test object", "evidence_ref":"evidence-1"}],
        "resource_dependencies":[{
            "identity":"test-store", "permissions":["write"], "configuration":{"bucket":"test"},
            "known_behavior":{"ordering":"per-key", "consistency":"strong", "retry":"bounded", "failure":"explicit"},
            "unresolved_assumptions":[{"behavior":"consistency", "assumption_id":"a1", "description":"ETag behavior", "reason":"not observed"}]
        }],
        "obligations":[{"obligation_id":"o1", "behavioral_fidelity":"equivalent-within-contract", "evidence_refs":["evidence-1"]}],
        "behavioral_notes":[{"note_type":"boundary", "description":"wrapper retains ordering", "evidence_ref":"evidence-1"}]
    })
}

#[test]
fn complete_source_contract_round_trips_without_losing_nested_data() {
    let value = contract();
    let document: BehavioralContract = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(
        document.operation_identity.wrapper_chain.as_ref().unwrap(),
        &["outer", "inner"]
    );
    assert_eq!(
        document.obligations[0].behavioral_fidelity,
        BehavioralFidelity::EquivalentWithinContract
    );
    assert_eq!(
        serde_json::from_str::<Value>(
            document.resource_dependencies[0].configuration["bucket"].get()
        )
        .unwrap(),
        json!("test")
    );
    assert_eq!(serde_json::to_value(document).unwrap(), value);
}

#[test]
fn required_source_fields_and_object_shapes_are_not_fabricated() {
    for key in [
        "id",
        "format_version",
        "operation_identity",
        "inputs",
        "outputs",
        "side_effects",
        "resource_dependencies",
        "obligations",
    ] {
        let mut value = contract();
        value.as_object_mut().unwrap().remove(key);
        assert!(
            serde_json::from_value::<BehavioralContract>(value).is_err(),
            "{key}"
        );
    }
    let mut value = contract();
    value["inputs"] = json!([]);
    assert!(serde_json::from_value::<BehavioralContract>(value).is_err());
    for invalid in ["[]", "null", "17"] {
        assert!(serde_json::from_str::<BehavioralContract>(invalid).is_err());
    }
    let duplicate = serde_json::to_string(&contract()).unwrap().replace(
        "\"id\":\"contract-1\"",
        "\"id\":\"first\",\"id\":\"second\"",
    );
    assert!(serde_json::from_str::<BehavioralContract>(&duplicate).is_err());
}

#[test]
fn unknown_assumptions_and_evidence_references_are_distinct_data() {
    let mut value = contract();
    value["obligations"][0] = json!({"obligation_id":"o1", "behavioral_fidelity":"unknown", "unresolved_assumption":{"assumption_id":"a1", "description":"unknown behavior", "reason":"no evidence"}});
    let document: BehavioralContract = serde_json::from_value(value.clone()).unwrap();
    assert!(document.obligations[0].evidence_refs.is_none());
    assert_eq!(
        document.obligations[0]
            .unresolved_assumption
            .as_ref()
            .unwrap()
            .reason,
        "no evidence"
    );
    assert_eq!(serde_json::to_value(document).unwrap(), value);
}

#[test]
fn source_fidelity_labels_are_closed_and_not_manifest_summaries() {
    for (variant, label) in [
        (
            BehavioralFidelity::EquivalentWithinContract,
            "equivalent-within-contract",
        ),
        (
            BehavioralFidelity::AdaptationRequired,
            "adaptation-required",
        ),
        (
            BehavioralFidelity::ArchitecturalChangeRequired,
            "architectural-change-required",
        ),
        (BehavioralFidelity::Unsupported, "unsupported"),
        (BehavioralFidelity::Unknown, "unknown"),
    ] {
        assert_eq!(
            serde_json::from_value::<BehavioralFidelity>(json!(label)).unwrap(),
            variant
        );
        assert_eq!(serde_json::to_value(variant).unwrap(), json!(label));
    }
    assert!(serde_json::from_value::<BehavioralFidelity>(json!("full")).is_err());
}

#[test]
fn every_evidence_class_preserves_its_recorded_support() {
    for (classification, support, kind) in [
        (
            "observed",
            json!({"provenance":{"source_artifact_identity":"source-1", "analysis_tool_identity":"tool-1"}}),
            EvidenceClassification::Observed,
        ),
        (
            "inferred",
            json!({"reasoning_chain":["observation", "bounded inference"]}),
            EvidenceClassification::Inferred,
        ),
        (
            "unknown",
            json!({"unresolved_assumptions":[{"assumption_id":"a1", "description":"claim unresolved", "reason":"no observation"}]}),
            EvidenceClassification::Unknown,
        ),
    ] {
        let mut value = json!({"id":"evidence-1", "format_version":"1.0.0", "claim":"ordering", "classification":classification});
        value
            .as_object_mut()
            .unwrap()
            .extend(support.as_object().unwrap().clone());
        let evidence: EvidenceRecord = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(evidence.classification, kind);
        assert_eq!(serde_json::to_value(evidence).unwrap(), value);
    }
    assert!(serde_json::from_value::<EvidenceClassification>(json!("verified")).is_err());
}

#[test]
fn extension_values_survive_at_root_and_nested_model_boundaries() {
    let mut value = contract();
    value["acme:meta"] = json!({"opaque":[null, "évidence", 17]});
    value["operation_identity"]["acme:trace"] = json!(["unchanged"]);
    value["resource_dependencies"][0]["known_behavior"]["acme:detail"] = json!(true);
    value["obligations"][0]["acme:annotation"] = json!({"confidence":"unverified"});
    let document: BehavioralContract = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(document).unwrap(), value);
    let evidence = json!({"id":"evidence-1", "format_version":"1.0.0", "claim":"ordering", "classification":"observed", "provenance":{"source_artifact_identity":"source", "analysis_tool_identity":"tool", "acme:raw":[false]}, "acme:extra":{"x":null}});
    let document: EvidenceRecord = serde_json::from_value(evidence.clone()).unwrap();
    assert_eq!(serde_json::to_value(document).unwrap(), evidence);
}

#[test]
fn typed_data_is_not_mistaken_for_schema_or_behavioral_validation() {
    let incomplete = json!({"id":"not-a-uuid", "format_version":"unverified", "claim":"claim", "classification":"observed"});
    let evidence: EvidenceRecord = serde_json::from_value(incomplete).unwrap();
    assert!(evidence.provenance.is_none()); // The structural validator must reject this later.
    let mut value = contract();
    value["obligations"][0]
        .as_object_mut()
        .unwrap()
        .remove("evidence_refs");
    let document: BehavioralContract = serde_json::from_value(value).unwrap();
    assert!(document.obligations[0].evidence_refs.is_none());
    assert!(document.obligations[0].unresolved_assumption.is_none()); // oneOf is not proven by DTO parsing.
}

#[test]
fn encoded_extension_value_bytes_survive_without_reencoding() {
    let raw = r#"{ "z" : "\u0061", "a" : [1,  2] }"#;
    let mut value = contract();
    value["acme:encoded"] = json!("__ROOT_RAW__");
    value["operation_identity"]["acme:encoded"] = json!("__NESTED_RAW__");
    value["inputs"]["metadata"] = json!({"acme:encoded":"__INPUT_RAW__"});
    value["resource_dependencies"][0]["configuration"]["acme:encoded"] = json!("__CONFIG_RAW__");
    let input = serde_json::to_string(&value).unwrap();
    let input = [
        "__ROOT_RAW__",
        "__NESTED_RAW__",
        "__INPUT_RAW__",
        "__CONFIG_RAW__",
    ]
    .into_iter()
    .fold(input, |text, marker| {
        text.replace(&format!("\"{marker}\""), raw)
    });
    let document: BehavioralContract = serde_json::from_str(&input).unwrap();
    let output = serde_json::to_string(&document).unwrap();
    assert_eq!(
        output.matches(raw).count(),
        4,
        "extension values were re-encoded: {output}"
    );

    let input = format!(
        r#"{{"id":"e1","format_version":"1.0.0","claim":"claim","classification":"observed","provenance":{{"source_artifact_identity":"s","analysis_tool_identity":"t","acme:encoded":{raw}}}}}"#
    );
    let document: EvidenceRecord = serde_json::from_reader(input.as_bytes()).unwrap();
    assert!(serde_json::to_string(&document).unwrap().contains(raw));
    for raw in [r#"1.2300e+04"#, r#""\u0061""#, "null", "true", "false"] {
        let input = format!(
            r#"{{"id":"e1","format_version":"1.0.0","claim":"c","classification":"unknown","acme:encoded":{raw}}}"#
        );
        let document: EvidenceRecord = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(document.extensions["acme:encoded"].get(), raw);
        assert!(
            serde_json::to_string(&document)
                .unwrap()
                .contains(&format!("\"acme:encoded\":{raw}"))
        );
    }
}

#[test]
fn resource_assumptions_have_a_typed_closed_dimension_and_existing_payload() {
    for (label, dimension) in [
        ("ordering", ResourceBehaviorDimension::Ordering),
        ("consistency", ResourceBehaviorDimension::Consistency),
        ("retry", ResourceBehaviorDimension::Retry),
        ("failure", ResourceBehaviorDimension::Failure),
    ] {
        let value = json!({"behavior":label,"assumption_id":"a1","description":"unknown details","reason":"not observed"});
        let assumption: ResourceUnresolvedAssumption =
            serde_json::from_value(value.clone()).unwrap();
        assert_eq!(assumption.behavior, dimension);
        let alias: clouddialect_contracts::validator::ResourceUnresolvedAssumption = assumption;
        let alias_dimension: clouddialect_contracts::validator::ResourceBehaviorDimension =
            dimension;
        assert_eq!(alias.behavior, alias_dimension);
        assert_eq!(serde_json::to_value(alias).unwrap(), value);
    }
    for invalid in [
        json!("timeout"),
        json!("Retry"),
        json!(""),
        json!(null),
        json!(1),
    ] {
        assert!(serde_json::from_value::<ResourceBehaviorDimension>(invalid).is_err());
    }
    let document: BehavioralContract = serde_json::from_value(contract()).unwrap();
    assert_eq!(
        document.resource_dependencies[0]
            .unresolved_assumptions
            .as_ref()
            .unwrap()[0]
            .behavior,
        ResourceBehaviorDimension::Consistency
    );
}

#[test]
fn resource_assumption_fields_cannot_be_missing_or_duplicated() {
    let value = json!({"behavior":"retry","assumption_id":"a1","description":"unknown","reason":"not observed"});
    for key in ["behavior", "assumption_id", "description", "reason"] {
        let mut missing = value.clone();
        missing.as_object_mut().unwrap().remove(key);
        assert!(
            serde_json::from_value::<ResourceUnresolvedAssumption>(missing).is_err(),
            "{key}"
        );
    }
    let duplicate = r#"{"behavior":"retry","behavior":"failure","assumption_id":"a1","description":"unknown","reason":"not observed"}"#;
    assert!(serde_json::from_str::<ResourceUnresolvedAssumption>(duplicate).is_err());
    let mut unbound = contract();
    unbound["resource_dependencies"][0]["unresolved_assumptions"][0]
        .as_object_mut()
        .unwrap()
        .remove("behavior");
    assert!(serde_json::from_value::<BehavioralContract>(unbound).is_err());
}

#[test]
fn resource_assumption_extensions_preserve_exact_encoded_value_bytes() {
    let raw = r#"{ "z" : "\u0061", "a" : [1.2300e+04,  2] }"#;
    let input = format!(
        r#"{{"behavior":"failure","assumption_id":"a1","description":"unknown","reason":"not observed","acme:encoded":{raw}}}"#
    );
    let assumption: ResourceUnresolvedAssumption = serde_json::from_str(&input).unwrap();
    assert_eq!(assumption.extensions["acme:encoded"].get(), raw);
    assert!(serde_json::to_string(&assumption).unwrap().contains(raw));
    let mut value = contract();
    value["resource_dependencies"][0]["unresolved_assumptions"][0]["acme:encoded"] =
        json!("__RESOURCE_RAW__");
    let input = serde_json::to_string(&value)
        .unwrap()
        .replace("\"__RESOURCE_RAW__\"", raw);
    let document: BehavioralContract = serde_json::from_str(&input).unwrap();
    assert!(serde_json::to_string(&document).unwrap().contains(raw));
}

#[test]
fn generic_assumptions_stay_generic_and_dto_decoding_does_not_prove_coverage() {
    let generic = json!({"assumption_id":"a1","description":"unknown","reason":"no evidence"});
    let assumption: UnresolvedAssumption = serde_json::from_value(generic.clone()).unwrap();
    assert_eq!(serde_json::to_value(assumption).unwrap(), generic);
    // The DTO records malformed-for-schema empty strings and incomplete dimensions;
    // actual resource coverage and populated values are the schema leaf's job.
    let resource = json!({"behavior":"retry","assumption_id":"","description":"","reason":""});
    let _: ResourceUnresolvedAssumption = serde_json::from_value(resource).unwrap();
    let mut value = contract();
    value["resource_dependencies"][0]
        .as_object_mut()
        .unwrap()
        .remove("known_behavior");
    let document: BehavioralContract = serde_json::from_value(value).unwrap();
    assert!(document.resource_dependencies[0].known_behavior.is_none());
}
