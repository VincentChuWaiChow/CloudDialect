use super::references::resolve_document;
use crate::{
    BehavioralContract, BehavioralFidelity, DocumentKind, DocumentStore, ErrorCategory,
    ManifestFidelity, PackManifest, Validator, ValidatorError,
};

/// Checks cross-document fidelity through the explicit active validator/store.
/// Callers must separately enforce local manifest rules; no result grants
/// execution authority, authenticates observations or accepts a transformation.
pub fn check_fidelity_mapping(
    manifest: &PackManifest,
    store: &dyn DocumentStore,
    validator: &Validator,
) -> Vec<ValidatorError> {
    let mut errors = Vec::new();
    for operation in &manifest.operations {
        if operation.behavioral_fidelity == ManifestFidelity::Unsupported {
            continue;
        }
        let key = serde_json::to_string(&operation.id).expect("JSON string encoding");
        let reference_path = super::structural::property_path("contracts", &operation.id);
        let Some(reference) = manifest.contracts.get(&operation.id) else {
            errors.push(error(
                manifest,
                ErrorCategory::UnresolvableReference,
                format!("{reference_path}.id"),
                "Operation has no exact contract binding".to_owned(),
            ));
            continue;
        };
        let record = match resolve_document(
            validator,
            store,
            &reference.id,
            Some(&reference.format_version),
            DocumentKind::BehavioralContract,
        ) {
            Ok(record) => record,
            Err(diagnostics) => {
                errors.extend(diagnostics.into_iter().map(|mut diagnostic| {
                    diagnostic.document_id = Some(manifest.pack.id.clone());
                    diagnostic.format_version = None;
                    diagnostic.field_path = Some(match diagnostic.field_path {
                        Some(path) => format!("{reference_path}.{path}"),
                        None => format!("{reference_path}.id"),
                    });
                    diagnostic
                }));
                continue;
            }
        };
        let contract: BehavioralContract = match serde_json::from_slice(&record.bytes) {
            Ok(contract) => contract,
            Err(_) => {
                errors.push(error(
                    manifest,
                    ErrorCategory::UnresolvableReference,
                    format!("{reference_path}.id"),
                    "Validated contract could not decode".to_owned(),
                ));
                continue;
            }
        };
        for obligation in &contract.obligations {
            let permitted = match operation.behavioral_fidelity {
                ManifestFidelity::Full => {
                    obligation.behavioral_fidelity == BehavioralFidelity::EquivalentWithinContract
                }
                ManifestFidelity::Partial => matches!(
                    obligation.behavioral_fidelity,
                    BehavioralFidelity::EquivalentWithinContract
                        | BehavioralFidelity::AdaptationRequired
                ),
                ManifestFidelity::Unsupported => unreachable!("unsupported operations skipped"),
            };
            if !permitted {
                let mode = if operation.behavioral_fidelity == ManifestFidelity::Full {
                    "full"
                } else {
                    "partial"
                };
                let fidelity = match obligation.behavioral_fidelity {
                    BehavioralFidelity::EquivalentWithinContract => "equivalent-within-contract",
                    BehavioralFidelity::AdaptationRequired => "adaptation-required",
                    BehavioralFidelity::ArchitecturalChangeRequired => {
                        "architectural-change-required"
                    }
                    BehavioralFidelity::Unsupported => "unsupported",
                    BehavioralFidelity::Unknown => "unknown",
                };
                errors.push(error(manifest, ErrorCategory::Semantic,
                    format!("operations[{key}].behavioral_fidelity"),
                    format!("manifest fidelity '{mode}' but contract obligation {:?} has fidelity '{fidelity}'", obligation.obligation_id)));
            }
        }
    }
    errors
}

fn error(
    manifest: &PackManifest,
    category: ErrorCategory,
    path: String,
    description: String,
) -> ValidatorError {
    ValidatorError {
        document_id: Some(manifest.pack.id.clone()),
        format_version: None,
        category,
        field_path: Some(path),
        description,
        byte_offset: None,
    }
}
