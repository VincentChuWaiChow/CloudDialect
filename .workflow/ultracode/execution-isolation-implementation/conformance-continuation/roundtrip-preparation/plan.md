# Raw extension round-trip readiness — drafts only

Exactly the three named task-21 families in bounded batches 3.2.36–3.2.37:
behavioral-contract, evidence-record and verification-result. Six draft files.
Use real typed DTO parse/serialize and the existing runner's raw-value byte
comparison; Value equality is not sufficient. Frozen payload includes whitespace,
1e+04, a Unicode escape, ordering, nested values and negative zero. A nested
accepted=true is opaque data, not an acceptance outcome or authority.

Verify 3 draft cases, isolated batch counts, removal-of-extension rejection with
round_trip still required, ordinary positive baseline with round_trip disabled,
and composite39 with all prepared positive/negative drafts/current seed. Nothing
is admitted; positive owner question is still separate. No code/agents/dependencies,
probes or pushes. Preparation deadline remains 03:32:02 UTC.
