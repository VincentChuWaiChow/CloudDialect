# Verified corpus preparation — not canonical admission

Within the remaining preparation window ending 03:32:02 UTC, parent-only work
made these exact batches review-ready without new agents or production changes:

| Set | Cases / bounded batches | Final actual runner checks | State |
| --- | --- | --- | --- |
| Positive | 12 / 7 | 19 matched + 19 source-derived content criteria | `9067a21`, unadmitted |
| Negative | 23 / 13 | 41 matched + 15 criteria | `bf83e90`, unadmitted |
| Raw round-trip | 3 / 2 | 10 matched, including temporary composite39 | Frozen drafts, unadmitted |

All 38 draft cases in 22 batches have exact input/oracle/hash ownership, maximum
two directories/four files per batch. The admitted corpus is still one seed.
Initial negative hypotheses matched22/23; the second empty-reasoning diagnostic
was independently traced through the schema and pinned Boon evaluator before
refinement. Its original mismatch and rationale are preserved. Newer MINOR is
Warn, not Fail; the other22 negative documents are expected to fail. Manifest
shape decoder errors explicitly retain null field paths, not invented locations.

Raw extension values include lexical numeric/Unicode/whitespace/order/nesting
features; tests do not substitute decoded Value equality. No fixture declaration,
approver/origin/capability string or opaque accepted=true establishes authority.
Provider-note topics remain unresolved rather than fabricating real-service proof.

The one submitted approval question remains **positive admission only** (12 cases,
24 files, new45-minute tranche). Negative and round-trip admission proposals are
review-ready but have not superseded that question or received approval. No
canonical/source/schema/manifest/lock changes or completed task marks in this
preparation. All 21 Spec2 tasks and UD/BG/native/consumer gates remain open; full
required corpus, fresh coverage/MSRV and legacy-checker reconciliation are still
unfinished. Do not treat the temporary39 result as a delivered/default-suite count.
