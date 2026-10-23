//! Contracts and evidence foundation for CloudDialect.
//!
//! Catalog inventory checks and phase/document declarations are available.
//! Structural validation is available; later phases remain incomplete.
//! No result authorizes acceptance.

mod documents;

pub mod errors;
pub mod manifest;
pub mod schema;
pub mod validator;

pub use schema::{CatalogSnapshot, CatalogUrlLoader};
pub use validator::{Validate, Validator, check_fidelity_mapping, verify_artifact_hash};

pub use manifest::{
    CapabilityAction, CapabilityDeclaration, CapabilityRequirement, CompatibilitySection,
    ContractRef, ManifestFidelity, ManifestOutcome, ManifestValidationResult, ManifestValidator,
    OperationEntry, PackManifest, PackSection, SchemaRefsSection, SourceSection, TargetSection,
    ToolchainEntry, ValidateManifest,
};

pub use errors::{
    BehavioralOutcome, BehavioralResult, BlockingGap, ErrorCategory, FailureDescription,
    SemanticOutcome, SemanticResult, StructuralOutcome, StructuralResult, TestOutcome, TestResult,
    ValidatorError, ValidatorWarning,
};
pub use validator::{
    AcceptanceRecord, ArtifactRef, VerificationBinding, VerificationResult, VersionedArtifact,
};
pub use validator::{
    BehavioralContract, BehavioralFidelity, BehavioralNote, EvidenceClassification,
    EvidenceProvenance, EvidenceRecord, KnownBehavior, Obligation, OperationIdentity,
    ResourceBehaviorDimension, ResourceDependency, ResourceUnresolvedAssumption, SideEffect,
    UnresolvedAssumption,
};
pub use validator::{
    ContentHash, DocumentKind, DocumentRef, DocumentStore, RawObject, StoredDocument,
};
