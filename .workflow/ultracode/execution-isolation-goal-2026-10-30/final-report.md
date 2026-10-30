# Execution-isolation goal continuation — 2026-10-30

> **Historical snapshot:** this report predates the subsequently approved offline
> preparation. Its clean tracked diff, task counts, graph counts and assessment
> of remaining preparation describe that earlier checkpoint only. The later
> [offline preparation handoff](offline-preparation/final-report.md) supersedes
> those current-state claims; runtime gates remain open. In particular, the
> earlier statement that no useful preparation remained was corrected.

**Status: BLOCKED on runtime work; authorized local navigation and verification delivered.**
The overall goal is not complete. All 21 numbered tasks remain unchecked.

## Decisions and next input

The user confirmed **Docker images as Firecracker guest inputs**, not Docker-only
isolation. This matches the existing proposal at
`.kiro/specs/execution-isolation/decisions/UD-1-isolation.md:54-79`; it does not
identify an admitted build/test host or approve pulling, building, mounting,
exporting or booting images. No such actions ran.

Task 1.7 explicitly waits for all six gates, reconciled contracts, recorded license
review and separate implementation authorization (`tasks.md:116-121`). Each decision
file still says OPEN at line 3. The Linux/KVM Firecracker+jailer candidate approval
is checkpoint A only (`UD-1-isolation.md:18-37`). The laptop remains excluded, and
no paid-cloud fallback is authorized. Independent security review confirmed this.

**Next owner input:** identify a disposable Linux/KVM target and accountable
operator, then approve the existing checkpoint-B artifact/action manifest,
including image/rootfs/kernel identities, privileges, finite limits, observers,
cleanup and retention (`qualification/isolation-plan.md:93-120`). Alternatively,
separately authorize only the already-prepared hosted metadata workflow for an
exact repository/ref/revision; that still does not qualify a native host.

No additional generic prep was found that justifies another helper or a runtime
crate before those decisions. BG-01's recorded failed license scan is historical
evidence, not a fresh scan in this run; its policy was not changed.

## Criteria and evidence

| Criterion | Status | Evidence |
|---|---|---|
| Binding prerequisites identified | PASS | Parent source inspection and independent `security_reviewer`; cited locations above |
| Runtime task completion | MISS | Deterministic checkbox check: 21 numbered tasks, all unchecked; no `crates/clouddialect-execution/` |
| Existing offline metadata controls | PASS | `rtk proxy python3 -m unittest discover -s scripts/verification/execution-isolation -p test_hosted_preflight.py -v` → exit 0, 24 tests |
| Source/policy/approval preservation | PASS | Initial clean baseline `8ba032e662153828267c6242d9773b1b77e8bb93`; tracked diff empty; 1,649 corpus content hashes unchanged |
| Full frozen source inventory | PASS | `graphify-out/SOURCE_COVERAGE.json`: all 1,649 paths represented by final graph nodes or verified source aliases |
| Graph JSON integrity | PASS | Parent assertions: unique IDs and every final edge endpoint resolves; 4,075 nodes, 4,686 edges |
| Exhaustive semantic graph | MISS | Workers interpreted only first 66 sources; remaining documents have deterministic title/section/link indexing, not deep semantic review |
| Lossless AST graph | MISS | 639 raw unresolved edges retained in `unresolved-ast-edges.json` and omitted from navigation graph; 8 raw self-loops and 58 undirected same-endpoint collapses reported, not hidden |
| Useful task navigation | PASS | `rtk proxy graphify query 'isolation decision gates upstream docker' --budget 1700` → exit 0; output budget truncation disclosed |
| Exact task-to-packaging path | PASS | `rtk graphify path 'Implementation Plan: execution-isolation' 'Docker image packaging — proposed, not executed'` → two hops via UD-1 |
| HTML generation | PASS | `rtk graphify export html` → exit 0, `graphify-out/graph.html`; browser rendering not tested |
| Native containment, budgets, restore, signing, owner acceptance | UNVERIFIED | Not run/not supplied; synthetic tests and graph links are not substitutes |

