# Packet 03: independent integrated review

## Objective
Review new decision/qualification drafts and run artifacts for unsafe gate closure, unsupported claims and scope drift.
## Context
This is a partial preparation tranche; runtime completion must not be claimed.
## Sources
New execution-isolation decisions/qualification files, this run directory, requirements/task gate clauses and git diff.
## Ownership
Read-only. Parent owns all writes and repairs.
## Do
Cite concrete file/line findings; check proposals vs facts, missing evidence, approval fields, no invented measurements, upstream non-acceptance, native observation controls and preserved user work. Use RTK for shell.
## Do not
Edit, execute adversarial/native probes, access secrets, use external mutations or delegate recursively.
## Expected output
PASS/FAIL with bounded actionable findings and residual uncertainty.
## Verification
Read-only inspection; never infer security from document tests.
## Handoff format
Verdict; findings; evidence; scope; unverified checks.
