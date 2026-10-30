# Eval contract

## Goal
Accurately reconcile every numbered Spec 2 task and completed supporting item
against current source and verification, without silently implementing runtime
work or bypassing prerequisites.

## Success criteria
1. TASK-001 through TASK-021 each have an explicit PASS/MISS/UNVERIFIED audit
   verdict with a source/artifact or command and the missing completion proof.
2. Any new checked item describes only the verified supporting deliverable.
   Numbered task/epic closure requires its full done criteria, not partial work.
3. UD-1 through UD-5 and BG-01 remain open unless actual required owner approvals
   and qualification are present. None may be inferred from test execution.
4. Stale current upstream counts/revisions are superseded by dated actual results;
   historical RED/GREEN/absence evidence remains explicitly historical.
5. Current local Python and frozen Rust checks, or concrete blockers, are recorded.
   Runner helper cases are not counted as independent control/security proofs.
6. Native isolation/offline observation, true cleanup/retention/backend/credential
   guarantees, Spec 2 consumer acceptance and release readiness stay unverified.

## Integration surfaces and downstream consumers
Spec 2's tasks and BG-01 handoff documentation inform future runtime planning.
Only the parent edits these shared status surfaces; both reviewers read them.
Public Rust APIs/schemas, normative requirements and approvals do not change.

## Required checks
- Inventory execution crate, all 21 leaves and six gate records; resolve checked
  supporting-item links and compare with the actual contents.
- `cargo test --frozen --offline`, `cargo check --all-targets --frozen --offline`,
  format and warning-denying Clippy on the existing contracts crate only.
- Current Python tests and release file-contract checks; actual license scan if
  its previously provisioned temporary tool remains available. A failed license
  scan remains failed; no allowances or exceptions may be added.
- Preserve the historical `check_spec.py` as-is; report its actual failure rather
  than weakening its original baseline or marking native evidence verified.
- Parent structural audit of task/status/link consistency, protected file hashes,
  exact diff scope, no credential/private account data in exported evidence.

## Deliverables
Updated checklist and current bounded BG-01 evidence, per-task audit, exact command
results, final report and state. Tests and review are evidence, not approval.

## Blocking conditions
Missing cache/tooling blocks affected checks; no network fetch/install fallback.
Missing owner/native evidence blocks runtime task completion, not this audit.
Unexpected source changes, unsupported claims or out-of-scope actions stop the
affected route. Any runtime implementation requires a separate approved re-gate.

## Planning-only lab continuation — owner requested 2026-10-29

Parent owns the existing isolation qualification/decision documentation and this
audit root only. Deliver a separately hosted, disposable candidate lab proposal;
keep the daily-use laptop protected. Verify current primary-source feasibility,
propose finite experimental ceilings distinctly from approved/measured UD-5
budgets, and list missing provider/pins/commands/privileges/spend/cleanup fields.
No cloud/account access, provisioning, installation, schedule, native probe or
spend is authorized. Grade authority fields, open tasks/gates, protected hashes,
local links and `git diff --check`; do not rerun Rust merely for unchanged-source
planning edits. One bounded read-only security review, <=5 minutes, no fan-out;
one parent repair at most. Documentation claims remain subject to that review.

No-paid-cloud revision: paid candidates/USD 5 are historical only. Grade the
zero-paid-usage constraint, protected laptop, unknown public-repository/KVM
eligibility, no new workflow/visibility/publication/dispatch authority, and the
metadata-only proposal's distinction from native admission. One read-only review
of that bounded revision, <=5 minutes, no fan-out, one parent repair at most.
