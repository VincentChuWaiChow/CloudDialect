# Result35 — frozen semantic oracles and local nontrust
## Summary
Independent native reviewer35 reported no actionable P1/P2 in the five frozen pairs.
## Evidence
10/10 draft hashes and4/4 source hashes match; five oracles equal their frozen contracts. JSON Schema: five negative inputs and five positive baselines pass. Each changes one invariant: duplicate test at tests[1].test_id; unmapped obligation at tests[0].obligation_ids[0];63-character SHA256 at candidate.content_hash.value; approval-result ID/version mismatch at corresponding acceptance paths. Independently traced verification_semantics.rs24–39,64–74,102–124; repaired baselines have none. Semantic.rs15–19,42–45 explicitly non-authenticating.
## Handoff
Retain one-document two-field preservation example; clarify two separately validated A/Bclaims remain structurally valid with reserved warnings; each mandatoryacme independently gets blocking-gap. Not cross-document conflict/authenticated ownership. Mirror beside Req10.7verification; task20 remains unchecked.
## Files changed
None by reviewer.
## Decisions
Parent accepts source-backed oracle and precise approved clarification recommendation.
## Risks
Static tracing is not real Rust-runner execution or trusted policy evidence.
## Verification run
Read-only source/spec inspection, hash assertions,10JSON Schema checks and narrow independent semantic tracing.
## Open questions
None within approved scope. Rust/build/native/network/Git mutations/credential inspection/recursive delegation not performed.
