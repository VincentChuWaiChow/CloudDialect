//! Test-only fixture data and oracles; never a production execution sandbox.

use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use clouddialect_contracts::*;
use serde::Deserialize;
use serde_json::{Value, json, value::RawValue};

pub const FIXTURE_TIMEOUT: Duration = Duration::from_secs(5);

pub fn discover(root: &Path, prefix: Option<&str>) -> Result<Vec<PathBuf>, String> {
    let prefix = prefix
        .map(|value| {
            if value.is_empty()
                || value.contains('\\')
                || value
                    .split('/')
                    .any(|part| part == "." || part == ".." || part.is_empty())
                || Path::new(value)
                    .components()
                    .any(|part| !matches!(part, Component::Normal(_)))
            {
                Err(format!("invalid fixture directory prefix {value:?}"))
            } else {
                Ok(PathBuf::from(value))
            }
        })
        .transpose()?;
    let mut found = Vec::new();
    visit(root, &mut found)?;
    if found.is_empty() {
        return Err(format!("no fixtures discovered under {}", root.display()));
    }
    if let Some(prefix) = prefix {
        found.retain(|path| {
            path.strip_prefix(root)
                .is_ok_and(|p| p.starts_with(&prefix))
        });
    }
    found.sort();
    if found.is_empty() {
        return Err("no fixtures selected by directory prefix".to_owned());
    }
    Ok(found)
}

