use clouddialect_contracts::{VerificationResult, verify_artifact_hash};
use serde_json::json;

#[path = "support/documents.rs"]
mod documents;

// Standard empty/"abc" vectors, independently checked with Python hashlib.
// No expected digest is computed with the implementation's Rust dependencies.
const VECTORS: [(&str, &str, &str); 5] = [
    (
        "sha-256",
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    ),
    (
        "sha-384",
        "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7",
        "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b",
    ),
    (
        "sha-512",
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
    ),
    (
        "sha3-256",
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a",
    ),
    (
        "sha3-512",
        "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0",
        "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26",
    ),
];

fn result(algorithm: &str, value: &str) -> VerificationResult {
    let mut document = documents::minimal_documents()[2].1.clone();
    document["candidate"]["content_hash"] = json!({"algorithm":algorithm,"value":value});
    serde_json::from_value(document).unwrap()
}

#[test]
fn all_declared_algorithms_match_independent_abc_vectors() {
    for (algorithm, digest, _) in VECTORS {
        assert!(
            verify_artifact_hash(&result(algorithm, digest), b"abc"),
            "{algorithm}"
        );
    }
}

#[test]
fn empty_artifacts_are_hashed_not_treated_as_missing() {
    for (algorithm, _, empty_digest) in VECTORS {
        let document = result(algorithm, empty_digest);
        assert!(verify_artifact_hash(&document, b""), "{algorithm}");
        assert!(!verify_artifact_hash(&document, b"abc"), "{algorithm}");
    }
}

#[test]
fn any_candidate_byte_change_invalidates_the_recorded_hash() {
    for (algorithm, digest, _) in VECTORS {
        let document = result(algorithm, digest);
        for changed in [b"abd".as_slice(), b"ab", b"abc\0", b"abc\n", b"\xffabc"] {
            assert!(
                !verify_artifact_hash(&document, changed),
                "{algorithm}: {changed:?}"
            );
        }
    }
}

#[test]
fn malformed_values_and_unapproved_algorithm_spellings_fail_closed() {
    for (algorithm, digest, _) in VECTORS {
        for malformed in [
            String::new(),
            digest[..digest.len() - 1].to_owned(),
            format!("{digest}0"),
            digest.to_uppercase(),
            format!(" {digest}"),
            format!("{digest}\n"),
            format!("0x{digest}"),
            format!("g{}", &digest[1..]),
            format!("é{}", &digest[2..]),
        ] {
            assert!(
                !verify_artifact_hash(&result(algorithm, &malformed), b"abc"),
                "{algorithm}: {malformed:?}"
            );
        }
    }
    for algorithm in [
        "",
        "md5",
        "sha256",
        "SHA-256",
        "sha-256 ",
        "sha3-384",
        "keccak-256",
    ] {
        assert!(
            !verify_artifact_hash(&result(algorithm, VECTORS[0].1), b"abc"),
            "{algorithm}"
        );
    }
}

#[test]
fn matching_candidate_bytes_do_not_validate_other_document_claims() {
    let mut document = result("sha-256", VECTORS[0].1);
    document.id.clear();
    document.format_version = "not-a-version".into();
    document.candidate.identity = "untrusted-identity".into();
    document.configuration.content_hash.value = "wrong".into();
    document.pack.content_hash.value = "wrong".into();
    document.toolchain.content_hash.value = "wrong".into();
    document.binding.input.content_hash.value = "wrong".into();
    document.tests.clear();
    document.required_extensions = Some(vec!["unrecognized".into()]);
    document.extensions.insert(
        "acme:approval".into(),
        serde_json::value::RawValue::from_string("true".into()).unwrap(),
    );
    assert!(document.acceptance.is_none());
    // This helper checks only bytes against the candidate hash, not acceptance,
    // structural/semantic validity, identity, observations or policy authority.
    assert!(verify_artifact_hash(&document, b"abc"));
}

#[test]
fn verification_does_not_mutate_the_document_or_candidate() {
    for (algorithm, digest, _) in VECTORS {
        let document = result(algorithm, digest);
        let before_document = serde_json::to_vec(&document).unwrap();
        let candidate = b"abc".to_vec();
        let before_candidate = candidate.clone();
        assert!(verify_artifact_hash(&document, &candidate));
        assert_eq!(serde_json::to_vec(&document).unwrap(), before_document);
        assert_eq!(candidate, before_candidate);
    }
}
