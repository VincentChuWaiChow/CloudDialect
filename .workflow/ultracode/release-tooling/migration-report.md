# Approved cargo-deny migration — 2026-10-28

Status: **READY FOR REVIEW** for the approved syntax/spec migration. Task 26
remains **OPEN**: the real license scan is a verified **MISS**, not a missing-tool
or configuration-syntax problem. On 2026-10-29 the owner chose to retain the
seven-license policy and leave release blocked pending license review. No
additional licenses, exceptions or root-license clarification were approved.

## Authorization and scope

The user explicitly approved current cargo-deny policy migration and matching
task/design edits. Keep the original seven allowed expressions; all others are
denied by default. No license additions, private-crate exclusions, exceptions,
clarifications, dependency/pin/lockfile changes or root-license relabeling were
approved or implemented. Req 12's normative requirements remain unchanged.

Changed delivery files: `deny.toml`, `release/README.md`, task 26 in
`.kiro/specs/contracts-evidence-foundation/tasks.md`, License Screening in the
matching `design.md`, and the canonical
`scripts/verification/contracts-evidence-foundation/test_release_tooling.py`.
The public-key placeholder is unchanged. Run evidence/state is kept in this
directory. Parent owns all integration and writes.

## Checks

| Criterion | Status | Authoritative evidence |
| --- | --- | --- |
| Supported allow-only policy, exact original seven expressions | PASS | cargo-deny 0.20.2 reads the actual file without removed-field errors; structural check forbids extra keys/allowances |
| Matching task/design/README semantics | PASS | Design TOML parsed and compared with root policy; independent review; all unallowed licenses rejected by default |
| New structural tests | PASS | Updated tests first failed on the legacy policy/spec (3 failures); after migration, 7/7 pass |
| Existing Python regressions | PASS | 15/15 tests, exit 0 |
| Guard sensitivity | PASS | Three temporary-copy controls reject restored `deny`, broadened allow list and design drift |
| Rust conformance | PASS | Real frozen/offline execution: 101/101 canonical fixtures, exit 0; saved `results/conformance.txt` |
| Current locked graph passes license gate | MISS | Real cargo-deny 0.20.2 scan exits 4 with 21 errors; saved raw and named reports below |
| Named report exists with package/version/license | PASS | `results/license-incompatibilities.json` binds 20 rejected dependency identities to the current lockfile and names the separate root classification error |
| No forbidden tracked changes | PASS | Source, catalog, Cargo.toml/Cargo.lock, CI, agent config and requirements document unchanged |
| Real SBOM/archive/signature or release proof | UNVERIFIED | Not generated and not claimed; public key is intentionally still a placeholder |

The real scan rejects `borrow-or-share 0.2.4` (`MIT-0`) and 19 dependency
expressions requiring `Unicode-3.0`. In particular, `unicode-ident 1.0.27`
declares `(MIT OR Apache-2.0) AND Unicode-3.0`; the allowed MIT/Apache branch
does not satisfy the additional Unicode term. The final error is the scanner's
inability to confidently identify the existing custom root `LICENSE.md`, not a
finding that the project has no license. These are technical policy findings
requiring owner review, not legal advice or a legal incompatibility conclusion.

Before the owner-authorized Git commit/push, the absolute local repository
prefix in `results/license-check-0.20.2.txt` was replaced with `<REPOSITORY>` to
avoid publishing the local account identifier, and trailing horizontal whitespace
was removed from the export for the staged whitespace check. No license findings
or scanner values changed. The JSON report retains `raw_report_sha256` for the unredacted
original held locally under `/tmp` and adds `published_report_sha256` for the
redacted committed copy; the two hashes intentionally differ.

## Reproduction and actual command results

