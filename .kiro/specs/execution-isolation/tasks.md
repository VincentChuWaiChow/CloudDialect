# Implementation Plan: execution-isolation

## Overview

**Conditional, decision-gated plan.** Strategy: risk-first decisions and upstream compatibility, then foundation-first independent modules, then integrated qualification. Checked supporting deliverables below record completed preparation or separately authorized Spec 1 work, not completion of their numbered task or approval of a gate. UD-1 through UD-5 and BG-01 must close with owner-approved evidence before implementation task 1.7 or any runtime work starts.

Proposed implementation home: `crates/clouddialect-execution/` (abbreviated `EXEC/` below); it is not present today and this plan does not create it. Spec 1 owns `clouddialect-contracts`; its corrections are upstream prerequisites under separate approval, not permission to implement Spec 1 in these tasks. Paths and module interfaces are conditional on approved decision records; re-gate affected design/tasks if they change.

Estimates are 2–4 engineering hours per leaf after prerequisites are available, not elapsed approval time or a delivery commitment. The sandbox/credential/persistence feasibility spikes are timeboxed investigations: unresolved results leave gates open and require a scoped follow-up plan, never an unreviewed default or an arbitrarily enlarged task. Platform/backend implementation must be re-estimated after selection; split any leaf exceeding four hours before dispatch.

## Progress reconciled — 2026-10-24

Decision recommendations and qualification plans have been drafted. Spec 1 contract prerequisites have been partially implemented and reviewed; they are tracked in the [upstream delivery plan](../contracts-evidence-foundation/implementation-plan.md), not counted as execution-runtime implementation here. No `crates/clouddialect-execution/` crate exists. All 21 numbered tasks remain open because their approval, compatibility, implementation or qualification obligations are not fully satisfied.

The checked supporting items under 1.1–1.6 identify what is actually delivered. UD-1–UD-5 and BG-01 remain **OPEN**; native qualification is **NOT RUN**, and budgets remain unmeasured. The original BG-01 missing-export probe describes an old stub revision, not the current contracts crate. The semantic continuation adds actual JSON/TOML reference validation and verification consistency. The completeness diagnostic omission found by independent review was reproduced and repaired under explicit 2026-10-24 approval; Spec 1 task 14 is checked again after its 10 focused tests and full integrated checks pass. The bounded runner and 42 admitted canonical fixtures are delivered: default Cargo tests pass 286 Rust tests plus 42 fixture oracles, and an empty fixture selector still fails. Spec1 positive/negative tasks18/19 are complete within their stated scope; the current104-case corpus is not exhaustive delivery yet, with62cases still missing. Fresh coverage and complete consumer/conformance/native handoff remain unfinished. See the [admission report](../../../.workflow/ultracode/execution-isolation-implementation/conformance-continuation/admission-2026-10-24/final-report.md). Current upstream progress does not replace owner approvals or complete any numbered runtime task.

## Tasks

