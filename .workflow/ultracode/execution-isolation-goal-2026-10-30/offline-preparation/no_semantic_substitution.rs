use clouddialect_contracts::{SemanticResult,StructuralResult};
pub fn promote(x: SemanticResult) -> StructuralResult { x.into() }
