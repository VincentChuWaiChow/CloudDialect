# Execution/isolation checklist reconciliation — 2026-10-29

**Local-only delivery before publication approval:** after explicit owner approval, created
`.github/workflows/isolation-preflight.yml` and
`scripts/verification/execution-isolation/test_hosted_preflight.py`. The relevant
missing-workflow RED check exited 1; the completed draft passes 24 synthetic
offline tests and independent YAML/inline-Python parsing. Tests use fake host,
signal and stdout ports only. A static security review's loader/deadline-scope
findings were corrected by the parent, without claiming an independent reread.
No live runner, laptop metadata probe, KVM opening, installation, publication,
commit/push, default-branch change, dispatch, workload or spend occurred. All 21
numbered tasks and six gates remain open. Command evidence, hashes, review scope
and skipped checks are in `state.json` under `local_draft_verification`.
Required AST-only Graphify updates completed; newly generated untracked graph
outputs were retained under `/tmp/clouddialect-preflight-ast-prgq4zuu/graphify-out`
to keep the implementation change to the two authorized draft files.

**Current publication checkpoint:** the owner subsequently requested **"git commit
and git push everything"**. This authorizes the reviewed current worktree on its
existing branch, not workflow dispatch, default-branch merge, spend or native
qualification. This snapshot is prepared before publication; final commit/push
success must come from the actual Git command and remote-ref readback. The failed
license gate and all open runtime tasks remain accurately open.

