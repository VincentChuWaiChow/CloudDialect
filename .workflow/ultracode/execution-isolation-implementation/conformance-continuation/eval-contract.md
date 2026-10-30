# Eval contract
## Goal
Real phase-aware conformance runner, not a production execution sandbox.
## Success criteria
Explicit trusted kind/phase; structural pass/warn/fail; semantic pass/fail;
behavioral deferred/evidence-complete/blocked; artifact expected_match boolean
without outcome. Exact diagnostics/warnings/blockers, wrong oracle detected,
empty discovery/selection failure, real directory prefix filter, per-fixture
process termination/reaping within5seconds. No code embedded in fixtures runs.
## Integration surfaces
Existing explicit-snapshot APIs, scoped synthetic DocumentStore, four runner
files and two URI repair files. No new public production API or dependency.
## Downstream consumers
All canonical fixture batches, future downstream compatibility and release gates.
## Required checks
Actual RED/GREEN with reachable Git checkpoints, full frozen test/check/fmt/
Clippy/Python,>=80% measured scoped coverage, independent frozen diff review.
## Deliverables
Runner/custom target, control tests, safe URI regression/fix, exact evidence and
honest task state. Future corpus/native qualification is not completed here.
## Blocking conditions
Unexpected semantic/model ambiguity, tool failure, exhausted30dispatch/elapsed
limits or further review repair; preserve evidence and replan rather than waive.
