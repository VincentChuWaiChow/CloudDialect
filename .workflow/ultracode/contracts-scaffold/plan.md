# Task 1: contracts crate scaffold

## Goal
Implement only task 1 of `.kiro/specs/contracts-evidence-foundation/tasks.md` and open a PR.

## Success criteria
- Standalone library crate with the seven exact-pinned dependencies and committed lockfile.
- Public `schema`, `validator`, `manifest`, and `errors` module stubs; empty conformance target.
- Deterministic scaffold checks demonstrate RED before implementation and GREEN afterwards.
- Locked Cargo check/test, formatting, and warning-denying Clippy pass.
- Independent scope review; remote branch SHA and PR base/head verified after publication.

## Current context
Baseline: `0b9bf5f`; active branch initially `master`, tracking `origin/master`.
Existing untracked `research.md` and `cloud-transformation-framework-research.pdf` are excluded.
Codex CLI 0.162.1 reports ChatGPT login; this does not attest the app session's profile/model.
Native agents and Context7 are exposed. Custom specialist catalog absent; use native generic agents.
Parent/worker model, effort, selected profile and token usage remain UNVERIFIED where not exposed.

## Constraints
Owned paths: new crate, `tests/test_contracts_scaffold.py`, `.gitignore`, third-party dependency notices,
task 1 checkbox, and this run directory. No later task implementation, root workspace, CI, deployment,
package publication, secret reads, credential changes, or research-file edits.
Use `errors/` as explicitly named by task 1; design's singular `error/` is a naming inconsistency.
There are no public types in empty stubs: defer type re-exports to their defining tasks, not fake types.
Edition 2024, version 0.1.0, non-publishable crate with repository license-file metadata.

## Risk level
Medium: first Rust dependency graph and external PR, but no runtime behavior.

## Approval gates
User explicitly authorized task 1 implementation and a PR, including necessary checkpoint commits/push.
Sandbox approval is separate; request it for Git metadata writes/network as required.
Stop for material API/license incompatibility, unrelated changes, failed verification or ambiguous writes.
CONTRIBUTING.md also gates substantive PR submission on an accepted contributor agreement.
An asynchronous question requests rights-owning maintainer or accepted-agreement confirmation;
continue local work but do not publish the PR until answered. Do not assume legal authority from GitHub ADMIN.

## Mode
Delegated; Ultracode is the sole coordinator. Agent-teams supplies native routing;
eval-harness defines checks before implementation; TDD supplies RED/GREEN checkpoints.

## Work packets
1. Parent: registry resolution, eval/test authoring, RED checkpoint, minimal scaffold, GREEN checkpoint.
2. Read-only sidecar: inspect task/design contradictions and dependency/scaffold risks.
3. Read-only independent review after integration; parent verifies and publishes.

## Eval contract
- Outcome: buildable library skeleton, no claimed validator behavior.
- Shared surfaces: parent alone owns source, lockfile, Git and external PR state.
- Required checks: Python structural tests; locked Cargo check/test; fmt; Clippy; diff scope; PR query.
- Blocking conditions: unavailable required checks, incompatible dependencies, review defect, wrong PR target.
- Handoff evidence: exact commands/exits, RED/GREEN commit IDs, final SHA, PR URL.
- Coverage: executable Rust coverage is not applicable to empty modules; no invented 80% claim.
- Deterministic first-attempt result only; pass@3/pass^3 not measured.

## Integration policy
Agents receive read-only behavioral instructions, not claimed runtime isolation. No agent writes,
commits, external mutations, or recursive delegation. Parent integrates source-based findings.

## Verification plan
1. Add stdlib structural tests and run before crate creation: intended missing-scaffold RED.
2. Commit RED on task branch; create crate, resolve exact versions, generate lockfile; repeat tests.
3. Run Cargo check/test/fmt/Clippy; inspect licenses and diff; independent review.
4. Commit GREEN; push only task branch; verify remote SHA; create/re-query PR against remote default.

## Completion criteria
All required local checks pass, review resolved, task 1 marked complete, PR exists with exact head/base.
Budgets: at most 3 cumulative sidecar runs (including follow-ups), 2 waves, 1 task-wide repair,
90 minutes elapsed; no hidden agent subprocesses. Replan if exceeded. Token usage unknown.
