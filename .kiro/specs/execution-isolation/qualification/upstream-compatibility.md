# Upstream compatibility plan — TASK-006

Status: **BLOCKED** by missing implementation and owner agreement; package 0.1.0 does not establish API compatibility.

## Observed prerequisite probe
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
