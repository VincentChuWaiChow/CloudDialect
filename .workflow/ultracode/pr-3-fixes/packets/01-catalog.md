# Packet 01-catalog
## Objective
R1/R2 indexed schema identity collision and optional date serialization
## Context
Review findings in ../pr-3-code-review/results/findings.json.
## Sources
Current code and relevant approved requirements/design/tasks/amendments.
## Ownership
crates/clouddialect-contracts/schema/catalog.rs, tests/catalog_loading.rs, tests/catalog_extension_roundtrip.rs
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
