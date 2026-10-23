use boon::{Compiler, SchemaIndex, Schemas};
use clouddialect_contracts::{CatalogSnapshot, CatalogUrlLoader};
use serde_json::{Value, json};

fn documents() -> [(&'static str, Value); 2] {
    [
        (
            "catalog-index",
            serde_json::from_slice(include_bytes!("../../../catalog/index.json")).unwrap(),
        ),
        (
            "extension-registry",
            serde_json::from_slice(include_bytes!("../../../catalog/extension-registry.json"))
                .unwrap(),
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
fn support_kinds_allow_absent_empty_and_declaration_only_mandatory_prefixes() {
    for (kind, base) in documents() {
        let (schemas, root) = engine(kind);
        assert!(schemas.validate(&base, root).is_ok());
        for prefixes in [
            json!([]),
            json!(["unimplemented"]),
            json!(["acme", "Acme", "Acme-Tools"]),
        ] {
            let mut value = base.clone();
            value["required_extensions"] = prefixes;
            assert!(schemas.validate(&value, root).is_ok(), "{kind}");
            assert!(value.get("acme:payload").is_none());
        }
    }
}

#[test]
fn support_kinds_reject_invalid_prefix_grammar_and_whitespace() {
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
fn support_kinds_reject_wrong_types_and_duplicate_decoded_prefixes() {
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
            assert!(schemas.validate(&value, root).is_err(), "{kind}");
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
fn declarations_preserve_existing_support_root_openness_and_do_not_attest_registration() {
    for (kind, mut value) in documents() {
        let (schemas, root) = engine(kind);
        value["required_extensions"] = json!(["acme"]);
        value["unrelated_open_metadata"] = json!({"claimed_implementation":true});
        value["acme:payload"] = json!({"owner":"untrusted"});
        if kind == "extension-registry" {
            value["extensions"] = json!([{"namespace_prefix":"acme","owner":"untrusted","purpose":"claim","registration_date":"2026-10-22"}]);
        }
        assert!(
            schemas.validate(&value, root).is_ok(),
            "unrelated openness {kind}"
        );
        // Registry presence, ownership and optional payload are not implemented
        // semantics; the separate extension validator must still emit a blocker.
    }
}
