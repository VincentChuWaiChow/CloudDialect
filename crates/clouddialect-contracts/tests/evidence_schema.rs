mod support;

use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use support::SchemaHarness;

fn harness() -> SchemaHarness {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/schemas/evidence-record/1.0/schema.json");
    assert!(
        path.is_file(),
        "required evidence-record schema is not delivered"
    );
    let bytes = std::fs::read(path).unwrap();
    let schema: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        schema["$id"],
        "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json"
    );
    SchemaHarness::new(
        BTreeMap::from([(schema["$id"].as_str().unwrap().into(), bytes)]),
        schema["$id"].as_str().unwrap(),
        true,
    )
    .unwrap()
}
fn valid(h: &SchemaHarness, v: &Value) -> bool {
    h.validate_json(&serde_json::to_vec(v).unwrap()).unwrap()
}
fn evidence() -> Value {
    json!({"id":"550e8400-e29b-41d4-a716-446655440000","format_version":"1.0.0","claim":"ordering","classification":"observed","provenance":{"source_artifact_identity":"source","analysis_tool_identity":"tool"}})
}
#[test]
fn explicit_classifications_require_their_declared_support() {
    let h = harness();
    assert!(valid(&h, &evidence()));
    let mut inferred = evidence();
    inferred.as_object_mut().unwrap().remove("provenance");
    inferred["classification"] = json!("inferred");
    inferred["reasoning_chain"] = json!(["source observation", "bounded inference"]);
    assert!(valid(&h, &inferred));
    let mut unknown = inferred;
    unknown.as_object_mut().unwrap().remove("reasoning_chain");
    unknown["classification"] = json!("unknown");
    unknown["unresolved_assumptions"] =
        json!([{"assumption_id":"a1","description":"unknown","reason":"not observed"}]);
    assert!(valid(&h, &unknown));
}
#[test]
fn observed_and_inferred_records_can_preserve_valid_unresolved_assumptions() {
    let h = harness();
    for classification in ["observed", "inferred"] {
        let mut v = evidence();
        if classification == "inferred" {
            v.as_object_mut().unwrap().remove("provenance");
            v["classification"] = json!("inferred");
            v["reasoning_chain"] = json!(["bounded inference"]);
        }
        v["unresolved_assumptions"] =
            json!([{"assumption_id":"a1","description":"unknown retry","reason":"not observed"}]);
        assert!(valid(&h, &v), "{classification}");
        v["unresolved_assumptions"][0]
            .as_object_mut()
            .unwrap()
            .remove("reason");
        assert!(!valid(&h, &v), "invalid assumption on {classification}");
        v["unresolved_assumptions"] = json!([]);
        assert!(valid(&h, &v), "no residual assumptions on {classification}");
    }
}
#[test]
fn required_base_fields_and_classification_are_not_defaulted() {
    let h = harness();
    for key in ["id", "format_version", "claim", "classification"] {
        let mut v = evidence();
        v.as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v), "{key}");
    }
    let mut v = evidence();
    v["classification"] = json!("verified");
    assert!(!valid(&h, &v));
}
#[test]
fn observed_provenance_requires_both_identities() {
    let h = harness();
    for key in ["source_artifact_identity", "analysis_tool_identity"] {
        let mut v = evidence();
        v["provenance"].as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v));
    }
    let mut v = evidence();
    v.as_object_mut().unwrap().remove("provenance");
    assert!(!valid(&h, &v));
}
#[test]
fn inferred_reasoning_cannot_be_empty_or_absent() {
    let h = harness();
    let mut v = evidence();
    v.as_object_mut().unwrap().remove("provenance");
    v["classification"] = json!("inferred");
    assert!(!valid(&h, &v));
    v["reasoning_chain"] = json!([]);
    assert!(!valid(&h, &v));
}
#[test]
fn unknown_assumptions_require_nonempty_complete_entries() {
    let h = harness();
    let mut v = evidence();
    v.as_object_mut().unwrap().remove("provenance");
    v["classification"] = json!("unknown");
    assert!(!valid(&h, &v));
    v["unresolved_assumptions"] = json!([]);
    assert!(!valid(&h, &v));
    for key in ["assumption_id", "description", "reason"] {
        let mut bad = v.clone();
        bad["unresolved_assumptions"] =
            json!([{"assumption_id":"a1","description":"unknown","reason":"missing observation"}]);
        bad["unresolved_assumptions"][0]
            .as_object_mut()
            .unwrap()
            .remove(key);
        assert!(!valid(&h, &bad));
    }
}
#[test]
fn extensions_are_preserved_data_not_implicit_evidence() {
    let h = harness();
    let mut v = evidence();
    v["acme:source"] = json!({"opaque":[null,17]});
    assert!(valid(&h, &v));
    v.as_object_mut().unwrap().remove("classification");
    assert!(!valid(&h, &v));
}
#[test]
fn version_and_identifier_shapes_are_enforced() {
    let h = harness();
    let mut v = evidence();
    v["format_version"] = json!("1.2.3-rc.1+build.2");
    assert!(valid(&h, &v));
    for version in ["01.2.3", "1.2.3\n", "1.2.3-01"] {
        v["format_version"] = json!(version);
        assert!(!valid(&h, &v));
    }
    v = evidence();
    v["id"] = json!("550e8400-e29b-11d4-a716-446655440000");
    assert!(!valid(&h, &v));
}
