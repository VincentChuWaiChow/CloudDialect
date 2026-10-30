# Implementation Plan: python-oss-to-s3-pack

## Overview

Conditional, risk-first plan: close decisions/upstream contracts → implement bounded modules with paired tests → integrate, qualify and independently reproduce. **No leaf task is complete as of the 2026-10-30 implementation-admission review.** All five UDs and three BGs remain open. Completing this plan requires their actual closure evidence; a broad implementation request does not supply workload identities, independent oracle assets or service authorization.

The user approved explicit six-level leaf dependencies inside three parent scheduling waves for Spec 3. Waves are sequential containers, NOT claims that their children run simultaneously. Leaf prerequisites and arrows are authoritative. No dependency is removed to satisfy the skill's usual three-level heuristic.

Estimates are planning timeboxes, not promises. Stop an unsuccessful decision spike and report its blocker; do not invent closure. Workload-dependent corpus/contract/recipe entries are **bounded templates**: after UD-1/2/3, replace them with named per-operation/per-case slices of 2–4 hours, ≤5 files and ordinarily ≤10 tests. Preserve incoming/outgoing dependencies and revalidate the expanded graph. Their unknown number is not hidden inside one four-hour estimate.

Proposed future roots (not created here): `packs/python-oss-to-s3/` (P), `tests/packs/python-oss-to-s3/` (T), evaluator-owned `qualification/python-oss-to-s3/` (Q). Confirm placement before implementation. Q is a logical protected surface: repository permissions alone are not enforcement; Spec 2 excludes its answer assets from patch-worker/model snapshots.

## Supporting progress — 2026-10-30

- [x] Re-inspect actual Spec 1 catalog/validation/result interfaces; correct the
  stale stub description and schema inventory in requirements/design.
- [x] Independently review admission and preserve all 17 leaf prerequisites;
  the initial review found no workload/pins/corpus/environment approval or qualifying oracle.
- [x] After explicit user authorization, build the purpose-built
  [document-store candidate](../../../examples/python-oss-document-store/README.md)
  with three OSS call sites, local demo and TDD tests; obtain a bounded Python
  correctness/inventory review. [Evidence](../../../.workflow/ultracode/python-oss-to-s3-pack/results/reference-workload-2026-10-30.md).

These support TASK-001/002, **not either task's completion**. The
[admission report](../../../.workflow/ultracode/python-oss-to-s3-pack/results/admission-2026-10-30.md)
records the earlier checks and source evidence. Next inputs: final candidate
admission, owner-approved immutable revision and SDK constraints (1.1); upstream-owned applicability and authenticated runtime
handoff (1.2); designated synthetic service profile and owner approval (1.4).
Selection then enables the independent corpus review (1.3) and AI decision (1.5).
AI remains unavailable, which is not an approved no-AI closure. No 2.x module can
start under the unchanged graph before its prerequisites close.

## Tasks

