//! Closed snapshot-backed adapter for the pinned Boon compiler.

use std::{error::Error, io};

use boon::UrlLoader;
use serde_json::Value;

use super::CatalogSnapshot;

/// Owns frozen bytes; never delegates to a network or live-filesystem loader.
/// Callers must install it before compilation and separately configure the
/// compiler's format assertions. Inventory validity does not attest provenance.
pub struct CatalogUrlLoader {
    snapshot: CatalogSnapshot,
}

impl CatalogUrlLoader {
    pub fn new(snapshot: CatalogSnapshot) -> Self {
        Self { snapshot }
    }
}

impl UrlLoader for CatalogUrlLoader {
    fn load(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let bytes = self.snapshot.schema_bytes(url).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Unresolvable catalog URI: {url}"),
            )
        })?;
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_indexed_schema_resolves_from_nonempty_frozen_bytes() {
        let snapshot = CatalogSnapshot::embedded().unwrap();
        let uri = "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json";
        assert!(!snapshot.schema_bytes(uri).unwrap().is_empty());
        assert!(
            CatalogUrlLoader::new(snapshot)
                .load(uri)
                .unwrap()
                .is_object()
        );
    }

    #[test]
    fn an_unlisted_uri_has_no_fallback_and_remains_in_the_error() {
        let uri = "https://schemas.clouddialect.io/unlisted/1.0.0/schema.json";
        let error = CatalogUrlLoader::new(CatalogSnapshot::embedded().unwrap())
            .load(uri)
            .unwrap_err();
        assert!(error.to_string().contains(uri));
    }
}
