//! Unvalidated verification records and non-accepting result projections.
//! Document claims, including approval/origin strings, do not authenticate authority.

use super::{
    ContentHash, DocumentRef, RawObject,
    raw_objects::{RawFields, raw_object_model},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestOutcome {
    Pass,
    Fail,
    Skip,
    Error,
}

/// Structured diagnostic data; the schema separately enforces Unicode length bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailureDescription {
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestResult {
    pub test_id: String,
    pub obligation_ids: Vec<String>,
    pub outcome: TestOutcome,
    pub origin_identity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<FailureDescription>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRef {
    pub identity: String,
    pub content_hash: ContentHash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionedArtifact {
    pub id: String,
    pub version: String,
    pub content_hash: ContentHash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationBinding {
    pub tenant_id: String,
    pub project_id: String,
    pub run_id: String,
    pub attempt_id: String,
    pub input: ArtifactRef,
    pub dependencies: ArtifactRef,
    pub suite: ArtifactRef,
    pub oracle: ArtifactRef,
    pub policy: ArtifactRef,
    pub obligation_ids: Vec<String>,
}

/// An approval claim requiring independent identity, time, policy and binding checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceRecord {
    pub approver_identities: Vec<String>,
    pub timestamp: String,
    pub verification_result: DocumentRef,
    pub approval_evidence: ArtifactRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct VerificationResult {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub id: String,
    pub format_version: String,
    pub candidate: ArtifactRef,
    pub pack: VersionedArtifact,
    pub toolchain: VersionedArtifact,
    pub configuration: ArtifactRef,
    pub binding: VerificationBinding,
    pub tests: Vec<TestResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<AcceptanceRecord>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

/// Every variant is non-accepting, including EvidenceComplete with projected tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BehavioralOutcome {
    Deferred,
    EvidenceComplete,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockingGap {
    pub candidate_id: String,
    pub obligation_id: String,
    pub assumption_text: String,
}

/// Non-accepting data. Completeness leaves the reference absent and tests empty;
/// a later verifier must validate any projection against its exact result record.
/// No phase can be promoted or substituted through From/Into:
///
/// ```compile_fail
/// use clouddialect_contracts::{BehavioralResult, StructuralResult};
/// fn promote(result: StructuralResult) -> BehavioralResult { result.into() }
/// ```
/// ```compile_fail
/// use clouddialect_contracts::{BehavioralResult, SemanticResult};
/// fn promote(result: SemanticResult) -> BehavioralResult { result.into() }
/// ```
/// ```compile_fail
/// use clouddialect_contracts::{BehavioralResult, StructuralResult};
/// fn substitute(result: BehavioralResult) -> StructuralResult { result.into() }
/// ```
/// ```compile_fail
/// use clouddialect_contracts::{BehavioralResult, SemanticResult};
/// fn substitute(result: BehavioralResult) -> SemanticResult { result.into() }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehavioralResult {
    pub candidate_id: String,
    pub outcome: BehavioralOutcome,
    pub verification_result: Option<DocumentRef>,
    pub test_results: Vec<TestResult>,
    pub blocking_gaps: Vec<BlockingGap>,
}

raw_object_model!(VerificationResult, [id, format_version, candidate, pack, toolchain, configuration, binding, tests], [schema => "$schema", acceptance => "acceptance"]);
