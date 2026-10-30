//! JSON reference consistency through the caller's active catalog and scope.

use serde_json::Value;

use super::{
    document_resolution::resolve_document, mandatory_extensions::check_mandatory_extensions,
    verification_semantics::check_verification,
};
use crate::{
    BehavioralContract, DocumentKind, DocumentStore, ErrorCategory, SemanticOutcome,
    SemanticResult, StructuralOutcome, Validator, ValidatorError, VerificationResult,
};

impl Validator {
    /// Check declared core references after structural validation of the same
    /// bytes. Success does not establish evidence completeness, authenticate
    /// document claims or authorize execution/acceptance. Catalog admission is
    /// separate from instance-reference validation; no schema material is read
    /// from the document store.
    pub fn validate_semantic(
        &self,
        kind: DocumentKind,
        document: &[u8],
        store: &dyn DocumentStore,
    ) -> SemanticResult {
        let structural = self.validate_structural(kind, document);
        let document_id = structural.document_id;
        let format_version = structural.format_version;
        let mut errors = structural.errors;
        if structural.outcome != StructuralOutcome::Fail {
            let value: Value = serde_json::from_slice(document)
                .expect("structural validation parsed identical bytes");
            errors.extend(check_mandatory_extensions(&value));
            if errors.is_empty() {
                match kind {
                    DocumentKind::BehavioralContract => {
                        match serde_json::from_slice::<BehavioralContract>(document) {
                            Ok(contract) => errors.extend(check_contract(self, &contract, store)),
                            Err(_) => errors.push(decode_error(&document_id, &format_version)),
                        }
                    }
                    DocumentKind::VerificationResult => {
                        match serde_json::from_slice::<VerificationResult>(document) {
                            Ok(record) => errors.extend(check_verification(&record)),
                            Err(_) => errors.push(decode_error(&document_id, &format_version)),
                        }
                    }
                    // These kinds declare no core document-instance references.
                    // Their local rules are structural; mandatory declarations
                    // still block independently of payload/registry membership.
                    DocumentKind::EvidenceRecord
                    | DocumentKind::CatalogIndex
                    | DocumentKind::ExtensionRegistry => {}
                }
            }
        }
        SemanticResult {
            document_id,
            outcome: if errors.is_empty() {
                SemanticOutcome::Pass
            } else {
                SemanticOutcome::Fail
            },
            errors,
        }
    }
}

fn decode_error(id: &Option<String>, version: &Option<String>) -> ValidatorError {
    ValidatorError {
        document_id: id.clone(),
        format_version: version.clone(),
        category: ErrorCategory::Semantic,
        field_path: Some("$".to_owned()),
        description: "Document could not decode as the declared core document type".to_owned(),
        byte_offset: None,
    }
}

fn check_contract(
    validator: &Validator,
    contract: &BehavioralContract,
    store: &dyn DocumentStore,
) -> Vec<ValidatorError> {
    let mut errors = Vec::new();
    let mut check = |id: &str, path: String| {
        if let Err(diagnostics) =
            resolve_document(validator, store, id, None, DocumentKind::EvidenceRecord)
        {
            errors.extend(diagnostics.into_iter().map(|mut error| {
                // Located failures describe the referenced document's original
                // core field, not a nonexistent child of this reference string.
                if error.field_path.is_none() {
                    error.document_id = Some(contract.id.clone());
                    error.format_version = Some(contract.format_version.clone());
                    error.field_path = Some(path.clone());
                    error.description =
                        format!("{} (referenced evidence {id:?})", error.description);
                }
                error
            }));
        }
    };
    for (index, obligation) in contract.obligations.iter().enumerate() {
        for (reference_index, id) in obligation.evidence_refs.iter().flatten().enumerate() {
            check(
                id,
                format!("obligations[{index}].evidence_refs[{reference_index}]"),
            );
        }
    }
    for (index, effect) in contract.side_effects.iter().enumerate() {
        if let Some(id) = &effect.evidence_ref {
            check(id, format!("side_effects[{index}].evidence_ref"));
        }
    }
    for (index, note) in contract.behavioral_notes.iter().flatten().enumerate() {
        check(
            &note.evidence_ref,
            format!("behavioral_notes[{index}].evidence_ref"),
        );
    }
    errors
}
