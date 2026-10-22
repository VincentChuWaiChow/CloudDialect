# Packet14: requested rule location correction
## Objective
Read-only verify canonical rules now live under .agents/rules, six native entrypoints point there and no duplicate .ai rule remains.
## Context
User explicitly corrected canonical folder. Parent preserves rule bytes and changes only path bindings/current references/tests. One bounded read-only sidecar under delegate-default,14 cumulative, no recursion/followups;15minutes andone repair maximum.
## Sources
Current rule/test/links, git HEAD~1 source before relocation, changed current Spec1 reference lines.
## Ownership
No writes.
## Do
Check content equality, exact target resolution, relative/Git120000 readiness, meaningful tests and no unrelated code changes.
## Do not
No edits, source execution, network/secrets/probes/commits/push/subagents/followups. Shellcommands prefixrtk. Do not claim native client activation or fullgoal completion.
## Expected output
PASS/FAIL with required findings and limits <=5minutes.
## Verification
Static source/link inspection; parent tests and remoteproof remain separate.
## Handoff format
Summary; findings; limits.
