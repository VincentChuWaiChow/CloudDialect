mod support;

use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};
use support::SchemaHarness;

fn harness() -> SchemaHarness {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/schemas/behavioral-contract/1.0/schema.json");
    assert!(
        path.is_file(),
        "required behavioral-contract schema is not delivered"
    );
    let bytes = std::fs::read(path).unwrap();
    let schema: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        schema["$id"],
        "https://schemas.clouddialect.io/behavioral-contract/1.0.0/schema.json"
    );
    SchemaHarness::new(
        BTreeMap::from([(schema["$id"].as_str().unwrap().into(), bytes)]),
        schema["$id"].as_str().unwrap(),
        true,
    )
    .unwrap()
}
fn valid(h: &SchemaHarness, value: &Value) -> bool {
    h.validate_json(&serde_json::to_vec(value).unwrap())
        .unwrap()
}
fn contract() -> Value {
    json!({"id":"550e8400-e29b-41d4-a716-446655440000","format_version":"1.0.0",
        "operation_identity":{"call_site":"pkg.upload"},"inputs":{},"outputs":{},"side_effects":[],"resource_dependencies":[],
        "obligations":[{"obligation_id":"o1","behavioral_fidelity":"equivalent-within-contract","evidence_refs":["e1"]}]})
}
#[test]
fn minimal_contract_and_explicit_schema_selector_validate() {
    let h = harness();
    let mut v = contract();
    assert!(valid(&h, &v));
    v["$schema"] = json!("https://schemas.clouddialect.io/behavioral-contract/1.0.0/schema.json");
    assert!(valid(&h, &v));
}
#[test]
fn every_required_base_field_is_enforced() {
    let h = harness();
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
        let mut v = contract();
        v.as_object_mut().unwrap().remove(key);
        assert!(!valid(&h, &v), "{key}");
    }
}
#[test]
fn each_obligation_has_exactly_one_nonempty_support_alternative() {
    let h = harness();
    let assumption = json!({"assumption_id":"a1","description":"unknown","reason":"not observed"});
    let mut v = contract();
    v["obligations"][0]
        .as_object_mut()
        .unwrap()
        .remove("evidence_refs");
    assert!(!valid(&h, &v));
    v["obligations"][0]["unresolved_assumption"] = assumption.clone();
    assert!(valid(&h, &v));
    for refs in [json!([]), json!(["e1"])] {
        let mut both = v.clone();
        both["obligations"][0]["evidence_refs"] = refs;
        assert!(!valid(&h, &both));
    }
    let mut empty = contract();
    empty["obligations"][0]["evidence_refs"] = json!([]);
    assert!(!valid(&h, &empty));
}
#[test]
fn fidelity_is_closed_without_promoting_structural_success() {
    let h = harness();
    for label in [
        "equivalent-within-contract",
        "adaptation-required",
        "architectural-change-required",
        "unsupported",
        "unknown",
    ] {
        let mut v = contract();
        v["obligations"][0]["behavioral_fidelity"] = json!(label);
        assert!(valid(&h, &v));
    }
    let mut v = contract();
    v["obligations"][0]["behavioral_fidelity"] = json!("full");
    assert!(!valid(&h, &v));
}
#[test]
fn complete_semver_syntax_rejects_leading_zeroes_empty_suffixes_and_whitespace() {
    let h = harness();
    for version in [
        "1.2.3",
        "1.2.3-rc.1",
        "1.2.3+build.1",
        "1.2.3-0",
        "1.2.3-x-y-z.--",
    ] {
        let mut v = contract();
        v["format_version"] = json!(version);
        assert!(valid(&h, &v), "{version}");
    }
    for version in [
        "01.2.3",
        "1.2.3-01",
        "1.2.3-",
        "1.2.3+",
        "1.2.3\n",
        "1.2.3\r\n",
        "1.2.3 ",
    ] {
        let mut v = contract();
        v["format_version"] = json!(version);
        assert!(!valid(&h, &v), "{version:?}");
    }
}
#[test]
fn identifiers_are_lowercase_hyphenated_uuid_v4() {
    let h = harness();
    for id in [
        "550E8400-E29B-41D4-A716-446655440000",
        "550e8400-e29b-11d4-a716-446655440000",
        "00000000-0000-0000-0000-000000000000",
        "not-a-uuid",
    ] {
        let mut v = contract();
        v["id"] = json!(id);
        assert!(!valid(&h, &v), "{id}");
    }
}
#[test]
fn community_extensions_are_valid_but_invalid_namespace_spellings_are_not() {
    let h = harness();
    for key in ["acme:custom", "Acme-Tools:payload", "CORE:warning-only"] {
        let mut v = contract();
        v[key] = json!({"opaque":true});
        assert!(valid(&h, &v), "{key}");
    }
    for key in [
        "unqualified",
        "a:short",
        "acme\\tools:bad",
        "соre:homoglyph",
    ] {
        let mut v = contract();
        v[key] = json!(true);
        assert!(!valid(&h, &v), "{key}");
    }
}
#[test]
fn a_declared_wrapper_chain_cannot_be_empty() {
    let h = harness();
    let mut v = contract();
    v["operation_identity"]["wrapper_chain"] = json!([]);
    assert!(!valid(&h, &v));
}
#[test]
fn wrappers_resources_and_open_note_types_keep_their_required_structure() {
    let h = harness();
    let mut v = contract();
    v["operation_identity"]["wrapper_chain"] = json!(["outer", "inner"]);
    v["side_effects"] = json!([{"description":"test write","evidence_ref":"e1"}]);
    v["resource_dependencies"] = json!([{"identity":"test-store","permissions":["write"],"configuration":{},"known_behavior":{"ordering":"per-key","consistency":"strong","retry":"bounded","failure":"explicit"}}]);
    v["behavioral_notes"] =
        json!([{"note_type":"community-defined","description":"boundary","evidence_ref":"e1"}]);
    assert!(valid(&h, &v));
    v["resource_dependencies"][0]
        .as_object_mut()
        .unwrap()
        .remove("configuration");
    assert!(!valid(&h, &v));
}

