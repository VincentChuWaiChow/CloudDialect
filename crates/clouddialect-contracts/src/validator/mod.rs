//! Validation API facade. Document declarations live in the document domain.
//! Validation algorithms are implemented separately; these types grant no authority.

mod artifact_hash;
mod extensions;
mod structural;
mod version;

pub use artifact_hash::verify_artifact_hash;
pub use structural::Validator;

/// Structural checking only; later phases have distinct, non-accepting results.
pub trait Validate {
    fn validate_structural(
        &self,
        kind: crate::DocumentKind,
        document: &[u8],
    ) -> crate::StructuralResult;
}

impl Validate for Validator {
    fn validate_structural(
        &self,
        kind: crate::DocumentKind,
        document: &[u8],
    ) -> crate::StructuralResult {
        Validator::validate_structural(self, kind, document)
    }
}

pub use crate::documents::{
    AcceptanceRecord, ArtifactRef, BehavioralContract, BehavioralFidelity, BehavioralNote,
    BehavioralOutcome, BehavioralResult, BlockingGap, ContentHash, DocumentKind, DocumentRef,
    DocumentStore, EvidenceClassification, EvidenceProvenance, EvidenceRecord, FailureDescription,
    KnownBehavior, Obligation, OperationIdentity, RawObject, ResourceBehaviorDimension,
    ResourceDependency, ResourceUnresolvedAssumption, SideEffect, StoredDocument, TestOutcome,
    TestResult, UnresolvedAssumption, VerificationBinding, VerificationResult, VersionedArtifact,
};
