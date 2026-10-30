use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use regex::Regex;
use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn document(name: &str) -> String {
    fs::read_to_string(root().join(format!("docs/traceability/{name}.md")))
        .expect("required traceability document is missing")
}

fn rows(text: &str) -> Vec<Vec<&str>> {
    text.lines()
        .filter(|line| line.starts_with('|'))
        .map(|line| line.trim_matches('|').split('|').map(str::trim).collect())
        .filter(|row: &Vec<&str>| !matches!(row[0], "Schema" | "Gate") && !row[0].starts_with('-'))
        .collect()
}

fn json_fields(node: &Value, base: &Value, path: &str, fields: &mut BTreeSet<String>) {
    if let Some(reference) = node["$ref"].as_str() {
        json_fields(
            base.pointer(reference.strip_prefix('#').unwrap()).unwrap(),
            base,
            path,
            fields,
        );
    }
    if let Some(properties) = node["properties"].as_object() {
        for (name, child) in properties {
            let child_path = if path.is_empty() {
                name.clone()
            } else {
                format!("{path}.{name}")
            };
            fields.insert(child_path.clone());
            json_fields(child, base, &child_path, fields);
        }
    }
    for keyword in ["items", "contains"] {
        if node[keyword].is_object() {
            json_fields(&node[keyword], base, &format!("{path}[*]"), fields);
        }
    }
    if node["additionalProperties"].is_object() {
        json_fields(
            &node["additionalProperties"],
            base,
            &format!("{path}.*"),
            fields,
        );
    }
    for key in ["allOf", "anyOf", "oneOf"] {
        for child in node[key].as_array().into_iter().flatten() {
            json_fields(child, base, path, fields);
        }
    }
    for key in ["if", "then", "else"] {
        if node[key].is_object() {
            json_fields(&node[key], base, path, fields);
        }
    }
    for child in node["dependentSchemas"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.values())
    {
        json_fields(child, base, path, fields);
    }
}

fn manifest_fields(
    name: &str,
    path: &str,
    types: &BTreeMap<String, Vec<(String, String)>>,
    fields: &mut BTreeSet<String>,
) {
    for (field, kind) in &types[name] {
        let mut child = if path.is_empty() {
            field.clone()
        } else {
            format!("{path}.{field}")
        };
        fields.insert(child.clone());
        let compact: String = kind.chars().filter(|c| !c.is_whitespace()).collect();
        let kind = compact
            .strip_prefix("Option<")
            .and_then(|s| s.strip_suffix('>'))
            .unwrap_or(&compact);
        let kind = if let Some(s) = kind.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
            child.push_str("[*]");
            s
        } else {
            kind
        };
        let kind = if let Some(s) = kind
            .strip_prefix("HashMap<String,")
            .and_then(|s| s.strip_suffix('>'))
        {
            child.push_str(".*");
            s
        } else {
            kind
        };
        if types.contains_key(kind) {
            manifest_fields(kind, &child, types, fields);
        }
    }
}

fn rust_structs(source: &str) -> BTreeMap<String, Vec<(String, String)>> {
    let structs = Regex::new(r"(?s)pub struct (\w+)\s*\{(.*?)\n\}").unwrap();
    let members = Regex::new(r"(?m)^\s*pub (\w+): (.+),$").unwrap();
    structs
        .captures_iter(source)
        .map(|s| {
            (
                s[1].to_owned(),
                members
                    .captures_iter(&s[2])
                    .map(|m| (m[1].to_owned(), m[2].to_owned()))
                    .collect(),
            )
        })
        .collect()
}

