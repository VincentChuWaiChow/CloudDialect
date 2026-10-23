use serde::{Deserialize, Serialize};

use super::{ValidatorError, ValidatorWarning};

/// Structural disposition only; a pass does not establish semantic correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StructuralOutcome {
    Pass,
    Warn,
    Fail,
}

/// Reference-integrity disposition only; a pass does not authorize acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SemanticOutcome {
    Pass,
    Fail,
}

/// Structural diagnostics, not semantic or behavioral verification evidence.
///
/// A structural result cannot be promoted into a semantic result:
///
/// ```compile_fail
/// use clouddialect_contracts::{SemanticResult, StructuralResult};
/// fn promote(result: StructuralResult) -> SemanticResult {
///     result.into()
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralResult {
    pub document_id: Option<String>,
    pub format_version: Option<String>,
    pub outcome: StructuralOutcome,
    pub errors: Vec<ValidatorError>,
    pub warnings: Vec<ValidatorWarning>,
}

/// Semantic diagnostics, not behavioral verification evidence or authorization.
///
/// A semantic result cannot substitute for structural validation:
///
/// ```compile_fail
/// use clouddialect_contracts::{SemanticResult, StructuralResult};
/// fn substitute(result: SemanticResult) -> StructuralResult {
///     result.into()
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticResult {
    pub document_id: Option<String>,
    pub outcome: SemanticOutcome,
    pub errors: Vec<ValidatorError>,
}
