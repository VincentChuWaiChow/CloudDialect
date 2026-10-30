//! Validation API facade. Document declarations live in the document domain.
//! Validation algorithms are implemented separately; these types grant no authority.

mod artifact_hash;
mod behavioral;
pub(crate) mod document_resolution;
mod extensions;
mod fidelity;
mod mandatory_extensions;
mod semantic;
mod structural;
mod verification_semantics;
mod version;

pub use artifact_hash::verify_artifact_hash;
pub use behavioral::check_evidence_completeness;
pub use fidelity::check_fidelity_mapping;
pub use structural::Validator;

/// Independently invocable structural and semantic, non-accepting phases.
pub trait Validate {
    fn validate_structural(
        &self,
        kind: crate::DocumentKind,
        document: &[u8],
    ) -> crate::StructuralResult;

    fn validate_semantic(
        &self,
        kind: crate::DocumentKind,
        document: &[u8],
        store: &dyn crate::DocumentStore,
    ) -> crate::SemanticResult;
}

impl Validate for Validator {
    fn validate_structural(
        &self,
        kind: crate::DocumentKind,
        document: &[u8],
    ) -> crate::StructuralResult {
        Validator::validate_structural(self, kind, document)
    }

    fn validate_semantic(
        &self,
        kind: crate::DocumentKind,
        document: &[u8],
        store: &dyn crate::DocumentStore,
    ) -> crate::SemanticResult {
        Validator::validate_semantic(self, kind, document, store)
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