The official release API identified cargo-deny 0.20.2. The temporary Linux-musl
archive's SHA-256 matched its GitHub asset metadata:
`9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`.
Source: [official release](https://github.com/EmbarkStudios/cargo-deny/releases/tag/0.20.2).
This is a download-integrity check, not independent signing/provenance proof.

Normal sandbox DNS failed. Approved network escalation fetched only public
release metadata, that binary archive and the existing locked registry crates.
The binary and Cargo cache are under
`/tmp/clouddialect-release-tooling-verification/`; no tool was installed into
the user's persistent tool directories. Existing dependency versions were not
updated, and `Cargo.lock` was not regenerated.

Commands:

```sh
rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home \
  cargo fetch --manifest-path crates/clouddialect-contracts/Cargo.toml --locked
# Exit 0, existing locked public dependencies downloaded into /tmp only.

rtk proxy /tmp/clouddialect-release-tooling-verification/bin/cargo-deny --version
# cargo-deny 0.20.2

rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home \
  /tmp/clouddialect-release-tooling-verification/bin/cargo-deny --frozen \
  --manifest-path crates/clouddialect-contracts/Cargo.toml --config deny.toml \
  check licenses
# Exit 4. Full findings with local paths redacted: results/license-check-0.20.2.txt.
# Executed through a stdlib subprocess capture to retain the complete output.

rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home \
  CARGO_TARGET_DIR=/tmp/clouddialect-release-tooling-verification/target \
  cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml \
  --frozen --offline --test conformance
# Exit 0, 101 fixtures. Repeated with output capture: results/conformance.txt.

rtk proxy python3 scripts/verification/contracts-evidence-foundation/test_release_tooling.py -v
# Exit 0, 7 tests; same suite independently repeated by reviewer.

rtk proxy python3 -m unittest discover -s tests -v
# Exit 0, 15 tests.

rtk proxy git diff --quiet -- crates/clouddialect-contracts/Cargo.toml crates/clouddialect-contracts/Cargo.lock
# Exit 0; no dependency or lockfile edits.

rtk proxy git diff --check
# Exit 0.
```

`rtk proxy graphify update .` also exited 0 using AST-only extraction (sequential
fallback), rebuilding 3,766 nodes and 5,079 edges. No graph existed at baseline;
the generated first-graph output was retained under
`/tmp/clouddialect-release-tooling-verification/graphify-out`, not added to the
delivery tree. This is navigation data, not runtime or sandbox evidence.

The current binary's `--help` confirms `--config` precedes the `check`
subcommand; the task/design/README command was corrected accordingly. The
`cargo deny ...` spelling in reusable instructions assumes cargo-deny is on PATH;
the verification above uses its explicit temporary path, not another version.

Negative controls used a stdlib temporary directory under `/tmp`, copied the
policy/docs and ran the focused unittest against each mutation; all three were
rejected by assertions with no test errors or delivery-tree mutations. These
are regression guards, not SPDX-engine emulation or reliability sampling.

## Team, limits and remaining decision

Parent implemented and verified; one reused `code_reviewer` reviewed the bounded
migration read-only. Configured profile is user-registered, schema checked,
`gpt-6.1-sol` / `high` / declared read-only. Actual model/effort and enforced
sandbox isolation are not exposed and remain UNVERIFIED. One continuation
reviewer dispatch, no recursive agents, no additional writers, no repair
dispatches. Actual per-agent tokens/cost are unavailable; no savings claim.

The reviewer independently passed 7 structural checks and diff whitespace checks,
inspected the raw scan, and confirmed the policy fails closed without exemptions.
Parent retained the failure instead of broadening the list to manufacture green.

The user was asked separately whether to approve `MIT-0`, `Unicode-3.0` and a
file-hash-bound, crate-specific clarification/allowance for the existing custom
Community License. On 2026-10-29 the user selected: "Keep the seven-license
policy; leave release blocked pending license review." The policy remains
unchanged, the license gate remains failed and task 26 stays unchecked. Future
remediation requires a new owner decision. No real key, signing, publication,
commit, push, CI change or
native sandbox/provider probe occurred. BG-01 and Spec 2 approvals stay open.
