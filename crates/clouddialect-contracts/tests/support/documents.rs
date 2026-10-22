use clouddialect_contracts::DocumentKind;
use serde_json::{Value, json};

/// Structurally valid declarations only; no authenticated observation/approval.
pub fn minimal_documents() -> [(DocumentKind, Value); 5] {
    let id = "550e8400-e29b-41d4-a716-446655440000";
    let artifact = json!({"identity":"artifact","content_hash":{"algorithm":"sha-256","value":"a".repeat(64)}});
    [
        (
            DocumentKind::BehavioralContract,
            json!({"id":id,"format_version":"1.0.0","operation_identity":{"call_site":"pkg.call"},"inputs":{},"outputs":{},"side_effects":[],"resource_dependencies":[],"obligations":[{"obligation_id":"o1","behavioral_fidelity":"unknown","unresolved_assumption":{"assumption_id":"a1","description":"unknown","reason":"not observed"}}]}),
        ),
        (
            DocumentKind::EvidenceRecord,
            json!({"id":id,"format_version":"1.0.0","claim":"claim","classification":"inferred","reasoning_chain":["explicit reason"]}),
        ),
        (
            DocumentKind::VerificationResult,
            json!({"id":id,"format_version":"1.0.0","candidate":artifact,"configuration":artifact,"pack":{"id":"pack","version":"1.0.0","content_hash":artifact["content_hash"]},"toolchain":{"id":"tool","version":"1.0.0","content_hash":artifact["content_hash"]},"binding":{"tenant_id":"tenant","project_id":"project","run_id":"run","attempt_id":"attempt","input":artifact,"dependencies":artifact,"suite":artifact,"oracle":artifact,"policy":artifact,"obligation_ids":["o1"]},"tests":[{"test_id":"t1","obligation_ids":["o1"],"outcome":"pass","origin_identity":"untrusted-claim"}]}),
        ),
        (
            DocumentKind::CatalogIndex,
            serde_json::from_slice(include_bytes!("../../../../catalog/index.json")).unwrap(),
        ),
        (
            DocumentKind::ExtensionRegistry,
            serde_json::from_slice(include_bytes!(
                "../../../../catalog/extension-registry.json"
            ))
            .unwrap(),
        ),
    ]
}
