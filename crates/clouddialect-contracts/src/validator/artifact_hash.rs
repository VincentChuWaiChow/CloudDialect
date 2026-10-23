//! Candidate byte integrity only; matching does not establish acceptance.

use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_512};

use crate::{ContentHash, VerificationResult};

/// Compare exact candidate bytes with the document's declared content hash.
/// Only the five schema-enumerated algorithm names and canonical lowercase
/// hexadecimal values match. Other document claims are not validated here;
/// matching is not proof of identity, provenance, observation or approval.
pub fn verify_artifact_hash(result_doc: &VerificationResult, candidate_bytes: &[u8]) -> bool {
    verify_content_hash(&result_doc.candidate.content_hash, candidate_bytes)
}

pub(super) fn verify_content_hash(hash: &ContentHash, candidate_bytes: &[u8]) -> bool {
    match hash.algorithm.as_str() {
        "sha-256" => digest_matches::<Sha256>(&hash.value, candidate_bytes),
        "sha-384" => digest_matches::<Sha384>(&hash.value, candidate_bytes),
        "sha-512" => digest_matches::<Sha512>(&hash.value, candidate_bytes),
        "sha3-256" => digest_matches::<Sha3_256>(&hash.value, candidate_bytes),
        "sha3-512" => digest_matches::<Sha3_512>(&hash.value, candidate_bytes),
        _ => false,
    }
}

fn digest_matches<D: Digest>(expected: &str, bytes: &[u8]) -> bool {
    let actual: String = D::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    actual == expected
}
