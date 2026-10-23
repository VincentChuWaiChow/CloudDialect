# Packet 01: isolation and credentials

## Objective
Supply bounded primary-source UD-1/UD-3 comparison and qualification proposals, without selecting or approving mechanisms.
## Context
Security spec is conditional; native tests and credentials are not authorized. Parent owns writes.
## Sources
Execution-isolation requirements/design/tasks; one nearby source hop; official platform/runtime documentation.
## Ownership
Read-only analysis, return findings to parent. No files owned for writes.
## Do
Compare Linux/macOS/CI, descendant cleanup, observer trust, environment/files/proxy credential exposure and effective revocation. Cite URLs and source lines; distinguish facts/inference/unverified evidence. Use RTK for shell.
## Do not
Edit, execute sandbox probes, read secrets/config, mutate external resources, spawn recursively or commit. No Context7 calls allocated; parent handles Context7 and other sidecar owns SQLite lookup.
## Expected output
Concise recommended proposal with alternatives, risks and exact qualification cases, not approval.
## Verification
Primary sources and requirement mapping; all native outcomes MISSING.
## Handoff format
Summary; evidence; decisions needed; residual risks; checks performed; files changed (none).