- [ ] 1. Resolve decision gates and establish shared contracts
  - [ ] 1.1 TASK-001 Prepare isolation and verifier-observation decision evidence
    - Compare security boundary, Linux/macOS/CI portability, maintenance, descendant control and independent observation options. Produce a supported-profile proposal, T-01/T-04 probes and explicit residual risks. Do not select technology on behalf of owners.
    - Files: `decisions/UD-1-isolation.md`, `qualification/isolation-plan.md` under this spec directory.
    - Tests: review escape/startup/forged-runner probes and observer positive controls; any actual feasibility execution needs a separately approved disposable environment.
    - Done criteria: security/operations owners approve a mechanism/profile with cited evidence, or record UD-1 still unresolved and stop dependent implementation. A written comparison alone does not close the gate.
    - Estimate: 4h timebox. Dependencies: none; no prerequisite on other decision packets.
    - Design: DES-001, DES-002, DES-008.
    - _Requirements: REQ-001, REQ-002, REQ-005, REQ-008_
    - Completed supporting deliverable:
      - [x] Drafted the [isolation/verifier-observation comparison and residual risks](decisions/UD-1-isolation.md) and [qualification plan](qualification/isolation-plan.md). Owner selection, supported-profile approval and native evidence remain outstanding.
  - [ ] 1.2 TASK-002 Prepare versioned IPC decision evidence
    - Define proposed negotiation, framing, messages, bounded parsing/backpressure, correlation and cancellation only during this future decision task. Compare the JSON-RPC-over-stdio recommendation with requirements; do not treat JSON-RPC as transport authorization.
    - Files: `decisions/UD-2-ipc.md`, `qualification/ipc-cases.md` under this spec directory.
    - Tests: malformed/oversized/replay/duplicate/out-of-order messages, startup mismatch, late results after stop, stdout/stderr separation.
    - Done criteria: core/worker owners approve exact wire/version/error contracts and test cases, or UD-2 remains open. Reconcile selected bounds with UD-5 at task 1.7.
    - Estimate: 3h. Dependencies: none.
    - Design: DES-001, DES-006, DES-010.
    - _Requirements: REQ-001, REQ-003, REQ-006, REQ-010_
    - Completed supporting deliverable:
      - [x] Drafted the [IPC profile recommendation](decisions/UD-2-ipc.md) and [protocol qualification cases](qualification/ipc-cases.md). Exact wire/schema/error contracts, reconciled bounds, owner approval and interoperability evidence remain outstanding.
  - [ ] 1.3 TASK-003 Prepare scoped credential delivery decision evidence
    - Compare delivery options, actual authority, lifetime/revocation and authorized-recipient exfiltration risk. Identify test-resource and operation scoping; no real secret is needed in this packet.
    - Files: `decisions/UD-3-credentials.md`, `qualification/credential-cases.md` under this spec directory.
    - Tests: synthetic credential canaries, ambient credential absence, expiry/revocation, metadata/agent access, model/log/checkpoint leakage, unknown external write outcome.
    - Done criteria: security/operations owners approve scoped-use mechanism, measured lifecycle guarantees and residual risk, or leave UD-3 unresolved. No environment/file/proxy choice is preselected here.
    - Estimate: 3h. Dependencies: none.
    - Design: DES-005, DES-006, DES-010.
    - _Requirements: REQ-004, REQ-005, REQ-006, REQ-010_
    - Completed supporting deliverable:
      - [x] Drafted the [credential-delivery comparison and lifecycle proposal](decisions/UD-3-credentials.md) and [synthetic-canary qualification cases](qualification/credential-cases.md). Mechanism/provider selection, measured guarantees and owner approval remain outstanding.
  - [ ] 1.4 TASK-004 Prepare persistence, retention and restore decision evidence
    - Compare audit/project persistence, integrity, authoritative commit ordering, access control, outage handling and offline behavior. Define every data-class retention/deletion rule, backup limitations and restore policy before claiming durability.
    - Files: `decisions/UD-4-persistence.md`, `qualification/restore-cases.md` under this spec directory.
    - Tests: append timeout, missing links, partial/duplicate writes, crash around acceptance, missing retention policy, restored deleted objects, same-tenant/different-project access.
    - Done criteria: operations/security owners approve backend, ordering, data-class retention and tested-restore plan, or leave UD-4 unresolved. Physical erase claims cannot exceed evidence.
    - Estimate: 4h. Dependencies: none.
    - Design: DES-007, DES-009, DES-010.
    - _Requirements: REQ-007, REQ-009, REQ-010_
    - Completed supporting deliverable:
      - [x] Drafted the [persistence options, ordering and retention decision table](decisions/UD-4-persistence.md) and [restore qualification cases](qualification/restore-cases.md). Backend/retention decisions, tested restore evidence and owner approval remain outstanding.
  - [ ] 1.5 TASK-005 Measure a reference workload and propose finite budgets
    - Define a reproducible reference workload and approved measurement environment; record CPU, aggregate memory, elapsed wall, process, output/storage, attempts, cleanup, and enabled external-use consumption with explicit units. Account for enforcement lag and cumulative retries.
    - Files: `decisions/UD-5-budgets.md`, `qualification/budget-measurements.json` under this spec directory.
    - Tests: configured-boundary cases at below/equal/above limits; unknown accounting and restart/reset attempts. Do not launch untrusted workload probes before environment approval.
    - Done criteria: core/operations owners approve measured values/accounting semantics, or record missing measurements and keep UD-5 open. No benchmark or limit is invented to satisfy this checkbox.
    - Estimate: 4h timebox. Dependencies: none for reference planning; selected-profile qualification is reconciled at 1.7 and tested at 12.3.
    - Design: DES-003, DES-001, DES-010.
    - _Requirements: REQ-001, REQ-003, REQ-010_
    - Completed supporting deliverable:
      - [x] Drafted the [accounting/reference-workload measurement proposal](decisions/UD-5-budgets.md) and [explicit missing-measurement record](qualification/budget-measurements.json). No benchmark was executed or finite budget approved; task 1.5 remains incomplete.
  - [ ] 1.6 TASK-006 Reconcile and verify the Spec 1 dependency contract
    - Inventory delivered types/API versions; obtain upstream resolution of BehavioralResult fields/outcomes, manifest capability representation, verification configuration/structured failures and trusted catalog override. Optional extensions cannot become security grants. Do not add an invented accepted/pass variant or modify Spec 1 locally.
    - Files: `decisions/BG-01-upstream.md`, `qualification/upstream-compatibility.md` under this spec directory.
    - Tests: inspect real source and record compile-tested signatures plus schema conformance supplied by the Spec 1 owner; document that EvidenceComplete and Deferred are non-accepting.
    - Done criteria: delivered compatible dependency and upstream-owner approval are verifiable, or BG-01 remains open and implementation stops.
    - Estimate: 3h excluding upstream implementation. Dependencies: none.
    - Design: DES-004, DES-008.
    - _Requirements: REQ-001, REQ-004, REQ-008_
    - Completed supporting deliverables:
      - [x] Prepared the [initial upstream gap inventory](decisions/BG-01-upstream.md), [compatibility qualification plan](qualification/upstream-compatibility.md) and [separate Spec 1 prerequisite plan](decisions/BG-01-prerequisite-plan.md). Their stub-absence observations are historical, not current API-readiness evidence.
      - [x] Delivered scoped upstream DTO/schema/catalog/structural/manifest/hash/reference/fidelity prerequisites and review fixes under separate Spec 1 authorization: [implementation handoff](../../../.workflow/ultracode/execution-isolation-implementation/reference-fidelity-report.md), [review corrections](../../../.workflow/ultracode/pr-3-fixes/final-report.md) and [catalog-entry correction](../../../.workflow/ultracode/pr-3-catalog-entry-fix/final-report.md). JSON semantics/evidence completeness, full conformance and the Spec 2 consumer/owner handoff remain unfinished; BG-01 is not closed.
      - [x] Subsequently implemented scoped JSON semantic validation, all-five-kind mandatory blockers, verification-link consistency and transitive TOML/fidelity evidence checks; reconciled the [current upstream inventory](decisions/BG-01-upstream.md). [Semantic evidence](../../../.workflow/ultracode/execution-isolation-implementation/semantic-continuation/final-report.md) includes bounded independent semantic review and supersedes the earlier JSON-absence statement, not the still-open full conformance, native or owner-acceptance requirements. Completeness now has 10 passing focused tests, explicit invalid/empty Deferred dispositions and the approved direct-assumption diagnostic repair. The bounded runner has 9 passing controls and 42 admitted fixtures; full required corpus/fresh coverage remain incomplete. Two independent bounded fixture-content reviews found no actionable issues; Rust execution and source integration remain separately evidenced in the current admission report. Compatibility/extension policies are upstream artifacts, not runtime qualification. No execution runtime task is complete.
  - [ ] 1.7 TASK-007 Re-gate the approved design and establish the minimal compile/test foundation
    - Parent integration owner reconciles all six packets against requirements/design; no unresolved mechanism becomes a default. After separate implementation authorization, establish the standalone execution library and shared internal typed ports/identities from the approved design, with a path dependency on the delivered contracts crate.
    - Files: `EXEC/Cargo.toml`, `EXEC/Cargo.lock`, `EXEC/src/lib.rs`, `EXEC/src/model.rs`, `EXEC/tests/contracts.rs` (five-file implementation scope; affected spec amendments reviewed before this task starts).
    - Tests: compile shared signatures; reject accepting conversion from structural/semantic/completeness results; prove input/output identity distinction and semantic execution errors separate from ValidatorError. No runnable unsandboxed fallback.
    - Done criteria: all UD/BG gates explicitly approved and consistent, dependency license review recorded without changing the project model, contracts compile, tests pass, runtime remains inert. If five files or four hours are insufficient, split the approved foundation task before implementation.
    - Estimate: 4h. Dependencies: 1.1, 1.2, 1.3, 1.4, 1.5, 1.6 and separate implementation authorization.
    - Design: DES-001, DES-002, DES-003, DES-004, DES-005, DES-006, DES-007, DES-008, DES-009, DES-010.
    - _Requirements: REQ-001, REQ-002, REQ-003, REQ-004, REQ-005, REQ-006, REQ-007, REQ-008, REQ-009, REQ-010_

