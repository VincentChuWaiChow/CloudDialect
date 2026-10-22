//! Contracts and evidence foundation for CloudDialect.
//!
//! Catalog inventory checks and phase/document declarations are available.
//! Full validation is not yet implemented; no result authorizes acceptance.

mod documents;

pub mod errors;
pub mod manifest;
pub mod schema;
pub mod validator;

pub use schema::{CatalogSnapshot, CatalogUrlLoader};

pub use manifest::{
    CapabilityAction, CapabilityDeclaration, CapabilityRequirement, CompatibilitySection,
    ContractRef, ManifestFidelity, ManifestOutcome, ManifestValidationResult, OperationEntry,
    PackManifest, PackSection, SchemaRefsSection, SourceSection, TargetSection, ToolchainEntry,
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
    ResourceDependency, SideEffect, UnresolvedAssumption,
};
pub use validator::{
    ContentHash, DocumentKind, DocumentRef, DocumentStore, RawObject, StoredDocument,
};