const DIMENSIONS: [&str; 4] = ["ordering", "consistency", "retry", "failure"];

fn resource(known_mask: u8) -> Value {
    let mut known = serde_json::Map::new();
    let mut assumptions = vec![];
    for (i, dimension) in DIMENSIONS.into_iter().enumerate() {
        if known_mask & (1 << i) != 0 {
            known.insert(dimension.to_owned(), json!("declared behavior"));
        } else {
            assumptions.push(json!({"behavior":dimension,"assumption_id":format!("a-{dimension}"),"description":"unknown details","reason":"not observed"}));
        }
    }
    let mut result = json!({"identity":"store","permissions":["write"],"configuration":{}});
    if !known.is_empty() {
        result["known_behavior"] = json!(known);
    }
    if !assumptions.is_empty() {
        result["unresolved_assumptions"] = json!(assumptions);
    }
    result
}

#[test]
fn every_resource_dimension_must_be_covered_on_that_dependency() {
    let h = harness();
    let mut missing_both = contract();
    missing_both["resource_dependencies"] =
        json!([{"identity":"store","permissions":[],"configuration":{}}]);
    assert!(!valid(&h, &missing_both));
    for (i, dimension) in DIMENSIONS.into_iter().enumerate() {
        let mut v = contract();
        let mut incomplete = resource(15);
        incomplete["known_behavior"]
            .as_object_mut()
            .unwrap()
            .remove(dimension);
        v["resource_dependencies"] = json!([incomplete.clone(), resource(15)]);
        assert!(
            !valid(&h, &v),
            "another dependency cannot cover {dimension}"
        );
        let unrelated = DIMENSIONS[(i + 1) % DIMENSIONS.len()];
        incomplete["unresolved_assumptions"] = json!([{"behavior":unrelated,"assumption_id":"a1","description":"unknown","reason":"not observed"}]);
        v["resource_dependencies"] = json!([incomplete.clone()]);
        assert!(
            !valid(&h, &v),
            "unrelated assumption cannot cover {dimension}"
        );
        incomplete["unresolved_assumptions"][0]["behavior"] = json!(dimension);
        v["resource_dependencies"] = json!([incomplete]);
        assert!(valid(&h, &v), "matching assumption covers {dimension}");
    }
}

