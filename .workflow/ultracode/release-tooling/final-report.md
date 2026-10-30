# Task 26 local delivery — 2026-10-28

**Superseded by the owner-approved migration:** see
[current migration report](migration-report.md). The report below records the
initial legacy-policy delivery and its earlier tooling blocker only. Current
tooling is provisioned temporarily, conformance passes, and the remaining blocker
is the real scan's license findings, not a missing tool.

Status: goal **BLOCKED** after the same tooling/policy verification blocker was
revalidated across three consecutive goal turns. Local files are **READY FOR
REVIEW**; task 26 remains **OPEN**, with release gates **UNVERIFIED**.
The exact requested legacy policy is delivered,
not represented as compatible with current cargo-deny. A policy/spec migration
was offered to the user; no approval or migration is assumed.

The second goal turn was no progress: authoritative checks reproduced the same
missing cargo-deny/uncached `boon` failures without changing the next action.
The third turn independently reproduced them again; all six file checks still
pass. No further file edits can substitute for the missing real gate evidence,
and no policy migration or tool installation is authorized by an unanswered
question. Work stops pending the stated user decision or a provisioned toolchain.

## Changed files

- `deny.toml`: exact seven allow/eight deny entries and copyleft deny.
- `release/README.md`: all three procedures, five ordered gates, nested crate
  paths, real-key prerequisite, current-tool compatibility caveats and sources.
- `release/minisign.pub`: deliberately unusable comment-only placeholder.
- `scripts/verification/contracts-evidence-foundation/test_release_tooling.py`:
  six deterministic file-contract tests, not a license or cryptographic engine.
- `.kiro/specs/contracts-evidence-foundation/tasks.md`: delivery note only;
  checkbox intentionally unchanged.
- This run's `plan.md`, `state.json`, `final-report.md`: scoped eval and evidence.

## Capability and regression results

| Criterion | Status | Evidence |
| --- | --- | --- |
| Exact requested policy; valid TOML | PASS | Six-test grader plus independent `tomllib` parse: seven allow, eight deny, copyleft deny |
| Required procedures and five-step gate | PASS | File-contract grader and independent source review |
| Public-key placeholder, no invented key | PASS | Comment-only file and explicit first-release blocker |
| Useful regression guards | PASS | Three temporary negative controls reject a missing deny entry, broadened allow list and missing gate |
| Existing local Python checks | PASS | 15 tests, exit 0 |
| Real license check exits zero | UNVERIFIED | cargo-deny absent, exit 101; syntax removed in 0.16.0; no compatible legacy tool verified against edition 2024/lockfile v4 |
| Rust conformance | UNVERIFIED | Exit 101 before compilation: offline cache lacks `boon` |
| SBOM generation / output filename / CycloneDX validation | UNVERIFIED | Tool absent; upstream CLI/source inspected only; design's unsupported `--output-file` explicitly marked reference-only |
| Real archive signing/offline verification | UNVERIFIED | No actual key, tool, archive or signature; release/signing not authorized |
| Scope preserved | PASS | Source, catalog, manifests, lockfile, CI and agent config unchanged; no commit/push/publication |

## Exact command evidence

- `rtk proxy python3 scripts/verification/contracts-evidence-foundation/test_release_tooling.py -v`
  - Before delivery files: exit 1, six missing-file errors (intended RED).
  - After implementation: exit 0, 6/6 tests (GREEN).
  - Reviewer reran the same script without `-v`: exit 0, 6/6.
- `rtk proxy python3 -m unittest discover -s tests -v` — exit 0, 15/15.
- `rtk proxy python3 - <<'PY' ... PY` — stdlib temporary-copy mutation controls
  ran each focused test with one change; all three failed by assertion as
  intended, no errors; control command exited 0. No delivery files mutated.
- `rtk proxy cargo deny --manifest-path crates/clouddialect-contracts/Cargo.toml check --config deny.toml licenses`
  — exit 101, `no such command: deny`.
- `rtk proxy cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml --locked --offline --test conformance`
  — exit 101, `no matching package named boon found`.
- `rtk proxy cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml --frozen --offline --test conformance`
  — same exit 101 and missing `boon`; this is not a Rust test failure or pass.
- `rtk proxy git diff --check` — exit 0; reviewer independently repeated it.
- `rtk proxy git diff --quiet -- crates/clouddialect-contracts catalog .agents .github scripts/README.md`
  — exit 0, no changes to protected tracked surfaces.
- `rtk proxy graphify update .` — exit 0, AST-only graph rebuilt. Parallel
  extraction unavailable; sequential fallback succeeded (3,757 nodes, 5,071
  edges). No graph existed initially. Fresh generated output was preserved in
  `/tmp/clouddialect-release-tooling-graphify-*/graphify-out`, not added as an
  unrelated first graph to this release-tooling delivery. No semantic/API
  extraction or runtime-isolation evidence is claimed.

No dependencies fetched, tools installed, credentials read, key generated,
archive built, signature created, CI changed or external mutation performed.
No native sandbox/provider probes. Requirements' release-artifact evidence and
Spec 2 UD-1–UD-5/BG-01 remain open. No statistical pass@k/pass^k claim.

## Team and review

Smallest capable route: parent-owned implementation plus one bounded read-only
`code_reviewer` packet. No recursive/hidden agents, additional implementation
workers or repair dispatches. Reviewer reported no actionable findings after
independently running structural tests and inspecting the final files. Parent
independently executed graders and three mutation controls.

Configured role: user-registered `~/.codex/agents/code-reviewer.toml`, schema-3
inventory `checked-fields-valid`, `gpt-6.1-sol`, `high`, declared `read-only`.
Role availability and successful execution are observed; actual child/parent
model, effort and enforced sandbox isolation are not exposed and remain
UNVERIFIED. Read-only was a behavioral contract, not a security attestation.
One dispatch of one allowed, zero repair passes, within 30-minute bound.
Actual per-agent token consumption is not available; no cost/savings claim.

## Required follow-through

Before closing task 26, obtain a successful real license scan. Either validate
an explicitly selected legacy tool against this current crate or obtain owner
approval for a supported policy plus matching specification migration. Do not
change licenses, exempt the root custom license or weaken policy to force green.
Re-run Rust conformance in a provisioned environment. Real release qualification
separately requires a validated SBOM, reviewed payload, actual committed public
key and successful offline signature verification under release authorization.
