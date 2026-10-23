//! Resolve caller-scoped bytes before any consumer uses their declarations.

use serde_json::Value;

use super::{artifact_hash::verify_content_hash, mandatory_extensions::check_mandatory_extensions};
use crate::{
    DocumentKind, DocumentStore, ErrorCategory, StoredDocument, StructuralOutcome, Validator,
    ValidatorError,
};

/// Identity, byte integrity and active-schema checks do not authenticate scope
/// or grant acceptance. The trusted caller supplies the already scoped store.
pub(crate) fn resolve_document(
    validator: &Validator,
    store: &dyn DocumentStore,
    id: &str,
    format_version: Option<&str>,
    kind: DocumentKind,
) -> Result<StoredDocument, Vec<ValidatorError>> {
    let failure = |path: Option<&str>, description: &str| {
        vec![ValidatorError {
            document_id: Some(id.to_owned()),
            format_version: format_version.map(str::to_owned),
            category: ErrorCategory::UnresolvableReference,
            field_path: path.map(str::to_owned),
            description: description.to_owned(),
            byte_offset: None,
        }]
    };
    if id.is_empty() {
        return Err(failure(
            Some("id"),
            "A nonempty document identity is required",
        ));
    }
    if format_version == Some("")
        || (format_version.is_none() && kind != DocumentKind::EvidenceRecord)
    {
        return Err(failure(
            Some("format_version"),
            "An exact version is required except for globally unique evidence references",
        ));
    }
    let stored = match store.get(id, format_version) {
        Ok(Some(document)) => document,
        Ok(None) => {
            return Err(failure(
                None,
                "Referenced document was not found in the supplied scope",
            ));
        }
        Err(_) => {
            return Err(failure(
                None,
                "Referenced document could not be resolved in the supplied scope",
            ));
        }
    };
    // An evidence lookup may omit a version, but returned metadata still names
    // the immutable record whose resolution failed. Keep that diagnostic context.
    let failure = |path, description| {
        let mut errors = failure(path, description);
        for error in &mut errors {
            error
                .format_version
                .get_or_insert_with(|| stored.reference.format_version.clone());
        }
        errors
    };
    if stored.reference.id != id {
        return Err(failure(
            Some("id"),
            "Returned document identity does not match the reference",
        ));
    }
    if let Some(expected) = format_version
        && stored.reference.format_version != expected
    {
        return Err(failure(
            Some("format_version"),
            "Returned document version does not exactly match the reference",
        ));
    }
    if !verify_content_hash(&stored.content_hash, &stored.bytes) {
        return Err(failure(
            Some("content_hash"),
            "Referenced bytes do not match a supported canonical content hash",
        ));
    }
    let structural = validator.validate_structural(kind, &stored.bytes);
    if structural.outcome == StructuralOutcome::Fail {
        return Err(structural
            .errors
            .into_iter()
            .map(|mut error| {
                error.document_id.get_or_insert_with(|| id.to_owned());
                error
                    .format_version
                    .get_or_insert_with(|| stored.reference.format_version.clone());
                error
            })
            .collect());
    }
    let document: Value = serde_json::from_slice(&stored.bytes)
        .map_err(|_| failure(None, "Referenced document is not parseable JSON"))?;
    if document.get("id").and_then(Value::as_str) != Some(id) {
        return Err(failure(
            Some("id"),
            "Document body identity does not match the reference",
        ));
    }
    if document.get("format_version").and_then(Value::as_str)
        != Some(stored.reference.format_version.as_str())
    {
        return Err(failure(
            Some("format_version"),
            "Document body version does not exactly match returned metadata",
        ));
    }
    let blockers = check_mandatory_extensions(&document);
    if !blockers.is_empty() {
        return Err(blockers);
    }
    Ok(stored)
}
