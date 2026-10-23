//! Positive import/type-identity checks keep negative doctests honest after moves.

use clouddialect_contracts as api;

fn same_type<T>(_: Option<T>) {}

macro_rules! aliases {
    ($module:ident: $($name:ident),+ $(,)?) => {
        $(same_type::<api::$name>(None::<api::$module::$name>);)+
    };
}

#[test]
fn existing_facades_reexport_the_same_public_definitions() {
    aliases!(validator:
        ContentHash, DocumentKind, DocumentRef, StoredDocument,
        RawObject, BehavioralContract, BehavioralFidelity, BehavioralNote,
        EvidenceClassification, EvidenceProvenance, EvidenceRecord,
        KnownBehavior, Obligation, OperationIdentity, ResourceDependency,
        SideEffect, UnresolvedAssumption, AcceptanceRecord, ArtifactRef,
        BehavioralOutcome, BehavioralResult, BlockingGap, FailureDescription,
        TestOutcome, TestResult, VerificationBinding, VerificationResult,
        VersionedArtifact,
    );
    aliases!(errors:
        BehavioralOutcome, BehavioralResult, BlockingGap, ErrorCategory,
        FailureDescription, ManifestOutcome, ManifestValidationResult,
        SemanticOutcome, SemanticResult, StructuralOutcome, StructuralResult,
        TestOutcome, TestResult, ValidatorError, ValidatorWarning,
    );
    aliases!(manifest:
        CapabilityAction, CapabilityDeclaration, CapabilityRequirement,
        CompatibilitySection, ContractRef, ManifestFidelity, ManifestOutcome,
        ManifestValidationResult, OperationEntry, PackManifest, PackSection,
        SchemaRefsSection, SourceSection, TargetSection, ToolchainEntry,
    );
    // These remain available in the existing schema namespace.
    same_type::<api::schema::CatalogIndex>(None);
    same_type::<api::schema::CatalogEntry>(None);
    same_type::<api::schema::ResolverResource>(None);
    let store: Option<&dyn api::DocumentStore> = None::<&dyn api::validator::DocumentStore>;
    assert!(store.is_none());
}
