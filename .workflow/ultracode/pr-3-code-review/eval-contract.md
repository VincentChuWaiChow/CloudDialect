# Eval contract
## Goal
Evidence-backed read-only review of PR 3.
## Success criteria
Report only concrete correctness defects introduced in delivered surfaces, with severity, location and trigger; no invented runtime assurances.
## Integration surfaces
Explicit catalog -> schema/version validation -> caller-bound resolution -> manifest fidelity; JSON raw DTOs and phase diagnostics.
## Downstream consumers
Spec 2 execution isolation depends on Spec 1 prerequisites but remains gated; no native integration exists to certify.
## Required checks
Compare frozen production diff and approved spec; parent verify findings; offline full Rust tests if cached; unchanged tracked code at end.
## Deliverables
Packet results, integration.md, final-report.md, final-audit.md, review findings in final reply.
## Blocking conditions
Head drift, missing source, prohibited resource needs or elapsed cap. Stop affected check, record unverified; do not broaden privileges.