- [ ] 2. Supervised worker lifecycle and IPC
  - [ ] 2.1 TASK-008 Implement bounded protocol handling and lifecycle orchestration
    - Bind admitted input context before spawn; negotiate approved IPC; reject replay/malformed output; bind newly produced candidate identities; invoke whole-domain cleanup on normal exit, crash, disconnect, cancellation and exhaustion.
    - Files: `EXEC/src/supervisor.rs`, `EXEC/tests/supervisor.rs`, `EXEC/tests/fixtures/protocol-cases.json`.
    - Tests: version mismatch, oversize/duplicate/late messages, unknown output until production, descendant crash cleanup, cleanup timeout/quarantine, raw success text cannot accept.
    - Done criteria: module passes focused tests against frozen fake ports, with no direct host process launch bypassing IsolationPort; integration and native cleanup remain for task 12.
    - Estimate: 4h, re-size after UD-2. Dependencies: 1.7.
    - Design: DES-001.
    - _Requirements: REQ-001, REQ-003, REQ-010_

- [ ] 3. Selected isolation adapter
  - [ ] 3.1 TASK-009 Implement the approved execution-domain adapter
    - Implement only the UD-1 approved profile's startup/handle/filesystem/network enforcement and full descendant termination/reaping. Unsupported profiles deny launch; no new platform support is implied.
    - Files: `EXEC/src/isolation.rs`, `EXEC/tests/isolation.rs`, `EXEC/tests/fixtures/isolation-probes.rs`.
    - Tests: restricted environment/handles, path/link races, host sockets/metadata, descendant escape, read-only acceptance material, unsupported host, positive network observer control.
    - Done criteria: approved disposable native profile passes targeted containment and cleanup tests; mock-only passes are explicitly insufficient. Stop and split adapter work after selection if beyond four hours.
    - Estimate: 4h conditional slice; platform expansion needs another approved plan. Dependencies: 1.7.
    - Design: DES-002.
    - _Requirements: REQ-001, REQ-002, REQ-005, REQ-008_

