# Result 05: implementation planning review

## Summary
Independent read-only reviewer `/root/planning_review` returned **FAIL** with three bounded issues. Parent applied one consolidated repair. No independent post-repair sign-off was obtained; the five-sidecar ceiling is exhausted.

## Evidence
1. ManifestValidationResult/ManifestOutcome were not allocated before the manifest validator. Parent assigned both exact canonical task-9 types to unit 1.11, added the errors registration path (five owned files), and retained dependency 1.11 -> 2.7. Unit 1.2 is unchanged.
2. Cargo default libtest name filtering does not select fixture directories and can exit successfully with zero tests. Parent assigned custom `harness = false` registration to unit 3.1, a fixture-prefix parser with escaping/invalid/zero-selection failures, observed counts, deliberately empty-prefix nonzero control, and separate normal runner-control target. The canonical task-22 note now identifies this planning requirement. Cargo.toml itself was NOT edited.
3. Bare THIRD-PARTY-NOTICES.md ownership could create a crate-local duplicate. Unit 3.6 now names the repository-root notice explicitly.

## Handoff
- Summary: bounded plan repaired by parent; no code authorized or delivered.
- Changed surfaces: implementation-plan.md and a planning elaboration in canonical tasks.md only.
- Contracts satisfied: dependency allocation, test selector fail-closed plan, root notice ownership.
- Assumptions: implementation must still verify actual Rust behavior and fixture counts.
- Local checks: verify-planning.py includes checks for all three repairs.
- Integration evidence: planning-check-results.json and results/planning-final-audit.json.
- Risks: source/document checks are not actual compile, conformance, security, or native evidence.

## Files changed
Reviewer: none. All repairs performed by parent.

## Decisions
Reject zero-selected-fixture success and undeclared prerequisite types. Do not broaden first slice or close BG-01.

## Risks
Two planning containers require future expansion. Complete locked cache, formats/version adoption, native environment and real upstream delivery remain unresolved.

## Verification run
Independent review: content/dependency review only, no execution. Parent verification: seven planning tests, ten P0 document tests, seven scaffold tests, whitespace and preservation audit; exact results recorded separately. Initial planning grader found a missing explicit Target label for unit 3.6; parent added it and reran. That document-check failure is not a runtime TDD RED.

## Open questions
Does the user authorize only implementation unit 1.2, after relevant tooling preflight? Code, checkpoint commits, dependency provisioning, probes and publication are not authorized by planning approval.
