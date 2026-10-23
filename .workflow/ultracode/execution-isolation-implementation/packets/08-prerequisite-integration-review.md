# Packet 08: prerequisite integration review

## Objective
Final read-only review of units 1.3–1.6 plus user-approved exact-extension-value repair, against canonical task 9/design/P0. Find concrete errors before later schema/validator consumers.
## Context
Remaining prerequisite code authorized with version/new-dependency gates intact, no commits/probes/push/release. User explicitly approved two additional read-only agents (cumulative max eight), and exact encoded extension-value bytes plus raw_value on existing serde_json pin. This is eighth/final agent, no followups/recursion. Packet 07 found literal-fidelity gap; parent added real assertion RED, then raw decoder/byte regressions GREEN. Clippy large CompileError boxed; no lint suppression. Current full suite/check/fmt/Clippy/scaffold all pass. Coverage is parent-measured concurrently; no claim yet.
## Sources
crates/clouddialect-contracts/src/validator/{references.rs,raw_objects.rs,source_documents.rs,verification_types.rs,mod.rs}, src/errors/mod.rs, src/lib.rs, Cargo.toml; tests/{reference_contracts.rs,source_document_models.rs,verification_models.rs,schema_harness.rs,support/mod.rs}. Spec 1 canonical task9, design4.1–4.3 and property2, requirement10.2, P0-A/C/E, implementation-plan units1.3–1.6; run prerequisite-models-eval.md and exact-byte RED/GREEN logs. One nearby source hop only.
## Ownership
Read-only instruction, not enforced tool isolation; parent owns all writes. Parent freezes named sources during review; coverage/report writes only.
## Do
Review exact fields/labels, genuine JSON raw-value preservation (root/nested/freeform, from_slice/reader, escapes/whitespace/numeric spelling), unknown/duplicate member handling, data-only DTO semantics, caller-injected store port, six-way phase conversion separation, and actual test-only Boon loader/format controls. Check raw decoder abstractions are necessary/reused rather than speculative. Treat qualified key identity and exact encoded value bytes as the approved boundary, not full document/key formatting. Test/API presence does not prove real backend scope, schema validity, crypto or authenticity.
Return PASS/FAIL with <=3 actionable required findings and lines; explicitly separate deferred later-unit work from actual defects. No independently rerun tests claimed.
## Do not
Edit, execute Cargo/build scripts, mutate caches, use network, inspect secrets/global configuration, commit/push, spawn/follow up, or approve gates. Do not ask for postponed production validators/native proof as a requirement of these DTO/test-helper units.
## Expected output
Verdict, bounded findings, evidence, files changed none and residual limitations. Parent owns one consolidated repair phase already in progress; no new broad rewrite.
## Verification
Source/spec and saved-output review only. Full native/runtime qualification is not in scope.
## Handoff format
Summary, required findings or none, limitations, files changed and recommended parent disposition.
