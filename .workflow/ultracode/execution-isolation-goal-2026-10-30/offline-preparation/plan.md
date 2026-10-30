# Approved offline preparation — 2026-10-30

Ultracode is the sole coordinator; agent-teams supplies routing, eval-harness the
checks, and TDD the RED/GREEN discipline for new artifact checks. The autonomous
harness is bounded to this local run: no schedules, remote dispatch or memory writes.

## Scope and ownership

User approved the four proposed offline workstreams, not decision closure.
Baseline HEAD: `8ba032e662153828267c6242d9773b1b77e8bb93`.
Pre-existing untracked work: this parent run directory; preserve it.

- IPC worker: only `decisions/UD-2-ipc.md`, `qualification/ipc-cases.md`.
- Credential worker: only `decisions/UD-3-credentials.md`, `qualification/credential-cases.md`.
- Persistence worker: only `decisions/UD-4-persistence.md`, `qualification/restore-cases.md`.
- Parent: BG-01/upstream-compatibility pair, supporting tasks checklist, scoped
  artifact checks under `scripts/verification/execution-isolation/`, run evidence.
  Document paths above are beneath `.kiro/specs/execution-isolation/`.
- Independent security reviewer: read-only integrated diff and evidence.

No runtime crate, upstream source edits, dependencies/locks/licenses changes,
Docker/KVM/host probing, real secrets, image downloads/builds, provider/native
tests, commits, push, deployment or decision approval. No simulated qualification.

## Limits and routing

One implementation wave, three disjoint writers, no fan-out; one read-only review.
At most five dispatches including one shared repair allowance. Writers use the
live built-in worker role with explicitly requested gpt-6.1-sol/high on fresh
contexts (skill-authorized routing); actual backend identity is UNVERIFIED.
Reviewer: configured security_reviewer, gpt-6.1-sol/high, read-only default;
effective per-agent enforcement is UNVERIFIED, not an isolation claim.
Writers stop after 15 minutes; entire packet timebox 45 minutes. Parent owns
shared files and integration. No additional waves without approval.

## Eval contract (defined before edits)

| Criterion | Grader / expected signal | Failure owner |
|---|---|---|
| Exact IPC proposal | Closed JSON schemas, standard/application error distinction, direction/state/cancellation table and concrete valid/invalid examples; local Draft 2020-12 schema checks pass using already-installed jsonschema, without fetch/install | IPC worker, parent |
| Concrete credential proposal | Exact synthetic operations/resources, no ambient grants, cross-project/expiry/revocation/retry/cancellation canaries with expected dispatch counts; bounded artifact review, not broker execution | Credential worker |
| Complete retention proposal | Nine classes with proposed triggers/durations/deletion/backup rules; ordering/lost-ack/restore/deleted-object scenarios, no durability claims | Persistence worker |
| Real upstream evidence | Immutable revision, exact source/schema/catalog hashes, real signatures, compile-only consumer and selected pure in-memory tests using cached locked dependencies, no full subprocess/native suite | Parent |
| Regressions / truthful scope | Existing document and hosted-preflight synthetic suites; numbered tasks and all gates remain open; no protected-source/lock changes; diff check | Parent |
| Independent review | Scoped static review finds no unresolved actionable mismatch; fix confirmed misses only | Reviewer, parent |

New fixture tests first fail on absent proposal artifacts; this is artifact-level
RED, not a runtime implementation test. Human review covers semantic guarantees
which schema validation cannot establish. Rust compile compatibility is not
consumer runtime qualification. Preserve historical evidence as dated history.

All criteria start UNVERIFIED. Final report records exact commands, exits,
PASS/MISS/UNVERIFIED, changed files, remaining approvals and qualification.