**Current continuation:** the user requested continued work toward runtime
implementation through explicit gate approvals. The first candidate/environment
review request is now in [UD-1](../../../.kiro/specs/execution-isolation/decisions/UD-1-isolation.md#first-owner-checkpoint--proposed-2026-10-29)
and its [qualification plan](../../../.kiro/specs/execution-isolation/qualification/isolation-plan.md#first-feasibility-slice--approval-request).
The owner subsequently approved checkpoint A's Linux/KVM Firecracker+jailer
candidate direction on 2026-10-29 and asked about Docker image packaging. A
Docker-derived guest filesystem is now documented as a proposed preparation
path, not an approved image/build/Docker host or a replacement security boundary.
No disposable target, native action or gate closure has been approved.
The owner then identified a Linux local laptop and approved a limited read-only
metadata preflight. Docker CLI presence and KVM node visibility outside the
restricted sandbox are recorded in `results/laptop-readonly-preflight-2026-10-29.json`.
That metadata preflight did not query Docker or open KVM. A later separately
approved bounded Docker inspection confirmed a local Unix endpoint, but the
version query returned access denied and stopped without retry/fallback. Its
record is `results/docker-readonly-preflight-2026-10-29.json`. The owner then
separately approved one outside-sandbox retry with the same bounds; its three
commands exited 0 and the server reported **29.8.1 / linux / amd64**. See
`results/docker-outside-sandbox-readonly-retry-2026-10-29.json`. This retry is
consumed, not ongoing Docker authority. No device opening, image operation or
native test occurred; none of these observations qualifies the laptop or a
disposable lab. A bounded read-only security review confirmed no execution
implementation may start under the current gates. The owner then confirmed
**"Daily-use laptop—protect it"**. The laptop is not disposable and is excluded
from native/adversarial testing. A separate disposable target and its operator
are now required; no local VM/container workaround, provisioning or spend is
authorized. Existing metadata observations do not admit another target.
With no existing target available, the owner authorized planning only. The
[lab proposal](../../../.kiro/specs/execution-isolation/qualification/isolation-plan.md#separate-disposable-lab-proposal--planning-only-2026-10-29)
now compares two unselected, documented nested-KVM cloud candidates, proposes
finite experimental ceilings and explicit teardown/evidence rules, and lists the
missing command/artifact/provider/cost approvals. The proposed USD 5 ceiling is
not a quote, payment authority or enforceable budget. No account access, resource
creation, installation, schedule or native execution occurred. No new numbered
task or supporting checkbox was marked complete from this proposal. One static
security review returned a CPU-unit ambiguity; the parent corrected the row to
CPU bandwidth/quota and explicitly retained cumulative CPU-time accounting and
enforcement as missing B fields. No independent post-correction reread is claimed.
See `results/lab-plan-review.md` and `results/lab-plan-checks.json` for the bounded
review and parent structural checks. Rust was not rerun for these source-unchanged
planning edits; the prior executed audit results remain dated evidence.
The audit status below records the completed reconciliation and its pre-resume
blocker; the continuation does not retroactively supply the missing evidence.

Current goal tool status: **BLOCKED; required environment/action approvals remain missing**.
The owner has now answered **"No paid cloud"**. The paid AWS/GCP proposal and USD 5
ceiling are historical and excluded. Planning continues with a conditional
[free public-repository hosted-runner candidate](../../../.kiro/specs/execution-isolation/qualification/isolation-plan.md#no-paid-cloud-revision--free-runner-candidate-planning-only),
not a selected/qualified platform. Repository eligibility, exact workflow/action
approval and usable KVM remain unverified. No account query, visibility change,
workflow creation/dispatch or native action occurred. The laptop remains protected.
The owner then reported **"Yes—already public; consider it"** for CloudDialect on
GitHub under their control. This approves candidate consideration only; live
repository/settings eligibility and workflow writing/publication/dispatch remain
unapproved/unverified. The next request is a local-only manual metadata workflow
and focused synthetic contract tests. GitHub's default-branch requirement for
manual dispatch is recorded; no feature-branch-only execution shortcut is assumed.
One static security review of this revision requested explicit public-log
disclosure and collector-only output-cap wording; the parent integrated that
correction. See `results/no-paid-cloud-plan-review.md` and
`results/no-paid-cloud-plan-checks.json`. No independent post-correction reread
or actual free-runner qualification is claimed.

Before that cost-constraint reply, the same missing target/provider/operator and B
approvals were revalidated across three consecutive goal turns. No further
authorized runtime action or live process exists. The owner's provider-choice
question was unanswered; further status-only loops did not count as progress.
All numbered tasks and six gates remain open, and the laptop stays protected.

Historical pre-resume status: **Goal BLOCKED — status audit delivered; runtime implementation gated**.
The same missing owner/native gate evidence was revalidated across three
consecutive goal turns. The intervening turn was no progress, not a live-process
wait: the local checks were already terminal and no authorized dependent runtime
action became available. At that point the goal stopped pending user direction
and the required approval/environment evidence; the subsequent owner direction
resumed it as described above. It is not marked complete.
No numbered execution/isolation task meets its full completion criteria. The
checklist now records **10 completed supporting deliverables**, including one new
checked current upstream verification item; all **21 numbered leaves and 12
scheduling epics remain open**. No task was checked merely to show progress.

## What changed

1. `.kiro/specs/execution-isolation/tasks.md`: added the current dated audit,
   clarified that an unresolved UD-1 is a blocked outcome rather than task
   completion, made stale supporting descriptions historical, and checked the
   newly completed bounded upstream re-verification under 1.6.
2. `.kiro/specs/execution-isolation/decisions/BG-01-upstream.md`: recorded the
   actual immutable source/current bounded results; retained previous evidence
   as a dated historical snapshot. BG-01 remains OPEN.
3. `.kiro/specs/execution-isolation/qualification/upstream-compatibility.md`:
   refreshed bounded evidence and preserved missing consumer/native/owner gates.
4. This audit directory: eval, state, per-task inventory, protected hashes,
   command records/outputs and two independent read-only review summaries.

During the original checklist audit, no runtime crate, API/schema/source change,
license allowance, owner approval, decision selection, environment probe,
provider operation, dependency fetch, commit, push or release occurred. The
separately scoped continuation approvals/preflights are described above, not
native qualification or gate closure. Earlier Git publication authorization was not
carried forward to this new goal. No memory or runtime-agent configuration changed.

## Completion verdicts

The complete 21-row matrix is in [task-audit.md](task-audit.md), with exact
declared-file presence and reasons in [task-audit.json](task-audit.json).

- **PASS — completed supporting work:** five drafted decision/qualification
  packets, BG-01 preparation and separately authorized upstream implementation,
  traceability, and the fresh bounded verification. Artifact availability and
  content were inspected; existence is not native or approval evidence.
- **UNVERIFIED — TASK-001–004/006:** approved decisions, authentic native or
  downstream-consumer evidence and the complete owner acceptance records remain
  missing. The five UD records explicitly say OPEN. BG-01 explicitly says OPEN.
- **MISS — TASK-005:** measurement JSON says `measured: false`, no observations,
  null approved limits and no owner approvals. No budget was invented.
- **MISS — TASK-007–021:** the planned execution crate/files are absent. No
  selected adapter, trusted runtime verification, real stores/credentials,
  checkpoint coordinator or native/restore/release qualification exists in this
  audited delivery. The specification forbids starting these before its gates.
- **UNVERIFIED — native acceptance:** AC-05/AC-06/AC-08 evidence stays MISSING;
  no isolated-environment/traffic-observer/descendant-domain proof was supplied.

## Actual current verification

Code source is the immutable `5b3dfa6954805cfbeba3e9724803dac064ba48e6`
tree. The status-document edits do not change that code; 290 protected file
hashes and the exact allowed diff scope were checked after integration.

| Check | Result | Meaning |
| --- | --- | --- |
| Python repository tests | PASS, exit 0, 18 cases | Existing local regression checks |
| Release file-contract tests | PASS, exit 0, 7 cases | Documentation/policy structure, not a successful license gate |
| Contracts frozen/offline tests | PASS, exit 0 | 291 Rust cases including 7 doctests; 101 canonical conformance fixtures separately |
| Contracts all-target build | PASS, exit 0 | Existing upstream crate builds |
| Format check | PASS, exit 0 | Existing upstream source formatting |
| All-target warning-denying Clippy | PASS, exit 0 | Existing upstream static checks |
| Historical Spec 2 document grader | Historical-contract mismatch, exit 1 | Rejects existing checked supporting items; preserved as-is |
| Real cargo-deny 0.20.2 license scan | MISS, exit 4 | Original seven allowances reject MIT-0/Unicode-3.0 expressions and scanner-unknown custom root license |
| Current task/status/link/scope audit | PASS | 21 leaves/12 epics open, 10 supporting marks, all six gates open, no protected changes |
| Seeded false completion | PASS rejection | Changing TASK-008 to checked is rejected by the current completion audit |
| Whitespace/export guard | PASS | Diff clean; exported logs redact local account repository prefixes |

Exact argv, isolated-cache environment, return codes and outputs are in
[command-results.json](command-results.json). These commands were executed by
the parent under `rtk proxy python3 -u -` using stdlib subprocess capture; the
`command` field is the equivalent direct reproduction spelling and
`executed_argv` records the actual child argv. No agent ran hidden subprocess
workers. The existing temporary Cargo cache/target directory was reused; no
dependencies were fetched to bypass offline restrictions.

Representative direct commands:

```sh
rtk proxy python3 -m unittest discover -s tests -v
rtk proxy python3 scripts/verification/contracts-evidence-foundation/test_release_tooling.py -v
rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home CARGO_TARGET_DIR=/tmp/clouddialect-release-tooling-verification/target cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml --frozen --offline
rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home CARGO_TARGET_DIR=/tmp/clouddialect-release-tooling-verification/target cargo check --manifest-path crates/clouddialect-contracts/Cargo.toml --all-targets --frozen --offline
rtk proxy cargo fmt --manifest-path crates/clouddialect-contracts/Cargo.toml -- --check
rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home CARGO_TARGET_DIR=/tmp/clouddialect-release-tooling-verification/target cargo clippy --manifest-path crates/clouddialect-contracts/Cargo.toml --all-targets --frozen --offline -- -D warnings
rtk proxy python3 scripts/verification/execution-isolation/check_spec.py
rtk proxy git diff --check
```

The existing historical grader failed with
`FAIL: no implementation tasks marked complete`: it rejects every checked box,
including the already-authorized supporting deliverables, before reaching its
old pinned-hash/change-scope assertions. Its source/baseline were not weakened.
This failure does not mean native controls failed or passed.

The runner-control target's 11 passing entries consist of **10 substantive
controls plus the `fixture_worker` subprocess helper**. Default helper entry
success is not an additional security proof. The timeout/reaping control covers
a direct helper child, not the complete execution domain required by Spec 2.
Synthetic scoped stores test data-boundary algorithms, not authenticated backend
authorization, observation provenance or real credential lifetime guarantees.
No fresh coverage, pass@k or stability statistic is claimed.

## Team and independent challenge

Smallest capable team: parent integration/execution plus two bounded read-only
audits with distinct scopes. `security_reviewer` checked gates/approvals and
runtime absence; `test_coverage_quality_review_agent` inspected assertion quality
and caught the helper-count/native-cleanup overclaim risk. Their summaries are
in `results/security-review.md` and `results/test-evidence-review.md`.

Both configured profiles are user-registered, schema checked, `gpt-6.1-sol` /
`high` / declared read-only. Role availability and successful review responses
are observed; actual model/effort and enforced isolation are UNVERIFIED. These
are behavioral read-only packets, not sandbox attestation. No recursion, new
writers, repair dispatches or hidden fan-out. Parent inspected their findings,
executed graders and verified final integrated documents. No independent
post-edit verdict or measured per-agent token/cost savings is claimed.

## Required next authority

Do not start TASK-007 or runtime work until the actual UD-1–UD-5 and BG-01
approval/qualification requirements are satisfied and separately authorized.
The decision documents identify the required security/operations/core/worker
owners and evidence. Draft comparisons are already delivered; rewriting a
proposal does not supply those missing approvals. Dependency license review
remains blocked by the owner's deliberate unchanged-policy decision.

## Current-branch publication preflight — owner authorized

All 33 current modified/non-ignored untracked files were reviewed. Parent checks
pass 49 local tests (24 preflight, 18 repository Python, 7 release-file checks),
independent YAML/Python syntax, 290 protected hashes and diff checks. One reused
read-only `code_reviewer` (configured `gpt-6.1-sol` / `high`; effective runtime
attestation unverified) found no publication blocker or tested credential/account
path exposure. No behavior repair or scope expansion was required. Staged checking
then found one extra EOF blank line in the generated Rust test log; the parent
removed exactly that newline, preserving the previous safe log and before/after
hashes in the command record. No output content or test result changed. The
license failure
and 21 open runtime tasks/six open gates remain unchanged. This is a validated
pre-publication snapshot, not a claim that publication has occurred; actual Git
commit/push and remote-ref results belong to the final command handoff. No workflow
dispatch, default-branch merge, native action or spend is authorized or performed.
