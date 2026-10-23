//! Bundled schema catalog loading and resolution.

mod catalog;
mod embedded;
mod loader;
mod snapshot;

pub use catalog::{CatalogEntry, CatalogIndex, ResolverResource};
pub use embedded::get_embedded_schema;
pub use loader::CatalogUrlLoader;
pub use snapshot::CatalogSnapshot;
