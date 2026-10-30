use clouddialect_contracts::{BehavioralResult,StructuralResult};
pub fn promote(x: StructuralResult) -> BehavioralResult { x.into() }
