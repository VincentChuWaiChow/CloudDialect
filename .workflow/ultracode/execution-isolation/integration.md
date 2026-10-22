# Integration

## Accepted
- Upstream source evidence: only module stubs exist; BG-01 must reconcile and deliver real types, capability declarations, result semantics and trusted catalog behavior.
- Threat review: independent observations, positive qualification control, run-wide offline scope, scoped credential residual risks and authoritative cancellation/commit ordering.
- Requirements QA: distinct input/output identity, cleanup initiation on every attempt-ending path and fail-closed retention absence.
- Partial final review: phase-specific grant binding, propagated into requirements 4.5, AdmissionPolicy and task 5.1 tests.

## Rejected
- Treating main as an alias without checking: remote default is master, substitution explicitly approved.
- Treating document, signature or evidence-completeness checks as proof of behavioral acceptance.
- Selecting any of the five unresolved technologies/values or changing upstream schemas to make the draft appear implementation-ready.
- Flattening real task prerequisites into unsafe parallel work solely to satisfy a depth heuristic.

## Conflicts
Spec 1 design and tasks disagree on BehavioralResult fields/outcomes, verification configuration/failure detail, capability representation and catalog overrides. No upstream files changed; this remains BG-01. User approved the actual six-level leaf graph within three parent waves on 2026-10-22.

## Decisions
Parent is sole writer/integration owner. Four worker turns consumed: three complete read-only reviews, one partial integrated review. Its handle was absent after continuation; no replacement or invented independent sign-off. Parent fallback completed review and one consolidated downstream repair, then reran all graders.

## Final changes
Four spec documents under `.kiro/specs/execution-isolation/`; planning, provenance, evals, grader/tests and reports under this run root. Application code, Spec 1, licensing, personal files and the two pre-existing research inputs unchanged. No commit, push, deployment or external mutation.

## Verification still needed
Future implementation qualification and owner approval of UD-1 through UD-5/BG-01. Native Kiro/SMT and independent post-repair integrated sign-off were not performed. These are not claimed as completed checks.

## Remaining risks
Conditional design cannot prove sandbox containment, data non-egress, credential revocation, durable ordering or restore. Selected adapters need re-sizing after decision approval. Every requirement and security gate remains MISSING implementation evidence.