- [ ] 1. Resolve scope, upstream interfaces and evaluation controls
  - [ ] 1.1 TASK-001: Select the workload and inspect exact SDK constraints
    - Scope: resolve UD-1/2 with immutable source identity/license permission, non-toy OSS use, ≥3 operation types, timed complete inventory <60 minutes, lineage and observed dependency versions. Purpose-built creation is a separately authorized fallback.
    - Files: future P/reference.md, P/scope.md. Estimate: 2–4-hour selection/inspection timebox.
    - Tests / done criteria: independent checklist passes every boundary; exact compatibility constraints and patterns documented; no opaque model-training exclusion asserted. If selection fails, retain the blocker and bounded search record.
    - Depends on: none. Owner: maintainer plus independent selection reviewer.
    - Trace: DES-001, REQ-001, REQ-008.
  - [ ] 1.2 TASK-002: Close upstream interface and full identity-binding gaps
    - Scope: obtain owner-approved BG-01/02/03 mappings and delivered conformance evidence for all five schemas, offline loader, inventory-aware applicability, SDK constraint representation, completed BehavioralResult, authenticated observations and audit linkage. Freeze proposed logical module interfaces here.
    - Files: future P/integration-contract.md, T/binding_cases.json. Upstream runtime changes remain separately owned, outside this patch scope. Estimate: 2–4-hour review timebox; external delivery wait excluded.
    - Tests / done criteria: consumed fields/errors/statuses map to actual versioned interfaces; omitted-operation and stale-binding cases pass against delivered APIs; absent APIs, incompatible test_results or ignored extension fields cannot count as closure.
    - Depends on: none. Owner: framework maintainer / integration owner.
    - Trace: DES-008, DES-006, DES-010, REQ-003, REQ-006, REQ-008, REQ-010.
  - [ ] 1.3 TASK-003: Freeze independently authored obligations, oracle and corpus protocol
    - Scope: resolve UD-3 through representativeness review, lineage separation, full manual inventory annotations, critical dependencies/unsupported patterns, functional/denial/failure expectations, positive/negative controls, metric units, output selection, deduplication, budgets and effort rules. Protect answers from the patch path.
    - Files: future Q/protocol.md, Q/corpus-index.json, Q/oracle-provenance.json, per-operation acceptance-suite slices under Q/suite/. Estimate: 2–4 hours per reviewed corpus/operation slice; expand before work.
    - Tests / done criteria: freeze predates comparison outputs; no patch-path-generated suite/oracle or exposed answers; complete reference demonstration specified; matching rules, criticality, provenance and uncertainty policy reviewed. Missing representative coverage blocks closure.
    - Depends on: 1.1. Owner: independent evaluator, not recipe/model implementer.
    - Trace: DES-003, DES-006, DES-009, REQ-002, REQ-003, REQ-006, REQ-009.
  - [ ] 1.4 TASK-004: Approve the real-service verification profile
    - Scope: resolve UD-5 with designated account/environment identities, authorized synthetic resource/operation envelope, access references, denial probes, cleanup owner and finite measured Spec 2 budget/retry/deadline requirements. This independent environment approval grants no worker access: 2.5 intersects it with frozen workload/test needs before execution. No production deletion or implicit provisioning.
    - Files: future Q/environment-profile.md, Q/environment-checklist.md. Estimate: 2–4-hour review timebox; access/provisioning requires separate authorization.
    - Tests / done criteria: authorized scope and enforceable execution-profile evidence exists; no credential values, permission escalation or unbounded SDK retries; local-only substitutes cannot close UD-5. No service calls without matching authority.
    - Depends on: none. Owner: environment owner and independent verifier maintainer.
    - Trace: DES-006, DES-010, REQ-006, REQ-007, REQ-010.
  - [ ] 1.5 TASK-005: Decide whether an AI experiment is admissible
    - Scope: resolve UD-4 as no-AI, or approve bounded cases, provider/model/settings identities, training/example exclusion evidence, allowed inputs and measurable predeclared benefit rule. Comparison waits for frozen corpus/modules.
    - Files: future P/ai-decision.md, Q/ai-comparison-plan.md if enabled. Estimate: 2-hour review timebox.
    - Tests / done criteria: opaque training provenance or no justified hypothesis yields no admitted AI profile; deterministic workflow remains mandatory; comparison uses identical obligations and charges unsuccessful effort. No model calls in this decision task.
    - Depends on: 1.1. Owner: maintainer and independent evaluator.
    - Trace: DES-005, DES-009, REQ-001, REQ-005, REQ-009.

