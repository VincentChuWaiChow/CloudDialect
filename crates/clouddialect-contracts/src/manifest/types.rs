//! Unvalidated pack declarations. Deserialization does not establish supported
//! versions, reference integrity, policy admission, authentication or authority.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::{ValidatorError, ValidatorWarning};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackManifest {
    pub pack: PackSection,
    pub source: SourceSection,
    pub target: TargetSection,
    pub schema_refs: SchemaRefsSection,
    pub compatibility: CompatibilitySection,
    pub capabilities: CapabilityDeclaration,
    pub operations: Vec<OperationEntry>,
    /// Absence is represented as empty; local validation requires dependencies
    /// when any operation is full or partial.
    #[serde(default)]
    pub toolchain: Vec<ToolchainEntry>,
    pub contracts: HashMap<String, ContractRef>,
    /// TOML values retain integer and datetime types. Extensions grant nothing;
    /// local validation separately rejects core-key shadowing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<toml::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackSection {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSection {
    pub cloud: String,
    pub service: String,
    pub language: String,
    pub language_version_range: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetSection {
    pub cloud: String,
    pub service: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaRefsSection {
    pub contract_schema_version: String,
    pub evidence_record_schema_version: String,
    pub verification_result_schema_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilitySection {
    pub catalog_min: String,
    pub catalog_max: String,
}

/// Requested needs only, never effective policy, credentials or dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDeclaration {
    pub format_version: String,
    pub requirements: Vec<CapabilityRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRequirement {
    pub id: String,
    pub operation_id: String,
    pub action: CapabilityAction,
    /// Opaque logical alias; parsing performs no path, URL or wildcard expansion.
    pub resource: String,
    pub purpose: String,
}

/// A closed request vocabulary. Even approve/deploy/delete are declarations,
/// never authority granted to a candidate-controlled worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CapabilityAction {
    Read,
    Write,
    Execute,
    Network,
    Model,
    Propose,
    Approve,
    Deploy,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationEntry {
    pub id: String,
    pub behavioral_fidelity: ManifestFidelity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumptions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManifestFidelity {
    Full,
    Partial,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolchainEntry {
    pub name: String,
    pub version_range: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractRef {
    pub id: String,
    pub format_version: String,
}

/// Local manifest validation only; neither outcome authenticates or admits work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManifestOutcome {
    Pass,
    Fail,
}

/// Local structural and semantic-rule diagnostics, not external-reference proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestValidationResult {
    pub outcome: ManifestOutcome,
    pub structural_errors: Vec<ValidatorError>,
    pub semantic_errors: Vec<ValidatorError>,
    pub warnings: Vec<ValidatorWarning>,
}
