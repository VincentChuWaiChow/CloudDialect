# Execution-isolation goal continuation — 2026-10-30

## Objective and authority
Complete the specified tasks only when their binding prerequisites and evidence exist.
Baseline: `8ba032e662153828267c6242d9773b1b77e8bb93`, clean working tree.
The current request authorizes local preparation/verification; it does not supply
the six decision approvals, native environment, external actions, or release evidence.
The user clarified Docker images are Firecracker guest inputs, not Docker-only
isolation, and explicitly requested Graphify over the full repository, including
workflow history. No host/device/Docker access, pull/build/boot, signing, installation,
commit, push, deployment, secrets access, or upstream implementation is authorized.

## Scope and route
- Parent owns integration, this run directory, and `graphify-out/` final outputs.
- One independent read-only security review of task/gate authority.
- One extraction wave of at most three lightweight workers, each owning separate
  graph fragment files. Process the document corpus in 20–25-file batches inside
  each worker; no recursive agents. This adapts per-chunk dispatch to the host's
  bounded-team policy and avoids dozens of separate agent runs.
- Code extraction is local Graphify AST parsing, not execution of repository code.
- Four total dispatches, at most three concurrent workers, one bounded repair,
  30-minute initial work limit. Stop blocked runtime work; do not manufacture tasks.
- Preserve source, specs, dependencies, policy, checkboxes, and prior evidence.

## Eval contract (before extraction or source changes)
| Criterion | Grader / pass signal | Failure owner |
|---|---|---|
| Identify binding prerequisites | Read tasks 1.1–1.7, requirements and design gates; independently review | Parent |
| Preserve authorization boundary | No runtime crate/probes, unchanged source and approvals | Parent |
| Full graph corpus | Detect full root; validate every expected fragment and source coverage before merge | Parent |
| Graph integrity | Unique node IDs, edges resolve, exact source paths; report collapse/ambiguity rather than hide it | Parent |
| Useful navigation | Query execution-isolation gate vocabulary and inspect source locations | Parent |
| Existing metadata controls | `rtk proxy python3 -m unittest discover -s scripts/verification/execution-isolation -p test_hosted_preflight.py -v` exits zero | Parent |
| Truthful task completion | No numbered task checked without its actual done criteria; no synthetic/native equivalence | Parent |

No production behavior change is planned while the gate remains open, so no RED
state or 80% coverage claim will be invented. Existing tests are regression checks.
TDD applies before any later authorized behavior change. Checkpoint commits are
not authorized. Model token usage is unknown unless supplied by native tools.

## Initial evidence
Graphify detection: 1,649 files, 592,378 words; 822 code, 826 document, one image.
No sensitive-file skips reported; no existing root graph. Largest groups:
`.workflow` 1,325; `crates` 251; `.kiro` 27; root 13; `scripts` 10.
The user explicitly chose the full corpus after the size warning.
