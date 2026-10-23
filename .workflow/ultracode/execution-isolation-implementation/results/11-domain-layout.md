# Result11: domain layout analysis
## Summary
Read-only mapper recommends keeping coherent manifest/schema/errors, moving document DTOs out of validator into private documents domain, splitting behavioral_contract/evidence/verification. No speculative repositories/aggregates or newpublicnamespace.
## Evidence
Current rawmacro hardcodes private validator path; update to documents without visibility widening. Serde try_from RawFields must stay in scope. Keep Catalog include_bytes file in same depth. Preserve all root/validator/errors/manifest/schema public paths with aliases of same definitions, positive import/typeidentity checks, six compile-fail conversion negatives. Existing specs prescribe historical paths; add bounded current layout note, don't rewrite evidence.
## Handoff
Parent implemented recommended minimal crate-only mapping while scopequestionpending; no repo-wide moves.
## Files changed
None by reviewer.
## Decisions
Domain-oriented ordering, not fully modeled DDD aggregates.
## Risks
Code is deliberately unvalidated data; layout must not imply admission or behavioral acceptance.
## Verification run
Inspection only; no builds/tests/network/actions.
## Open questions
User physical scope answer pending, narrow default stated.