fn visit(directory: &Path, found: &mut Vec<PathBuf>) -> Result<(), String> {
    let metadata =
        fs::symlink_metadata(directory).map_err(|e| format!("{}: {e}", directory.display()))?;
    if metadata.is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "not a regular fixture directory: {}",
            directory.display()
        ));
    }
    let entries = fs::read_dir(directory).map_err(|e| e.to_string())?;
    let mut children = Vec::new();
    let (mut json_input, mut toml_input, mut oracle) = (false, false, false);
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            return Err(format!("symlink in fixture tree: {}", path.display()));
        }
        if kind.is_dir() {
            children.push(path);
        } else if kind.is_file() {
            match entry.file_name().to_str() {
                Some("input.json") => json_input = true,
                Some("input.toml") => toml_input = true,
                Some("expected.json") => oracle = true,
                _ => {}
            }
        } else {
            return Err(format!("non-regular fixture entry: {}", path.display()));
        }
    }
    if json_input || toml_input || oracle {
        if !oracle || json_input == toml_input || !children.is_empty() {
            return Err(format!(
                "incomplete, ambiguous or non-leaf fixture: {}",
                directory.display()
            ));
        }
        found.push(directory.to_owned());
    }
    for child in children {
        visit(&child, found)?;
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Expected {
    phase: String,
    document_kind: String,
    outcome: Option<String>,
    expected_match: Option<bool>,
    candidate_id: Option<String>,
    candidate_bytes: Option<Vec<u8>>,
    #[serde(default)]
    errors: Vec<Value>,
    #[serde(default)]
    warnings: Vec<Value>,
    #[serde(default)]
    blockers: Vec<Value>,
    #[serde(default)]
    store: Vec<StoreEntry>,
    #[serde(default)]
    store_error: bool,
    #[serde(default)]
    round_trip: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StoreEntry {
    lookup_id: String,
    lookup_version: String,
    reference: DocumentRef,
    content_hash: ContentHash,
    bytes: Vec<u8>,
}

struct Store<'a>(&'a Expected);

impl DocumentStore for Store<'_> {
    fn get(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<Option<StoredDocument>, ValidatorError> {
        let matches: Vec<_> = self
            .0
            .store
            .iter()
            .filter(|entry| {
                entry.lookup_id == id && version.is_none_or(|v| entry.lookup_version == v)
            })
            .collect();
        if self.0.store_error || matches.len() > 1 {
            return Err(ValidatorError {
                document_id: None,
                format_version: None,
                category: ErrorCategory::UnresolvableReference,
                field_path: None,
                description: "Synthetic fixture store unavailable or ambiguous".to_owned(),
                byte_offset: None,
            });
        }
        Ok(matches.first().map(|entry| StoredDocument {
            reference: entry.reference.clone(),
            content_hash: entry.content_hash.clone(),
            bytes: entry.bytes.clone(),
        }))
    }
}

pub fn check_fixture(directory: &Path) -> Result<(), String> {
    check(directory).map_err(|error| format!("{}: {error}", directory.display()))
}

fn check(directory: &Path) -> Result<(), String> {
    let oracle = fs::read(directory.join("expected.json")).map_err(|e| e.to_string())?;
    let expected: Expected =
        serde_json::from_slice(&oracle).map_err(|e| format!("invalid expected.json: {e}"))?;
    let raw: Value = serde_json::from_slice(&oracle).map_err(|e| e.to_string())?;
    let manifest = expected.document_kind == "manifest";
    let kind = if manifest {
        None
    } else {
        Some(
            serde_json::from_value::<DocumentKind>(json!(expected.document_kind))
                .map_err(|e| format!("invalid document_kind: {e}"))?,
        )
    };
    let allowed = match expected.phase.as_str() {
        "structural" if manifest => ["pass", "fail"].as_slice(),
        "structural" => ["pass", "warn", "fail"].as_slice(),
        "semantic" => ["pass", "fail"].as_slice(),
        "behavioral" if kind == Some(DocumentKind::BehavioralContract) => {
            ["deferred", "evidence-complete", "blocked"].as_slice()
        }
        "artifact-hash" if kind == Some(DocumentKind::VerificationResult) => [].as_slice(),
        _ => return Err("unsupported phase/document_kind combination".to_owned()),
    };
    let hash_phase = expected.phase == "artifact-hash";
    if hash_phase {
        if raw.get("outcome").is_some()
            || expected.expected_match.is_none()
            || expected.candidate_bytes.is_none()
        {
            return Err(
                "artifact-hash requires expected_match/candidate_bytes and no outcome".to_owned(),
            );
        }
    } else if !expected
        .outcome
        .as_deref()
        .is_some_and(|value| allowed.contains(&value))
        || raw.get("expected_match").is_some()
        || raw.get("candidate_bytes").is_some()
    {
        return Err("unsupported phase-specific outcome or hash fields".to_owned());
    }
    if (expected.phase == "behavioral") != expected.candidate_id.is_some() {
        return Err("candidate_id is required only for behavioral completeness".to_owned());
    }
    if !matches!(expected.phase.as_str(), "semantic" | "behavioral")
        && (!expected.store.is_empty() || expected.store_error)
    {
        return Err(
            "store context is supported only for semantic/completeness fixtures".to_owned(),
        );
    }
    if expected.round_trip
        && (expected.phase != "structural"
            || !matches!(
                kind,
                Some(
                    DocumentKind::BehavioralContract
                        | DocumentKind::EvidenceRecord
                        | DocumentKind::VerificationResult
                )
            ))
    {
        return Err("round_trip requires structural JSON document DTO validation".to_owned());
    }
    let input = fs::read(directory.join(if manifest { "input.toml" } else { "input.json" }))
        .map_err(|e| e.to_string())?;
    let snapshot = CatalogSnapshot::embedded().map_err(|e| e.description)?;
    let validator = Validator::new(snapshot.clone());
    let store = Store(&expected);
    let mut actual = match (expected.phase.as_str(), kind) {
        ("structural", None) => serde_json::to_value(ManifestValidator::new(snapshot).validate_manifest(&input)),
        ("structural", Some(kind)) => serde_json::to_value(validator.validate_structural(kind, &input)),
        ("semantic", None) => serde_json::to_value(ManifestValidator::new(snapshot).validate_manifest_semantic(&input, &store)),
        ("semantic", Some(kind)) => serde_json::to_value(validator.validate_semantic(kind, &input, &store)),
        ("behavioral", _) => {
            let contract: BehavioralContract = serde_json::from_slice(&input).map_err(|e| format!("invalid typed completeness input: {e}"))?;
            let result = check_evidence_completeness(&contract, expected.candidate_id.as_deref().unwrap(), &store, &validator);
            if result.verification_result.is_some() || !result.test_results.is_empty() {
                return Err("completeness populated an accepting verification projection".to_owned());
            }
            if result.blocking_gaps.iter().any(|gap| Some(gap.candidate_id.as_str()) != expected.candidate_id.as_deref()) {
                return Err("blocking-gap candidate identity mismatch".to_owned());
            }
            serde_json::to_value(result)
        }
        ("artifact-hash", _) => {
            let result: VerificationResult = serde_json::from_slice(&input).map_err(|e| format!("invalid typed hash input: {e}"))?;
            Ok(json!({"expected_match":verify_artifact_hash(&result,expected.candidate_bytes.as_deref().unwrap())}))
        }
        _ => unreachable!("phase/kind union validated above"),
    }.map_err(|e| e.to_string())?;
    if manifest && expected.phase == "structural" {
        let errors: Vec<_> = ["structural_errors", "semantic_errors"]
            .into_iter()
            .flat_map(|field| actual[field].as_array().unwrap().iter().cloned())
            .collect();
        actual["errors"] = Value::Array(errors);
    }
    let (field, wanted) = if hash_phase {
        ("expected_match", json!(expected.expected_match.unwrap()))
    } else {
        ("outcome", json!(expected.outcome.as_deref().unwrap()))
    };
    if actual.get(field) != Some(&wanted) {
        return Err(format!(
            "{field}: expected {wanted}, actual {}",
            actual.get(field).unwrap_or(&Value::Null)
        ));
    }
    for (label, wanted, actual_field) in [
        ("errors", &expected.errors, "errors"),
        ("warnings", &expected.warnings, "warnings"),
        ("blockers", &expected.blockers, "blocking_gaps"),
    ] {
        let got = actual
            .get(actual_field)
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        match_diagnostics(label, wanted, got)?;
    }
    if expected.round_trip {
        if actual["outcome"] == "fail" {
            return Err("cannot round-trip structurally invalid input".to_owned());
        }
        round_trip(kind.unwrap(), &input, &validator)?;
    }
    Ok(())
}

fn match_diagnostics(label: &str, expected: &[Value], actual: &[Value]) -> Result<(), String> {
    if expected.len() != actual.len() {
        return Err(format!(
            "{label}: expected {} entries, actual {}",
            expected.len(),
            actual.len()
        ));
    }
    let mut compatible = Vec::new();
    for oracle in expected {
        let object = oracle
            .as_object()
            .ok_or_else(|| format!("{label} expectation must be an object"))?;
        let (required, substring, text, keys) = match label {
            "errors" => (
                "category",
                "description_contains",
                "description",
                ["category", "field_path", "description_contains"].as_slice(),
            ),
            "warnings" => (
                "field_name_verbatim",
                "description_contains",
                "description",
                ["field_name_verbatim", "description_contains"].as_slice(),
            ),
            _ => (
                "obligation_id",
                "assumption_text_contains",
                "assumption_text",
                ["obligation_id", "assumption_text_contains"].as_slice(),
            ),
        };
        if object.keys().any(|key| !keys.contains(&key.as_str()))
            || !object.get(required).is_some_and(Value::is_string)
            || object.get(substring).is_some_and(|v| !v.is_string())
            || object
                .get("field_path")
                .is_some_and(|v| !v.is_null() && !v.is_string())
        {
            return Err(format!("invalid {label} expectation fields"));
        }
        if label == "errors" {
            serde_json::from_value::<ErrorCategory>(object[required].clone())
                .map_err(|e| e.to_string())?;
        }
        compatible.push(
            actual
                .iter()
                .map(|got| {
                    got.get(required) == object.get(required)
                        && object
                            .get("field_path")
                            .is_none_or(|path| got.get("field_path") == Some(path))
                        && object.get(substring).is_none_or(|part| {
                            got.get(text)
                                .and_then(Value::as_str)
                                .is_some_and(|value| value.contains(part.as_str().unwrap()))
                        })
                })
                .collect::<Vec<_>>(),
        );
    }
    let mut assigned = vec![None; actual.len()];
    for index in 0..expected.len() {
        if !assign(
            index,
            &compatible,
            &mut assigned,
            &mut vec![false; actual.len()],
        ) {
            return Err(format!("{label}: expected {expected:?}, actual {actual:?}"));
        }
    }
    Ok(())
}

fn assign(
    index: usize,
    compatible: &[Vec<bool>],
    assigned: &mut [Option<usize>],
    seen: &mut [bool],
) -> bool {
    for actual in 0..assigned.len() {
        if compatible[index][actual] && !seen[actual] {
            seen[actual] = true;
            if assigned[actual].is_none_or(|old| assign(old, compatible, assigned, seen)) {
                assigned[actual] = Some(index);
                return true;
            }
        }
    }
    false
}

fn round_trip(kind: DocumentKind, input: &[u8], validator: &Validator) -> Result<(), String> {
    let encoded = match kind {
        DocumentKind::BehavioralContract => serde_json::to_vec(
            &serde_json::from_slice::<BehavioralContract>(input).map_err(|e| e.to_string())?,
        ),
        DocumentKind::EvidenceRecord => serde_json::to_vec(
            &serde_json::from_slice::<EvidenceRecord>(input).map_err(|e| e.to_string())?,
        ),
        DocumentKind::VerificationResult => serde_json::to_vec(
            &serde_json::from_slice::<VerificationResult>(input).map_err(|e| e.to_string())?,
        ),
        _ => unreachable!("typed round-trip kind validated above"),
    }
    .map_err(|e| e.to_string())?;
    let mut before = BTreeMap::new();
    let mut after = BTreeMap::new();
    extension_values(input, "$", &mut before)?;
    extension_values(&encoded, "$", &mut after)?;
    if before.is_empty() || before != after {
        return Err("round-trip extension value bytes missing or changed".to_owned());
    }
    if validator.validate_structural(kind, input) != validator.validate_structural(kind, &encoded) {
        return Err("round-trip structural result changed".to_owned());
    }
    Ok(())
}

fn extension_values(
    input: &[u8],
    path: &str,
    values: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    match input.iter().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => {
            let fields: BTreeMap<String, Box<RawValue>> =
                serde_json::from_slice(input).map_err(|e| e.to_string())?;
            for (key, raw) in fields {
                let path = format!("{path}[{key:?}]");
                if key.contains(':') {
                    values.insert(path, raw.get().to_owned());
                } else {
                    extension_values(raw.get().as_bytes(), &path, values)?;
                }
            }
        }
        Some(b'[') => {
            let fields: Vec<Box<RawValue>> =
                serde_json::from_slice(input).map_err(|e| e.to_string())?;
            for (index, raw) in fields.iter().enumerate() {
                extension_values(raw.get().as_bytes(), &format!("{path}[{index}]"), values)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn run_bounded(mut command: Command, timeout: Duration) -> Result<(), String> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    let started = Instant::now();
    let mut child = command
        .spawn()
        .map_err(|e| format!("could not start fixture validator: {e}"))?;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut detail = String::new();
                child
                    .stderr
                    .take()
                    .unwrap()
                    .take(16_384)
                    .read_to_string(&mut detail)
                    .map_err(|e| e.to_string())?;
                return if status.success() {
                    Ok(())
                } else {
                    Err(format!("fixture validator {status}: {detail}"))
                };
            }
            Ok(None) => {}
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("could not observe fixture validator: {error}"));
            }
        }
        if started.elapsed() >= timeout {
            if let Err(error) = child.kill()
                && child.try_wait().map_err(|e| e.to_string())?.is_none()
            {
                return Err(format!(
                    "fixture timeout; child termination unconfirmed: {error}"
                ));
            }
            child
                .wait()
                .map_err(|e| format!("fixture timeout; child reaping failed: {e}"))?;
            return Err(format!(
                "fixture timed out after {timeout:?}; child terminated and reaped"
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