- [ ] 4. Cumulative execution budget accounting
  - [ ] 4.1 TASK-010 Implement reservation, consumption and recovery accounting
    - Enforce approved finite bounds, aggregate active memory/process reservations, cumulative CPU/elapsed/external-use and retry consumption; preserve unknown accounting as a blocker.
    - Files: `EXEC/src/budget.rs`, `EXEC/tests/budget.rs`.
    - Tests: boundary reservation races, descendants/parallel attempts, no reset after restart, over-limit dispatch denial, unreconciled cleanup reservation, accounting corruption.
    - Done criteria: deterministic and generated-boundary tests pass with approved units and test clock; no unlimited defaults or retry replenishment.
    - Estimate: 3h. Dependencies: 1.7.
    - Design: DES-003.
    - _Requirements: REQ-003, REQ-010_

- [ ] 5. Admission, capability and provenance policy
  - [ ] 5.1 TASK-011 Implement external authorization and immutable admission
    - Implement grant intersection, separate propose/approve/deploy/delete categories, fresh approval checks, revocation, admitted executable/dependency identity and signer/provenance policy. Consume agreed capability representation only.
    - Files: `EXEC/src/policy.rs`, `EXEC/tests/policy.rs`, `EXEC/tests/fixtures/admission-cases.json`.
    - Tests: pre-candidate grant succeeds with bound input/context and no future digest; candidate-specific grant or approval requires frozen verification binding; unknown declarations, optional-extension escalation, tamper/substitution, disallowed signer/version, malicious valid signature, expired approval, project mismatch and missing policy.
    - Done criteria: denied cases never dispatch; approvals and signatures cannot waive isolation/verification; real Spec 1 fixtures validate consumed input.
    - Estimate: 4h. Dependencies: 1.7.
    - Design: DES-004.
    - _Requirements: REQ-004, REQ-008, REQ-009_

