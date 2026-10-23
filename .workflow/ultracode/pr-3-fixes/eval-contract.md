# Eval contract
## Goal
Correct5proven defects without scope or API expansion.
## Success criteria
Each finding mapped to a failing then passing repository regression. Valid original and caller-owned catalogs/opaque payloads remain supported.
## Integration surfaces
Catalog inventory -> frozen loader -> Boon; raw duplicate check -> structural phase -> resolver; resolver errors -> manifest/fidelity diagnostics.
## Downstream consumers
Current Rust crate consumers; Spec2 remains blocked with BG-01/UD gates open.
## Required checks
Targeted RED/GREEN; full Rust/Python/fmt/Clippy/check; source/hash preservation; final independent static review.
## Deliverables
Scoped source/tests plus workflow evidence, no Git actions.
## Blocking conditions
Dependency/API/policy change needed, overlapping edits or source drift, failed one allowed repair or elapsed budget. Stop affected route and report; no silent scope expansion.
