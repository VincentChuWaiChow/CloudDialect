# Three non-native integration admission proposal — not approved

Existing bounded batches (two files/input+expected each):
- 3.2.38: `integration/blocking-gap/unknown-evidence-mandatory-obligation/`.
- 3.2.39: `integration/semantic-resolution-failure/manifest-bad-contract-ref/`.
- 3.2.40: `integration/artifact-hash-mismatch/tampered-candidate/`.
Canonical destination root: `crates/clouddialect-contracts/fixtures/`.
Exact phases, raw bytes, hashes, stored-record metadata and diagnostics are frozen
in contract.json/drafts. Behavioral result is Blocked; manifest semantic result
is Fail with exact referring field; artifact helper result is boolean false with
no invented outcome. Every phase is non-accepting.

11 actual runner checks match, including three isolated batch counts, wrong
oracles, store outage, genuine candidate bytes and temporary composite42 with all
other prepared drafts/current seed. 10 content/integrity criteria pass. Synthetic
store fixtures prove only the actual API/code boundary, not backend authorization.
The network-disabled/unresolvable pair in batch3.2.41 remains unprepared; actual
approved-environment evidence is mandatory and cannot be replaced by mocks.

This proposal does not supersede the pending positive-only admission question.
Any canonical admission requires explicit bounded approval. No source changes,
new agents/dependencies, native/provider probes, pushes or releases. Full task21,
full corpus, coverage/MSRV/traceability/consumer and all Spec2 gates remain open.
