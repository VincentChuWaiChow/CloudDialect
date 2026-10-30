# Result37: P0 authority source-oracle review
## Summary
No actionable P1/P2 in seven proposed local-validation profiles: five Pass, two Fail, never authority/authentication decisions.
## Evidence
Capability types71–103declare requests; x data25–28is not authority. Manifest validator222–241rejects one direct x.capabilities shadow with category semantic, path x.capabilities and exact shadow description; structural manifest dispatcher combines local diagnostic channels. Semantic.rs15–25,42–45excludes auth/authorization; verification_semantics.rs24–129checks only declared consistency, with no protected expected-run/origin/approver context. Schema verification_binding259–272and structural.rs357–389yield two required errors for missing binding.run_id/attempt_id.
## Handoff
Use meaningful nonshadowing accepted/grant claims, uniquely identified approve/deploy/delete requests, forged approver/origin strings, stale self-consistent run/attempt/timestamp. Wrong Fail expectations must reject five local Pass cases; repair shadow/binding by removing/restoring only relevant fields. No unknown-action substitution to fake prohibited-action denial.
## Files changed
None by reviewer.
## Decisions
Parent accepts exact source-derived oracles and payload recommendations; final draft review belongs38.
## Risks
Parsed claims do not prove actual grant denial, freshness enforcement, observation or authenticated approval.
## Verification run
Read-only source/spec inspection, Python in-memory prototypes, four verification JSON Schema checks: three valid and incomplete binding has exactly two required failures, no extra unevaluated-field error. Narrow ID/obligation/hash/selfref and manifest tracing.
## Open questions
None for local profiles. Rust execution/builds/final draft-file review/edits/Git/network/probes/credentials/delegation skipped.