#[test]
fn all_known_unknown_combinations_and_partial_knowledge_are_valid() {
    let h = harness();
    for mask in 0..16 {
        let mut v = contract();
        v["resource_dependencies"] = json!([resource(mask)]);
        assert!(valid(&h, &v), "known mask {mask}");
    }
    let mut v = contract();
    let mut partial = resource(0);
    partial["known_behavior"] = json!({"retry":"some observed behavior"});
    v["resource_dependencies"] = json!([partial]);
    assert!(
        valid(&h, &v),
        "known and unknown details can coexist for retry"
    );
    v["resource_dependencies"] = json!([resource(15)]);
    v["resource_dependencies"][0]["unresolved_assumptions"] = json!([]);
    assert!(valid(&h, &v), "all-known resource needs no assumptions");
}

#[test]
fn resource_assumptions_are_populated_even_when_known_values_already_cover_everything() {
    let h = harness();
    for key in ["behavior", "assumption_id", "description", "reason"] {
        let mut v = contract();
        v["resource_dependencies"] = json!([resource(0)]);
        v["resource_dependencies"][0]["unresolved_assumptions"][0]
            .as_object_mut()
            .unwrap()
            .remove(key);
        assert!(!valid(&h, &v), "missing {key}");
        for malformed in [json!(""), json!(null), json!(17), json!([])] {
            let mut v = contract();
            let mut declared = resource(15);
            declared["unresolved_assumptions"] = resource(0)["unresolved_assumptions"].clone();
            declared["unresolved_assumptions"][0][key] = malformed.clone();
            v["resource_dependencies"] = json!([declared]);
            assert!(!valid(&h, &v), "malformed {key}: {malformed}");
        }
    }
}

#[test]
fn known_behavior_declarations_cannot_be_empty_or_wrongly_typed() {
    let h = harness();
    for dimension in DIMENSIONS {
        for malformed in [json!(""), json!(null), json!(17), json!([])] {
            let mut v = contract();
            let mut declared = resource(0);
            declared["known_behavior"] = json!({});
            declared["known_behavior"][dimension] = malformed.clone();
            v["resource_dependencies"] = json!([declared]);
            assert!(!valid(&h, &v), "invalid {dimension}: {malformed}");
        }
    }
}

#[test]
fn arbitrary_ids_text_extensions_and_duplicate_other_dimensions_do_not_fill_gaps() {
    let h = harness();
    for wrong in ["Retry", "timeout", ""] {
        let mut v = contract();
        v["resource_dependencies"] = json!([resource(0)]);
        v["resource_dependencies"][0]["unresolved_assumptions"][2]["behavior"] = json!(wrong);
        assert!(!valid(&h, &v), "unknown dimension {wrong}");
    }
    let mut v = contract();
    let mut declared = resource(0);
    declared["unresolved_assumptions"][2] = json!({"assumption_id":"retry","description":"retry is unknown","reason":"retry was not observed","acme:behavior":"retry"});
    v["resource_dependencies"] = json!([declared.clone()]);
    assert!(
        !valid(&h, &v),
        "free text and qualified fields are not bindings"
    );
    declared["unresolved_assumptions"][2] = declared["unresolved_assumptions"][3].clone();
    v["resource_dependencies"] = json!([declared]);
    assert!(!valid(&h, &v), "two failure assumptions cannot cover retry");
}

#[test]
fn generic_obligation_assumptions_do_not_require_resource_dimension_bindings() {
    let h = harness();
    let mut v = contract();
    v["resource_dependencies"] = json!([resource(0)]);
    v["obligations"][0] = json!({"obligation_id":"o1","behavioral_fidelity":"unknown","unresolved_assumption":{"assumption_id":"a1","description":"unknown","reason":"no evidence"}});
    assert!(valid(&h, &v));
}
