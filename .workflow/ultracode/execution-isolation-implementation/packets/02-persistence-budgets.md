# Packet 02: persistence and budgets

## Objective
Supply bounded UD-4/UD-5 comparison, retention/restore and reproducible measurement proposals.
## Context
No backend, retention durations, budget values or disposable environment approved. Parent owns writes.
## Sources
Execution-isolation requirements/design/tasks; official persistence/resource-accounting docs.
## Ownership
Read-only analysis, no file writes.
## Do
Compare simple local persistence with remote alternatives, authoritative ordering, crash uncertainty, project authorization, offline operation, tamper and deletion-aware restore. Specify metric units and controls without invented measurements. Use RTK for shell. One Context7 resolve/query pair allocated for SQLite; primary-source web research allowed.
## Do not
Edit, run workload/probes, install dependencies, read secrets, approve values, mutate external resources, spawn recursively or commit.
## Expected output
Compact recommendation plus missing owner decisions, retention classes, qualification matrix and measurement artifact fields.
## Verification
Primary sources; no actual backend durability or benchmark claims.
## Handoff format
Summary; evidence; proposed semantics; missing evidence; risks; checks; changed files (none).
