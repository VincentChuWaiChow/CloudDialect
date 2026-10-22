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