fn expected_fields() -> BTreeSet<(String, String)> {
    let mut expected = BTreeSet::new();
    for name in [
        "behavioral-contract",
        "evidence-record",
        "verification-result",
        "catalog-index",
        "extension-registry",
    ] {
        let schema: Value = serde_json::from_slice(
            &fs::read(root().join(format!("catalog/schemas/{name}/1.0/schema.json"))).unwrap(),
        )
        .unwrap();
        let mut fields = BTreeSet::new();
        json_fields(&schema, &schema, "", &mut fields);
        expected.extend(fields.into_iter().map(|field| (name.to_owned(), field)));
    }
    let types = rust_structs(include_str!("../src/manifest/types.rs"));
    let mut fields = BTreeSet::new();
    manifest_fields("PackManifest", "", &types, &mut fields);
    expected.extend(
        fields
            .into_iter()
            .map(|field| ("manifest".to_owned(), field)),
    );
    let projections = rust_structs(include_str!("../src/documents/verification.rs"));
    for name in ["BehavioralResult", "BlockingGap"] {
        expected.extend(
            projections[name]
                .iter()
                .map(|(field, _)| ("rust-api".to_owned(), format!("{name}.{field}"))),
        );
    }
    expected.extend(
        [
            "DocumentRef.id",
            "DocumentRef.format_version",
            "StoredDocument.reference",
            "StoredDocument.content_hash",
            "StoredDocument.bytes",
            "DocumentStore::get",
            "Validator::validate_structural",
            "Validator::validate_semantic",
            "ManifestValidator::validate_manifest",
            "ManifestValidator::validate_manifest_semantic",
            "check_fidelity_mapping",
            "check_evidence_completeness",
            "verify_artifact_hash",
            "CatalogSnapshot",
            "CatalogSnapshot::embedded",
            "CatalogSnapshot::from_parts",
            "CatalogSnapshot::digest",
            "CatalogSnapshot::schema_bytes",
            "CatalogUrlLoader::new",
            "CatalogUrlLoader::load",
            "Validator::new",
            "ManifestValidator::new",
        ]
        .map(|name| ("rust-api".to_owned(), name.to_owned())),
    );
    expected
}

fn refs(text: &str) -> BTreeSet<String> {
    Regex::new(r"Req [0-9]+(?:\.[0-9]+)?|NFR-[0-9]+(?:\.[0-9]+)?")
        .unwrap()
        .find_iter(text)
        .map(|m| m.as_str().to_owned())
        .collect()
}

fn valid_refs() -> BTreeSet<String> {
    let text = fs::read_to_string(
        root().join(".kiro/specs/contracts-evidence-foundation/requirements.md"),
    )
    .unwrap();
    let heading = Regex::new(r"^### (?:Requirement ([0-9]+):|(NFR-[0-9]+):)").unwrap();
    let item = Regex::new(r"^([0-9]+)\. ").unwrap();
    let mut prefix = String::new();
    let mut valid = BTreeSet::new();
    for line in text.lines() {
        if let Some(c) = heading.captures(line) {
            prefix = c
                .get(1)
                .map(|m| format!("Req {}", m.as_str()))
                .unwrap_or_else(|| c[2].to_owned());
            valid.insert(prefix.clone());
        } else if let Some(c) = item.captures(line)
            && !prefix.is_empty()
        {
            valid.insert(format!("{prefix}.{}", &c[1]));
        }
    }
    valid
}

fn check_fields(text: &str) -> Result<(), String> {
    let expected = expected_fields();
    let valid = valid_refs();
    let mut actual = BTreeSet::new();
    for row in rows(text) {
        if row.len() != 4 {
            return Err(format!("not four columns: {row:?}"));
        }
        let key = (row[0].to_owned(), row[1].trim_matches('`').to_owned());
        if !actual.insert(key.clone()) {
            return Err(format!("duplicate field: {key:?}"));
        }
        let cited = refs(row[2]);
        if cited.is_empty() || !cited.is_subset(&valid) || row[3].is_empty() {
            return Err(format!("invalid field mapping: {row:?}"));
        }
    }
    if actual != expected {
        let missing: Vec<_> = expected.difference(&actual).collect();
        let orphan: Vec<_> = actual.difference(&expected).collect();
        return Err(format!("missing {missing:?}; orphan {orphan:?}"));
    }
    for (schema, path, required) in [
        ("behavioral-contract", "id", "Req 2.1, Req 6.5"),
        (
            "behavioral-contract",
            "format_version",
            "Req 2.1, Req 6.1, Req 6.2",
        ),
        (
            "behavioral-contract",
            "obligations[*].behavioral_fidelity",
            "Req 2.3",
        ),
        ("evidence-record", "classification", "Req 3.1, Req 3.2"),
        (
            "verification-result",
            "tests[*].origin_identity",
            "Req 4.1, Req 8.4",
        ),
        (
            "verification-result",
            "candidate.content_hash.algorithm",
            "Req 4.2",
        ),
    ] {
        let row = rows(text)
            .into_iter()
            .find(|r| r[0] == schema && r[1].trim_matches('`') == path)
            .unwrap();
        if !refs(required).is_subset(&refs(row[2])) {
            return Err(format!("missing explicit Task25 mapping: {schema}.{path}"));
        }
    }
    if !refs("Req 4.8, Req 5.8, Req 8.6, Req 8.7, Req 9.5").is_subset(&refs(text)) {
        return Err("missing P0 requirement mappings".to_owned());
    }
    if !text.contains("BG-01 remains open") {
        return Err("missing non-accepting handoff boundary".to_owned());
    }
    Ok(())
}

