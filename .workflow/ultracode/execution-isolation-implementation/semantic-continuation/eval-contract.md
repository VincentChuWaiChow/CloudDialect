# Eval contract

## Goal
Implement the specified non-accepting semantic boundary, not runtime isolation or acceptance.

## Success criteria
Invalid structure never queries the store. Mandatory namespaces block all five kinds without relying on registry membership or payload. Every core contract evidence reference resolves unique immutable evidence by the supplied scoped port, validating identity/version/kind/hash/active schema. Unknown classification is semantically valid but remains a later completeness blocker. Verification documents reject duplicate test IDs, unmapped obligations, noncanonical hash lengths/bytes, invalid date/UTC even under a relaxed explicit schema, and mismatched acceptance self-reference. Valid non-pass records may be semantically valid; no result authenticates origins/approval or accepts.

## Integration surfaces
Existing Validator/Validate, DocumentStore, SemanticResult and private document resolver. No DTO/schema version/dependency change. Add one real Validate method per approved target API. Preserve diagnostics of referenced documents; locate missing-store failures at the referring core field without leaking store error content.

## Downstream consumers
Spec 1 completeness, conformance and real Spec 2 compatibility handoff. All remain incomplete until their own tests/evidence exist.

## Required checks
Focused actual RED then same-target GREEN; active snapshot positive/negative, trait dispatch, no ambient fallback, no extension authority, structural/semantic phase independence, full frozen test/check/fmt/Clippy, Python tests, coverage >=80% with scope disclosed and exact diff/preservation audit.

## Deliverables
Private verification_semantics.rs and focused tests; semantic.rs with real public consumer and integration tests; parent registration; evidence/report and prerequisite progress notes only when verified.

## Blocking conditions
Missing tools/dependencies, unresolved semantic interpretation, exhausted agent/repair/time allocation, failed integrated checks. Native qualification, runtime mechanisms and owner gates cannot be replaced with synthetic tests.
