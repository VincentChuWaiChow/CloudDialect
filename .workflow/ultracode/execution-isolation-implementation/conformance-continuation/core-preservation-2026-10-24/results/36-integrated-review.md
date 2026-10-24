# Result36 — independent integrated review
## Summary
No actionable P1/P2 in integrated harness/data. Reviewer found one stale status count; parent corrected9/42to11/47 during final documentation packaging, without code changes.
## Evidence
Independently executed existing binaries:11controls,47canonical fixtures,5exact-prefix cases pass. Five temporary wrong-pass oracles all rejected with expected/actual outcome context.10canonical files match frozen draft bytes/hashes. At review snapshot only five of182protected baseline paths changed; research preserved. No production/API/schema/dependency changes.
Actual typed round-trip invokes guard at tests/support/fixtures.rs399–413; lexical/structural comparisons retained; decoded core comparison clears only extensions at416–440. Controls327–399 exercise structurally valid outcome/binding/candidate rewrites, lexical-equal bytes and structural changes. Collision wording preserves warning-only behavior and denies authentication/conflict detection. Spec1tasks20/21/22 and all21Spec2leaves/gates remain open.
## Handoff
Parent accepts review. Final status packaging additionally updates only the Spec2 progress paragraph and latest workflow reports; final parent scope audit covers this sixth protected documentation change. Reviewer snapshot is not misrepresented as post-packaging sign-off.
## Files changed
None by reviewer.
## Decisions
No source repair requested or performed. Stale count is packaging-only and now corrected.
## Risks
No native/authentication/acceptance/consumer qualification. Historical RED inspected, not independently reproduced or rebuilt.
## Verification run
Actual prebuilt controls/runner, prefix selection, five wrong-oracle controls, exact-copy/protected-hash checks and static integrated review. No Cargo builds/shared-cache/repository/Git/network/probe/credential/recursive actions.
## Open questions
None within current approved scope.
