# Task 26: release tooling configuration and documentation

## Approved migration continuation (2026-10-28)

The user explicitly approved current cargo-deny allow-only policy migration and
matching specification edits. This supersedes the legacy-preservation restriction
below, not the original release-integrity requirements or forbidden side effects.
Additional owned paths: License Screening in `design.md` and corresponding task
26 bullets/acceptance in `tasks.md`. Req 12's normative requirements are unchanged.

Success before implementation: structural tests reject `deny`/`copyleft`, preserve
the exact seven-item allow list with no exceptions, and compare the design's TOML
excerpt with the real policy. The formerly denied eight licenses must remain
outside the allow list and documented as examples rejected by default. Update
the tests first and observe RED, then migrate and observe GREEN. Re-run existing
Python tests, real cargo-deny if tooling can be provisioned, frozen conformance,
diff checks and one independent read-only review.

Temporary verification tools and an isolated Cargo cache may be created under
`/tmp`; no persistent installation, dependency-pin/lockfile changes, policy
exemptions, real signing or publication. Network reads require normal sandbox
approval. Stop affected verification on rejection/unavailability, and report actual
failures; no synthetic graph may stand in for a current dependency license scan.
Root-custom-license or unallowed dependency diagnostics are genuine decisions,
not permission to widen the list or ignore crates.

New bounds: parent-only writes, one reused reviewer turn, one consolidated repair,
20 minutes. The blocked audit restarts with this approved resumed scope.

## Contract (defined before implementation, 2026-10-28)

Implement only task 26 of the contracts/evidence foundation specification.
Owned delivery paths: `deny.toml`, `release/README.md`, `release/minisign.pub`;
verification: `scripts/verification/contracts-evidence-foundation/test_release_tooling.py`;
evidence: this directory; task 26 delivery note only in the canonical task list.
No source/API/schema/dependency/lockfile, CI, runtime-agent configuration or other
task changes. No keys, installs, releases, archives, signing, commits, pushes,
external mutations, schedules or persistent memory changes.

Risk: local configuration/documentation; misleading release qualification is the
main risk. The user authorized these files, not a release or policy migration.

## Facts and assumptions

- Clean initial `git status --short`. No `graphify-out/graph.json` exists.
- The crate is standalone at `crates/clouddialect-contracts/Cargo.toml`, edition
  2024, lockfile v4. It is a library, not a distributable validator executable.
- `cargo` 1.99.1 is present; cargo-deny, cargo-cyclonedx and minisign are absent.
- The exact requested cargo-deny policy uses fields removed in 0.16.0. Preserve
  that policy, flag incompatibility; do not silently migrate it or assert that an
  older tool supports the present crate/lockfile.
- The design's `--output-file` SBOM option is absent from upstream's documented
  CLI. Preserve the design reference as non-executable and give the documented
  manifest-path/filename alternative. Actual generation remains unverified.
- Baseline conformance attempt with `--locked --offline` exits 101: `boon` is not
  cached. Do not change dependencies or access the network to mask this boundary.

## Checks and failure ownership

Parent owns failures and integration. Before delivery, add stdlib structural
tests, demonstrate missing-file RED, implement minimally and rerun GREEN.

1. TOML parses; exact seven allow/eight deny entries and `copyleft = "deny"`.
2. README covers SBOM/dependency versions/licenses, signing and offline
   verification, named incompatibility diagnostics and ordered five-step gate.
3. Placeholder is comment-only and clearly not a functioning verification key;
   README blocks signing/verification qualification until a real public key.
4. README accounts for nested manifest, old policy syntax and SBOM CLI drift.
5. Run the existing Python suite and frozen/offline conformance; distinguish
   structural passes from tool execution or authentic release proof.
6. Inspect diff/scope and preserve lockfile/source. Do not check off task 26 if
   its real zero-exit license-screening acceptance criterion is unverified.

No pass@k or pass^k estimate: one deterministic RED/GREEN trial, not reliability
sampling. The structural grader does not emulate license/SPDX or cryptography.

## Team and bounds

Parent implements/integrates; one `code_reviewer` read-only packet independently
challenges compatibility and final scope. Configured source is
`~/.codex/agents/code-reviewer.toml`, checked-fields-valid,
`gpt-6.1-sol`/`high`/declared read-only. Runtime tool exposes the role but not
effective model or isolation attestation; these remain UNVERIFIED. No isolation
claim is needed for this behavioral read-only review.

At most one dispatch, one review wave, one consolidated repair and 30 minutes.
No recursion. Stop affected checks for missing dependencies/tools or approval;
finish unaffected local work. No new autonomous harness is needed.
