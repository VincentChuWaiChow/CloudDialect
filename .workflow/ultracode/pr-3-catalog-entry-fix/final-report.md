# CatalogEntry Codex feedback correction

## Outcome
Remaining reported Codex P2 extension-loss defect is fixed locally. The second reported defect (absent-date serialization) was already fixed in0b3feab and its regression still passes. No commits, pushes, thread resolution, review requests, PR comments, releases or deployment.

## What changed
- `src/schema/catalog.rs`: reuse existing duplicate-aware raw decoder for CatalogEntry; add flattened public `extensions: RawObject`; route all5required fields plus optional end_of_life_date exactly through the existing macro. Unknown namespaced and bare entry values retain encoded JSON bytes. Dates remain omitted when None. No schemas/dependencies/exports/versions changed.
- `tests/catalog_extension_roundtrip.rs`:3new tests cover exact nested values after typed/inventory decoding, serialization/re-admission and second serialization; escaped strings, whitespace, exponent spelling, arrays, null/false, opaque duplicate payload members; duplicate decoded names and Unicode aliases; core duplicates and explicit-date retention.

**API impact:** public CatalogEntry struct literals now require extensions, including an empty RawObject. This is explicitly documented and is not a backward-compatible struct-literal claim. Repository search found no literal callers. Current user request to fix this exact reported loss was implemented as the bounded initial-MVP repair announced before code; no release or downstream compatibility certification follows.

## Verification
- Genuine RED: focused target8tests,6pass/2actual behavior failures (lost extension, duplicate extension accepted), exit101. `results/red.txt`.
- Minimal source correction, then exact focused target8pass/exit0. `results/green.txt`.
- Full frozen/offline suite **233integration+2unit+7compile-fail doctests=242pass**, Python15pass; frozen all-target check, fmt check, Clippy all-targets -D warnings and diff whitespace checks pass. `results/checks.json`.
- Fresh instrumented coverage: **1329/1419=93.66% emitted production lines**,31actual objects/fresh profiles. Canonical mapped source lines only; lexical test-copy aliases, dependencies, unmapped DTOs and inline test lines excluded. This is not coverage of all DTO declarations, branches, full spec, native runtime or MSRV. `results/coverage-result.json` and command receipts.
- Independent read-only reviewer found no actionable issue and confirmed exact reviewed hashes. No postreview edits or repairs. `results/01-review.md`.
- Only2authorized tracked files changed; other baseline hashes/research/symlinks, Git index and HEAD0b3feab preserved. `results/preservation.json`.

## Final audit
One native read-only reviewer, no recursion/follow-up dispatches; parent implementation/integration, zero repairs, within30minute cap. Inherited model/effort/runtime isolation not independently attested; usage/cost unknown. All Cargo frozen/offline in existing temporary cache/targets; no installs/fetches.

Context7 resolved /websites/serde_rs and queried flatten/try_from/Option omission. Official [Serde field attributes](https://serde.rs/field-attrs.html) support flatten-map capture and None omission; exact value-byte and duplicate guarantees are from pinned-source/tests, not inferred from generic examples.

## Skipped checks
No native/VM/provider probes, Rust1.88 MSRV qualification, CI certification, full historical documentation graders or release/legal/security audit. Codex remote review was still running on committed0b3feab at preflight; it has not reviewed this local unpublished change.

## Remaining risks
GitHub threads remain unresolved until authorized publication/verification. BG-01/UD-1–UD-5 and incomplete JSON/evidence/conformance/native scope remain open and unchanged. Future external struct-literal consumers must provide the new field.

## Next useful step
Review local diff; commit/push only on explicit request, then independently verify Codex feedback on the new head before merge.
