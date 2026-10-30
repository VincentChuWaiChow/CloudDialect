# Conformance prerequisite continuation

## Goal
Advance the full Spec2 execution-isolation objective through authorized Spec1
prerequisites; all runtime gates remain open until actual evidence/approval.
## Success criteria
Fix exact unresolvable-schema URI diagnostic; deliver phase-aware, strict,
killable5second/prefix conformance runner with actual positive/negative controls.
Full suite/check/fmt/Clippy, coverage>=80% with scope, and independent final review.
## Current context
feat/execution-isolation-runtime at5dc25f3 with previous verified275Rust/15Python
semantic/completeness/policy work uncommitted. Research files are unrelated.
## Constraints
No new dependencies/lock/model/version changes, production worker/runtime,
native sandbox/provider probes, pushes or releases. Parent alone owns shared
exports/manifests/Git. Latest user authorizes frequent thematic commits.
## Risk level
High: diagnostic disclosure and phase-aware verification tooling.
## Approval gates
User explicitly approved proposed next90minute tranche, two read-only sidecars
(30cumulative) and one known URI repair. No further review-triggered repair
presumed. Start18:44:04UTC; hard deadline20:14:04UTC; no recursion/follow-ups.
## Mode
Delegated:29completeness audit in parallel with parent URI fix;30final frozen diff.
## Work packets
Parent URI unit:src/validator/structural.rs+tests/structural_validation.rs only.
Parent3.1:Cargo.toml,tests/conformance.rs,tests/support/fixtures.rs,
tests/conformance_controls.rs only. Full fixture corpus3.2 is not smuggled into
runner ownership; control fixtures are created in temporary test directories.
Agent29:read-only completeness/model audit (20min); no shared source writes.
Agent30:read-only final integration/targeted checks (20min), after freeze.
## Eval contract
See eval-contract.md. Exact fixture metadata/runner cases will be frozen before
production runner changes; reject absent/unknown phases, outcome unions and
missing/extra diagnostic matches. No zero-test success or thread-only timeout.
## Integration policy
Parent integrates source evidence. Separate child process runs trusted validator
code on data only; no fixture scripts/commands or worker execution. No source
fix from review findings without further bounded approval.
## Verification plan
Baseline/current work checkpoint; URI RED commit→minimal fix/GREEN commit;
runner missing/incorrect behavior RED commit→implementation/GREEN commit;
full tests/lints/coverage, scope/preservation audit, final read-only review.
## Completion criteria
Only bounded prerequisite units may be checked. Full goal needs all21runtime
tasks, actual gate decisions, consumer/conformance/native evidence and remains
unfinished. Commit evidence proves local history, not push/CI/deployment.
