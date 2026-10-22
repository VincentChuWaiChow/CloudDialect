use serde_json::Value;

use crate::{ErrorCategory, ValidatorError};

/// Called only after structural success by the pending semantic consumer.
/// The current MVP implements no trusted extension semantics. Catalog/registry
/// membership and input payloads therefore cannot remove these blockers.
pub(super) fn check_mandatory_extensions(document: &Value) -> Vec<ValidatorError> {
    let Some(declaration) = document.get("required_extensions") else {
        return Vec::new();
    };
    let error = |category, path: String, description: String| ValidatorError {
        document_id: document
            .get("id")
            .and_then(Value::as_str)
            .map(str::to_owned),
        format_version: document
            .get("format_version")
            .and_then(Value::as_str)
            .map(str::to_owned),
        category,
        field_path: Some(path),
        description,
        byte_offset: None,
    };
    let Some(prefixes) = declaration.as_array() else {
        return vec![error(
            ErrorCategory::Structural,
            "required_extensions".to_owned(),
            "Mandatory extension declaration must be an array".to_owned(),
        )];
    };
    prefixes
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let path = format!("required_extensions[{index}]");
            match value.as_str() {
                Some(prefix) => error(
                    ErrorCategory::BlockingGap,
                    path,
                    format!("Unrecognized mandatory extension namespace {prefix:?}; acceptance is blocked"),
                ),
                None => error(
                    ErrorCategory::Structural,
                    path,
                    "Mandatory extension namespace must be a string".to_owned(),
                ),
            }
        })
        .collect()
}
