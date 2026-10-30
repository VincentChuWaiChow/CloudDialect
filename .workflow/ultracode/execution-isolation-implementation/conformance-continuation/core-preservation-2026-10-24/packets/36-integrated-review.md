# Packet 36: integrated-review
## Objective
Independent bounded read-only review of integrated guard, regression controls, exact admission and scope.
## Context
Explicit two-decision approval; no runtime qualification.
## Sources
Baseline d19c5d4 vs integrated diff, this plan and35result; named harness and fixtures
## Ownership
Read-only; parent owns all writes.
## Do
Inspect exact relevant source and one-hop dependencies, report actionable evidence with paths/lines. Limit15min, deadline11:21:56UTC.
## Do not
No edits, Git mutations, network, probes, credentials, recursion or follow-up dispatches.
## Expected output
P1/P2 findings or none, supporting evidence, skipped checks.
## Verification
Static checks only for35;36 may run existing local binaries in isolated temporary paths.
## Handoff format
Summary/evidence/risks/actual checks; no invented runtime proof.
