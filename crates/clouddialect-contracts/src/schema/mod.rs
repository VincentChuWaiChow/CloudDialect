//! Bundled schema catalog loading and resolution.

mod catalog;
mod snapshot;

pub use catalog::{CatalogEntry, CatalogIndex, ResolverResource};
pub use snapshot::CatalogSnapshot;