- [ ] 6. Scoped credential lifecycle
  - [ ] 6.1 TASK-012 Implement the approved credential-use facility
    - Use UD-3 delivery without ambient fallback; bind use to test resource/operation/project/lifetime; revoke local authority and enforce the selected effective credential expiry mechanism. Keep raw credentials out of serializable state.
    - Files: `EXEC/src/credentials.rs`, `EXEC/tests/credentials.rs`, `EXEC/tests/fixtures/credential-cases.json`.
    - Tests: no-credential worker, forbidden operation/cross-project access, expiry/revocation, retry reauthorization, inherited credential probes, synthetic secret leakage and residual-authority checks.
    - Done criteria: selected facility passes synthetic lifecycle/leak tests and documents actual revocation limits; no live cloud access or real credentials without separate authorization.
    - Estimate: 4h conditional slice. Dependencies: 1.7.
    - Design: DES-005.
    - _Requirements: REQ-004, REQ-005, REQ-010_

- [ ] 7. Injection-resistant tool and data egress guard
  - [ ] 7.1 TASK-013 Implement concrete tool and route authorization
    - Guard actual operations outside model context; enforce approved destination/data-class/retention rules; prevent provider fallback and acceptance-rule changes; emit only policy-permitted diagnostics.
    - Files: `EXEC/src/egress.rs`, `EXEC/tests/egress.rs`, `EXEC/tests/fixtures/injection-cases.json`.
    - Tests: injected code/docs/retrieval/tool output, encoded secret requests, alternate endpoints, allowed-tool wrong resource, unavailable provider, offline external call, raw diagnostic leak.
    - Done criteria: no untrusted input mutates authority or oracle; denied egress performs no external request; tests distinguish content classification from universal secret detection.
    - Estimate: 3h. Dependencies: 1.7.
    - Design: DES-006.
    - _Requirements: REQ-002, REQ-005, REQ-006_

- [ ] 8. Project artifacts, caches and retention
  - [ ] 8.1 TASK-014 Implement scoped storage and deletion-aware restore
    - Apply trusted tenant/project context to every store operation and private cache/retrieval lookup; freeze identities, verify provenance, gate persistence on retention policy, restrict restored/deleted data.
    - Files: `EXEC/src/project.rs`, `EXEC/tests/project.rs`, `EXEC/tests/fixtures/project-cases.json`.
    - Tests: equal hash/different project, forged IDs, cache poisoning, public immutable reuse exception, missing retention, expiry/live access denial, retained backup receipt and restore-after-delete.
    - Done criteria: both cross-tenant and same-tenant cross-project adversarial fixtures pass; no production/source-cloud deletion path; retention absence blocks dependent admission through port result.
    - Estimate: 4h conditional slice. Dependencies: 1.7.
    - Design: DES-007.
    - _Requirements: REQ-001, REQ-006, REQ-007, REQ-010_

