# Upstream compatibility plan — TASK-006

Status: **INCOMPLETE** — partial implementation now exists; complete conformance, native qualification and upstream/core owner acceptance remain outstanding. Package 0.1.0 alone does not establish API compatibility.

## Current bounded evidence — 2026-10-29

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
