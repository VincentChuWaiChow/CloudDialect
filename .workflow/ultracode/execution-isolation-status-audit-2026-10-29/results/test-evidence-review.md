# Independent static test-evidence audit

Role: `test_coverage_quality_review_agent`; its role forbids suite/coverage
execution. No tests, probes, network actions or edits performed. Configured
Sol/high/read-only and schema checked; effective runtime settings unverified.

Meaningful bounded assertions exist in `evidence_completeness.rs`,
`public_api_compatibility.rs`, negative conversion doctests, document-resolution,
catalog snapshot, verification-semantic and traceability tests. Runner controls
reject wrong/missing diagnostics, core-data rewrites and lexically changed
extension bytes despite equivalent parsed values. These are stronger than
existence or mock-call-only checks.

Current source has 10 completeness tests, 101 fixture oracle files and 11
runner-control test functions. One (`fixture_worker`) is a subprocess helper
whose ordinary default branch does nothing; do not count it as an independent
control. Parent must execute and record actual pass totals separately.

The timeout control at `conformance_controls.rs:412–446` and helper kill/wait
in `tests/support/fixtures.rs` cover a direct child. They do not prove descendant
domain cleanup, confinement or network observation. Non-Linux observation is
weaker still. Synthetic document stores verify data boundaries, not authentic
backend authorization or observations. Fresh coverage is unverified.

Keep TASK-006/BG-01 and all runtime tasks open. Refresh stale 9/42/286 snapshots
only as bounded current upstream evidence. Preserve the historical document
grader: it rejects all checked boxes and pins old input hashes by design; a
failure today is not a reason to weaken it or claim native controls passed.
