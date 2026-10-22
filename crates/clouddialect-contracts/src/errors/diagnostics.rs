use serde::{Deserialize, Serialize};

/// Canonical validation diagnostic categories, not execution or acceptance states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorCategory {
    Structural,
    Semantic,
    Version,
    UnresolvableReference,
    Unparseable,
    UnsupportedVersion,
    UnsupportedMajorVersion,
    Unsupported,
    BlockingGap,
}

/// Diagnostic context as observed by a validator; unavailable context stays absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorError {
    pub document_id: Option<String>,
    pub format_version: Option<String>,
    pub category: ErrorCategory,
    /// Dot-notation path, when the failing field is identifiable.
    pub field_path: Option<String>,
    pub description: String,
    /// Approximate parse-failure position, when known; zero is a valid offset.
    pub byte_offset: Option<usize>,
}

/// A non-blocking diagnostic retaining the original field spelling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorWarning {
    pub field_name_verbatim: String,
    pub description: String,
}