- [ ] 9. BehavioralResult completion and protected verification
  - [ ] 9.1 TASK-015 Implement trusted verification control using delivered contracts
    - Consume real Spec 1 structural/semantic/results/errors without redefining schemas; bind exact candidate/suite/configuration/policy/attempt; enforce independent observation provenance; return non-accepting obligation diagnostics and acceptance intents only.
    - Files: `EXEC/src/verifier.rs`, `EXEC/tests/verifier.rs`, `EXEC/tests/fixtures/verification-cases.json`.
    - Tests: Deferred/EvidenceComplete non-acceptance, unknown/failed/skipped/stale/missing evidence, false pass/import shadowing, assertion removal, candidate substitution, unauthorized approval; include valid positive candidate.
    - Done criteria: actual contracts compile, all mapped outcomes are representable, every declared seeded critical defect blocked/rejected, valid reference path produces only an intent pending final commit; mocks alone cannot establish observation authenticity.
    - Estimate: 4h conditional slice. Dependencies: 1.7.
    - Design: DES-008.
    - _Requirements: REQ-004, REQ-006, REQ-008, REQ-009, REQ-010_

- [ ] 10. Audit integrity and durability
  - [ ] 10.1 TASK-016 Implement the approved audit persistence contract
    - Persist linked admission/transition/approval/budget records with project authorization and approved retention; detect tampering/missing links; return durability receipts and explicit write uncertainty.
    - Files: `EXEC/src/audit.rs`, `EXEC/tests/audit.rs`, `EXEC/tests/fixtures/audit-cases.json`.
    - Tests: append failure/timeout, missing links, duplicate identity, forged worker records, restore corruption, project access, secrets excluded, expired content reference still honestly unavailable.
    - Done criteria: approved backend's real persistence tests pass; failure never produces a durability receipt; emergency cleanup can proceed without waiting on this port.
    - Estimate: 4h conditional slice. Dependencies: 1.7.
    - Design: DES-009.
    - _Requirements: REQ-005, REQ-007, REQ-009, REQ-010_

- [ ] 11. Checkpoints and authoritative terminal state
  - [ ] 11.1 TASK-017 Implement ordered cancellation, recovery and acceptance commitment
    - Implement state transition table/CP-0–CP-4, current-policy/accounting/effect checks, immutable terminal attempts, and acceptance ordering. Document transient versus permanent failures; no blanket retry or replay of uncertain writes.
    - Files: `EXEC/src/recovery.rs`, `EXEC/tests/recovery.rs`, `EXEC/tests/fixtures/checkpoint-cases.json`.
    - Tests: every permitted/forbidden transition, cancellation-before/after-commit, unknown cleanup/effects, corrupt checkpoint, late old-attempt output, retry budget preservation, duplicate commit readback.
    - Done criteria: model/state tests demonstrate safe continuation or terminal blocker at each boundary; final ordering/durability must still pass real integrated fault injection in 12.2.
    - Estimate: 4h. Dependencies: 1.7.
    - Design: DES-010.
    - _Requirements: REQ-001, REQ-003, REQ-005, REQ-008, REQ-009, REQ-010_