No production behavior changed. TDD RED/GREEN and fresh code coverage are not
claimed for artifact-only work. Full Rust regressions were not rerun because no
Rust or dependency files changed. No commits or checkpoint commits were authorized.

## Graph build scope and exceptions

Full-root Graphify detection found 1,649 files (~592,000 words): 822 code/data,
826 documents and one image. User explicitly approved the full corpus after the
size warning. This run's own generated artifacts are excluded to avoid recursion.

- Local installed Graphify AST extraction visited all 822 code/data paths;
  process-pool creation was denied and the tool safely fell back to sequential.
- Three extraction workers received disjoint assignments totaling 39 batches.
  Each returned only its first batch, so full-worker completion was rejected.
- One parent repair completed file/title/section navigation for all 827 non-code
  sources and added 224 explicit Markdown references. The logo was inspected
  visually. No paid/API backend or further agents were used.
- Integration exposed one duplicate AST identity for an actual compatibility
  symlink; canonical source and aliases were retained. Same-stem inventory IDs
  were distinguished by extension, and final source coverage was rerun.
- Raw AST endpoints that cannot be resolved were not invented. Both raw and
  filtered graph-health diagnostics are preserved. Graphify also merges some
  same-source document nodes; source-path coverage, not raw ID preservation,
  was verified after that merge.
- Temporary integration command:
  `rtk proxy /home/vchu@maureva.com/.local/share/uv/tools/graphifyy/bin/python /tmp/clouddialect-graph-2026-10-30.py`.
  Inline parent Python assertions checked fragment schemas, source hashes,
  coverage, task checkboxes and endpoint integrity.
- Code extraction manifest saved; document semantic cache deliberately remains
  unstamped. Numeric token counters required by Graphify are placeholders;
  `cost.json` records UNKNOWN/null rather than invented zero semantic cost.
- No source code changed, so `graphify update .` was unnecessary after this fresh
  build. Do not infer that a later update will preserve the custom inventory;
  recheck `SOURCE_COVERAGE.json` after future extraction.

## Primary documentation check

Context7 resolve → query used `/firecracker-microvm/firecracker`; official source
was also read through web browsing. The documented Docker/rootfs recipe and KVM
prerequisite agree with the repository proposal:

- https://github.com/firecracker-microvm/firecracker/blob/main/docs/rootfs-and-kernel-setup.md
- https://github.com/firecracker-microvm/firecracker/blob/main/docs/getting-started.md

These moving documentation URLs are not selected artifact versions or qualification.
No example commands from them were executed.

## Team, ownership and usage

| Agent | Configured/requested profile | Observed limits |
|---|---|---|
| Parent | Native Codex session; model/auth/profile not independently exposed | Integration owner; inherited workspace-write scope |
| `gate_review` / `security_reviewer` | User role TOML and live catalog: `gpt-6.1-sol`, high, read-only default | Read-only review completed; effective isolation not independently attested |
| `graph_documents_1..3` / `worker` | Explicitly requested `gpt-6-luna`, medium, fresh context | Spawn succeeded; actual engine/effort not returned by tool; partial first-batch results only |

Four dispatches, no recursive agents, at most three simultaneous workers, one
parent repair. Semantic/model token usage is UNKNOWN. The reviewer supplied an
independent authorization check; extraction work was kept away from source and
native resources. The extraction route under-delivered, so no savings or full
semantic-coverage claim is made; deterministic parent integration was sufficient
for the requested whole-corpus navigation index.

## Changed artifacts and remaining risk

- Added this run directory: `plan.md`, `state.json`, `final-report.md`,
  `document-coverage.json`, `graph-health.txt`, `graph-health-raw.txt`,
  `unresolved-ast-edges.json`.
- Added `graphify-out/`: graph JSON/HTML/report, source coverage, code cache and
  manifest, extraction fragments/diagnostics, interpreter/root pointers and cost
  metadata. These are untracked outputs; `.gitignore` was not changed.
- Temporary integration helper under `/tmp`; no user infrastructure modified.
- No tracked source, spec checkbox, dependency, license policy, credential,
  runtime, Git history or external resource changed.

Remaining risk: graph semantics are partial and AST relationships incomplete;
native/security/release evidence and required owner decisions remain absent.
