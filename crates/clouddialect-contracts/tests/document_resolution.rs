#[path = "../src/validator/artifact_hash.rs"]
mod artifact_hash;
#[path = "support/documents.rs"]
mod documents;
#[path = "../src/validator/mandatory_extensions.rs"]
mod mandatory_extensions;
#[path = "../src/validator/document_resolution.rs"]
mod references;

pub use clouddialect_contracts::*;
use references::resolve_document;
use serde_json::{Value, json};
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_512};
use std::{cell::RefCell, collections::BTreeMap};

const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const OTHER: &str = "550e8400-e29b-41d4-a716-446655440001";

struct Store {
    result: Result<Option<StoredDocument>, ValidatorError>,
    calls: RefCell<Vec<(String, Option<String>)>>,
}

impl DocumentStore for Store {
    fn get(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<Option<StoredDocument>, ValidatorError> {
        self.calls
            .borrow_mut()
            .push((id.to_owned(), version.map(str::to_owned)));
        self.result.clone()
    }
}

fn validator() -> Validator {
    Validator::new(CatalogSnapshot::embedded().unwrap())
}

fn hash(algorithm: &str, bytes: &[u8]) -> ContentHash {
    let bytes = match algorithm {
        "sha-256" => Sha256::digest(bytes).to_vec(),
        "sha-384" => Sha384::digest(bytes).to_vec(),
        "sha-512" => Sha512::digest(bytes).to_vec(),
        "sha3-256" => Sha3_256::digest(bytes).to_vec(),
        "sha3-512" => Sha3_512::digest(bytes).to_vec(),
        _ => unreachable!(),
    };
    ContentHash {
        algorithm: algorithm.to_owned(),
        value: bytes.iter().map(|b| format!("{b:02x}")).collect(),
    }
}

fn stored(value: &Value, algorithm: &str) -> StoredDocument {
    let bytes = serde_json::to_vec_pretty(value).unwrap();
    StoredDocument {
        reference: DocumentRef {
            id: ID.to_owned(),
            format_version: "1.0.0".to_owned(),
        },
        content_hash: hash(algorithm, &bytes),
        bytes,
    }
}

fn store(value: StoredDocument) -> Store {
    Store {
        result: Ok(Some(value)),
        calls: RefCell::new(Vec::new()),
    }
}

fn evidence() -> Value {
    documents::minimal_documents()[1].1.clone()
}

fn errors(s: &Store, kind: DocumentKind) -> Vec<ValidatorError> {
    resolve_document(&validator(), s, ID, Some("1.0.0"), kind).unwrap_err()
}

#[test]
fn valid_exact_references_preserve_original_bytes_with_every_supported_digest() {
    for algorithm in ["sha-256", "sha-384", "sha-512", "sha3-256", "sha3-512"] {
        let expected = stored(&evidence(), algorithm);
        let mut verification: VerificationResult =
            serde_json::from_value(documents::minimal_documents()[2].1.clone()).unwrap();
        verification.candidate.content_hash = expected.content_hash.clone();
        assert!(artifact_hash::verify_artifact_hash(
            &verification,
            &expected.bytes
        ));
        let s = store(expected.clone());
        assert_eq!(
            resolve_document(
                &validator(),
                &s,
                ID,
                Some("1.0.0"),
                DocumentKind::EvidenceRecord
            )
            .unwrap(),
            expected
        );
        assert_eq!(
            *s.calls.borrow(),
            vec![(ID.to_owned(), Some("1.0.0".to_owned()))]
        );
        let mut tampered = expected;
        tampered.bytes.push(b' ');
        assert!(!errors(&store(tampered), DocumentKind::EvidenceRecord).is_empty());
    }
}

#[test]
fn unversioned_lookup_is_reserved_for_globally_unique_evidence() {
    let s = store(stored(&evidence(), "sha-256"));
    assert!(resolve_document(&validator(), &s, ID, None, DocumentKind::EvidenceRecord).is_ok());
    assert_eq!(*s.calls.borrow(), vec![(ID.to_owned(), None)]);
    for kind in [
        DocumentKind::BehavioralContract,
        DocumentKind::VerificationResult,
        DocumentKind::CatalogIndex,
        DocumentKind::ExtensionRegistry,
    ] {
        let s = store(stored(&evidence(), "sha-256"));
        assert!(resolve_document(&validator(), &s, ID, None, kind).is_err());
        assert!(s.calls.borrow().is_empty());
    }
    let s = store(stored(&evidence(), "sha-256"));
    assert!(
        resolve_document(
            &validator(),
            &s,
            "",
            Some("1.0.0"),
            DocumentKind::EvidenceRecord
        )
        .is_err()
    );
    assert!(s.calls.borrow().is_empty());
    let s = store(stored(&evidence(), "sha-256"));
    assert!(
        resolve_document(&validator(), &s, ID, Some(""), DocumentKind::EvidenceRecord).is_err()
    );
    assert!(s.calls.borrow().is_empty());
}

#[test]
fn returned_metadata_body_and_requested_identity_and_version_must_match_exactly() {
    let mut cases = Vec::new();
    let original = stored(&evidence(), "sha-256");
    let mut wrong = original.clone();
    wrong.reference.id = OTHER.to_owned();
    cases.push(wrong);
    let mut wrong = original.clone();
    wrong.reference.format_version = "1.0.0+build".to_owned();
    cases.push(wrong);
    for (field, value) in [("id", OTHER), ("format_version", "1.0.0+build")] {
        let mut document = evidence();
        document[field] = json!(value);
        cases.push(stored(&document, "sha-256"));
    }
    for wrong in cases {
        let errors = errors(&store(wrong), DocumentKind::EvidenceRecord);
        assert!(
            errors
                .iter()
                .any(|e| e.category == ErrorCategory::UnresolvableReference)
        );
    }
    let mut unversioned = original;
    unversioned.reference.format_version = "1.0.0+other".to_owned();
    assert!(
        resolve_document(
            &validator(),
            &store(unversioned),
            ID,
            None,
            DocumentKind::EvidenceRecord
        )
        .is_err()
    );
}

#[test]
fn missing_and_unavailable_stores_fail_without_echoing_backend_details() {
    for result in [
        Ok(None),
        Err(ValidatorError {
            document_id: Some("PRIVATE-SENTINEL".into()),
            format_version: None,
            category: ErrorCategory::Semantic,
            field_path: Some("PRIVATE-SENTINEL".into()),
            description: "PRIVATE-SENTINEL".into(),
            byte_offset: Some(123),
        }),
    ] {
        let s = Store {
            result,
            calls: RefCell::new(Vec::new()),
        };
        let failures = errors(&s, DocumentKind::EvidenceRecord);
        assert!(!failures.is_empty());
        assert!(
            failures
                .iter()
                .all(|e| e.category == ErrorCategory::UnresolvableReference)
        );
        assert!(!format!("{failures:?}").contains("PRIVATE-SENTINEL"));
    }
}

#[test]
fn tampered_bytes_unknown_hashes_and_noncanonical_digests_never_resolve() {
    let original = stored(&evidence(), "sha-256");
    let mut cases = Vec::new();
    let mut wrong = original.clone();
    wrong.bytes.push(b' ');
    cases.push(wrong);
    let mut wrong = original.clone();
    wrong.content_hash.algorithm = "SHA-256".into();
    cases.push(wrong);
    for digest in [
        "0".repeat(64),
        original.content_hash.value.to_uppercase(),
        "00".into(),
        "g".repeat(64),
    ] {
        let mut wrong = original.clone();
        wrong.content_hash.value = digest;
        cases.push(wrong);
    }
    for wrong in cases {
        assert!(!errors(&store(wrong), DocumentKind::EvidenceRecord).is_empty());
    }
}

#[test]
fn trusted_kind_schema_parse_and_duplicate_checks_cannot_be_bypassed_by_a_valid_hash() {
    let s = store(stored(&evidence(), "sha-256"));
    assert!(!errors(&s, DocumentKind::BehavioralContract).is_empty());
    let mut invalid = evidence();
    invalid.as_object_mut().unwrap().remove("claim");
    assert!(
        !errors(
            &store(stored(&invalid, "sha-256")),
            DocumentKind::EvidenceRecord
        )
        .is_empty()
    );
    for bytes in [
        b"not-json PRIVATE-SENTINEL".to_vec(),
        format!(
            "{{\"id\":\"{ID}\",{}",
            String::from_utf8(serde_json::to_vec(&evidence()).unwrap())
                .unwrap()
                .trim_start_matches('{')
        )
        .into_bytes(),
    ] {
        let mut document = stored(&evidence(), "sha-256");
        document.content_hash = hash("sha-256", &bytes);
        document.bytes = bytes;
        let failures = errors(&store(document), DocumentKind::EvidenceRecord);
        assert!(!failures.is_empty());
        assert!(!format!("{failures:?}").contains("PRIVATE-SENTINEL"));
    }
}

#[test]
fn resolution_uses_the_callers_active_catalog_not_an_embedded_fallback() {
    let index = include_bytes!("../../../catalog/index.json").to_vec();
    let material = [
        (
            "behavioral-contract",
            include_bytes!("../../../catalog/schemas/behavioral-contract/1.0/schema.json")
                .as_slice(),
        ),
        (
            "evidence-record",
            include_bytes!("../../../catalog/schemas/evidence-record/1.0/schema.json").as_slice(),
        ),
        (
            "verification-result",
            include_bytes!("../../../catalog/schemas/verification-result/1.0/schema.json")
                .as_slice(),
        ),
        (
            "catalog-index",
            include_bytes!("../../../catalog/schemas/catalog-index/1.0/schema.json").as_slice(),
        ),
        (
            "extension-registry",
            include_bytes!("../../../catalog/schemas/extension-registry/1.0/schema.json")
                .as_slice(),
        ),
    ];
    let mut material: BTreeMap<String, Vec<u8>> = material
        .into_iter()
        .map(|(kind, bytes)| (format!("schemas/{kind}/1.0/schema.json"), bytes.to_vec()))
        .collect();
    let path = "schemas/evidence-record/1.0/schema.json";
    let mut schema: Value = serde_json::from_slice(&material[path]).unwrap();
    schema["properties"]["claim"]["const"] = json!("required-by-active-catalog");
    material.insert(path.into(), serde_json::to_vec(&schema).unwrap());
    let active = Validator::new(CatalogSnapshot::from_parts(index, material).unwrap());
    let s = store(stored(&evidence(), "sha-256"));
    assert!(
        resolve_document(
            &validator(),
            &s,
            ID,
            Some("1.0.0"),
            DocumentKind::EvidenceRecord
        )
        .is_ok()
    );
    assert!(
        resolve_document(&active, &s, ID, Some("1.0.0"), DocumentKind::EvidenceRecord).is_err()
    );
}

#[test]
fn mandatory_extension_declarations_block_even_without_payload_or_with_approval_claims() {
    for payload in [false, true] {
        let mut value = evidence();
        value["required_extensions"] = json!(["acme"]);
        if payload {
            value["acme:approved"] = json!({"accepted":true,"owner":"trusted-core"});
        }
        let failures = errors(
            &store(stored(&value, "sha-256")),
            DocumentKind::EvidenceRecord,
        );
        assert!(
            failures
                .iter()
                .any(|e| e.category == ErrorCategory::BlockingGap
                    && e.field_path.as_deref() == Some("required_extensions[0]"))
        );
    }
}
