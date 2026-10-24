# Approved repair/seed renewal — verified bounded delivery

## Outcome
Both approved items are implemented and locally committed. Spec 1 task 14/unit
2.10, bounded runner unit 3.1 and seed batch 3.2.1 are checked within their actual
scope. Full canonical tasks 18–22, traceability/consumer handoff, all 21 numbered
Spec 2 tasks, UD-1–UD-5 and BG-01 remain open. Full original goal is unachieved.

## Local checkpoints
- `03738e4`: actual regression RED. Compiled/executed target: 9 pass, new test
  fails with 2 gaps instead of 4. Production helper was unchanged at this point.
- `0396883`: minimal three-line branch move; same target GREEN, 10 pass. Direct
  assumptions and root blockers coexist, zero store calls, no accepting result.
- `408af0d`: exact approved inferred-evidence seed pair admitted. Data addition,
  not a fabricated behavioral RED. Input/expected bytes match reviewed drafts.

## Verification
- Full default frozen Cargo test: **286 Rust tests (including 7 doctests), plus
  exactly 1 conformance fixture**, exit 0. No integration target disabled.
- All-target frozen check and Clippy `-D warnings`, formatting, and 15 Python
  regression checks pass.
- Default and positive-prefix custom runner invocations each execute 1 fixture;
  guaranteed-empty selector exits 1. All 9 runner controls pass.
- Re-executed all 67 previously declared CLI controls against the current repaired
  binary: 5 positive controls and 62 expected rejections all matched. This is
  parent-only temporary-fixture evidence, not 67 canonical corpus deliveries.
- Exact commands/logs: `results/renewal-2026-10-24/integrated-checks.json`,
  `repair-green-checks.json`, `seed-checks.json` and adjacent logs.
- Parent source/scope audit: only approved production/test repair and exact seed
  pair changed under `crates/`; manifests, lockfile, runner source and unrelated
  research preserved. RED/GREEN checkpoints remain reachable on the same branch.
- Additional spec-checker suite: **8 pass, 1 error**. Its legacy blanket ban on
  any checked task also rejects the 8 pre-existing supporting-deliverable marks.
  Baseline and current source have the identical failing predicate/checker; all
  21 numbered runtime tasks remain open. This failure is recorded, not repaired
  or waived, and does not invalidate the separately executed Cargo/15-check
  repository Python results. See `spec-checker-baseline-comparison.json`.

## Review and delegation boundary
No new agents, recursion or follow-ups: prior allocation remains 30/30 consumed.
Reviewer 30 statically reviewed the proposed patch and unchanged runner/seed
before this renewal. The applied repair and integrated execution are parent-only
verified, not a fresh independent repaired-tree signoff. No model-switch claim.

## Tooling exception and skipped coverage
The old isolated /tmp Cargo cache and LLVM tools were absent. Initial frozen
failure was missing-cache setup, **not RED**. Restoring existing locked packages
used the default public registry cache because CARGO_HOME was omitted; this was
immediately disclosed. Only registry data was then copied to /tmp, and all build/
test execution used the isolated cache. No credentials/config inspected or copied;
manifest/lock pins are unchanged. Details: `setup.json`.

Fresh coverage was **not run**: matching LLVM 23.1.1 utilities are unavailable and
no new tooling installation is authorized. Historical runner 89.05% line/58.33%
function measurements are not current repaired-tree or full-library coverage.
The broader coverage gate remains incomplete, not waived by green tests. Checks
used Rust 1.99.1; the declared Rust 1.88 MSRV was not separately qualified.

## Remaining work and safety
Only 1 of the draft 104 cases/60 batches is delivered. Remaining fixture batches
need exact oracle/scope approval; some require actual approved environment/backend
qualification and cannot be replaced with synthetic success. Field/gate
traceability and real downstream compatibility handoff remain unfinished.
No pushes, native/provider probes, release, deployment or runtime gate closure.
No new dependencies, features, versions or public API. No execution crate exists.
The 45-minute approval began 02:47:02 UTC, hard deadline 03:32:02 UTC; this delivery
has not silently extended the timer or delegated allowance.
