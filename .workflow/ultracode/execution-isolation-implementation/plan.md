# Execution-isolation implementation

## Goal
Execute `.kiro/specs/execution-isolation` on `feat/execution-isolation`, honoring its dependency and owner-approval gates.

## Success criteria
- Branch exists at the verified starting commit; unrelated user files remain untouched.
- TASK-001–006 decision evidence and qualification plans identify proposals versus approved or measured facts.
- No TASK-007/runtime implementation before UD-1–UD-5 and BG-01 close with actual evidence.
- If prerequisites close, define executable acceptance tests before code, validate RED/GREEN, and create branch-local TDD checkpoints. Otherwise report precise blockers without claiming completion.
- Integrated native qualification, coverage >=80%, build/lint/regression and independent review are required for runtime completion; document checks cannot substitute.

## Current context
Baseline: `3b9cb6bd0ee82dbef3b731a35ce68fc778c4906a` on `spec/framework-specs`. Contracts crate contains only module stubs. Existing execution-isolation workflow records a completed documentation run, not runtime work; preserve it.
Pre-existing untracked files: `cloud-transformation-framework-research.pdf`, `research.md`; do not edit or stage.

## Constraints
Owned writes: this run directory; new `decisions/` and `qualification/` files under the execution-isolation spec. Runtime crate is conditional, not currently admitted. Spec 1 code/schema changes, existing specification rewrites, unrelated work, deployment, publishing and real credentials are excluded.
Filesystem: workspace-write, but `.git`, `.codex`, `.agents`, `.aws` mounted read-only. Local Git metadata writes require scoped escalation. No configuration or credential-store reads. Documentation lookup uses authorized Context7 and primary public documentation only; no proprietary source in queries.
Native agents have shared workspace tools: read-only contracts are behavioral restrictions, not proven process isolation. No security qualification relies on agent isolation.

## Risk level
High: untrusted execution, authority, secrets, durability and acceptance control.

## Approval gates
User requested new branch and end-to-end implementation with TDD. That does not fabricate mechanism approval or approve a disposable probe environment. Ask for decision records, upstream scope and qualification environment. No host-adversarial execution, packages/global settings, external mutations, push or deployment.
User subsequently approved recommendation drafts and a separately scoped Spec 1 prerequisite plan, and explicitly limited qualification to draft plans (no approved disposable environment). These are preparation approvals, not mechanism selection or Spec 1 implementation authorization.

## Mode
Delegated. Parent retains readiness decisions, Context7 IPC/Rust lookup, branch, integration, all writes and final verification. Two independent read-only sidecars supply bounded decision evidence; one final read-only review is planned.

## Work packets
1. Parent: IPC proposal, BG-01 source inventory and compile probe; integrate six gate records.
2. Isolation/credentials sidecar: primary-source comparison and adversarial plan for UD-1/UD-3.
3. Persistence/budgets sidecar: primary-source comparison and measurement plan for UD-4/UD-5.
4. Final sidecar: scope, honesty and gate-safety review of integrated artifacts.

## Eval contract
Full contract: `eval-contract.md`. Gate status must remain OPEN without approver identity, bound evidence and measurement/compatibility proof.

## Integration policy
Parent alone writes files and shared state. Maximum three cumulative agent turns (two discovery, one review), at most two concurrent workers, two waves, no recursive delegation, one consolidated repair, and 60 minutes for this gate-preparation tranche. Missing token usage remains unknown. Stop/replan before exceeding limits.

## Verification plan
Inspect all commands/scripts before execution. Run existing scaffold and documentation/grader sensitivity suites, diff checks, JSON parsing, immutable-input hashes, branch state, and real-export compile probe. Record exact outcomes; report native/security/coverage checks NOT RUN until admitted.

## Completion criteria
Only the full requested goal completes after runtime and native qualification evidence. Gate-preparation completion is partial progress, not goal completion. If decisions or upstream artifacts remain unavailable, stop dependent work and request concrete inputs.

## Authorized P0 continuation — 2026-10-22
User explicitly authorized Spec 1 documentation reconciliation only. Additional owned paths: `.kiro/specs/contracts-evidence-foundation/{requirements,design,tasks}.md` and `p0-contract-amendments.md`. Existing decision/qualification drafts remain preserved. No Rust, catalog, dependency, license, Spec 2/3 main-spec changes, commits or probes.
Parent reconciles result/outcome fields, declared capabilities, verification binding/failure records, trusted catalog source and reference-store APIs. Changes are proposals pending owner approval, not accepted contracts. Preserve unrelated requirements and stable task numbers.
P0 eval defined before these edits: `p0-eval.md`. One new bounded independent read-only review dispatch is allocated for the new authorized scope (four cumulative agents total, no follow-ups, no recursive fan-out); do not reset prior counts. One P0 consolidated repair maximum; 45-minute tranche ceiling. Prior preparation used three agents and one repair. No new implementation wave or native qualification is admitted.

## Planning-only approval continuation — 2026-10-22
User approved P0-A–F **for implementation planning**, explicitly not code/probes or BG-01 closure. Record that approval precisely; produce a bounded executable delivery plan in Spec 1 `implementation-plan.md`, preserving the canonical task IDs and remaining adoption/version/tooling gates. Owned edits: that new file, an approval note in the four P0 Spec 1 docs, and this run's planning/evidence artifacts. No Rust/catalog/dependency/license or Spec 2/3 source edits, commits, pushes, deployments or benchmark/native execution.
Parent owns planning and read-only pinned-package/source verification. Final independent read-only planning reviewer is one additional dispatch (five cumulative total; zero follow-ups); this exhausts the Ultracode five-sidecar ceiling. One consolidated planning repair maximum, 35-minute planning tranche. Prior counts are retained, not reset. Eval: `planning-eval.md`.

## First code continuation
User requested implementation and continued the first-slice proposal. Parent stated interpretation: implement prerequisite unit 1.2 only, no Spec 2 decision approval inferred. Exact pre-code eval, five owned paths and 45-minute/one-repair bounds in phase-results-eval.md. No commits/pushes/native probes. Locked cache/coverage setup isolated to /tmp; all spec/source outside five paths preserved.

## Remaining prerequisite implementation authorized
User approved remaining Spec 1 prerequisite delivery with TDD, stopping at unresolved version/dependency decisions; no commits/probes/pushes/releases. Current units 1.3–1.6, pre-code eval prerequisite-models-eval.md, 90 minutes and one consolidated repair. Parent owns shared files; two further read-only sidecars require pending explicit approval. ContentHash allocation moves into references (1.3) to avoid a circular dependency with 1.5; no crypto/version adoption.

## Initial-MVP version gate resolved
User confirmed initial project from scratch and current example versions as first MVP versions (document/catalog1.0.0, crate0.1.0). Record is user attestation, not independent registry history. Proceed schema units1.7–1.10 under catalog-schemas-eval.md; full review budget eight consumed, parent-only tests/review. No crypto/version new dependencies or native/publication actions approved.
