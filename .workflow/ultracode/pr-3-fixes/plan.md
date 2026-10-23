# PR3 five-defect correction

## Goal
Fix R1–R5 from ../pr-3-code-review/results/findings.json with real failing regressions before minimal production changes.
## Success criteria
Indexed resources cannot be shadowed by nested schema IDs; valid catalog serialization revalidates; nested core duplicates fail without disturbing opaque extension bytes/embedded schemas; fidelity paths use numeric indices; referenced diagnostics retain a valid document identity/path/version.
## Current context
HEAD10a4717; tracked tree clean. Preserve prior untracked review run plus research files. Existing frozen isolated Cargo cache present; no new dependencies/features/exports/schema revisions.
## Constraints
No commits, pushes, PR changes, installs/fetches, probes, releases/deployment, secret reads or unrelated cleanup. Parent integrates and owns shared resources. Workers have disjoint source/test ownership, own Cargo targets, offline/frozen only. Runtime isolation/model/effort not attested.
## Risk level
High: schema admission and ambiguity boundaries.
## Approval gates
Current user explicitly authorizes fixes and skills. Bounded new repair task: two write workers plus one final read-only reviewer (3 total), one implementation and one review wave, no recursive/follow-up dispatches, one consolidated integration repair, 60-minute cap starting18:06:23UTC/deadline19:06:23UTC. This does not extend old execution-isolation implementation budget or close security gates.
## Mode
Delegated; parent handles R3 and integrations.
## Work packets
01 catalog(R1/R2);02 diagnostics(R4/R5);03 final read-only integrated review. Parent nested-duplicate regression/fix, docs lookup, full suite/format/Clippy/scope check.
## Eval contract
Full eval-contract.md.
## Integration policy
Read all diffs/results, retain RED/GREEN outputs, no blanket typed parse gate or arbitrary recursive data filtering. One bounded consolidated postreview repair only.
## Verification plan
Per-slice exact RED -> GREEN; full frozen offline Cargo tests/all-target check, fmt, Clippy-Dwarnings, Python15; independent review and baseline preservation. Coverage may use available matched tools without altering historical receipts; otherwise explicitly skipped.
## Completion criteria
All five defects closed with regressions, integrated checks pass, review integrated and scope/preservation verified; report limits and local-only status.
