//! Internal document-domain declarations; public facades preserve existing paths.

mod behavioral_contract;
mod evidence;
mod raw_objects;
mod references;
mod verification;

pub use behavioral_contract::{
    BehavioralContract, BehavioralFidelity, BehavioralNote, KnownBehavior, Obligation,
    OperationIdentity, ResourceDependency, SideEffect, UnresolvedAssumption,
};
pub use evidence::{EvidenceClassification, EvidenceProvenance, EvidenceRecord};
pub use raw_objects::RawObject;
pub use references::{ContentHash, DocumentKind, DocumentRef, DocumentStore, StoredDocument};
pub use verification::{
    AcceptanceRecord, ArtifactRef, BehavioralOutcome, BehavioralResult, BlockingGap,
    FailureDescription, TestOutcome, TestResult, VerificationBinding, VerificationResult,
    VersionedArtifact,
};
