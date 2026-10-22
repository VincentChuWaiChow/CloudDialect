//! Unvalidated behavioral contract data. Schema conditions, reference integrity and authenticity
//! must be checked separately; parsing these types is never behavioral evidence.

use super::{
    RawObject,
    raw_objects::{RawFields, raw_object_model},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BehavioralFidelity {
    EquivalentWithinContract,
    AdaptationRequired,
    ArchitecturalChangeRequired,
    Unsupported,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct OperationIdentity {
    pub call_site: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_chain: Option<Vec<String>>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct UnresolvedAssumption {
    pub assumption_id: String,
    pub description: String,
    pub reason: String,
    #[serde(flatten)]
    pub extensions: RawObject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceBehaviorDimension {
    Ordering,
    Consistency,
    Retry,
    Failure,
}

/// Resource-only dimension binding. Populated text and complete coverage are
/// enforced by the schema, not proven by decoding this declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct ResourceUnresolvedAssumption {
    pub behavior: ResourceBehaviorDimension,
    pub assumption_id: String,
    pub description: String,
    pub reason: String,
    #[serde(flatten)]
    pub extensions: RawObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct SideEffect {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct KnownBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct ResourceDependency {
    pub identity: String,
    pub permissions: Vec<String>,
    pub configuration: RawObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_behavior: Option<KnownBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved_assumptions: Option<Vec<ResourceUnresolvedAssumption>>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

/// The schema, not this DTO, enforces exactly one nonempty source of support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct Obligation {
    pub obligation_id: String,
    pub behavioral_fidelity: BehavioralFidelity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_refs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved_assumption: Option<UnresolvedAssumption>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct BehavioralNote {
    pub note_type: String,
    pub description: String,
    pub evidence_ref: String,
    #[serde(flatten)]
    pub extensions: RawObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawFields")]
pub struct BehavioralContract {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub id: String,
    pub format_version: String,
    pub operation_identity: OperationIdentity,
    pub inputs: RawObject,
    pub outputs: RawObject,
    pub side_effects: Vec<SideEffect>,
    pub resource_dependencies: Vec<ResourceDependency>,
    pub obligations: Vec<Obligation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavioral_notes: Option<Vec<BehavioralNote>>,
    #[serde(flatten)]
    pub extensions: RawObject,
}

raw_object_model!(OperationIdentity, [call_site], [wrapper_chain => "wrapper_chain"]);
raw_object_model!(
    UnresolvedAssumption,
    [assumption_id, description, reason],
    []
);
raw_object_model!(
    ResourceUnresolvedAssumption,
    [behavior, assumption_id, description, reason],
    []
);
raw_object_model!(SideEffect, [description], [evidence_ref => "evidence_ref"]);
raw_object_model!(KnownBehavior, [], [ordering => "ordering", consistency => "consistency", retry => "retry", failure => "failure"]);
raw_object_model!(ResourceDependency, [identity, permissions, configuration], [known_behavior => "known_behavior", unresolved_assumptions => "unresolved_assumptions"]);
raw_object_model!(Obligation, [obligation_id, behavioral_fidelity], [evidence_refs => "evidence_refs", unresolved_assumption => "unresolved_assumption"]);
raw_object_model!(BehavioralNote, [note_type, description, evidence_ref], []);
raw_object_model!(BehavioralContract, [id, format_version, operation_identity, inputs, outputs, side_effects, resource_dependencies, obligations], [schema => "$schema", behavioral_notes => "behavioral_notes"]);
