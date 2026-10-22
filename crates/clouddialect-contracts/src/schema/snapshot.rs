//! Immutable catalog bytes and content identity, not a provenance attestation.

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use super::CatalogIndex;
use crate::{ContentHash, ErrorCategory, ValidatorError};

/// Caller policy must separately authenticate and admit this content identity.
/// The snapshot provides no mutable access to its bytes or digest.
///
/// ```compile_fail
/// fn rewrite(snapshot: &clouddialect_contracts::CatalogSnapshot) {
///     snapshot.digest().value.push('x');
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CatalogSnapshot {
    index: CatalogIndex,
    material: BTreeMap<String, Vec<u8>>,
    digest: ContentHash,
}

impl CatalogSnapshot {
    /// Freeze explicitly supplied bytes after inventory checks. No files,
    /// environment variables or network resources are consulted.
    pub fn from_parts(
        index: Vec<u8>,
        mut schemas: BTreeMap<String, Vec<u8>>,
    ) -> Result<Self, ValidatorError> {
        let parsed = CatalogIndex::from_parts(&index, &schemas)?;
        // Inventory checks reject an existing index.json material alias.
        schemas.insert("index.json".to_owned(), index);
        let mut hasher = Sha256::new();
        hasher.update(b"CloudDialectCatalogSnapshot/v1\0");
        // String ordering is lexicographic UTF-8 byte ordering; no normalization.
        for (path, bytes) in &schemas {
            for part in [path.as_bytes(), bytes.as_slice()] {
                let length = u64::try_from(part.len()).map_err(|_| ValidatorError {
                    document_id: None,
                    format_version: None,
                    category: ErrorCategory::Unsupported,
                    field_path: None,
                    description: "Catalog identity entry length exceeds unsigned 64-bit encoding"
                        .to_owned(),
                    byte_offset: None,
                })?;
                hasher.update(length.to_be_bytes());
                hasher.update(part);
            }
        }
        let value = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        Ok(Self {
            index: parsed,
            material: schemas,
            digest: ContentHash {
                algorithm: "sha-256".to_owned(),
                value,
            },
        })
    }

    /// Identity of all supplied parts, including exact index.json bytes.
    /// Equality is not evidence of owner approval, signature or safe schemas.
    pub fn digest(&self) -> &ContentHash {
        &self.digest
    }

    /// Return immutable bytes only for an exact indexed application/support URI.
    /// Other digest-bound material is never implicitly resolvable.
    pub fn schema_bytes(&self, uri: &str) -> Option<&[u8]> {
        let path = self
            .index
            .schemas
            .iter()
            .map(|entry| (&entry.schema_id, &entry.catalog_path))
            .chain(
                self.index
                    .resolver_resources
                    .iter()
                    .map(|entry| (&entry.schema_id, &entry.catalog_path)),
            )
            .find_map(|(id, path)| (id == uri).then_some(path))?;
        self.material.get(path).map(Vec::as_slice)
    }
}
