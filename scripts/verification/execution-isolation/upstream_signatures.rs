//! Compile-only Spec 2 handoff probe against the real Spec 1 library.
//! No runtime consumer, execution implementation, store backend or acceptance.

use clouddialect_contracts::*;
use std::collections::BTreeMap;

pub fn exact_signatures() {
    let _: fn() -> Result<CatalogSnapshot, ValidatorError> = CatalogSnapshot::embedded;
    let _: fn(Vec<u8>, BTreeMap<String, Vec<u8>>) -> Result<CatalogSnapshot, ValidatorError> =
        CatalogSnapshot::from_parts;
    let _: fn(&CatalogSnapshot) -> &ContentHash = CatalogSnapshot::digest;
    let _: for<'a> fn(&'a CatalogSnapshot, &str) -> Option<&'a [u8]> =
        CatalogSnapshot::schema_bytes;
    let _: fn(CatalogSnapshot) -> CatalogUrlLoader = CatalogUrlLoader::new;
    let _: fn(CatalogSnapshot) -> Validator = Validator::new;
    let _: fn(&Validator, DocumentKind, &[u8]) -> StructuralResult = Validator::validate_structural;
    let _: fn(&Validator, DocumentKind, &[u8], &dyn DocumentStore) -> SemanticResult =
        Validator::validate_semantic;
    let _: fn(&BehavioralContract, &str, &dyn DocumentStore, &Validator) -> BehavioralResult =
        check_evidence_completeness;
    let _: fn(CatalogSnapshot) -> ManifestValidator = ManifestValidator::new;
    let _: fn(&ManifestValidator, &[u8]) -> ManifestValidationResult =
        ManifestValidator::validate_manifest;
    let _: fn(&ManifestValidator, &[u8], &dyn DocumentStore) -> SemanticResult =
        ManifestValidator::validate_manifest_semantic;
}

// Type-check the port without supplying a mock or performing a lookup.
pub fn scoped_store_signature(
    store: &dyn DocumentStore,
    id: &str,
    version: Option<&str>,
) -> Result<Option<StoredDocument>, ValidatorError> {
    store.get(id, version)
}

pub fn exact_result_fields(value: BehavioralResult) {
    let BehavioralResult {
        candidate_id,
        outcome,
        verification_result,
        test_results,
        blocking_gaps,
    } = value;
    let _: String = candidate_id;
    let _: BehavioralOutcome = outcome;
    let _: Option<DocumentRef> = verification_result;
    let _: Vec<TestResult> = test_results;
    let _: Vec<BlockingGap> = blocking_gaps;
}

pub fn verification_fields(value: VerificationResult) {
    let VerificationResult {
        schema: _,
        id: _,
        format_version: _,
        required_extensions: _,
        candidate: _,
        pack: _,
        toolchain: _,
        configuration: _,
        binding: _,
        tests: _,
        acceptance: _,
        extensions: _,
    } = value;
}

// Exhaustiveness is deliberate: a changed upstream outcome requires review.
// These labels describe DTO values; none is an acceptance decision.
pub fn behavioral_labels(value: BehavioralOutcome) -> &'static str {
    match value {
        BehavioralOutcome::Deferred => "deferred (non-accepting)",
        BehavioralOutcome::EvidenceComplete => "evidence-complete (non-accepting)",
        BehavioralOutcome::Blocked => "blocked (non-accepting)",
    }
}

pub fn declared_capability(value: CapabilityRequirement) {
    let CapabilityRequirement {
        id: _,
        operation_id: _,
        action: _,
        resource: _,
        purpose: _,
    } = value;
}
