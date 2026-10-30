# BG-01 — actual upstream readiness

Status: **OPEN — partial dependency delivery, incomplete qualification/handoff**. TASK-006, reconciled 2026-10-29. No upstream/core compatibility acceptance or gate closure is recorded. REQ-001/004/008; DES-004/008.

## Offline handoff preparation — 2026-10-30, not owner-accepted

Inspected upstream revision: `8ba032e662153828267c6242d9773b1b77e8bb93`.
The standalone crate is `clouddialect-contracts` **0.1.0**, edition 2024,
Rust minimum 1.88, `publish = false`; the catalog/document format is **1.0.0**.
The approved work changes no upstream source, schema, dependency, lock or license.
These identities describe existing bytes, not a release or compatibility approval.

The [bounded handoff evidence](../qualification/upstream-compatibility.md#offline-handoff-preparation--2026-10-30)
now supplies exact schema IDs/hashes, source hashes, typed signatures, a successful
compile-only consumer, three expected compiler rejections and **44 passing
in-memory tests across seven selected targets**. No execution crate, mock runtime,
provider, Docker/KVM probe or native runner was needed. The default Cargo cache
was missing `boon` (exit 101); the recorded existing offline cache succeeded
without fetching dependencies. Full suite/conformance/coverage/license scans were
not rerun; the older results below remain dated evidence only.

### Delivered boundary and required Spec 2 disposition

| Delivered public surface | Exact source contract / consumer rule | Still not supplied |
|---|---|---|
| `Validate` / `Validator` | `validate_structural(kind, &[u8]) -> StructuralResult`; `validate_semantic(kind, &[u8], &dyn DocumentStore) -> SemanticResult`. `Validator::new(CatalogSnapshot)` fixes the catalog. Both phases are independently invoked. | Structural `Pass/Warn` cannot substitute for semantic checking; neither phase accepts a candidate. |
| `check_evidence_completeness` | `(&BehavioralContract, &str, &dyn DocumentStore, &Validator) -> BehavioralResult`. Fields: `candidate_id`, `outcome`, `verification_result`, `test_results`, `blocking_gaps`. | Completeness leaves verification absent/tests empty. `Deferred`, `EvidenceComplete`, `Blocked` are all non-accepting. No `Accepted`/`Passed` behavioral variant exists. |
| `DocumentStore` | `get(&self, id: &str, format_version: Option<&str>) -> Result<Option<StoredDocument>, ValidatorError>`. Core supplies a tenant/project-bound port. `Ok(None)` is missing; `Err` is a resolution failure. | Trait implementation does not authenticate caller/project. Contract references require an exact version; evidence-only `None` must uniquely resolve, not choose an arbitrary latest record. |
| `ManifestValidator` / capabilities | Separate local TOML and scoped semantic methods. `CapabilityRequirement` has `id`, `operation_id`, `action`, `resource`, `purpose`; the resource is an opaque logical alias. | Declared read/write/execute/network/model/propose/approve/deploy/delete actions grant nothing, including optional `x` data. No fallback from unmet mandatory capability/extension support. |
| `VerificationResult` | Required candidate, pack, toolchain, configuration, binding and tests; optional acceptance record. Binding includes tenant/project/run/attempt plus input/dependencies/suite/oracle/policy and obligation IDs. | IDs, test origins and approval strings are claims. Core must independently check current policy, protected expected bindings, observations, approvals, remaining budgets and cleanup. |
| `CatalogSnapshot` / `CatalogUrlLoader` | Embedded or explicit admitted `from_parts` bytes; exact-URI lookup; immutable content digest. No ambient loader override. | Content identity is not provenance/approval. Actual network-disabled isolation and trusted admission remain unqualified. |
| Diagnostics / extensions | `errors` and root exports preserve the same definitions; structured validator diagnostics and raw extension values stay upstream-owned. | Untrusted raw descriptions are transient-only, with bounded typed processing. Map to approved bounded categories before any persistence, log, model or export; UD-4 does not permit raw diagnostic retention. Do not invent `ValidatorError` variants, drop exact extension bytes or treat extensions as grants. |

**Proposed outcome mapping, not implemented dispatch logic:** structural or semantic
failure stops dependent admission. Behavioral `Blocked` stops progression;
`Deferred` is unevaluated; `EvidenceComplete` permits only consideration of a
separately authorized verification step, never acceptance or automatic launch.
Missing/unknown/failed/skipped/stale verification obligations prevent acceptance;
`TestOutcome::Pass` alone is insufficient. Actual acceptance still requires the
full ordered core gate in DES-008/010. No existing DTO creates that authority.

**Approval ledger:** upstream owner identity/sign-off — **MISSING**; core consumer
compatibility sign-off — **MISSING**; actual Spec 2 consumer/native conformance —
**NOT RUN**; release/license clearance — **NOT CLEARED**. This preparation does not
close BG-01 or task 1.6, and does not authorize task 1.7. Recheck identities on any
upstream change rather than relying on the package version alone.

## Historical bounded delivery — 2026-10-29, not owner-accepted

The audited immutable source is `5b3dfa6954805cfbeba3e9724803dac064ba48e6`.
The existing contracts crate passes frozen/offline tests: **291 Rust cases,
including 7 doctests, plus 101 fixture oracles**; build, formatting and
warning-denying Clippy pass. Completeness has 10 focused tests. The runner-control
target has 10 substantive controls and one subprocess helper (11 reported cases).
Actual source/tests cover non-accepting phases, real catalog/reference/hash rules,
core-data and exact extension-byte regression controls, public type identity and
source-bound traceability. Synthetic stores and helper processes prove neither
backend authorization, independent observation authenticity nor domain isolation.

The [current audit and exact commands](../../../../.workflow/ultracode/execution-isolation-status-audit-2026-10-29/final-report.md)
supersede the old uncommitted-source/count descriptions below. Fresh coverage,
complete required/native conformance, actual Spec 2 consumer qualification and
the complete upstream/core acceptance record remain unverified. The current
dependency license scan exits 4; the owner retained the seven-license policy
pending review. No new license allowance or readiness approval follows.

## Historical local delivery — 2026-10-23, superseded snapshot

All source/review/count statements in this section describe that earlier
snapshot only, not the present dependency or accepted integration evidence.

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

Current crate checks pass within their declared scope. Reviewer 28 independently checked the earlier semantic/compatibility snapshot (36 selected tests; no new actionable diff defect). The later extension convention has parent-only audit. The mismatched `$schema` URI defect was corrected by RED/GREEN checkpoints `36db142`/`cb36beb`. Reviewer 29's separate P2 direct-assumption diagnostic omission was subsequently reproduced and repaired with explicit approval in `03738e4`/`0396883`: all 10 completeness tests pass. Reviewer 30 statically reviewed the proposed repair and unchanged runner earlier; current repaired-tree execution is parent-verified. The phase-aware runner now has 9 passing controls and the exact approved seed `408af0d`; the current default Cargo suite passes 286 Rust tests plus 42 admitted conformance fixtures, with empty selection still rejected. Two bounded independent content reviews cover the admitted fixtures; this is not the full required corpus. Fresh coverage is unavailable, and no execution crate, approved native environment, full consumer qualification, immutable delivered handoff or owner acceptance is supplied by this work. BG-01 stays open.

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
