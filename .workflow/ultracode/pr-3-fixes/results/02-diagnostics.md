# R4/R5 diagnostics correction

## Outcome
PASS for the owned slice: actual numeric operation indices and referenced diagnostic attribution, demonstrated RED before production edits then GREEN after focused formatting.

## Owned production/test files
- `crates/clouddialect-contracts/src/validator/fidelity.rs`
- `crates/clouddialect-contracts/src/manifest/validator.rs`
- `crates/clouddialect-contracts/tests/fidelity_mapping.rs`
- `crates/clouddialect-contracts/tests/manifest_semantic.rs`

## Decisions
- R4 enumerates the original operations array (including skipped unsupported entries), emits `operations[index].behavioral_fidelity`, and retains the operation ID in Debug-escaped diagnostic prose. Updated the old string-index assertion.
- R5 preserves every already-located resolver diagnostic unchanged, including referenced identity/version/path/category/offset. Structural and mandatory-blocker failures no longer acquire fabricated manifest children or lose referenced-document context.
- Locationless missing/unavailable-store failures still identify the actual manifest `contracts.<key>.id` binding. Their description includes the Debug-escaped exact referenced ID/version. Store-provided error messages remain sanitized by the existing resolver; no new trust or acceptance semantics.
- No public API, exports, dependencies, schemas, versions or gates changed. The existing `SemanticResult.document_id` remains the manifest identity while each nested error retains its own source document identity.

## Verification
Exact commands, exits and evidence filenames: `diagnostics-checks.json`.
1. RED: frozen/offline targeted Cargo invocation, fidelity target 4 passed/4 failed, exit101. Cargo stopped before manifest target; then a separate manifest invocation produced 5 passed/2 failed, exit101. Failures directly demonstrate numeric-index, source-location and missing-reference-detail expectations.
2. Minimal production edits.
3. GREEN: same targeted targets, fidelity8 + manifest7 pass, exit0.
4. `rustfmt --edition 2024` applied only to the four owned files, exit0; exact targeted Cargo invocation rerun, same15 passing, exit0.
- All Cargo checks use `CARGO_HOME=/tmp/clouddialect-phase-cargo` and dedicated `CARGO_TARGET_DIR=/tmp/clouddialect-pr3-diagnostics-target`, `--frozen --offline`. No downloads or shared target writes.
- Coverage includes full/partial fidelity, a preceding unsupported array entry, original referenced structural and mandatory-blocker context, and explicit unsupported/orphan reference structural failures and missing records.
- Final owned source SHA-256 values: `diagnostics-source-hashes.json`.

## Evidence note
One initial attempt to save tool output through an incorrectly shell-quoted Python command failed exit1 (SyntaxError/command-not-found); no source modification occurred. Corrected evidence persistence used apply_patch; raw RED outputs are retained.

## Limits
Parent owns full combined Rust/Python/check/fmt/Clippy and independent review. This worker ran only the focused15 tests, not the full suite/coverage/MSRV/native probes. No Git commands, commits, pushes, PR changes, network, credentials, subdelegation or unrelated edits. Runtime isolation/model/effort and token usage remain unattested/unknown. No postintegration repair consumed by this packet.
