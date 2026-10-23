//! P0-C reference consistency, never authentication of a verification claim.

use std::collections::BTreeSet;

use boon::{Compiler, Schemas};
use serde_json::{Value, json};

use crate::{ErrorCategory, ValidatorError, VerificationResult};

/// The consumer first validates the same bytes against its active schema.
/// Core cross-field/date rules also apply when that schema is less restrictive.
pub(super) fn check_verification(document: &VerificationResult) -> Vec<ValidatorError> {
    let mut errors = Vec::new();
    let mut report = |path: String, description: &str| {
        errors.push(ValidatorError {
            document_id: Some(document.id.clone()),
            format_version: Some(document.format_version.clone()),
            category: ErrorCategory::Semantic,
            field_path: Some(path),
            description: description.to_owned(),
            byte_offset: None,
        });
    };
    let mut test_ids = BTreeSet::new();
    for (index, test) in document.tests.iter().enumerate() {
        if !test_ids.insert(&test.test_id) {
            report(
                format!("tests[{index}].test_id"),
                "Test identity is duplicated in this verification result",
            );
        }
        for (obligation_index, obligation) in test.obligation_ids.iter().enumerate() {
            if !document.binding.obligation_ids.contains(obligation) {
                report(
                    format!("tests[{index}].obligation_ids[{obligation_index}]"),
                    "Test obligation is absent from the declared binding",
                );
            }
        }
    }
    let binding = &document.binding;
    let mut hashes = vec![
        ("candidate.content_hash", &document.candidate.content_hash),
        ("pack.content_hash", &document.pack.content_hash),
        ("toolchain.content_hash", &document.toolchain.content_hash),
        (
            "configuration.content_hash",
            &document.configuration.content_hash,
        ),
        ("binding.input.content_hash", &binding.input.content_hash),
        (
            "binding.dependencies.content_hash",
            &binding.dependencies.content_hash,
        ),
        ("binding.suite.content_hash", &binding.suite.content_hash),
        ("binding.oracle.content_hash", &binding.oracle.content_hash),
        ("binding.policy.content_hash", &binding.policy.content_hash),
    ];
    if let Some(acceptance) = &document.acceptance {
        hashes.push((
            "acceptance.approval_evidence.content_hash",
            &acceptance.approval_evidence.content_hash,
        ));
        if acceptance.verification_result.id != document.id {
            report(
                "acceptance.verification_result.id".to_owned(),
                "Approval reference must identify this exact verification document",
            );
        }
        if acceptance.verification_result.format_version != document.format_version {
            report(
                "acceptance.verification_result.format_version".to_owned(),
                "Approval reference must match this exact document-format version",
            );
        }
        // Fixed core semantics, not a replacement/default catalog. This literal
        // resource has no external references or caller-controlled schema input.
        let mut compiler = Compiler::new();
        compiler.enable_format_assertions();
        let uri = "urn:clouddialect:semantic-utc-timestamp";
        compiler
            .add_resource(
                uri,
                json!({"$schema":"https://json-schema.org/draft/2020-12/schema", "type":"string", "format":"date-time"}),
            )
            .expect("constant timestamp rule is valid");
        let mut schemas = Schemas::new();
        let schema = compiler
            .compile(uri, &mut schemas)
            .expect("constant timestamp rule compiles without external resources");
        if !acceptance.timestamp.ends_with('Z')
            || schemas
                .validate(&Value::String(acceptance.timestamp.clone()), schema)
                .is_err()
        {
            report(
                "acceptance.timestamp".to_owned(),
                "Approval timestamp must be a valid date-time using normalized UTC ending in Z",
            );
        }
    }
    for (path, hash) in hashes {
        let length = match hash.algorithm.as_str() {
            "sha-256" | "sha3-256" => Some(64),
            "sha-384" => Some(96),
            "sha-512" | "sha3-512" => Some(128),
            _ => None,
        };
        match length {
            None => report(
                format!("{path}.algorithm"),
                "Content hash algorithm is unsupported",
            ),
            Some(length)
                if hash.value.len() != length
                    || !hash
                        .value
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) =>
            {
                report(
                    format!("{path}.value"),
                    "Content hash must be lowercase hexadecimal of the algorithm's exact length",
                );
            }
            Some(_) => {}
        }
    }
    errors
}
