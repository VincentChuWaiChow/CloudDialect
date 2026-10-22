//! Unvalidated evidence records. Schema conditions and authenticity are separate.

use super::{
    RawObject, UnresolvedAssumption,
    raw_objects::{RawFields, raw_object_model},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EvidenceClassification {
    Observed,
    Inferred,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct EvidenceProvenance {
    pub source_artifact_identity: String,
    pub analysis_tool_identity: String,
    #[serde(flatten)]
    pub extensions: RawObject,
}

raw_object_model!(
    EvidenceProvenance,
    [source_artifact_identity, analysis_tool_identity],
    []
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct EvidenceRecord {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub id: String,
    pub format_version: String,
    pub claim: String,
    pub classification: EvidenceClassification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<EvidenceProvenance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_chain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved_assumptions: Option<Vec<UnresolvedAssumption>>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

raw_object_model!(EvidenceRecord, [id, format_version, claim, classification], [schema => "$schema", provenance => "provenance", reasoning_chain => "reasoning_chain", unresolved_assumptions => "unresolved_assumptions"]);
