//! Validation diagnostics and non-interchangeable phase results.

mod diagnostics;
mod phase;

pub use crate::manifest::{ManifestOutcome, ManifestValidationResult};

pub use crate::validator::{
    BehavioralOutcome, BehavioralResult, BlockingGap, FailureDescription, TestOutcome, TestResult,
};
pub use diagnostics::{ErrorCategory, ValidatorError, ValidatorWarning};
pub use phase::{SemanticOutcome, SemanticResult, StructuralOutcome, StructuralResult};