- [ ] 2. Implement bounded modules against frozen contracts with paired tests
  - [ ] 2.1 TASK-006: Build baseline analysis and conservative unknown handling
    - Files: future P/analysis.py, T/test_analysis.py, analysis-owned fixtures. Estimate: 2–4 hours per bounded syntax/pattern slice; choose/pin LibCST or equivalent only after compatibility review.
    - Tests / done criteria: source/config/dependency/tool identities bound; core-side comparison reconciles analyzer output with the frozen manual selection inventory; missing/extra/reclassified entries block pending independent review; relevant unknowns block until discharged; no application import or leaked annotations; TP/FP/FN/unknown counts checked. Parser capability does not imply dynamic completeness.
    - Depends on: 1.1, 1.2, 1.3. Owner: analysis implementer, only analysis paths.
    - Trace: DES-002, REQ-002, REQ-010.
  - [ ] 2.2 TASK-007: Populate per-operation contracts and primary evidence
    - Files: future P/contracts/<operation>.json, P/evidence/<operation>.json, T/test_contract_<operation>.py. Estimate: 2–4 hours per selected operation; expand named slices before work.
    - Tests / done criteria: each obligation has evidence/fidelity and independent expected-outcome/test links; concrete ETag, actual expiry and metadata notes where applicable; unknown mandatory behavior blocks; actual upstream structural/semantic validation passes without being called behavioral verification.
    - Depends on: 1.1, 1.2, 1.3. Owner: contract maintainer; evaluator alone approves expected outcomes.
    - Trace: DES-003, DES-008, REQ-003, REQ-008.
  - [ ] 2.3 TASK-008: Implement deterministic code/configuration recipe slices
    - Files: future P/recipes/<operation>.py, T/test_recipe_<operation>.py, recipe-owned golden fixtures. Estimate: 2–4 hours per approved operation/pattern slice, not the unknown whole scope.
    - Tests / done criteria: frozen obligations drive edits without consuming other writers' unfinished outputs; byte-identical candidate/diff on identical inputs; baseline unchanged; stale/unsupported/partial inputs cannot pass; edit provenance complete; zero model requests; no assumed milliseconds conversion.
    - Depends on: 1.1, 1.2, 1.3. Owner: recipe implementer, never qualifying-oracle owner.
    - Trace: DES-004, REQ-004.
  - [ ] 2.4 TASK-009: Wire the authoritative manifest and validation adapter
    - Files: future P/pack.toml, P/validation.py (or agreed core-side location), T/test_validation.py. Estimate: 2–4 hours.
    - Tests / done criteria: actual Spec 1 TOML/schema/extension rules enforced; omitted operation, explicit unsupported entry, version and pattern mismatch get upstream unsupported results; missing references/APIs fail closed. Fixture documents can test the adapter, but real operation references are checked in 3.1.
    - Depends on: 1.1, 1.2, 1.3. Owner: integration implementer, only manifest/validation paths.
    - Trace: DES-001, DES-008, REQ-001, REQ-003, REQ-008, REQ-010.
  - [ ] 2.5 TASK-010: Integrate the independent verifier interface
    - Files: future P/verification.py, T/test_verification.py. Estimate: 2–4 hours per bounded binding/control slice.
    - Tests / done criteria: full binding and independent origin/control checked; workload/test grants are the intersection of actual required operations and 1.4's approved envelope; incompatible environment blocks rather than widens permission; forged/missing/skip/fail results and oracle tampering prevent acceptance; positive control defeats reject-all; local/service labels distinct. Evaluator assets remain independent and read-only.
    - Depends on: 1.2, 1.3, 1.4. Owner: verifier adapter implementer separate from recipe/model authors.
    - Trace: DES-006, REQ-006, REQ-010.
  - [ ] 2.6 TASK-011: Build policy-controlled evidence export
    - Files: future P/export.py, T/test_export.py, export-owned canary fixtures. Estimate: 2–4 hours.
    - Tests / done criteria: required links/artifacts present; substitutions detected; observed/inferred/unknown labels retained; secret/usable-URL export denied; sanitized derivatives separately identified; incomplete bundle never qualifies; failed proposals reviewable only within policy.
    - Depends on: 1.2, 1.3. Owner: export implementer, only export paths.
    - Trace: DES-007, REQ-007, REQ-010.
  - [ ] 2.7 TASK-012: Implement history-based qualification measurements
    - Files: future P/metrics.py, T/test_metrics.py, metric-owned hand-counted history fixtures. Estimate: 2–4 hours.
    - Tests / done criteria: three rates/discovery/effort/unknown counts match independent arithmetic; retries with changed attempt IDs/timestamps cannot enlarge stable output denominators; repaired outputs cannot erase events; zero denominator N/A; provisional bounds correct; post-result exclusions invalidate comparison; critical false acceptance persists. Aggregation never mutates audit history.
    - Depends on: 1.2, 1.3. Owner: reporting implementer; evaluator reviews arithmetic.
    - Trace: DES-009, REQ-002, REQ-005, REQ-006, REQ-009.
  - [ ] 2.8 TASK-013: Enforce optional AI boundary or document approved exclusion
    - Files: future P/ai_gate.py, T/test_ai_gate.py; no-AI branch adds only exclusion evidence/test in this namespace, no provider SDK. Estimate: 2–4 hours for an admitted adapter, otherwise ≤1 hour for exclusion verification.
    - Tests / done criteria: disabled/unresolved profile sends no request; explicitly admitted comparison can call only the approved provider before benefit is known, without being called qualified; authorized data only; no oracle/credential access or fallback; output untrusted; no-AI branch does not pretend a comparison occurred. Benefit is evaluated later.
    - Depends on: 1.2, 1.3, 1.5. Owner: optional adapter implementer, not oracle author.
    - Trace: DES-005, REQ-005.
  - [ ] 2.9 TASK-014: Implement sequencing and terminal-state recovery
    - Files: future P/coordinator.py (or agreed core-side location), T/test_coordinator.py, coordinator-owned fake-port fixtures. Estimate: 2–4 hours per bounded state/fault slice.
    - Tests / done criteria: gate order, terminal/intermediate states and retry classification documented; gaps never fall through; cancellation/audit loss invokes Spec 2 cleanup; new attempts retain history and remaining shared budget; no automatic cloud-write reruns. Fakes prove orchestration only, not isolation.
    - Depends on: 1.2, 1.3. Owner: sole integration coordinator owner.
    - Trace: DES-010, REQ-004, REQ-005, REQ-006, REQ-007, REQ-010.

