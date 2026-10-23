# PR3 fixes — final report

## Outcome
All five reviewed P2 defects corrected locally with failing regressions before minimal source edits. The final independent static reviewer identified one additional attribution edge within R5; the sole consolidated parent repair fixed it with its own RED/GREEN regression. No commits, pushes, GitHub writes, probes, releases or deployment.

## What changed
- R1: catalog admission detects indexed/nested resource identity collisions using pinned Boon's own draft traversal and URI normalization on isolated probe copies. Frozen original schema material remains unchanged. Tests cover application/support collisions, relative/normalized IDs, supported drafts/custom meta, non-conflicting bundles and const/examples payloads.
- R2: absent end_of_life_date is omitted on serialization; valid catalog output re-admits, while explicit dates survive.
- R3: original nested core object members are checked before Value erases duplicates, including arrays, hashes/bindings and escaped keys across all five kinds. Opaque extension values, schema fragments, resource configuration and unknown root additions are not recursively interpreted. The small private shape table follows current DTO object edges, not alternate schema validation.
- R4: fidelity diagnostic paths use actual numeric operation indices, including preceding unsupported entries, with operation IDs retained in escaped prose.
- R5: referenced diagnostics preserve document identity/version/path/category/offset. Missing context on malformed/duplicate bytes is filled from the exact reference; locationless missing/store failures point to the real manifest .id binding with escaped target details. Full/partial/unsupported/orphan paths tested.

Four production files: src/schema/catalog.rs, src/validator/{structural,fidelity}.rs, src/manifest/validator.rs. Four existing test files changed, one new tests/nested_duplicate_fields.rs. No manifests/locks/dependencies/public exports/schemas/version records/rules modified. See results/final-preservation.json and final-source-hashes.json.

## Verification
- Actual targeted RED/GREEN for R1/R2, R3, R4/R5 and final attribution repair; raw outputs and packet reports retained under results/.
- Final frozen/offline Cargo tests: **230 integration + 2 unit + 7 compile-fail doctests = 239 passed**. Conformance target still executes0; no conformance delivery inferred.
- Final frozen/offline all-target check, fmt check, Clippy all-targets -D warnings, Python15 and diff whitespace checks all pass. Exact commands/exits in results/final-checks.json.
- Fresh postrepair instrumented source coverage: **1329/1419 = 93.66% emitted production lines**,31actual test objects, fresh profiles. Canonical production source mappings only; lexical test-copy aliases, dependencies, unmapped DTOs and17inline test lines excluded. Not branch/native/full-spec/MSRV coverage. Exact commands/outputs in results/postrepair-coverage-*.
- All unrelated tracked-file/research hashes and rule links preserved; index and HEAD10a4717 unchanged.

## Final audit
Two bounded write workers and one read-only reviewer, two waves, no recursion or follow-up dispatches. Parent owns combined verification and one consolidated repair (consumed). Inherited model/effort/runtime isolation not independently attested; token usage unknown; no cost/savings claim. Review preceded the final narrow attribution repair; parent verified it, not an independent postrepair signoff.

Context7 resolved high-reputation /websites/serde_rs and queried map/sequence/duplicate visitor semantics. Official [Serde duplicate-field example](https://serde.rs/deserialize-struct.html) informed the existing RawFields boundary; actual fixes are verified against pinned dependencies/source/tests, not assumed from generic docs.

## Skipped checks
No native/provider sandbox qualification, CI query, Rust1.88 MSRV run, release/license/security audit or historical full documentation graders. No new downloads/installations; all Cargo execution frozen/offline in existing isolated cache with separate worker targets.

## Remaining risks
This fixes the five review defects, not the larger execution-isolation goal. BG-01/UD-1–UD-5 remain OPEN; incomplete JSON semantics/evidence/completeness/full conformance/downstream qualification are unchanged. Future DTO object additions must update the private duplicate-edge vocabulary alongside regressions. Byte offsets for ambiguous nested JSON remain approximate, not an exact-location guarantee.

## Next useful step
Review the local diff and explicitly request commit/push when ready to update PR3.
