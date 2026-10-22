use clouddialect_contracts::{
    ContentHash, DocumentKind, DocumentRef, DocumentStore, ErrorCategory, StoredDocument,
    ValidatorError,
};
use serde_json::json;

fn stored(version: &str, bytes: &[u8]) -> StoredDocument {
    StoredDocument {
        reference: DocumentRef {
            id: "evidence-1".into(),
            format_version: version.into(),
        },
        content_hash: ContentHash {
            algorithm: "sha-256".into(),
            value: "a".repeat(64),
        },
        bytes: bytes.into(),
    }
}

// A consumer double demonstrating the port contract, not a production scoped store.
struct ScopedStore<'a> {
    project: &'a str,
    documents: &'a [(&'a str, StoredDocument)],
    unavailable: bool,
}

impl DocumentStore for ScopedStore<'_> {
    fn get(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<Option<StoredDocument>, ValidatorError> {
        let failure = || ValidatorError {
            document_id: Some(id.into()),
            format_version: version.map(str::to_owned),
            category: ErrorCategory::Semantic,
            field_path: None,
            description: "scoped reference resolution failed".into(),
            byte_offset: None,
        };
        if self.unavailable {
            return Err(failure());
        }
        let matches: Vec<_> = self
            .documents
            .iter()
            .filter(|(project, document)| {
                *project == self.project
                    && document.reference.id == id
                    && version.is_none_or(|version| document.reference.format_version == version)
            })
            .collect();
        match matches.as_slice() {
            [] => Ok(None),
            [(_, document)] => Ok(Some(document.clone())),
            _ => Err(failure()),
        }
    }
}

#[test]
fn document_kinds_are_explicit_and_closed() {
    for (kind, label) in [
        (DocumentKind::BehavioralContract, "behavioral-contract"),
        (DocumentKind::EvidenceRecord, "evidence-record"),
        (DocumentKind::VerificationResult, "verification-result"),
        (DocumentKind::CatalogIndex, "catalog-index"),
        (DocumentKind::ExtensionRegistry, "extension-registry"),
    ] {
        assert_eq!(serde_json::to_value(kind).unwrap(), json!(label));
        assert_eq!(
            serde_json::from_value::<DocumentKind>(json!(label)).unwrap(),
            kind
        );
    }
    for unknown in [
        "manifest",
        "pack-manifest",
        "accepted",
        "BehavioralContract",
    ] {
        assert!(serde_json::from_value::<DocumentKind>(json!(unknown)).is_err());
    }
}

#[test]
fn document_reference_retains_the_exact_version_without_resolving_it() {
    let value = json!({"id": "contract-1", "format_version": "1.2.3-rc.1+build.2"});
    let reference: DocumentRef = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(reference.format_version, "1.2.3-rc.1+build.2");
    assert_eq!(serde_json::to_value(reference).unwrap(), value);
    assert!(serde_json::from_value::<DocumentRef>(json!({"id": "contract-1"})).is_err());
}

#[test]
fn stored_bytes_and_hash_claims_are_not_rewritten_or_authenticated() {
    let document = stored("1.0.0", b" {\"id\":\"evidence-1\"}\n");
    assert_eq!(document.bytes, b" {\"id\":\"evidence-1\"}\n");
    assert_eq!(document.content_hash.value, "a".repeat(64));
    assert_eq!(document.clone(), document);
    let hash: ContentHash =
        serde_json::from_value(json!({"algorithm":"unsupported", "value":"unverified"})).unwrap();
    assert_eq!(hash.algorithm, "unsupported"); // Later validators must check algorithm and bytes.
}

#[test]
fn exact_lookup_and_missing_reference_are_distinct() {
    let documents = [
        ("alpha", stored("1.0.0", b"old")),
        ("alpha", stored("2.0.0", b"new")),
    ];
    let store: &dyn DocumentStore = &ScopedStore {
        project: "alpha",
        documents: &documents,
        unavailable: false,
    };
    assert_eq!(
        store
            .get("evidence-1", Some("1.0.0"))
            .unwrap()
            .unwrap()
            .bytes,
        b"old"
    );
    assert!(store.get("evidence-1", Some("3.0.0")).unwrap().is_none());
    assert!(store.get("absent", None).unwrap().is_none());
}

#[test]
fn unversioned_evidence_lookup_requires_a_single_match_in_the_consumer() {
    let documents = [
        ("alpha", stored("1.0.0", b"old")),
        ("alpha", stored("2.0.0", b"new")),
    ];
    let store = ScopedStore {
        project: "alpha",
        documents: &documents,
        unavailable: false,
    };
    assert_eq!(
        store.get("evidence-1", None).unwrap_err().category,
        ErrorCategory::Semantic
    );
    let single = ScopedStore {
        documents: &documents[..1],
        ..store
    };
    assert_eq!(
        single.get("evidence-1", None).unwrap().unwrap().bytes,
        b"old"
    );
}

#[test]
fn outage_is_not_normalized_into_a_missing_document() {
    let store = ScopedStore {
        project: "alpha",
        documents: &[],
        unavailable: true,
    };
    let error = store.get("evidence-1", Some("1.0.0")).unwrap_err();
    assert_eq!(error.description, "scoped reference resolution failed");
    assert_eq!(error.format_version.as_deref(), Some("1.0.0"));
}

#[test]
fn receiver_scope_is_supplied_by_the_caller_not_by_the_document() {
    let documents = [
        ("alpha", stored("1.0.0", b"alpha")),
        ("beta", stored("1.0.0", b"beta")),
    ];
    for (project, expected) in [("alpha", b"alpha".as_slice()), ("beta", b"beta".as_slice())] {
        let store: &dyn DocumentStore = &ScopedStore {
            project,
            documents: &documents,
            unavailable: false,
        };
        assert_eq!(
            store
                .get("evidence-1", Some("1.0.0"))
                .unwrap()
                .unwrap()
                .bytes,
            expected
        );
        assert!(store.get("beta/evidence-1", None).unwrap().is_none());
    }
}