- [ ] 3. Integrate, qualify and obtain independent reproduction evidence
  - [ ] 3.1 TASK-015: Verify the combined pack with actual upstream foundations
    - Files: future T/test_integration.py, Q/integration-report.md; corrections to module files only by owners or integration owner after parallel work ends. Estimate: 2–4 hours per bounded integration/fault slice.
    - Tests / done criteria: actual schemas/catalog, manifest references, execution boundary, completed results and audit linkage work together; selection inventory bootstraps scope before analysis and core reconciles differences before transformation; no stub/native-proof conflation; unsupported/dynamic/stale/secret/partial/cancelled cases stay non-accepted; deterministic full reference candidate produced. Interface drift loops back upstream, never bypassed locally.
    - Depends on: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9. Owner: sole integration owner with independent verifier review.
    - Trace: DES-001, DES-002, DES-003, DES-004, DES-005, DES-006, DES-007, DES-008, DES-009, DES-010, REQ-001, REQ-002, REQ-003, REQ-004, REQ-005, REQ-006, REQ-007, REQ-008, REQ-009, REQ-010.
  - [ ] 3.2 TASK-016: Run frozen qualification and record a local decision
    - Files: future Q/results/<run>/ (machine artifacts), Q/qualification-report.md; no public publication. Estimate: 2–4-hour bounded run/review slices within approved budgets.
    - Tests / done criteria: all critical discovery annotations found; independent adjudication complete; mandatory functional/denial/failure tests and real-service receipts pass; positive control plus critical negatives; successful deterministic complete reference migration; three rates/raw counts/unknowns/effort/limits exported. Critical false acceptance blocks the version despite repairs; AI without demonstrated benefit is excluded.
    - Depends on: 3.1. Owner: independent evaluator; cloud resource lease exclusive for the run.
    - Trace: DES-006, DES-009, DES-010, REQ-002, REQ-003, REQ-004, REQ-005, REQ-006, REQ-007, REQ-008, REQ-009, REQ-010.
  - [ ] 3.3 TASK-017: Independently reproduce and finalize the bounded handoff
    - Files: future P/README.md, Q/reproduction-report.md, Q/limitations.md. Estimate: 2–4-hour reproduction/review slices; account/approval wait excluded.
    - Tests / done criteria: another engineer reproduces the six-stage workflow, qualifying verdicts and counts using versioned non-secret instructions; scoped environment differences disclosed; unchanged licensing; all AC-01/02/03/04/05/10 gates evidenced. Irreproducible results remain unqualified. No commit/push/release included.
    - Depends on: 3.2. Owner: engineer independent of generation authors, with separate authorized resources or confirmed prior lease cleanup.
    - Trace: DES-001, DES-007, DES-009, REQ-001, REQ-004, REQ-005, REQ-007, REQ-009.

