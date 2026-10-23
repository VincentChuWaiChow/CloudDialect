# Packet 03-review
## Objective
Read-only review integrated5fixes, adversarial edge cases and scope
## Context
Review findings in ../pr-3-code-review/results/findings.json.
## Sources
Current code and relevant approved requirements/design/tasks/amendments.
## Ownership
crates/clouddialect-contracts/No write scope
## Do
Tests first, retain actual failing/passing output, minimal surgical changes, rtk commands, offline/frozen checks.
## Do not
No dependencies/features/exports/schemas/version changes, Git actions, network, unrelated edits or delegation.
## Expected output
Files changed, RED/GREEN checks and results, decisions/risks.
## Verification
Parent independently verifies integrated source.
## Handoff format
Summary, ownership, changes, exact commands/exits, raw evidence, limits.
## Coordination rule
You are not alone in the codebase. Do not revert edits made by others. Adapt to nearby changes.
