# Integration

## Accepted
Task 1 explicit `errors/` path; no invented public types in empty stubs.
## Rejected
Root workspace and future validator behavior.
## Conflicts
Design diagram uses `error/`, task 1 uses `errors/`; task 1 governs this scaffold.
## Decisions
Use stdlib Python scaffold graders because the Rust modules contain no executable behavior.
## Final changes
Pending.
## Verification still needed
RED/GREEN, locked Cargo checks, final review and remote PR proof.
## Remaining risks
Scaffold proves build/layout only, not JSON Schema compliance or runtime offline safety.
