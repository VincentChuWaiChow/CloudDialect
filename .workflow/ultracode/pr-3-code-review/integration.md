# Integration

## Accepted
Five P2 defects R1–R5 independently reproduced against the frozen production library. Nested-$id finding is limited to collisions with independently indexed material, not legitimate resource bundling. Duplicate finding is limited to structural/typed-parser disagreement, not an unproven acceptance bypass.
## Rejected
No findings for known partial semantic/native/conformance scope or documented historical grader failures. Large-exponent optional raw JSON lead omitted because the supported numeric-domain policy is not explicit.
## Conflicts
None among reviewers; severities calibrated to actual trust boundary and consequences.
## Decisions
Review only. No implementation repair, commits, pushes, PR comments/reviews or merge actions.
## Final changes
Only new local review artifacts; production/lock/spec/rules untouched.
## Verification still needed
Later fixes need tests that assert corrected behavior. Current adversarial probes deliberately assert observed incorrect behavior; their passing result is reproduction evidence, not correction.
## Remaining risks
No native/runtime/provider qualification, MSRV check, fresh coverage measurement, full documentation graders or live CI verification performed.
