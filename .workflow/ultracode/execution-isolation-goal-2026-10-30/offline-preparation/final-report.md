# Offline preparation handoff — 2026-10-30

Status: **READY FOR OWNER REVIEW — bounded static review PASS**. The four authorized preparation
workstreams are delivered; the original execution-runtime goal is not complete.
All 21 numbered tasks and UD-1–UD-5/BG-01 remain open. Only supporting work is
checked (15 supporting deliverables total, four new in this packet).

## Delivered and inspected

- **UD-2:** closed Draft 2020-12 request/result/error schema; proposed codes,
  version/direction/state/cancellation rules; 36 decoded-message examples plus
  separate, unexecuted byte/state/native cases. Unknown operational bounds stay
  UNSET. Reference-only tool payloads require a separate admitted operation schema.
- **UD-3:** exact synthetic read/create-only resources and bytes; expiry,
  cancellation/revocation, duplicate dispatch, unknown effect and restart rules;
  20 vector rows with expected counts. Parent clarified logical payload versus
  UD-2 outer wire references. No provider/mechanism is selected or enabled.
- **UD-4:** nine concrete **unapproved** retention proposals, deletion/copy
  limitations and restore timelines; twelve cases distinguish durable state,
  acceptance, access and dispatch. No store, transaction or retention engine exists.
- **BG-01:** immutable revision `8ba032e662153828267c6242d9773b1b77e8bb93`,
  crate/catalog versions, 38 exact source/catalog/manifest/lock hashes, schema
  IDs and source-derived snapshot digest; actual signatures compile against
  the existing upstream crate. This is not an implemented Spec 2 consumer.

## Criteria and exact evidence

| Criterion | Status | Evidence / scope |
|---|---|---|
| Draft regression / IPC examples | PASS | `rtk proxy python3 -B -m unittest discover -s scripts/verification/execution-isolation -p test_offline_proposals.py -v`: initially exit 1 (schema absent), finally exit 0, three checks / 36 vector examples. `red.txt`, `green.txt`. Installed jsonschema 4.26.0, no install/fetch; references restricted to local fragments. Shape/document consistency only. |
| Credential/retention draft consistency | PASS | Parent inspected all changed pairs; inline Python independently checked synthetic SHA-256/lengths, 20 vector rows and count arithmetic, nine unapproved policies and twelve STORE IDs. `integration-checks.json`. These are proposed outcomes, not execution results. |
| Upstream identity preservation | PASS | All 38 hashed files match `git show 8ba032e:<path>` byte-for-byte; requirements/design unchanged, runtime crate absent. `upstream-identities.json`, `integration-checks.json`. |
| Compile-only consumer | PASS | `rtk proxy rustc --edition=2024 --crate-type lib --emit=metadata -D warnings --extern clouddialect_contracts=... -L dependency=... scripts/verification/execution-isolation/upstream_signatures.rs`: exit 0. Exact expanded commands in `compile-results.json`. Two disallowed Into probes exit 1/E0277; invented Accepted variant exits 1/E0599, as expected. |
| Selected upstream regression | PASS | `rtk proxy env CARGO_HOME=/tmp/clouddialect-release-tooling-verification/cargo-home cargo test --offline --locked --manifest-path crates/clouddialect-contracts/Cargo.toml --target-dir /tmp/clouddialect-offline-oB17k0 --test public_api_compatibility --test phase_results --test catalog_snapshot --test evidence_completeness --test extension_validation --test verification_semantics --test required_extension_models`: exit 0, **44 tests**. `rust-tests-cached.txt`. All selected tests use in-memory/synthetic data; no native runner or child-process controls selected. |
| Existing hosted-preflight synthetic regression | PASS | `rtk proxy python3 -B -m unittest discover -s scripts/verification/execution-isolation -p test_hosted_preflight.py -v`: exit 0, **24 tests**. `hosted-synthetic.txt`. Fake host/signal/stdout interfaces only; actual collector not run. |
| Historical document grader | MISS — pre-existing stale rule | Both HEAD and current draft fail `no implementation tasks marked complete` because the old checker rejects supporting checkboxes too. `historical-checker.json`. Left unchanged; its negative tests are not counted as valid evidence when the positive baseline fails. New scoped check verifies all 21 numbered tasks and six gates stay open. |
| Independent static review | PASS | Reviewer found one P2 raw-diagnostic retention contradiction; parent added a failing document regression, corrected BG-01, and reran successfully (`review-red.txt` → `green.txt`). Reviewer re-read the correction and independently ran all three draft checks plus scoped diff whitespace checks. No unresolved actionable finding in this scope; not runtime qualification. |
| Native/security/owner qualification | UNVERIFIED / NOT RUN | Wire parsing, effective authorization, revocation timing, actual provider/restore/durability, measurements, full conformance, coverage and approvals are not established by this packet. |

