//! Evidence support only. No observations are executed or authenticated here.

use super::{
    document_resolution::resolve_document, mandatory_extensions::check_mandatory_extensions,
};
use crate::{
    BehavioralContract, BehavioralOutcome, BehavioralResult, BlockingGap, DocumentKind,
    DocumentStore, EvidenceClassification, EvidenceRecord, StructuralOutcome, Validator,
};

/// Check declared obligation support against the explicit catalog and scoped
/// store. Every outcome is non-accepting and leaves verification/tests absent.
/// Invalid source or a valid empty obligation scope is unevaluated (`Deferred`).
/// Side-effect/note integrity remains the separately invoked semantic phase.
pub fn check_evidence_completeness(
    contract: &BehavioralContract,
    candidate_id: &str,
    store: &dyn DocumentStore,
    validator: &Validator,
) -> BehavioralResult {
    let mut result = BehavioralResult {
        candidate_id: candidate_id.to_owned(),
        outcome: BehavioralOutcome::Deferred,
        verification_result: None,
        test_results: Vec::new(),
        blocking_gaps: Vec::new(),
    };
    let Ok(bytes) = serde_json::to_vec(contract) else {
        return result;
    };
    if validator
        .validate_structural(DocumentKind::BehavioralContract, &bytes)
        .outcome
        == StructuralOutcome::Fail
        || contract.obligations.is_empty()
    {
        return result;
    }
    let value = serde_json::from_slice(&bytes).expect("structurally validated identical JSON");
    let mandatory = check_mandatory_extensions(&value);
    let mut gap = |obligation_id: &str, assumption_text: String| {
        result.blocking_gaps.push(BlockingGap {
            candidate_id: candidate_id.to_owned(),
            obligation_id: obligation_id.to_owned(),
            assumption_text,
        });
    };
    for obligation in &contract.obligations {
        let id = &obligation.obligation_id;
        for resource in &contract.resource_dependencies {
            for assumption in resource.unresolved_assumptions.iter().flatten() {
                gap(id, assumption.description.clone());
            }
        }
        if let Some(assumption) = &obligation.unresolved_assumption {
            gap(id, assumption.description.clone());
        }
        for error in &mandatory {
            gap(id, error.description.clone());
        }
        // No lookup can remove a root mandatory blocker. Empty scopes returned
        // Deferred above; their mandatory blockers belong to semantic validation.
        if !mandatory.is_empty() {
            continue;
        }
        for reference in obligation.evidence_refs.iter().flatten() {
            let stored = match resolve_document(
                validator,
                store,
                reference,
                None,
                DocumentKind::EvidenceRecord,
            ) {
                Ok(stored) => stored,
                Err(errors) => {
                    // The scoped resolver masks backend-private failure detail.
                    // Even an empty diagnostic list cannot imply completeness.
                    if errors.is_empty() {
                        gap(id, format!("Evidence {reference:?} could not be validated"));
                    }
                    for error in errors {
                        gap(id, format!("Evidence {reference:?}: {}", error.description));
                    }
                    continue;
                }
            };
            let Ok(evidence) = serde_json::from_slice::<EvidenceRecord>(&stored.bytes) else {
                gap(
                    id,
                    format!("Evidence {reference:?} could not decode as evidence"),
                );
                continue;
            };
            let assumptions = evidence.unresolved_assumptions.as_deref().unwrap_or(&[]);
            for assumption in assumptions {
                gap(id, assumption.description.clone());
            }
            if evidence.classification == EvidenceClassification::Unknown && assumptions.is_empty()
            {
                gap(id, format!("Evidence {reference:?} has unknown support"));
            }
        }
    }
    result.outcome = if result.blocking_gaps.is_empty() {
        BehavioralOutcome::EvidenceComplete
    } else {
        BehavioralOutcome::Blocked
    };
    result
}