## Notes

- All 17 entries are unchecked. Documentation checks close no UD/BG or runtime requirement.
- Task 1.2 has external Spec 1/2 delivery prerequisites. This plan does not absorb upstream validator, sandbox, IPC, credential, audit-storage or resource-limit implementation.
- Expand bounded templates only after scope freeze. Changes to interfaces, privileges, budget or acceptance rules need affected-owner approval and graph review.
- Modules may use frozen fake ports for isolated tests, never for native service/schema/isolation claims. Task 3.1 verifies the integrated actual state.
- Every implementation task requires actual-toolchain build/lint, targeted tests, applicable regression suite, spec review and scope diff. No commands are invented for packages/toolchains that do not yet exist.
- Patch-writing owners cannot author qualifying suite/oracle assets. Developer unit fixtures are not replacement qualification evidence.
- Shared manifests/lockfiles, audit storage, exports and cloud resources have one integration owner. Serialize tasks if disjoint-file or immutable-input assumptions fail; update dependencies first.
- Changes to SDK/recipe/contract/suite/rule/relevant environment invalidate qualification outside unchanged scope; prior acceptance and disqualification evidence remains visible.

## Task Dependency Graph

Parent scheduling containers, not leaf parallel groups:

```json
{"waves":[{"id":0,"tasks":["1"]},{"id":1,"tasks":["2"]},{"id":2,"tasks":["3"]}]}
```

Leaf edges; every `Depends on` above has one matching edge:

```text
1.1 → 1.3
1.1 → 1.5
1.1 → 2.1
1.2 → 2.1
1.3 → 2.1
1.1 → 2.2
1.2 → 2.2
1.3 → 2.2
1.1 → 2.3
1.2 → 2.3
1.3 → 2.3
1.1 → 2.4
1.2 → 2.4
1.3 → 2.4
1.2 → 2.5
1.3 → 2.5
1.4 → 2.5
1.2 → 2.6
1.3 → 2.6
1.2 → 2.7
1.3 → 2.7
1.2 → 2.8
1.3 → 2.8
1.5 → 2.8
1.2 → 2.9
1.3 → 2.9
2.1 → 3.1
2.2 → 3.1
2.3 → 3.1
2.4 → 3.1
2.5 → 3.1
2.6 → 3.1
2.7 → 3.1
2.8 → 3.1
2.9 → 3.1
3.1 → 3.2
3.2 → 3.3

1.1 || 1.2 || 1.4
1.3 || 1.5
2.1 || 2.2 || 2.3 || 2.4 || 2.5 || 2.6 || 2.7 || 2.8 || 2.9
```

Maximum explicit chain: six leaf levels, e.g. selection → frozen corpus → analyzer → integration → qualification → independent reproduction. Parent barriers wait for every earlier-wave task; they never unblock a child with unmet prerequisites. Parallel groups require disjoint files and immutable inputs and are NOT a request to launch nine agents; host/session concurrency and cumulative budgets take precedence. Cloud qualification and reproduction remain serialized.

## Coverage and Release Gates

| Design element | Primary tasks |
|---|---|
| DES-001 | TASK-001, TASK-009 |
| DES-002 | TASK-006 |
| DES-003 | TASK-003, TASK-007 |
| DES-004 | TASK-008 |
| DES-005 | TASK-005, TASK-013 |
| DES-006 | TASK-002, TASK-004, TASK-010, TASK-016 |
| DES-007 | TASK-011, TASK-017 |
| DES-008 | TASK-002, TASK-007, TASK-009 |
| DES-009 | TASK-003, TASK-005, TASK-012, TASK-016 |
| DES-010 | TASK-002, TASK-004, TASK-014, TASK-015 |

Documentation completion is distinct from pack qualification. Final qualification requires actual UD/BG closure and all six AC gates, including independent reproduction. Publication is a separately authorized action, never implied by a checked task.
