# EX-1 declaration delivery — 2026-10-22

**Delivered:** optional mandatory-extension declarations in all five JSON schemas and four existing DTOs. **Not delivered:** actual extension blocker/recognition logic (unit2.6), full validators/conformance, BG-01 or native isolation qualification. Original goal stays active/incomplete.

## Scope and implementation

Owner-approved EX-1 applies to optional root required_extensions: unique decoded ASCII namespace-prefix strings, minimum two characters, letter start then alphanumeric/hyphen, no colon/whitespace. Omission/empty declares none. Exact case remains significant. Explicit whitespace exclusion rejects terminal-newline regex edges.

BehavioralContract, EvidenceRecord, VerificationResult and CatalogIndex retain Option<Vec<String>> explicitly; the three primary RawFields conversions handle the field as core rather than unknown extension data. Existing exact encoded JSON extension value bytes survive. CatalogIndex::from_parts now performs typed conversion from original bytes, preventing duplicate core fields being erased by the earlier schema-checking Value. No registry DTO, public namespace, dependency, authority grant or acceptance conversion was introduced.

The three primary roots remain closed; catalog/registry roots retain their prior openness. Structurally valid unknown mandatory declarations pass without payload. Registry membership, claimed ownership and optional payload do not establish implemented recognition. Requirement10.4 acceptance blockers remain MISSING until the actual later validator and fixtures exist.

## Tests first and reachable checkpoints

| Leaf | Intended RED | Minimal GREEN |
|---|---|---|
| Existing DTOs | cec592d: E0609 on all four missing explicit fields | a28cb70: required_extension_models4/4 |
| Primary schemas | 5bc422e: actual target4tests,2 intended failures because closed roots reject new core field | 1ae8b66: required_extensions4/4 |
| Catalog/registry schemas | 8cfd8a0: actual target4tests,2 intended failures because open roots admit malformed declarations | c4a2601: required_extension_catalogs4/4 |

After the DTO compile RED, its runtime target additionally reproduced duplicate CatalogIndex overwrite (3pass/1fail); original-byte typed decoding fixes it before the DTO GREEN. This is planned duplicate-preservation behavior, not a post-review repair. Raw logs and hashes are retained in results/extension-evidence-manifest.json. Checkpoint reachability and signature-block presence are checked separately; signer identity is UNVERIFIED and signing policy is unchanged.

## Final integrated verification

All commands/results: results/extension-final-checks.json and corresponding text logs. Frozen Cargo used isolated /tmp Cargo/target directories; no new dependency fetch was needed.

- **136 integration +2 source-local unit +7 compile-fail doctests pass.** Twelve new integration tests across the three EX targets. Actual negative diagnostics: one E0596 (immutable snapshot), six E0277 (forbidden phase conversions). Empty conformance target still contains zero tests and is not delivered conformance.
- Frozen all-target check, fmt check and Clippy -Dwarnings pass.
-15 current Python checks and8 named historical P0 content cases pass. Historical full draft/scope/hash graders are not claimed green and were not rerun as EX acceptance gates.
- Corrected emitted production lines **315/330=95.45%**: results/extension-final-production-coverage.json. Raw src-file totals332/347 include17inline unit-test lines; actual llvm-cov show counters identify/remove those. No branch, unmapped DTO, whole-spec or native-runtime coverage claim.
- results/extension-schema-parity.json: identical new property across all five schemas; removing only that property restores baseline JSON values exactly. Four available design examples contain the matching property. Five pre-existing non-EX behavioral design/source differences remain explicit; whole-schema design parity is not claimed.
- results/extension-scope-audit.json records baseline55351ac, exact twelve source/test paths, four approved Spec1 documents and run-only metadata. Research, dependency/lock/notices, rules/symlinks, custom scripts and unrelated sources remain unchanged.

Packet18 independent bounded static PASS, no required findings/repairs; no independently executed tests/coverage. Packet17 earlier RC-1 review also PASS. See results/18-extension-integration.md. Cumulative18 sidecars exhausted; zero recursion/followups. The shared75-minute tranche retains original start13:41:20UTC; one consolidated repair allowed, zero consumed. No timer/repair reset.

## Publication and next work

User authorized normal feature-branch commit/push. Final evidence commit and independent remote-SHA receipt follow outside this self-referential report at /tmp/clouddialect-extension-publication.json and in the conversation. No force push, release or deployment. Rule availability is not native-client activation; SSH signature presence is not signer authentication.

Next approved prerequisite code: version helper2.4 under the owner-approved stable/version-support profile, then structural entrypoint2.5 and actual extension blockers2.6. Subsequent manifest/semantic/behavioral validation, hashes, complete fixture inventory/runner and downstream compatibility remain unfinished. BG-01/UD-1–UD-5 stay OPEN; no approved disposable probe environment, native probes, measured budgets or provider actions.
