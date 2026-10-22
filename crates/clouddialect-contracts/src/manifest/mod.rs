//! Transformation pack declarations and local-validation result types.
//! Parsing a declaration does not grant authority or establish reference integrity.

mod types;
mod validator;

pub use validator::ManifestValidator;

pub use types::{
    CapabilityAction, CapabilityDeclaration, CapabilityRequirement, CompatibilitySection,
    ContractRef, ManifestFidelity, ManifestOutcome, ManifestValidationResult, OperationEntry,
    PackManifest, PackSection, SchemaRefsSection, SourceSection, TargetSection, ToolchainEntry,
};

/// Local declaration validation only; external references and fidelity remain
/// separate semantic checks and no result grants execution authority.
pub trait ValidateManifest {
    fn validate_manifest(&self, toml_bytes: &[u8]) -> ManifestValidationResult;
    fn validate_manifest_semantic(
        &self,
        toml_bytes: &[u8],
        store: &dyn crate::DocumentStore,
    ) -> crate::SemanticResult;
}

impl ValidateManifest for ManifestValidator {
    fn validate_manifest(&self, toml_bytes: &[u8]) -> ManifestValidationResult {
        ManifestValidator::validate_manifest(self, toml_bytes)
    }

    fn validate_manifest_semantic(
        &self,
        toml_bytes: &[u8],
        store: &dyn crate::DocumentStore,
    ) -> crate::SemanticResult {
        ManifestValidator::validate_manifest_semantic(self, toml_bytes, store)
    }
}
