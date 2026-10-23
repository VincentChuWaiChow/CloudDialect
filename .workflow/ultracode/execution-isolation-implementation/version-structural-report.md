# Version / structural delivery — 2026-10-22

**Delivered bounded units2.4/2.5:** private document-version helper and real Validator::new(CatalogSnapshot)/validate_structural with structural-only Validate trait. Actual five initial-MVP JSON kinds are tested. **Original execution-isolation goal remains active/incomplete**; full canonical tasks/conformance, later phases, extension blockers and BG-01/native gates are not complete.

## Behavior and authority

Owner-approved profile: exact admitted stable precedence supports build metadata without inventing schema URIs. Same-MINOR unindexed PATCH, unindexed lower MINOR, all prereleases and valid core-u64 overflow are UnsupportedVersion; malformed SemVer is helper InvalidFormat/structural Version. Unknown stable MAJOR is UnsupportedMajorVersion. A stable MINOR above the highest admitted same-MAJOR MINOR may select an actually admitted MAJOR.0.0 base, including a nonzero newer-MINOR PATCH. No base or no stable profile fails closed. Full suffix syntax remains representable; pack ranges/dialect URLs are not document versions.

Trusted caller kind filters immutable admitted entries. Optional instance $schema must equal the actual selected entry URI, including build/future-MINOR cases. Compiler uses the actual closed snapshot adapter with format assertions, not instance shape, environment, default catalog or network-loader injection. Errors aggregate safe child/dot paths, original parseable ID/version and bounded approximate JSON offsets, without raw input values or engine/resource internals in descriptions. Duplicate root members reuse the same existing RawFields decoder, not duplicate-erasing Value conversion alone.

Newer-MINOR validates a separate owned view: original bytes/reported version stay intact; only the view gets effective base version and root-only projection. Exact escaped paths are in warning descriptions; field_name_verbatim remains the original key. Conditional/composed/local-ref declarations, declared wildcard/global constraints and dependent names remain checked. Closed nested fields still reject; existing open objects/freeform embedded schema content are not recursively stripped. Global object predicates retain all members for the unchanged base validator, rather than guessing names or dropping constrained data.

Public additions are only Validator/Validate at existing root/validator paths. Snapshot entries and shared decoder access are crate-private; no new public document namespace, accepting conversion, default validator, semantic success/panic stub, dependency, schema or rule change.

## Actual TDD evidence

| Sequence | RED | GREEN |
|---|---|---|
| Private helper |8d3589b: actual E0432 missing private helper/result |5695df0: same7tests; full143integration+2unit+7doctests/check/fmt/Clippy |
| Structural consumer |b4d8623: actual E0432 missing Validator/Validate; subsequent duplicate-boundary runtime13pass/1fail; d8c7ff1 wildcard runtime15pass/1fail;939b83e scope/dependency runtime16pass/2fail |3c2c2a1: same expanded18tests GREEN; full161integration+2unit+7doctests/check/fmt/Clippy |
| One consolidated review repair |c1c4bcf:19tests execute,17pass/2fail; global const/enum/cardinality mismatch and missing dependent-child path |a4ceec8: same19GREEN; final full162integration+2unit+7doctests/check/fmt/Clippy |

The private helper's exact source is initially test-included before production registration with its real consumer; no public API was invented for tests. Final structural tests execute the actual registered production helper. Root duplicate reuse required a distinct2.5U internal-visibility prerequisite (two small files), not a new parser or changed decoding semantics. Routine pre-GREEN Clippy corrections are retained in raw evidence; no failed setup is called RED.

Packet19 returned concrete static contract risks, integrated into source/tests/task text. Packet20 found one required P2 const/enum projection bypass. Parent reproduced it with actual Boon, covered the same global coverage gap for min/maxProperties plus a dependent child-path assertion, and used the sole consolidated repair. Forbidden instances now fail and allowed predicate/cardinality controls pass. **No independent postrepair sign-off**; reviewers did not execute tests/coverage.20 cumulative sidecars exhausted, zero recursion/followups;90-minute tranche retains14:38:03UTC original start, one repair consumed, no reset.

## Final postrepair verification

Commands and exits: results/version-structural-verified-checks.json; raw/presentation hashes: results/version-structural-evidence-manifest.json. Earlier version/structural/final logs are dated pre-integration/pre-review evidence, not proof of repaired source.

- **162 integration +2 unit +7 compile-fail doctests pass** (26 new integration tests relative to3a7d163). Actual diagnostics: E0596 once and E0277 six times. Empty conformance target remains0, not conformance delivery.
- Frozen all-target check, fmt check, Clippy-Dwarnings,15 current Python and8 selected historical P0 content cases pass. Historical full draft/scope/hash graders remain explicitly not claimed green; not rerun as this leaf's gates.
- Corrected actual registered production lines **701/738=94.99%** in results/version-structural-verified-production-coverage.json. Raw src totals718/755 include17inline loader-test lines, removed using actual llvm-cov show counters. Test-included helper alias, dependency/test files and unmapped DTOs are excluded. No branch/full-spec/native percentage claim. Private source-only helper coverage was44/44 before registration; final production module is measured separately, not double-counted.
- Scope/hash audit: results/version-structural-scope-audit.json. Nine source/test paths (two helper, five consumer, two decoder reuse) plus four reconciled Spec1 documents and this run's metadata. Research, dependencies/lock/notices, catalog bytes, AI rules/symlinks and custom scripts unchanged. All checkpoint commits reachable with SSH signature blocks; signer identity UNVERIFIED, signing policy retained.

## Limitations and remaining work

Opaque/external/dynamic or changed-$id root-reference projection scopes conservatively return Unsupported instead of guessing fields. Exact admitted versions still use Boon; all five actual current schema kinds and their conditional root metadata work. This is an explicit conservative implementation limitation, **not an owner-approved waiver or proof of arbitrary-schema compatibility**. Broader compatibility remains to be qualified before claiming it. Catalog layout still allows only one PATCH per kind/MINOR; this leaf does not silently redesign it. Minimum declared Rust version is not independently exercised merely by a build on the installed compiler.

Structural validity never recognizes extension semantics or clears mandatory blockers. Reserved-prefix warnings and actual mandatory extension enforcement remain unit2.6. Manifest/semantic/fidelity/behavioral checks, artifact hashing, complete fixture inventory/runner, compatibility policy and downstream handoff remain unfinished. BG-01/UD1–UD5 remain OPEN; no native probes, approved disposable environment, measured budgets, releases, deployments or provider actions.

User-authorized normal feature-branch publication follows the evidence commit; independently verified exact remote SHA is recorded outside this self-referential report at /tmp/clouddialect-version-structural-publication.json and in the conversation. No force push. New parallel-worker allocation was requested for the next distinct wave, not assumed or used to retry this repair.
