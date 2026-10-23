use std::sync::OnceLock;

use regex::Regex;
use serde_json::Value;

use super::structural::property_path;
use crate::{ErrorCategory, ValidatorError, ValidatorWarning};

/// Root extension names are checked against the original, unprojected input.
/// Prefix ownership and payload claims never establish implemented semantics.
pub(super) fn scan_extension_fields(
    document: &Value,
) -> (Vec<ValidatorWarning>, Vec<ValidatorError>) {
    static PREFIX: OnceLock<Regex> = OnceLock::new();
    static RESERVED: OnceLock<Regex> = OnceLock::new();
    let prefix_pattern =
        PREFIX.get_or_init(|| Regex::new(r"\A[A-Za-z][A-Za-z0-9-]+\z").expect("static regex"));
    let reserved_pattern = RESERVED.get_or_init(|| {
        Regex::new(r"(?i-u)\A(core|core-[a-z0-9-]*|cdc|cdc-[a-z0-9-]*)\z").expect("static regex")
    });
    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    for key in document.as_object().expect("validated object root").keys() {
        let Some((prefix, _)) = key.split_once(':') else {
            continue;
        };
        if !prefix_pattern.is_match(prefix) {
            errors.push(ValidatorError {
                document_id: document
                    .get("id")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                format_version: document
                    .get("format_version")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                category: ErrorCategory::Structural,
                field_path: Some(property_path("", key)),
                description: "Extension prefix must match [A-Za-z][A-Za-z0-9-]+".to_owned(),
                byte_offset: None,
            });
        } else if reserved_pattern.is_match(prefix) {
            warnings.push(ValidatorWarning {
                field_name_verbatim: key.clone(),
                description: format!("prefix '{prefix}' is reserved for core use"),
            });
        }
    }
    (warnings, errors)
}
