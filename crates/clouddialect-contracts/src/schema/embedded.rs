//! Compiled catalog material only; no runtime directory or environment selection.

use std::collections::BTreeMap;

use include_dir::{Dir, include_dir};

use super::{CatalogIndex, CatalogSnapshot};
use crate::ValidatorError;

static CATALOG: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../../catalog");
const INDEX: &[u8] = include_bytes!("../../../../catalog/index.json");

fn material() -> BTreeMap<String, Vec<u8>> {
    fn collect(dir: &Dir<'_>, parts: &mut BTreeMap<String, Vec<u8>>) {
        for file in dir.files() {
            // include_dir constructs these paths from UTF-8 string literals.
            let path = file.path().to_str().expect("embedded paths are UTF-8");
            if path != "index.json" {
                parts.insert(path.to_owned(), file.contents().to_vec());
            }
        }
        for child in dir.dirs() {
            collect(child, parts);
        }
    }
    let mut parts = BTreeMap::new();
    collect(&CATALOG, &mut parts);
    parts
}

impl CatalogSnapshot {
    /// Freeze the entire compiled catalog, including unindexed catalog metadata.
    /// This supplies content identity, not signer authority or acceptance.
    pub fn embedded() -> Result<Self, ValidatorError> {
        Self::from_parts(INDEX.to_vec(), material())
    }
}

/// Return static bytes for an exact indexed URI in the compiled catalog.
/// Paths, URI prefixes and ambient overrides are never alternative selectors.
pub fn get_embedded_schema(uri: &str) -> Option<&'static [u8]> {
    let index = CatalogIndex::from_parts(INDEX, &material()).ok()?;
    let path = index
        .schemas
        .iter()
        .map(|entry| (&entry.schema_id, &entry.catalog_path))
        .chain(
            index
                .resolver_resources
                .iter()
                .map(|entry| (&entry.schema_id, &entry.catalog_path)),
        )
        .find_map(|(id, path)| (id == uri).then_some(path))?;
    CATALOG.get_file(path).map(|file| file.contents())
}
