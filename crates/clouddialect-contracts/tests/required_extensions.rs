use boon::{Compiler, SchemaIndex, Schemas};
use clouddialect_contracts::{CatalogSnapshot, CatalogUrlLoader};
use serde_json::{Value, json};

fn documents() -> [(&'static str, Value); 3] {
    let id = "550e8400-e29b-41d4-a716-446655440000";
    let artifact = json!({"identity":"artifact","content_hash":{"algorithm":"sha-256","value":"a".repeat(64)}});
    [
        (
            "behavioral-contract",
            json!({"id":id,"format_version":"1.0.0","operation_identity":{"call_site":"pkg.call"},"inputs":{},"outputs":{},"side_effects":[],"resource_dependencies":[],"obligations":[{"obligation_id":"o1","behavioral_fidelity":"unknown","unresolved_assumption":{"assumption_id":"a1","description":"unknown","reason":"not observed"}}]}),
        ),
        (
            "evidence-record",
            json!({"id":id,"format_version":"1.0.0","claim":"claim","classification":"inferred","reasoning_chain":["explicit reason"]}),
        ),
        (
            "verification-result",
            json!({"id":id,"format_version":"1.0.0","candidate":artifact,"configuration":artifact,
            "pack":{"id":"pack","version":"1.0.0","content_hash":artifact["content_hash"]},"toolchain":{"id":"tool","version":"1.0.0","content_hash":artifact["content_hash"]},
            "binding":{"tenant_id":"tenant","project_id":"project","run_id":"run","attempt_id":"attempt","input":artifact,"dependencies":artifact,"suite":artifact,"oracle":artifact,"policy":artifact,"obligation_ids":["o1"]},"tests":[{"test_id":"t1","obligation_ids":["o1"],"outcome":"pass","origin_identity":"untrusted-claim"}]}),
        ),
    ]
}

fn engine(kind: &str) -> (Schemas, SchemaIndex) {
    let mut compiler = Compiler::new();
    compiler.use_loader(Box::new(CatalogUrlLoader::new(
        CatalogSnapshot::embedded().unwrap(),
    )));
    compiler.enable_format_assertions();
    let mut schemas = Schemas::new();
    let root = compiler
        .compile(
            &format!("https://schemas.clouddialect.io/{kind}/1.0.0/schema.json"),
            &mut schemas,
        )
        .unwrap();
    (schemas, root)
}

#[test]
fn all_primary_kinds_allow_absence_empty_and_declaration_only_unknown_prefixes() {
    for (kind, base) in documents() {
        let (schemas, root) = engine(kind);
        assert!(schemas.validate(&base, root).is_ok(), "baseline {kind}");
        for prefixes in [
            json!([]),
            json!(["unimplemented"]),
            json!(["acme", "Acme", "Acme-Tools"]),
        ] {
            let mut value = base.clone();
            value["required_extensions"] = prefixes;
            assert!(
                schemas.validate(&value, root).is_ok(),
                "declaration only {kind}"
            );
            assert!(value.get("acme:payload").is_none());
        }
        // A structural pass is not implemented support or behavioral acceptance.
    }
}

#[test]
fn primary_prefixes_use_exact_ascii_namespace_grammar_without_colons_or_whitespace() {
    for (kind, base) in documents() {
        let (schemas, root) = engine(kind);
        for prefix in [
            "",
            "a",
            "1acme",
            "acme:payload",
            "acme:",
            "acme_tools",
            "acme/tools",
            "acme\\tools",
            "acme tools",
            " acme",
            "acme ",
            "acme\n",
            "acme\r\n",
            "acme\t",
            "équipe",
            "соre",
        ] {
            let mut value = base.clone();
            value["required_extensions"] = json!([prefix]);
            assert!(
                schemas.validate(&value, root).is_err(),
                "{kind}: {prefix:?}"
            );
        }
    }
}

#[test]
fn primary_declarations_reject_wrong_types_and_duplicate_decoded_prefixes() {
    for (kind, base) in documents() {
        let (schemas, root) = engine(kind);
        for malformed in [
            json!(null),
            json!(false),
            json!("acme"),
            json!({}),
            json!([null]),
            json!([17]),
            json!(["acme", "acme"]),
        ] {
            let mut value = base.clone();
            value["required_extensions"] = malformed;
            assert!(
                schemas.validate(&value, root).is_err(),
                "invalid declaration {kind}"
            );
        }
        let mut value = base;
        value["required_extensions"] = json!(["acme", "__ESCAPED__"]);
        let encoded = serde_json::to_string(&value)
            .unwrap()
            .replace("\"__ESCAPED__\"", r#""\u0061cme""#);
        let decoded: Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded["required_extensions"], json!(["acme", "acme"]));
        assert!(
            schemas.validate(&decoded, root).is_err(),
            "decoded duplicate {kind}"
        );
    }
}

#[test]
fn adding_the_core_declaration_does_not_open_primary_roots_or_promote_payload_claims() {
    for (kind, mut value) in documents() {
        let (schemas, root) = engine(kind);
        value["required_extensions"] = json!(["acme"]);
        value["acme:payload"] = json!({"implemented":true,"owner":"untrusted-claim"});
        assert!(schemas.validate(&value, root).is_ok());
        value["unexpected_unqualified"] = json!(true);
        assert!(schemas.validate(&value, root).is_err(), "closed {kind}");
        // Registration/owner/payload cannot clear the future semantic blocker.
    }
}