The first Cargo attempt exited 101 because default-cache `boon` was unavailable
(`rust-tests.txt`). Reusing the previously recorded local cache succeeded; no
dependency download or fallback to online resolution occurred. The full suite,
conformance runner and license scan were deliberately not rerun. Historical
license findings remain unresolved, not waived by these passes.

The old checker comparison first failed due to its repository-relative-path
assumption when staging the comparison under `/tmp`; rerunning with a disposable
subdirectory of this evidence run produced the matching baseline/current failure.
This was a grader setup correction, not a product regression or TDD result.

## Team and scope

Ultracode parent owned scope, evals, BG-01, integration and reporting. Three
disjoint `worker` packets (`offline_ipc`, `offline_credentials`,
`offline_retention`) used requested **gpt-6.1-sol / high** on fresh contexts.
The live catalog supports that route; observed backend model identity is
UNVERIFIED. `gate_review` uses configured **security_reviewer,
gpt-6.1-sol / high**, read-only sandbox default; effective per-agent enforcement
is not attested. No security conclusion depends on delegate sandbox isolation.
One implementation wave, one read-only review wave, four dispatches, one parent
repair with reviewer reread; no fan-out. Actual per-packet token totals are not exposed.
Three independent decision pairs warranted parallelism; shared compatibility,
checklist and grader surfaces stayed with one integration owner.

Changed deliverable files (all under repository root):

1. `.kiro/specs/execution-isolation/decisions/UD-2-ipc.md`
2. `.kiro/specs/execution-isolation/qualification/ipc-cases.md`
3. `.kiro/specs/execution-isolation/decisions/UD-3-credentials.md`
4. `.kiro/specs/execution-isolation/qualification/credential-cases.md`
5. `.kiro/specs/execution-isolation/decisions/UD-4-persistence.md`
6. `.kiro/specs/execution-isolation/qualification/restore-cases.md`
7. `.kiro/specs/execution-isolation/decisions/BG-01-upstream.md`
8. `.kiro/specs/execution-isolation/qualification/upstream-compatibility.md`
9. `.kiro/specs/execution-isolation/tasks.md`
10. `scripts/verification/execution-isolation/test_offline_proposals.py`
11. `scripts/verification/execution-isolation/upstream_signatures.rs`

Run evidence is in this directory. Existing parent-run artifacts were preserved.
No changes to upstream source, catalog schemas, manifests/locks, licenses, runtime
configuration, credentials, workflows or deployment resources. No commit/push.

## Graph and remaining limits

Graphify navigation was used before exploration. Both `rtk graphify update .`
passes exited 0 after code changes; the final AST-only graph has **3,437 nodes /
4,373 edges** (`graphify-update.txt`). The first pass backed up the curated graph;
the updates reported community-label changes, 53 unclassified files,
five code files with no extracted symbols and pending document semantic updates.
This does **not** claim a complete fresh semantic index of the full repository or
its workflow history. No external LLM indexing, API key or paid fallback was used.

Real IPC/credential/restore acceptance still needs owner decisions and the
qualified environment/profile; no laptop host inspection or Docker/KVM access
was attempted. Reconcile these drafts with UD-1/UD-5/BG-01 before task 1.7 and
obtain separate implementation authorization. A schema-valid string, digest or
passed local test is not an authenticated grant or native isolation evidence.
