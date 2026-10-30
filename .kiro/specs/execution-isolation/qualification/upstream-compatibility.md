# Upstream compatibility plan — TASK-006

Status: **INCOMPLETE** — partial implementation now exists; complete conformance, native qualification and upstream/core owner acceptance remain outstanding. Package 0.1.0 alone does not establish API compatibility.

## Offline handoff preparation — 2026-10-30

Source revision: **`8ba032e662153828267c6242d9773b1b77e8bb93`**; standalone
`crates/clouddialect-contracts/Cargo.toml`, crate **0.1.0**, catalog/documents
**1.0.0**. This is a compile-only handoff probe and selected in-memory regression
run, not the approved runtime consumer or complete conformance qualification.

### Frozen identities

All exact public API source, manifest/lock and catalog hashes (38 files) are in
[`upstream-identities.json`](../../../../.workflow/ultracode/execution-isolation-goal-2026-10-30/offline-preparation/upstream-identities.json).
Hashes establish byte identity only, not a signed or accepted release.
The schema URI for each row below is exactly
`https://schemas.clouddialect.io/<document-type>/1.0.0/schema.json`.
All five entries are `current`; the resolver-resource list is empty.

| Document type | Exact schema bytes: SHA-256 |
|---|---|
| behavioral-contract | `4a159debf540e781bb4dfed001e31f6c68915876ab0bc9cd9f52ffb143f77f53` |
| evidence-record | `d25acf15e02251188dfca4f34774699a326de1c8ef629c226f34cba6c6158df3` |
| verification-result | `c6246397e248f89fecae1639ae838153a88946b9cdfa15dde15de1b3e6b06ba9` |
| catalog-index | `7c231690c96aa107fe192f6256c6eefbfae7cbb859a55364590738e824395e29` |
| extension-registry | `f968ec530254a23012f6e4941dd21b02870eb48ad6a97ed19a6b101ca6069c2d` |

`catalog/index.json` SHA-256:
`695e026957c83c31f0e03b0b05428bccf5097df02c1b4d7f7fe04c4b8f3298ce`.
The whole-catalog snapshot digest, independently recomputed from current source's
`CloudDialectCatalogSnapshot/v1\0` domain separator and sorted UTF-8 paths/content
with unsigned 64-bit big-endian lengths, is
`58db9c1e7c9d339530de0bb2b2eede3c2cf64acc2d815ba137436c46200360c9`.
It is not a plain directory or index hash, nor an observation of a deployed loader.

### Compile and regression evidence

The canonical [compile-only consumer](../../../../scripts/verification/execution-isolation/upstream_signatures.rs)
checks exact callable types, `BehavioralResult` fields, the complete verification
field set, capability fields and exhaustiveness of behavioral outcomes against
the actual compiled library. No local shim or altered upstream crate is used.
[`compile-results.json`](../../../../.workflow/ultracode/execution-isolation-goal-2026-10-30/offline-preparation/compile-results.json)
records the exact `rtk proxy rustc --edition=2024 --crate-type lib --emit=metadata
-D warnings --extern ... -L dependency=...` commands and exits:

| Check | Observed | Meaning |
|---|---|---|
| Positive consumer | Exit 0 | Listed current signatures/fields compile; no execution or trust assertion. |
| Structural → behavioral `.into()` | Exit 1, E0277 | This forbidden promotion is not provided. |
| Semantic → structural `.into()` | Exit 1, E0277 | These phase result types cannot substitute via `Into`. |
| `BehavioralOutcome::Accepted` | Exit 1, E0599 | No invented accepting variant. |

Selected real tests (all **PASS**, 44 total): `public_api_compatibility` (1),
`phase_results` (8), `catalog_snapshot` (8), `evidence_completeness` (10),
`extension_validation` (7), `verification_semantics` (6),
`required_extension_models` (4). They cover public-path identity, closed outcome
labels, immutable/closed catalog lookup, incomplete evidence, optional extension
bytes, required-extension declarations and verification-record consistency.
The required-extension *model* tests do not alone prove semantic blocking or
runtime authorization. These are Rust in-memory tests, not IPC/broker/store tests.

Reproduction (from repository root, using an already populated offline cache):

```sh
rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home cargo test --offline --locked --manifest-path crates/clouddialect-contracts/Cargo.toml --target-dir /tmp/clouddialect-offline-oB17k0 --test public_api_compatibility --test phase_results --test catalog_snapshot --test evidence_completeness --test extension_validation --test verification_semantics --test required_extension_models
```

The `/tmp` paths are local run artifacts, not required deployment locations;
another machine needs a separately populated compatible cache. No automatic
download fallback is permitted. Initial default-cache execution exited 101
(`boon` unavailable); the recorded cache retry exited 0. See
[`rust-tests-cached.txt`](../../../../.workflow/ultracode/execution-isolation-goal-2026-10-30/offline-preparation/rust-tests-cached.txt).

