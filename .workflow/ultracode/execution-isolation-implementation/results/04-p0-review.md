# Result 04: P0 independent review

## Summary
`/root/p0_review` returned **FAIL** with five documentation contradictions. No file edits or runtime tests by reviewer. Parent applied one consolidated P0 repair; no independent post-repair sign-off.

## Evidence and parent disposition
| Finding | Reviewed evidence (pre-repair line numbers) | Parent correction |
|---|---|---|
| No coherent schema discriminator | task 10 selected instance `$schema`; design rejected/omitted it | P0-E explicit trusted DocumentKind for JSON entrypoints; optional declared URI must match; task/schema/diagram/API aligned. |
| Fixture outcomes incompatible with API | design expected pass/fail/warn for behavioral; task 21 used fail/invalid | Exact phase-specific union: completeness deferred/evidence-complete/blocked; artifact-hash expected_match boolean; runner checks not accepting normalization. |
| Catalog/store/overlay contradictions | design architecture/context/taxonomy and task 21 retained private records in catalog | Five application schema names corrected; project instances route to DocumentStore; snapshots replace live overlays; resolver_resources separates meta-schema support. |
| Wrong TOML semantic entrypoint | task 17 still called JSON validate_semantic for manifests | ManifestValidator::validate_manifest_semantic named consistently. |
| Numeric-only verification version pattern | verification could not represent valid prerelease/build pack versions | Full SemVer pattern, including prerelease/build and invalid numeric/empty suffix boundaries, applied consistently to documented version constraints. |

## Handoff
- Summary: review-only reconciliation defects, not a runtime vulnerability report.
- Changed surfaces: none by agent; parent repaired only authorized Spec 1 docs/run artifacts.
- Contracts satisfied: bounded independent review, concrete contradictions, gate/authority boundaries preserved.
- Assumptions: review snapshot preceded consolidated repair; user-approved draft is not owner-approved contract.
- Local checks: read-only content and one nearby Spec 2 hop.
- Integration evidence: `verify-p0.py`, `p0-check-results.json`, final source/diff/preservation checks.
- Risks: pinned engine API, dates/semantic identity/authentication/native checks remain unverified; no post-repair independent verdict.

## Files changed
None by reviewer.
## Decisions
Parent retained P0 draft/pending-owner-approval status and all security gates OPEN.
## Verification run
Reviewer inspection only. Parent subsequently ran 10 focused document tests and seven existing scaffold tests successfully; broader generic validators still fail as recorded.
## Open questions
Owner approval/revisions for P0-A–F, publication/version disposition, actual upstream implementation and runtime qualification environment.
