//! Validation API facade. Document declarations live in the document domain.
//! Validation algorithms are implemented separately; these types grant no authority.

pub use crate::documents::{
    AcceptanceRecord, ArtifactRef, BehavioralContract, BehavioralFidelity, BehavioralNote,
    BehavioralOutcome, BehavioralResult, BlockingGap, ContentHash, DocumentKind, DocumentRef,
    DocumentStore, EvidenceClassification, EvidenceProvenance, EvidenceRecord, FailureDescription,
    KnownBehavior, Obligation, OperationIdentity, RawObject, ResourceDependency, SideEffect,
    StoredDocument, TestOutcome, TestResult, UnresolvedAssumption, VerificationBinding,
    VerificationResult, VersionedArtifact,
};
