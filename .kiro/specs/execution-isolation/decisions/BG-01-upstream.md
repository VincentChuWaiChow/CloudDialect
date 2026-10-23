# BG-01 — actual upstream readiness

Status: **OPEN — partial dependency delivery, incomplete qualification/handoff**. TASK-006, reconciled 2026-10-23. No upstream/core compatibility acceptance or gate closure is recorded. REQ-001/004/008; DES-004/008.

## Current local delivery — not an accepted dependency revision

Baseline `5dc25f3f31ce500d17aa2322793d3a970f9b73d9` plus the current uncommitted semantic continuation is the inspected source state. The user approved crate API **0.1.0** and document/catalog **1.0.0** as initial MVP versions; that version decision is not BG-01 compatibility approval. See the [continuation report and exact source hashes](../../../../.workflow/ultracode/execution-isolation-implementation/semantic-continuation/final-report.md).

| Required surface | Current source/evidence | Remaining boundary |
|---|---|---|
| StructuralResult / SemanticResult / ValidatorError | Public, distinct non-accepting types; actual catalog-backed structural and scoped JSON/TOML semantic methods compile and are tested. | Document validity never authenticates scope, observations or acceptance. |
| BehavioralResult / BehavioralOutcome / BlockingGap | Non-accepting completeness helper delivered with explicit validator/store; invalid/valid-empty source Deferred, scoped evidence gaps, resource assumptions block every obligation. Verification absent/tests empty. | Parent-audited9test implementation, not reviewer28signoff or behavioral verification. Consumer/conformance/native handoff remains incomplete. |
| CatalogUrlLoader / CatalogSnapshot | Embedded or explicitly supplied immutable admitted bytes, closed resolver, digest identity; no ambient override. | Native network-disabled qualification and complete conformance remain outstanding. Digest identity is not signer authentication. |
| Pack manifest / capabilities / fidelity | Typed, versioned capabilities separate from extensions; local and scoped semantic validation, exact contract references, transitive evidence resolution and fidelity rules are implemented. | Declarations are not grants; no worker dispatch or project-store backend is implemented. |
| VerificationResult | Configuration and immutable artifact bindings, structured failures and approval-reference DTO/schema; semantic checks cover test-ID uniqueness, obligation membership, hash encodings, UTC/calendar and exact self-reference. | Trusted expected context/test selection, real observations, approval authenticity and artifact freshness beyond supplied bytes remain Spec 2 responsibilities. |
| Mandatory extensions | Actual semantic blockers for all five JSON kinds, including declaration-only and registry/payload authority attacks; no implemented namespace recognition in this MVP. | Full versioned adversarial fixture inventory and downstream integration remain unfinished. |
| Compatibility / extension policies | Initial-MVP compatibility, migration/EOL/version spaces and namespace/capability convention documents with7source/public-API tests. | Local document delivery is not publication, registry-owner authentication, release packaging or native qualification. |

Current frozen crate checks pass within their declared unit-test scope. Reviewer28 independently checked the frozen semantic/compatibility snapshot (36selected tests; no new actionable diff defect). The later extension convention has parent-only audit. The reviewer confirmed a pre-existing conformance blocker: the mismatched instance `$schema` diagnostic omits the URI required by Spec1task21; that defect is subsequently corrected by the URI RED/GREEN checkpoints36db142/cb36beb. Reviewer29found a separate P2 omission of direct assumption gaps alongside root mandatory blockers; task14is reopened pending bounded repair approval. The existing zero-test conformance scaffold is not a completed conformance runner or fixture corpus. No execution crate, approved native environment, full consumer qualification, immutable delivered semantic revision or owner acceptance is supplied by this work. BG-01 stays open.

## Historical source evidence — superseded absence finding
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

## Separate prerequisite handoff
The initial recommendation below led to the separately authorized [Spec 1 delivery plan](../../contracts-evidence-foundation/implementation-plan.md). P0 representation decisions and substantial implementation now exist; the historical missing-export probe must not be rerun as if it described current source. Remaining completeness, conformance and downstream units still need their actual evidence. API presence alone does not close this gate. Spec 2 must rerun real compatibility fixtures at the delivered immutable revision and re-gate affected design/tasks.

The original preparation modified no Spec 1 implementation. Subsequent Spec 1 work has separate user authorization; it is not silently absorbed into execution-isolation or permission for native/runtime work.

## Closure record required
Upstream owner identity/approval; delivered Git revision and crate version; exact API/schema/catalog identities; resolution links for all five items; successful compile/conformance reports; explicit non-accepting mappings; core owner compatibility approval. Partial source/tests/version decisions exist, but this complete closure record is not supplied. See [`../qualification/upstream-compatibility.md`](../qualification/upstream-compatibility.md).