- [ ] 12. Integrate and qualify the selected execution profile
  - [ ] 12.1 TASK-018 Wire the real modules and verify end-to-end authority boundaries
    - Parent integration owner alone updates shared registry/manifest/lock surfaces. Wire real ports; run admitted input → produced candidate → independent verification → approval → durable commit, plus no-approval/invalid-control paths. Preserve actual Spec 1 compatibility.
    - Files: `EXEC/src/lib.rs`, `EXEC/src/coordinator.rs`, `EXEC/tests/integrated.rs`, `EXEC/Cargo.toml`, `EXEC/Cargo.lock`.
    - Tests: valid candidate accepted, invalid candidate blocked, missing decision/retention/control denies launch, structural success never accepts, actual worker/credential/verification cleanup gates commit.
    - Done criteria: whole crate builds/lints/tests; no production operations; real selected adapters—not only mocks—complete the positive path and reject unsafe authority paths.
    - Estimate: 4h. Dependencies: 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 8.1, 9.1, 10.1, 11.1.
    - Design: DES-001, DES-002, DES-003, DES-004, DES-005, DES-006, DES-007, DES-008, DES-009, DES-010.
    - _Requirements: REQ-001, REQ-002, REQ-003, REQ-004, REQ-005, REQ-006, REQ-007, REQ-008, REQ-009, REQ-010_
  - [ ] 12.2 TASK-019 Qualify AC-05 and AC-08 under adversarial failure injection
    - Execute protected-oracle/forged-result/stale-candidate cases and all CP-0–CP-4 before/after failures, including allocation, credential use, external effects, audit write and acceptance race. Record suite and executable identities.
    - Files: `EXEC/tests/acceptance_adversarial.rs`, `EXEC/tests/interruption.rs`, `qualification/ac-05-ac-08.md` under this spec directory for the report.
    - Tests: every seeded critical defect rejected/blocked plus a valid accepted reference; every declared checkpoint yields valid resumption or explicit terminal blocker; no stale attempt pass reuse.
    - Done criteria: real integrated-profile evidence satisfies AC-05/08 scope or gates remain MISSING/failed; distinguish unavailable observer evidence from a pass.
    - Estimate: 4h per approved bounded fixture slice. Dependencies: 12.1.
    - Design: DES-001, DES-003, DES-005, DES-008, DES-009, DES-010.
    - _Requirements: REQ-001, REQ-003, REQ-004, REQ-005, REQ-006, REQ-008, REQ-009, REQ-010_
  - [ ] 12.3 TASK-020 Qualify AC-06 and measured limits on the selected profile
    - Run T-01–T-06 containment/access cases including malicious signed packs, startup/build scripts, host files/sockets, cross-project cache/index/export and run-serving core egress. Capture offline traffic with tested positive observer controls.
    - Files: `EXEC/tests/containment_adversarial.rs`, `EXEC/tests/offline.rs`, `qualification/ac-06-profile.md` under this spec directory for the report.
    - Tests: zero forbidden accesses and zero run-attributable outbound traffic offline; approved budget boundary/overshoot checks; absent retained-data policy and deleted-object restore cannot expose data.
    - Done criteria: supported-profile report states observer coverage, measured limits, environment identities and residual risks; no extrapolation to untested Linux/macOS/CI profiles.
    - Estimate: 4h per approved bounded fixture slice. Dependencies: 12.1.
    - Design: DES-001, DES-002, DES-003, DES-004, DES-005, DES-006, DES-007.
    - _Requirements: REQ-001, REQ-002, REQ-003, REQ-004, REQ-005, REQ-006, REQ-007, REQ-009_
  - [ ] 12.4 TASK-021 Verify restore/runbooks and produce release-readiness evidence
    - Drill audit/project restore, quarantine cleanup, cancellation and uncertain-commit recovery in a disposable environment. Link actual signed-build evidence, dependency inventory/provenance, supported-profile qualification and named compatibility ownership. This task does not publish a release.
    - Files: `docs/execution-isolation-runbook.md`, `qualification/restore-report.md`, `qualification/release-readiness.md` (qualification reports under this spec directory).
    - Tests: restored deletion restrictions and audit chain checked before access; signatures/inventory/provenance match tested executable identities; runbook steps produce recorded outcomes.
    - Done criteria: AC reports, restore evidence, ownership and license/dependency review are complete and reproducible; unavailable signing/restore evidence remains explicit blocker. No publication/deployment without separate authorization.
    - Estimate: 4h. Dependencies: 12.2, 12.3.
    - Design: DES-004, DES-007, DES-009, DES-010.
    - _Requirements: REQ-004, REQ-007, REQ-009, REQ-010_

