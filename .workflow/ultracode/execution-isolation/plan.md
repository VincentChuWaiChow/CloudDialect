# Execution and isolation specification

## Goal
Create Spec 2 requirements, adversarial analysis, conditional design, and decision-gated tasks without implementing the runtime.

## Success criteria
- Branch `spec/execution-isolation` starts at verified remote default `master`, ad2b4bd0823c1a092e9a331c69a600ecad96bb6c.
- Canonical Kiro artifacts cover all ten requested groups, six threats, and AC-05/06/08.
- Every requirement has a verification method and evidence status MISSING.
- Five mandated unresolved decisions remain unresolved; upstream API gaps are explicit.
- Every requirement maps to design and tasks; no dependency cycles or unsafe parallel ownership.
- Independent review and deterministic checks are recorded without claiming sandbox qualification.

## Current context
2026-10-22: only contracts module stubs exist. Existing Spec 1 design/tasks disagree on some public types. Remote has master, not main. User explicitly approved using the verified default and conditional downstream artifacts. Existing untracked research.md and cloud-transformation-framework-research.pdf are preserved.

## Constraints
Own only `.kiro/specs/execution-isolation/` and this run directory. No source, Spec 1, licensing, credential, personal configuration, dependency, commit, push, or external mutation. `.codex` is read-only: eval contract stays in this existing workflow convention. Read-only Context7 and primary documentation requests contain generic public questions only.

## Risk level
Medium documentation change; high-impact future security design. No execution of untrusted worker fixtures in this run.

## Approval gates
Current user request authorizes local spec drafting and native review. The branch exception and conditional design were explicitly approved. On 2026-10-22 the user also approved the truthful six-level leaf dependency chain within three parent waves rather than unsafe parallelism. Future technology selection, contract changes, implementation, and publication need their respective approvals. Stop affected gates rather than invent answers.

## Mode
Delegated; Ultracode owns orchestration, agent-teams routing, eval-harness predeclared checks. Native collaboration tools are available. Named explorer/worker runtime identities are not exposed, so use bounded generic native agents with inherited model/effort. Client authentication, selected profile, and observed model/effort are UNVERIFIED; no separate CLI is used to claim session identity. Agents are behaviorally read-only, not sandbox-isolated.

## Work packets
01 upstream compatibility; 02 threat completeness; 03 requirements adversarial gate; 04 integrated review. Parent owns all edits and integration. No recursive delegation.

## Eval contract
- Outcome: requirements plus explicitly conditional design/tasks, not implementation-ready approval.
- Shared surfaces: spec text; parent is sole writer.
- Required checks: skill validators, stronger structural/traceability checks, existing scaffold regression, scope/diff/baseline checks.
- Blocking conditions: unresolved stakeholder choice, nonexistent upstream API, ambiguous security guarantee, or missing required evidence.
- Handoff evidence: source paths, findings, command exits, residual risks.

## Integration policy
Use repository source as implementation evidence, briefs as intent, docs as API evidence; do not equate any with runtime security proof. Fix only scoped contradictions. One consolidated review-repair cycle, not unlimited retries.

## Verification plan
Run inspected skill validators and a local stdlib-only spec grader. Verify preserved research hashes and unchanged tracked files outside owned paths. Run seven existing Python scaffold tests. Native Kiro/SMT and sandbox adversarial qualification are unavailable/not run and remain UNVERIFIED/MISSING.

## Completion criteria
All requested documentation exists, deterministic document checks pass, reviewers' blocking findings are resolved or explicitly decision-gated, no unauthorized scope changes. User approval of a conditional document is not approval of unresolved choices.