fn check_gates(text: &str) -> Result<(), String> {
    let source = fs::read_to_string(
        root().join(".kiro/specs/contracts-evidence-foundation/requirements.md"),
    )
    .unwrap();
    let expected: BTreeMap<_, _> = rows(&source)
        .into_iter()
        .filter(|r| r[0].starts_with("AC-"))
        .map(|r| (r[0].to_owned(), refs(r[2])))
        .collect();
    let known: BTreeSet<_> = expected_fields()
        .into_iter()
        .map(|(s, p)| format!("{s}.{p}"))
        .collect();
    let quoted = Regex::new(r"`([^`]+)`").unwrap();
    let mut seen = BTreeSet::new();
    for row in rows(text) {
        if row.len() != 4 || !seen.insert(row[0].to_owned()) {
            return Err(format!("invalid/duplicate gate: {row:?}"));
        }
        if expected.get(row[0]) != Some(&refs(row[2])) {
            return Err(format!("gate requirement drift: {row:?}"));
        }
        let paths: Vec<_> = quoted
            .captures_iter(row[3])
            .map(|c| c[1].to_owned())
            .collect();
        if row[1].is_empty() || paths.is_empty() || paths.iter().any(|p| !known.contains(p)) {
            return Err(format!("unknown/absent gate field: {row:?}"));
        }
    }
    if seen != expected.keys().cloned().collect() || seen.len() != 5 {
        return Err("missing canonical gates".to_owned());
    }
    if !text.contains("BG-01 remains open") {
        return Err("missing non-accepting handoff boundary".to_owned());
    }
    Ok(())
}

#[test]
fn every_schema_and_manifest_field_has_a_real_requirement_mapping() {
    check_fields(&document("field-to-requirement")).unwrap();
}

#[test]
fn all_five_gates_match_canonical_requirements_and_declared_fields() {
    check_gates(&document("gate-to-requirement")).unwrap();
}

#[test]
fn missing_duplicate_orphan_and_fabricated_traceability_is_rejected() {
    let fields = document("field-to-requirement");
    check_fields(&fields).unwrap();
    let row = fields
        .lines()
        .find(|line| {
            line.starts_with("| behavioral-contract | id |")
                || line.starts_with("| behavioral-contract | `id` |")
        })
        .unwrap();
    assert!(check_fields(&fields.replace(row, "")).is_err());
    assert!(check_fields(&format!("{fields}\n{row}")).is_err());
    assert!(
        check_fields(&format!(
            "{fields}\n| evidence-record | invented | Req 3.1 | fake |\n"
        ))
        .is_err()
    );
    assert!(check_fields(&fields.replace(row, &row.replacen("Req 2.1", "Req 999.1", 1))).is_err());
    for key in [
        "BehavioralResult.test_results",
        "CatalogSnapshot::from_parts",
    ] {
        let row = fields
            .lines()
            .find(|line| line.contains(&format!("| `{key}` |")))
            .unwrap();
        assert!(check_fields(&fields.replace(row, "")).is_err());
    }
    let gates = document("gate-to-requirement");
    check_gates(&gates).unwrap();
    let row = gates
        .lines()
        .find(|line| line.starts_with("| AC-05 |"))
        .unwrap();
    assert!(check_gates(&gates.replace(row, "")).is_err());
    assert!(check_gates(&format!("{gates}\n{row}")).is_err());
    let field = Regex::new(r"`([^`]+)`").unwrap().captures(row).unwrap()[1].to_owned();
    let bad_row = row.replacen(&format!("`{field}`"), "`verification-result.invented`", 1);
    assert!(check_gates(&gates.replace(row, &bad_row)).is_err());
    assert!(check_gates(&gates.replace(row, &row.replacen("Req 10", "Req 999", 1))).is_err());
}