## Notes

- Parent epics are scheduling containers, not four-hour tasks. Leaf tasks are the bounded units; a security adapter's size cannot be known until its mechanism is selected. Re-estimate and split before dispatch, not after quietly overrunning.
- Only three **parent scheduling levels** are used. Leaf prerequisites inside foundation and integration epics are explicit below; the actual end-to-end leaf chain has six levels. The user explicitly approved retaining these dependencies within three parent waves on 2026-10-22. This is an approved exception to the skill's depth heuristic, not permission to run a dependent task early.
- Wave 1 module tasks own disjoint source/test/fixture files and read frozen `model.rs`. They do not edit shared `lib.rs`, manifest, lockfile or shared fixtures. Their focused integration-test targets can include their own module source against the frozen shared ports; task 12.1 registers and tests the real integrated library. No mock-only qualification is claimed.
- Foundation decisions own disjoint decision/report files. Task 1.7 waits for their actual approvals and reconciles interactions; a pending review is not completion.
- All future build/test invocations use the approved environment, isolated build output directories and reviewed commands. Proposed integration commands once the crate exists: `rtk cargo test --manifest-path crates/clouddialect-execution/Cargo.toml --locked`, corresponding `cargo fmt --check` and `cargo clippy -- -D warnings`. Offline use requires pre-provisioned approved dependencies; no automatic fetching to bypass offline policy.
- Every leaf's verification includes focused tests, current full crate regression, build/lint, scope diff, linked requirement reread and design compliance. Platform qualification requires real evidence beyond local mocks. Do not mark tasks complete merely because a command exits zero.
- Native host/adversarial tests need dedicated per-task sandboxes, network observers and synthetic resource namespaces. Shared host policy, backend stores, credentials, ports or cloud resources serialize even when source paths are disjoint. No worker may commit, push, publish or deploy without separate authorization.
- Baseline existing scaffold regression: `rtk proxy python3 -m unittest discover -s tests -p test_contracts_scaffold.py -v`. Documentation-only checks do not pass security gates.

## Task Dependency Graph

Kiro canonical waves reference **parent** IDs; each parent completes only when all its children meet their done criteria. Three parent scheduling levels, no circular dependencies:

```json
{
  "waves": [
    { "id": 0, "tasks": ["1"] },
    { "id": 1, "tasks": ["2", "3", "4", "5", "6", "7", "8", "9", "10", "11"] },
    { "id": 2, "tasks": ["12"] }
  ]
}
```

Explicit leaf prerequisites (all listed edges are binding):

```text
1.1 → 1.7
1.2 → 1.7
1.3 → 1.7
1.4 → 1.7
1.5 → 1.7
1.6 → 1.7
1.7 → 2.1
1.7 → 3.1
1.7 → 4.1
1.7 → 5.1
1.7 → 6.1
1.7 → 7.1
1.7 → 8.1
1.7 → 9.1
1.7 → 10.1
1.7 → 11.1
2.1 → 12.1
3.1 → 12.1
4.1 → 12.1
5.1 → 12.1
6.1 → 12.1
7.1 → 12.1
8.1 → 12.1
9.1 → 12.1
10.1 → 12.1
11.1 → 12.1
12.1 → 12.2
12.1 → 12.3
12.2 → 12.4
12.3 → 12.4
```

Parallel opportunities after prerequisites: `1.1 || 1.2 || 1.3 || 1.4 || 1.5 || 1.6` for disjoint decision research; `2.1 || 3.1 || 4.1 || 5.1 || 6.1 || 7.1 || 8.1 || 9.1 || 10.1 || 11.1` for disjoint module work and isolated test resources. `12.2 || 12.3` only with separate qualified test environments and observers; otherwise serialize. Source-safe parallel opportunities exceed 30% of leaves, but no concurrency benefit is claimed before effective resource isolation is verified.
