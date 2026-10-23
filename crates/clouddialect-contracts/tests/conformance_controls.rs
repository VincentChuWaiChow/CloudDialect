use std::{fs, path::{Path, PathBuf}, process::Command, time::{Duration, Instant}};

use serde_json::{Value, json};

#[path = "support/documents.rs"]
mod documents;
#[path = "support/fixtures.rs"]
mod fixtures;

struct Tree(PathBuf);

impl Tree {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("clouddialect-conformance-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn fixture(&self, name: &str, input: &[u8], expected: Value) -> PathBuf {
        let path = self.0.join(name);
        fs::create_dir_all(&path).unwrap();
        let file = if expected["document_kind"] == "manifest" { "input.toml" } else { "input.json" };
        fs::write(path.join(file), input).unwrap();
        fs::write(path.join("expected.json"), serde_json::to_vec(&expected).unwrap()).unwrap();
        path
    }
}

impl Drop for Tree {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

fn expected(phase: &str, kind: &str, outcome: &str) -> Value {
    json!({"phase":phase, "document_kind":kind, "outcome":outcome})
}

fn evidence() -> Value { documents::minimal_documents()[1].1.clone() }

fn bytes(value: &Value) -> Vec<u8> { serde_json::to_vec(value).unwrap() }

#[test]
fn discovery_filters_components_and_rejects_empty_or_escaping_selection() {
    let tree = Tree::new();
    assert!(fixtures::discover(&tree.0, None).is_err());
    for name in ["positive/evidence/one", "positive/evidence/two", "positive-neighbor/three"] {
        tree.fixture(name, &bytes(&evidence()), expected("structural", "evidence-record", "pass"));
    }
    let selected = fixtures::discover(&tree.0, Some("positive")).unwrap();
    assert_eq!(selected.len(), 2);
    assert!(selected.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(selected.iter().all(|p| p.starts_with(tree.0.join("positive"))));
    assert_eq!(fixtures::discover(&tree.0, None).unwrap().len(), 3);
    for invalid in ["", "missing", "../positive", "positive/../positive", "/positive", "positive\\evidence"] {
        assert!(fixtures::discover(&tree.0, Some(invalid)).is_err(), "{invalid}");
    }
}

#[test]
fn discovery_rejects_incomplete_ambiguous_and_symlinked_pairs() {
    let tree = Tree::new();
    let path = tree.fixture("positive/case", &bytes(&evidence()), expected("structural", "evidence-record", "pass"));
    fs::write(path.join("input.toml"), "").unwrap();
    assert!(fixtures::discover(&tree.0, None).is_err());
    fs::remove_file(path.join("input.toml")).unwrap();
    fs::remove_file(path.join("expected.json")).unwrap();
    assert!(fixtures::discover(&tree.0, None).is_err());
    #[cfg(unix)] {
        std::os::unix::fs::symlink(path.join("input.json"), path.join("expected.json")).unwrap();
        assert!(fixtures::discover(&tree.0, None).is_err());
    }
}

#[test]
fn structural_outcomes_and_verbatim_warnings_use_real_public_validation() {
    let tree = Tree::new();
    for (kind, value) in documents::minimal_documents() {
        let name = serde_json::to_value(kind).unwrap();
        let path = tree.fixture("case", &bytes(&value), expected("structural", name.as_str().unwrap(), "pass"));
        fixtures::check_fixture(&path).unwrap();
    }
    let mut value = evidence();
    value["CORE:field"] = json!(true);
    let mut oracle = expected("structural", "evidence-record", "pass");
    oracle["warnings"] = json!([{"field_name_verbatim":"CORE:field", "description_contains":"reserved for core use"}]);
    let path = tree.fixture("case", &bytes(&value), oracle);
    fixtures::check_fixture(&path).unwrap();
    value.as_object_mut().unwrap().remove("CORE:field");
    value["format_version"] = json!("1.1.0");
    let mut oracle = expected("structural", "evidence-record", "warn");
    oracle["warnings"] = json!([{"field_name_verbatim":"format_version", "description_contains":"1.1.0"}]);
    fixtures::check_fixture(&tree.fixture("case", &bytes(&value), oracle)).unwrap();
    value["format_version"] = json!("1.0.0");
    value["$schema"] = json!("https://example.com/unknown-schema");
    let mut oracle = expected("structural", "evidence-record", "fail");
    oracle["errors"] = json!([{"category":"unresolvable-reference", "field_path":"$schema", "description_contains":"https://example.com/unknown-schema"}]);
    fixtures::check_fixture(&tree.fixture("case", &bytes(&value), oracle)).unwrap();
}

#[test]
fn wrong_oracles_missing_extra_diagnostics_and_invalid_unions_fail_with_context() {
    let tree = Tree::new();
    let mut bad = evidence();
    bad["$schema"] = json!("https://example.com/unknown-schema");
    let good_error = json!({"category":"unresolvable-reference", "field_path":"$schema", "description_contains":"unknown-schema"});
    for errors in [json!([]), json!([good_error.clone(), good_error.clone()]), json!([{"category":"structural"}]), json!([{"category":"unresolvable-reference", "field_path":null}]), json!([{"category":"unresolvable-reference", "field_path":"wrong"}]), json!([{"category":"unresolvable-reference", "description_contains":"missing-text"}]), json!([{"category":"unresolvable-reference", "typo":true}])] {
        let mut oracle = expected("structural", "evidence-record", "fail");
        oracle["errors"] = errors;
        let path = tree.fixture("negative/case", &bytes(&bad), oracle);
        let error = fixtures::check_fixture(&path).unwrap_err();
        assert!(error.contains(path.to_str().unwrap()), "{error}");
    }
    for mut oracle in [expected("structural", "evidence-record", "pass"), expected("behavioral", "evidence-record", "pass"), expected("semantic", "evidence-record", "blocked"), expected("unknown", "evidence-record", "pass"), expected("structural", "unknown", "pass")] {
        oracle["typo"] = json!(true);
        assert!(fixtures::check_fixture(&tree.fixture("case", &bytes(&evidence()), oracle)).is_err());
    }
    for oracle in [expected("behavioral", "evidence-record", "pass"), expected("semantic", "evidence-record", "blocked"), expected("unknown", "evidence-record", "pass"), expected("structural", "unknown", "pass"), json!({"phase":"structural"}), json!({"document_kind":"evidence-record", "outcome":"pass"})] {
        assert!(fixtures::check_fixture(&tree.fixture("case", &bytes(&evidence()), oracle)).is_err());
    }
    let path = tree.fixture("deliberate-mismatch", &bytes(&bad), expected("structural", "evidence-record", "pass"));
    let error = fixtures::check_fixture(&path).unwrap_err();
    assert!(error.contains("outcome") && error.contains("pass") && error.contains("fail"));
}

#[test]
fn semantic_and_completeness_phases_use_explicit_synthetic_scoped_store_bytes() {
    use sha2::{Digest, Sha256};
    let tree = Tree::new();
    let mut contract = documents::minimal_documents()[0].1.clone();
    let record = evidence();
    let id = record["id"].as_str().unwrap();
    contract["obligations"][0].as_object_mut().unwrap().remove("unresolved_assumption");
    contract["obligations"][0]["evidence_refs"] = json!([id]);
    let raw = bytes(&record);
    let stored = json!({"lookup_id":id, "lookup_version":"1.0.0", "reference":{"id":id,"format_version":"1.0.0"}, "content_hash":{"algorithm":"sha-256", "value":format!("{:x}",Sha256::digest(&raw))}, "bytes":raw});
    let mut oracle = expected("semantic", "behavioral-contract", "pass");
    oracle["store"] = json!([stored.clone()]);
    fixtures::check_fixture(&tree.fixture("case", &bytes(&contract), oracle)).unwrap();
    let mut oracle = expected("behavioral", "behavioral-contract", "evidence-complete");
    oracle["candidate_id"] = json!("candidate");
    oracle["store"] = json!([stored.clone()]);
    fixtures::check_fixture(&tree.fixture("case", &bytes(&contract), oracle)).unwrap();
    for (entries, outage) in [(json!([]),false),(json!([stored.clone(),stored]),false),(json!([]),true)] {
        let mut oracle = expected("behavioral", "behavioral-contract", "blocked");
        oracle["candidate_id"] = json!("candidate");
        oracle["store"] = entries;
        oracle["store_error"] = json!(outage);
        oracle["blockers"] = json!([{"obligation_id":"o1", "assumption_text_contains":"Evidence"}]);
        fixtures::check_fixture(&tree.fixture("case", &bytes(&contract), oracle)).unwrap();
    }
    contract["obligations"] = json!([]);
    let mut oracle = expected("behavioral", "behavioral-contract", "deferred");
    oracle["candidate_id"] = json!("candidate");
    fixtures::check_fixture(&tree.fixture("case", &bytes(&contract), oracle)).unwrap();
}

#[test]
fn artifact_hash_uses_boolean_only_and_roundtrip_checks_actual_encoded_values() {
    let tree = Tree::new();
    let mut verification = documents::minimal_documents()[2].1.clone();
    verification["candidate"]["content_hash"]["value"] = json!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    for (candidate, matches) in [(b"abc".as_slice(),true),(b"tampered".as_slice(),false)] {
        let oracle = json!({"phase":"artifact-hash", "document_kind":"verification-result", "candidate_bytes":candidate, "expected_match":matches});
        fixtures::check_fixture(&tree.fixture("case", &bytes(&verification), oracle.clone())).unwrap();
        let mut wrong = oracle.clone();wrong["expected_match"] = json!(!matches);
        assert!(fixtures::check_fixture(&tree.fixture("case", &bytes(&verification), wrong)).is_err());
        let mut wrong = oracle;wrong["outcome"] = json!("pass");
        assert!(fixtures::check_fixture(&tree.fixture("case", &bytes(&verification), wrong)).is_err());
    }
    let mut raw = serde_json::to_string(&evidence()).unwrap();raw.pop();
    raw.push_str(r#", "acme:meta":{ "n":1e+04, "u":"\u0061", "order":[ 2, 1 ] }}"#);
    let mut oracle = expected("structural", "evidence-record", "pass");oracle["round_trip"] = json!(true);
    fixtures::check_fixture(&tree.fixture("case", raw.as_bytes(), oracle)).unwrap();
}

fn child(mode: &str, fixture: &Path) -> Command {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command.args(["--exact", "fixture_worker", "--nocapture"]).env("CD_CONFORMANCE_WORKER_MODE", mode).env("CD_CONFORMANCE_WORKER_PATH", fixture);
    command
}

#[test]
fn bounded_child_reports_mismatches_and_kills_and_reaps_a_stuck_validator() {
    assert_eq!(fixtures::FIXTURE_TIMEOUT, Duration::from_secs(5));
    let tree = Tree::new();
    let path = tree.fixture("case", &bytes(&evidence()), expected("structural", "evidence-record", "pass"));
    fixtures::run_bounded(child("fixture", &path), fixtures::FIXTURE_TIMEOUT).unwrap();
    let mut value = evidence();value["classification"] = json!("bad");
    tree.fixture("case", &bytes(&value), expected("structural", "evidence-record", "pass"));
    assert!(fixtures::run_bounded(child("fixture", &path), fixtures::FIXTURE_TIMEOUT).unwrap_err().contains("outcome"));
    let pid_path = tree.0.join("child.pid");
    let started = Instant::now();
    let error = fixtures::run_bounded(child("sleep", &pid_path), Duration::from_secs(1)).unwrap_err();
    assert!(error.contains("timed out"), "{error}");
    assert!(started.elapsed() < Duration::from_secs(4));
    let pid = fs::read_to_string(pid_path).unwrap();
    #[cfg(target_os = "linux")]
    assert!(!Path::new("/proc").join(pid.trim()).exists(), "child not reaped");
    #[cfg(not(target_os = "linux"))]
    assert!(!pid.trim().is_empty());
}

#[test]
fn fixture_worker() {
    match std::env::var("CD_CONFORMANCE_WORKER_MODE").ok().as_deref() {
        Some("sleep") => {
            fs::write(std::env::var_os("CD_CONFORMANCE_WORKER_PATH").unwrap(), std::process::id().to_string()).unwrap();
            std::thread::sleep(Duration::from_secs(60));
        }
        Some("fixture") => {
            let path = PathBuf::from(std::env::var_os("CD_CONFORMANCE_WORKER_PATH").unwrap());
            if let Err(error) = fixtures::check_fixture(&path) {
                eprintln!("{error}");std::process::exit(1);
            }
        }
        None => {}
        Some(other) => panic!("unsupported control worker mode {other:?}"),
    }
}
