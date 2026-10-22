# Catalog snapshot and embedded loading — 2026-10-22

## Delivered scope
Approved remaining Spec1 prerequisite leaves2.2/2.3, on `feat/execution-isolation`. Original execution-isolation goal remains active and incomplete. Canonical rules remain `.agents/rules/engineering.md`; no rule, dependency, lock, notice, custom-script or unrelated research change is part of this tranche.

- `CatalogSnapshot::from_parts` freezes inventory-checked caller-owned bytes. SHA-256 binds the exact index plus all supplied material using P0-D's domain separator, UTF-8 byte ordering and checked u64 big-endian length prefixes. Four independent Python vectors cover ordering, UTF-8/empty values and concatenation ambiguity. Exact index whitespace changes identity. No mutable digest/bytes access or provenance claim.
- `CatalogSnapshot::embedded` embeds the entire catalog via pinned include_dir0.7.4. `schema::get_embedded_schema` returns static bytes only for indexed resources. Unknown digest-bound metadata is not implicitly resolvable; both ambient catalog overrides are irrelevant in fresh child processes.
- `CatalogUrlLoader::new(snapshot)` implements actual pinned Boon0.6.1 `UrlLoader::load -> Result<Value, Box<dyn Error>>`. No network/live-file fallback or public network-loader injection. Real compiler tests cover all five schemas, custom indexed support, local fragments, unknown file/HTTP/meta references, malformed material and explicit format-assertion controls. The adapter alone does not configure every caller compiler; production Validator is still unfinished.

## Review and single repair
Two bounded read-only sidecars: packet15 contract interactions, packet16 catalog integration.16 cumulative dispatches, zero recursion/followups, within the 60-minute catalog tranche. Model/effort inherited and unverified; behavioral restrictions are not separate sandbox-isolation evidence.

Packet16 found that engine-owned built-ins precede UrlLoader and could silently bypass supplied support bytes. Actual runtime RED proves the adapter returns `false` while the engine accepts an object against its built-in. Repair rejects identifiers the actual pinned compiler cannot dispatch as that exact external resource through an in-memory deny callback. This avoids a copied production URI table and broad host-prefix rules; no supplied schema content is compiled by that admission check or acquired externally. Positive built-ins remain engine-owned. All19 pinned resource paths plus latest-draft alias under HTTP/HTTPS (40 cases), normalized/fragment aliases, ordinary same-host names and custom URNs are covered. Parent verified the repair; no independent post-repair signoff is claimed.

Clippy caught `.err().expect()` in the regression; fixed idiomatically without a lint suppression or dumping schema bytes. The failed initial check remains in `results/catalog-repaired-check-results.json`; final checks are `results/catalog-verified-check-results.json`.

## TDD checkpoints and evidence
- Snapshot RED `a60173b`: E0432 missing API. GREEN `00b70fa`:8 focused tests, full regression.
- Loader RED `78dcd1a`: E0432/E0599 missing APIs. Initial GREEN `05b81cf`:9 focused tests; review finding explicitly still open at that checkpoint.
- Collision RED `551e8cd`: actual compiled test fails admission assertion after proving byte substitution. GREEN `cc65d4a`:12 focused integration tests and2 source-local unit tests; full verified checks below.

Captured RED/GREEN logs and normalization hashes are in `results/catalog-evidence-manifest.json`. Raw copies remain under `/tmp`; presentation logs strip trailing whitespace only. Source/schema/digest bytes are not normalized.

Final frozen suite: **114 integration tests,2 unit tests,7 compile-fail doctests**. All pass; the old empty conformance target does not count as conformance delivery. Actual negative diagnostics are E0596 for immutable digest access and six E0277 phase conversions, not broken imports. Check, fmt, Clippy `-D warnings`,15 Python layout/scaffold checks and8 historical P0 content cases pass. Old full draft/hash/scope graders are not represented as green; their earlier documented failures are retained, and they were not rerun as catalog acceptance checks. A fresh baseline preservation audit replaces their obsolete scope assumptions.

Coverage: **314/330 emitted production lines =95.15%**. See `results/catalog-production-coverage-summary.json` and actual LLVM line counters. Excludes tests, dependencies, unmapped DTOs and17 executable lines from the two inline unit tests. The canonical coverage helper's raw source-file summary now includes those inline tests, so its raw95.39% is not the production-only figure. LLVM's attempted name allowlist did not alter per-file summary counters and is not relied on. Functions/regions are not re-reported after the line-only correction. No branch, full-spec or native-runtime coverage claim.

Length overflow uses checked conversion. On this host, usize cannot exceed u64, so an allocated-length overflow cannot be dynamically reproduced; vectors and the checked source guard are scoped evidence, not a fabricated overflow test.

## Remaining work and owner decisions
[Owner decisions](contract-reconciliation-decisions.md): root-only newer-MINOR projection; resource-specific dimension assumptions/coverage; optional mandatory-extension declarations across all five JSON kinds with trusted implemented recognition. Those amendments require their own documentation/model/schema/validator TDD units. Unasked version edge-case policy must still be reconciled. Artifact digest verification, structural/semantic/completeness validators, manifest validation, full conformance fixtures/runner, downstream compatibility and upstream handoff remain unfinished.

BG-01/UD1–UD5 remain OPEN. No approved disposable qualification environment, native offline/isolation measurement, finite measured budgets, production/provider mutation, release or deployment. Snapshot construction/content identity is not host admission, signature verification or acceptance. Native-client loading of AI-rule symlinks is also unverified.

User-authorized normal feature-branch commit/push only. SSH signing policy retained; signature blocks do not independently verify signer identity without an allowed-signers configuration. Independent remote-SHA verification is required after the final evidence commit, recorded outside that self-referential commit in `/tmp/clouddialect-catalog-publication.json` and the conversation. No force push or release.
