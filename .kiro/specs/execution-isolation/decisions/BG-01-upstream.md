# BG-01 — actual upstream readiness

Status: **OPEN — required exports absent**. TASK-006, 2026-10-22. Upstream/core approvers and accepted contract version not supplied. REQ-001/004/008; DES-004/008.

## Verified source evidence
At `3b9cb6bd0ee82dbef3b731a35ce68fc778c4906a`, `clouddialect-contracts` is version 0.1.0, edition 2024, `publish = false`, with empty `errors`, `schema`, `manifest`, `validator` modules. Package version is not an implemented-API version.

| Required surface | Actual source | Blocker |
|---|---|---|
| StructuralResult / SemanticResult | No definitions or exports | Cannot integrate real structural/semantic gates. |
| BehavioralResult / BehavioralOutcome | No definitions or exports | Upstream task 9 omits design `test_results`; deferred/completeness outcomes cannot become acceptance. |
| ValidatorError / BlockingGap | No definitions or exports | Execution diagnostics must not invent upstream variants. |
| CatalogUrlLoader | No definition or export | Offline catalog loading/trusted override not delivered. |
| Pack manifest and validation | Comment-only module; no schemas/catalog | Capability, version and reference integrity not implementable against real contracts. |

A direct `rustc` build of the actual stub library succeeded (exit 0). Importing the seven required names through task-specified modules failed with E0432 (exit 1). This verifies **absence**, not dependency/build compatibility, and is not a runtime TDD cycle. Probe and full diagnostics: [run results](../../../../.workflow/ultracode/execution-isolation-implementation/upstream-probe.json), `results/upstream-exports.rs`, `results/upstream-compile-2.txt` beside that JSON.

## Required upstream resolutions
1. Reconcile Spec 1 design result fields/outcomes with task 9 and tasks 14/22. Return agreed schemas, representative fixtures and typed APIs. Do not silently add `Passed`; `Deferred` and `EvidenceComplete` remain non-accepting.
2. Define the declared capability representation and validation/version semantics. Optional manifest extensions are untrusted metadata, never security grants.
3. Represent protected verification configuration, per-test outcomes, structured failure details and immutable candidate/suite/oracle bindings sufficiently for Spec 2 to consume without a competing document schema.
4. Define catalog override ownership and immutable identity. Worker-controlled environment/repository data cannot replace the trusted catalog; preserve offline-only resolution.
5. Resolve public export locations (`errors/` in tasks versus `error/` in diagrams, root re-exports/traits) and deliver compile-tested signatures with error mappings. No local shim around missing types.

## Proposed separate prerequisite handoff — not authorized implementation
Spec 1 owner should first reconcile the above document disagreements, identify the minimal dependency-closed set of its existing tasks (schemas/types, offline catalog, validation, manifests, fixtures), then execute those tasks with their own eval/TDD evidence. API stubs alone do not close this gate. Spec 2 must rerun real compatibility fixtures at the delivered immutable revision and re-gate affected design/tasks.

No Spec 1 source, schema, result enum, task or design was modified in this preparation. A separate prerequisite scope needs user approval; it is not silently absorbed into execution-isolation.

## Closure record required
Upstream owner identity/approval; delivered Git revision and crate version; exact API/schema/catalog identities; resolution links for all five items; successful compile/conformance reports; explicit non-accepting mappings; core owner compatibility approval. All currently MISSING. See [`../qualification/upstream-compatibility.md`](../qualification/upstream-compatibility.md).