**NOT RUN this packet:** full Cargo/conformance suite, child-process controls,
native network/isolation, provider credential lifecycle, persistence/restore,
coverage and release/license scan. Prior full-suite and license findings below
are historical, not refreshed by these 44 tests. Upstream/core approvals and
actual consumer qualification remain **MISSING**; TASK-006/BG-01 remain open.

## Historical bounded evidence — 2026-10-29

At immutable source `5b3dfa6954805cfbeba3e9724803dac064ba48e6`, the parent
re-ran the actual contracts crate: **291 Rust cases (7 doctests included),
101 conformance fixtures, build, formatting and warning-denying Clippy pass**.
Completeness has 10 focused cases; the runner-control target reports 11 cases,
of which 10 are substantive controls and one is a subprocess helper. Public-path
type identity and negative conversion doctests provide real bounded consumer/type
evidence, not a delivered Spec 2 runtime consumer or authenticated acceptance.

See the [fresh task audit and command outputs](../../../../.workflow/ultracode/execution-isolation-status-audit-2026-10-29/final-report.md).
The scan against the unchanged seven-license policy exits 4 on actual findings;
owner review remains required. No measured coverage, approved native profile,
runtime crate, real backend/credential/restore proof or upstream/core acceptance
record exists in this audited delivery. BG-01 and TASK-006 stay open.

## Historical bounded evidence — 2026-10-23, superseded snapshot

The following observations/counts are retained as history only; use the dated
current audit above for present source and execution evidence.

The [current BG-01 inventory](../decisions/BG-01-upstream.md) replaces the historical absence claim below. Baseline `5dc25f3` plus uncommitted semantic changes includes real JSON/TOML semantic methods, verification-link rules, all-five-kind mandatory blockers and transitive contract/evidence checks using the active catalog and explicitly scoped store. Current unit/integration checks are recorded with exact source hashes in the [continuation report](../../../../.workflow/ultracode/execution-isolation-implementation/semantic-continuation/final-report.md).

These tests validate the port's data boundary, not a real tenant/project store backend or authenticated observations/approval. Completeness now has9focused tests and parent-only audit, not reviewer28signoff. Exhaustive conformance, actual downstream consumer qualification, required native checks and an immutable accepted delivery revision remain unfinished. No gate is closed.

Reviewer 28's earlier bounded semantic/compatibility review independently passed 36 tests and found no new actionable diff defect. The later extension convention is parent-audited only. The `$schema` URI diagnostic was fixed with actual RED/GREEN checkpoints `36db142`/`cb36beb`. Reviewer 29's separate completeness diagnostic omission was reproduced and repaired under explicit approval in `03738e4`/`0396883`; all 10 completeness tests pass. The runner has 9 passing controls. The original seed `408af0d` initially ran once; the separately approved current corpus has 42 fixtures, with positive-prefix count13 and empty selection still rejected. Full default Cargo checks now pass 286 Rust tests plus 42 admitted fixtures. Reviewer 30 supplied prior static patch/runner review, not independent repaired-tree execution. Fresh coverage, full corpus, native evidence and downstream handoff remain unfinished.

## Historical prerequisite probe — not current API evidence
The actual library root compiled directly with `rustc --edition=2024 --crate-type rlib --crate-name clouddialect_contracts`; consumer imports of StructuralResult, SemanticResult, BehavioralResult, BehavioralOutcome, ValidatorError, BlockingGap and CatalogUrlLoader failed with E0432. The direct compile bypasses unused Cargo dependencies and proves only the stub's missing exports. No full Cargo integration result is claimed.

## Required checks after upstream delivery
- Build a consumer against the real locked path dependency at the recorded Git revision. Instantiate agreed public result/error/manifest/loader signatures; exercise real schemas and positive/negative fixtures.
- Prove structural success cannot imply semantic success, and neither result converts to behavioral acceptance. `Deferred`, `EvidenceComplete`, missing/unknown/failed/skipped/stale evidence remain non-accepting.
- Validate capability declarations; unknown optional extensions never grant authority; unknown mandatory security declarations deny admission.
- Validate per-test/failure/configuration evidence and exact candidate/input/attempt/suite/oracle/policy identity agreement. Replayed/mismatched evidence cannot create an acceptance intent.
- Offline loader resolves only admitted catalog content; tampered, missing or worker-selected overrides block without a network fallback.
- Errors preserve upstream diagnostic details without adding execution categories to ValidatorError or leaking raw candidate diagnostics/secrets.
- Freeze immutable API/schema/catalog fixtures and record compile, conformance, dependency/license and current-policy compatibility reviews.

Acceptance remains stronger than any upstream document validation result: Spec 2 additionally requires independently attributable behavioral evidence, current approval, budget/policy checks, cleanup confirmation and a durable ordered commit. No test may accept by matching a worker's success field.
