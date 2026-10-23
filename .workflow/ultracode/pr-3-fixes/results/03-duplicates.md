# R3 nested core duplicate correction

## Summary
Preserve existing duplicate-aware RawFields at every current document core object boundary, including object arrays, without reinterpreting arbitrary extension values, schema fragments or behavioral resource configuration. A private small shape/edge table follows the current document DTOs; this is lexical ambiguity checking, not a second schema validator or catalog fallback. Leaf fields retain opaque values. Wrong value types remain the actual active schema's responsibility.
## Evidence
RED duplicates-red.txt: three tests execute, one opaque-positive passes and two duplicate regressions fail on actual unexpected structural Pass. GREEN duplicates-green.txt: all3pass, exercising all5kinds, decoded escaped names, resource assumptions/known behavior, verification hashes/bindings/tests, opaque exact bytes and newer-MINOR unknown root additions.
## Files changed
src/validator/structural.rs; tests/nested_duplicate_fields.rs.
## Decisions
No blanket typed deserialize gate or recursive arbitrary-payload filtering. No exports/schema/dependencies/versions/gates changed.
## Risks
The private object-edge table must stay in sync with future document-model changes. Byte offsets on nested parse failures remain approximate per existing interface; no exact-location claim.
## Verification run
Frozen/offline targeted tests via existing /tmp/clouddialect-phase-cargo and /tmp/clouddialect-phase-target. Parent combined suite/check/fmt/Clippy/Python pass before final review.
## Setup corrections
First postimplementation compilation encountered the catalog worker's intermediate compile typo, corrected by that worker in its active tranche. First executable run exposed a test fixture problem: switching inferred evidence to observed incorrectly retained reasoning_chain, which the closed schema forbids. Removed the stale field from that fixture only (duplicates-fixture-failure.txt); no schema/source loosening. These are not counted as behavioral RED or postreview repairs.
## Handoff
Full integrated static review pending.
## Open questions
None for current5document shapes.
