# Proposed separate Spec 1 prerequisite plan

Status: **PROPOSAL FOR OWNER REVIEW — not implementation authorization**. User authorized this plan and decision/qualification drafts on 2026-10-22; no sandbox probes are authorized. This file supplements BG-01, not Spec 1's approved task list.

## Objective and boundary
Deliver the real dependency-closed contracts/catalog/validation surface consumed by execution-isolation. Do not build fake success enums or a narrow shim that makes imports compile while schemas and semantic validation remain absent. Current evidence: only Spec 1 task 1 is implemented; its source modules are empty.

Proposed future ownership: Spec 1 owner plus core compatibility reviewer, on a separately agreed prerequisite branch/workstream. This run does not create that branch or modify Spec 1. Owned paths, if separately authorized: `.kiro/specs/contracts-evidence-foundation/` amendments, `catalog/`, `crates/clouddialect-contracts/`, named contract-support documentation and dependency/license notices where actually required. Preserve user research, execution runtime and transformation-pack code. No release/push/deploy, private-key creation, global installation or production resources.

## P0 — agree contracts before implementing
Owner-approved amendments must reconcile:
1. BehavioralResult fields (`test_results` disagreement), outcome/diagnostic mapping and representable verification linkage; evidence completeness is never acceptance.
2. Versioned capability declaration representation; optional extensions cannot self-grant authority.
3. Verification configuration/per-test failure detail and candidate/input/attempt/suite/oracle/policy identity linkage, including how independently authenticated approval is represented versus untrusted document strings.
4. Trusted offline catalog injection and override contract. Existing tasks use `CLOUDDIALECT_SCHEMA_CATALOG`; design also names `CLOUDDIALECT_CATALOG_DIR`. Resolve name, input semantics, authority and immutable identity explicitly; worker environment is not a trusted selector.
5. Public export/trait locations (`errors/` versus `error/`), semantic reference-store API and schema/index registration agreement (task 7 corrects task 5's document-type list).

Evidence: Spec 1 design result/public API/catalog sections and tasks 5/7/8/9/13–16; current source and BG-01 export probe. If required fields cannot fit existing formats, upstream owner reviews format/version changes before tests or downstream integration; this plan chooses no invented wire/schema fields.

## Proposed delivery packets and existing task mapping
These are planning groups, **not dispatch-sized implementation tickets**. Re-read amended requirements/design/tasks and split into bounded dependency-safe TDD units before execution; estimates remain unverified.

| Order | Spec 1 tasks | Deliverable / verification |
|---|---|---|
| P1 | 2–6 and 9, after P0 | Real document/catalog/extension schemas and non-interchangeable result/error types; positive/negative schema and API tests, unknown outcomes non-accepting. |
| P2 | 7 and 15, after relevant P1 | Correct catalog index and typed manifest/capability representation; paths/versions/reference shapes validated, no extension authority. |
| P3 | 8 then 10, after catalog/types | Bundled/trusted offline loader and structural validation; actual schema dialect behavior, missing/tampered catalog and unknown URI tests. No network resolution. |
| P4 | 11/12/16 then 13/17, respecting upstream graph | Version/extension/manifest and semantic/fidelity checks; real reference store, invalid references and mandatory unknown declarations blocked. |
| P5 | 14 and 18–22, fixtures developed alongside each preceding unit | Gap/completeness and staleness diagnostics; full positive/negative/adversarial/round-trip/integration fixture runner with deliberate failing-expectation controls. No independent behavioral execution or accepting conversion. |
| P6 | 23–25 plus dependency/license review from 26 | Compatibility/extension/traceability artifacts and verified dependency provenance/license inventory for delivered set. No fake signing proof; full task 26 release tooling/publication remains a separate milestone. |
| P7 | Spec 2 TASK-006, then TASK-007 only after all gates | Consumer builds against immutable delivered revision; real schema/manifest/loader compatibility and every non-accepting mapping tested; upstream/core owner approval bound to actual artifacts. |

This is substantial unfinished Spec 1 work, not a small prerequisite patch. The conservative baseline includes functional tasks 2–22 plus their contract-support documentation; any reduction must demonstrate a dependency-closed subset against actual Spec 2 consumption and receive upstream approval. Creating types alone is insufficient. This does **not** claim all Spec 1 release requirements are fulfilled by the prerequisite delivery.

## Eval/TDD and integration contract
- Define capability/regression graders per bounded unit before code; preserve failing tests that execute the intended missing/incorrect behavior. Unrelated dependency/setup failure is not valid RED.
- Create branch-local RED checkpoint after validated failure, implement minimal code, rerun the same target GREEN and checkpoint; optional refactor only while green. Historical task-1 checkpoints do not count for this new work.
- Parent owns shared manifest/lockfile/lib/schema index registration. Parallel edits only on disjoint files after frozen API agreement; no independent competing public types.
- Run locked Cargo tests/check, formatting, lint and >=80% measured coverage with an approved available tool. Missing coverage tooling or network fixtures remains UNVERIFIED, not guessed coverage.
- Retain real positive and seeded-negative conformance controls, zero-network loader evidence in an approved test environment, scope/hash review and independent integration review. A mocked unavailable network proves code-path behavior only, not observed offline containment.
- Verify current library APIs/tool versions via Context7 before implementation. Do not install/fetch tools or dependencies silently to bypass offline restrictions.

## Handoff and stop conditions
Required handoff: approved P0 amendments and exact schemas/API/catalog hashes; delivered Git SHA and crate version; locked dependency/license evidence; test/coverage/lint results and checkpoint SHAs; unresolved limitations; upstream/core approver identities and compatible Spec 2 mapping.

Stop on ambiguous formats, missing owner authority, unavailable required tooling, failed real conformance, incompatible licensing or unrepresentable non-accepting outcomes. No Spec 2 runtime activation until BG-01 **and** UD-1–UD-5 close and TASK-007 is re-gated. Separate owner approval and environment are still required for native execution/credential/budget qualification; the current authorization is drafts only.
