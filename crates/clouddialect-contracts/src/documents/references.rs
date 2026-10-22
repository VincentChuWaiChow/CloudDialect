use serde::{Deserialize, Serialize};

use crate::ValidatorError;

/// JSON document purpose selected by the trusted caller, not inferred from input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentKind {
    BehavioralContract,
    EvidenceRecord,
    VerificationResult,
    CatalogIndex,
    ExtensionRegistry,
}

/// An exact document identity; parsing this value does not resolve or validate it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentRef {
    pub id: String,
    pub format_version: String,
}

/// An unverified digest claim. Validators must check algorithm, length and bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentHash {
    pub algorithm: String,
    pub value: String,
}

/// Returned bytes and claimed identity; neither the store nor this type authenticates them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredDocument {
    pub reference: DocumentRef,
    pub content_hash: ContentHash,
    pub bytes: Vec<u8>,
}

/// Read port supplied by trusted core code, already bound to its tenant/project.
///
/// Contract references require `Some(exact_version)`. `None` is reserved for
/// globally unique evidence IDs and must resolve to exactly one version or error.
/// `Ok(None)` means missing; `Err` means a safe resolution failure, not absence.
/// Implementors must not search other projects or fall back to ambient catalogs.
/// Callers must still validate returned identity, version, type, bytes and hash.
/// These obligations are not enforced or authenticated by implementing the trait.
pub trait DocumentStore {
    fn get(
        &self,
        id: &str,
        format_version: Option<&str>,
    ) -> Result<Option<StoredDocument>, ValidatorError>;
}
