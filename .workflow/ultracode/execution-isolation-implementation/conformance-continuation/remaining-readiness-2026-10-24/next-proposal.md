> Historical proposal: both decisions were explicitly approved on2026-10-24.
> Delivered by the separate [bounded harness/five-pair tranche](../core-preservation-2026-10-24/final-report.md).
> The earlier deadline was not extended; approval started a new45-minute scope.

# Proposed next local tranche — requires fresh explicit approval

## Decisions first
1. Clarify namespace-collision class as independent local non-recognition of A/B
   untrusted claims with unknown mandatory semantics; never claim cross-document
   conflict detection/authenticated ownership. Registry governance remains separate.
   Alternatively require a new trusted conflict-admission contract; do not guess.
2. Authorize minimal test-harness-only core verification preservation comparison
   (at least tests/outcomes before/after actual typed roundtrip), with real RED/
   GREEN and independent review, in tests/support/fixtures.rs and
   tests/conformance_controls.rs. No public production API or library change.

## Optional same 45-minute local tranche after approval
Admit these five exact P0semantic fixture pairs (input+expected only), each under
crates/clouddialect-contracts/fixtures/, from semantic-contract.json and
verification-semantic-drafts. Existing batch boundaries retained; a partial
neighbor batch is not marked wholly complete merely by these cases:
- 3.2.53: `integration/verification-consistency/duplicate-test-ids/` — exact semantic Fail/category/path/description.
- 3.2.53: `integration/verification-consistency/unmapped-obligations/` — exact semantic Fail/category/path/description.
- 3.2.55: `integration/verification-consistency/invalid-hash-length/` — exact semantic Fail/category/path/description.
- 3.2.55: `integration/verification-consistency/approval-result-id-mismatch/` — exact semantic Fail/category/path/description.
- 3.2.56: `integration/verification-consistency/approval-result-version-mismatch/` — exact semantic Fail/category/path/description.

13 actual draft controls match, including five valid-baseline counterexamples and
temporary composite47. Canonical remains42. Expected errors derive from the
existing verification consistency code before execution, not captured output.
No semantic consistency pass authenticates approvals/origin/protected run state.

New45-minute clock begins only on approval; no reset of current09:53:34deadline.
As-needed agents already permitted; select only useful bounded reviews, no hidden
fan-out. No production-library/schema/API/dependency changes, native/provider
probes, pushes/releases/deployment or runtime gate closure. Stop on new contract
ambiguity or unexpected source repair. Five data pairs and harness two-file unit
are separate bounded ownership scopes; related approved doc clarification only.
