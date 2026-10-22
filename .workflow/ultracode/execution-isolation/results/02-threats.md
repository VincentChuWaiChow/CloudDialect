# Result 02-threats

## Summary
Separate acceptance control from candidate-linked test execution; prevent vacuous qualification and stale/racing acceptance.

## Evidence
User brief lines 67-101, 132-134; research.md:121, 138-163, 173-176. Review read only, not runtime evidence.

## Handoff
Parent integrated into requirements before QA.

## Files changed
None by reviewer.

## Decisions
Added 8.12 positive control, 8.13 independent observations, 10.13 post-commit cancellation. Preserve credentialed-worker residual risk and five unresolved choices.

## Risks
No implemented isolation or dependency API; finite qualification cannot prove universal security.

## Verification run
Read-only source review; no runtime tests.

## Open questions
UD-1 through UD-5 and BG-01 remain open.
