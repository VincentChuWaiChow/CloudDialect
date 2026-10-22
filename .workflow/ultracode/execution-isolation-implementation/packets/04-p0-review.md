# Packet 04: P0 independent review

## Objective
Review proposed Spec 1 contract reconciliation for cross-document contradictions, unsafe authority implications and missing compatibility cases.
## Context
User authorized Spec 1 documentation only, not implementation or accepted contracts. Parent owns all writes. Prior three dispatches consumed; this is the only new P0 dispatch.
## Sources
Spec 1 requirements/design/tasks and p0-contract-amendments.md; P0 eval; Spec 2 BG-01/REQ-004/008/010 within one nearby hop.
## Ownership
Read-only; no file writes.
## Do
Check exact fields/signatures/return semantics, capability deny-by-default vs metadata, schema/example consistency, protected verifier linkage, evidence completeness, trusted catalog/reference-store separation and version compatibility. Cite lines. Use RTK.
## Do not
Edit, execute untrusted/native probes, inspect credentials/config, install/fetch packages, commit, approve architecture or spawn recursively.
## Expected output
PASS/FAIL with bounded actionable findings; explicitly identify unverified claims.
## Verification
Read-only content review, no security or runtime certificate.
## Handoff format
Verdict, findings with paths/lines, cross-document consistency, risks, checks performed, changed files (none).
