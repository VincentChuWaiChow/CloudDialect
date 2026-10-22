//! Transformation pack declarations and local-validation result types.
//! Parsing a declaration does not grant authority or establish reference integrity.

mod types;

pub use types::{
    CapabilityAction, CapabilityDeclaration, CapabilityRequirement, CompatibilitySection,
    ContractRef, ManifestFidelity, ManifestOutcome, ManifestValidationResult, OperationEntry,
    PackManifest, PackSection, SchemaRefsSection, SourceSection, TargetSection, ToolchainEntry,
};
