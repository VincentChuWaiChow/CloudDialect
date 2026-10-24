# Round-trip admission proposal — not approved/applied

Exactly canonical task-21 round-trip subset, existing batches:
- 3.2.36: `round-trip/extension-field-preservation/behavioral-contract/` and
  `round-trip/extension-field-preservation/evidence-record/` (four files).
- 3.2.37: `round-trip/extension-field-preservation/verification-result/` (two files).
All canonical paths are under `crates/clouddialect-contracts/fixtures/`; each owns
only input.json + expected.json. Exact draft bytes/hashes and raw extension value
are fixed in contract.json. Each expected document uses structural pass, exact
kind, empty diagnostics and round_trip=true.

All 10 actual runner checks match: three-family run, two bounded batches, three
missing-extension rejections, three ordinary baseline passes and composite39 with
prepared positives/negatives/current seed. Typed DTO serialization is compared
using exact raw values, not decoded Value equality. Whitespace, 1e+04, Unicode
escape, nested ordering and negative zero are deliberately non-normalized. Nested
accepted=true remains opaque data and grants no authority.

Admission needs separate explicit scope/time approval; this proposal does not
supersede the pending positive-only question. No canonical files/source changes,
new agents/dependencies, native/provider probes, pushes, releases or runtime gate
closure. After future approval copy exact pairs, run scoped/full tests and record
counts; task21 remains open for its integration/P0/native cases. No broad coverage,
consumer or native correctness is inferred from these three cases.
