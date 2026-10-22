# Packet 07: upstream model/spec ambiguity check

## Objective
Read-only comparison of new units 1.3/1.4 with canonical task 9, design 4.1/4.2 and P0-A/C/E. Identify concrete mismatches that would invalidate later validators, not speculative new features.
## Context
User explicitly approved remaining Spec 1 prerequisites, stopping at unresolved version/dependency decisions; no commits/probes/push/release. User approved two additional read-only sidecars, cumulative maximum eight. This is the seventh; one later integrated reviewer reserved. One response, no followups or recursive agents. Parent implements verification DTOs/harness concurrently; do not inspect their half-written files as finished work.
## Sources
validator/references.rs, validator/source_documents.rs, tests/reference_contracts.rs, tests/source_document_models.rs; canonical task 9/design sections 4.1/4.2 and P0 A/C/E; prerequisite-models-eval.md. One nearby requirement hop if needed.
## Ownership
Read-only behavioral instruction, not runtime isolation. Parent owns all edits. Shared ContentHash moved into references to avoid cyclic allocation with unit 1.5, retaining algorithm:String/value:String exactly.
## Do
Check field coverage, exact labels, required DTO fields versus later JSON Schema conditions, unknown data preservation, object-safe store contract and clear non-authenticating semantics. Flag genuine source/spec contradictions. Assess whether JSON-value round-trip differs from lexical-byte preservation; do not claim original bytes are retained by DTO serialization. Return at most three actionable findings with paths/lines and limited PASS/FAIL verdict.
## Do not
Edit, execute Cargo/build scripts, mutate cache, use network, inspect secrets/config, commit, spawn or follow up. Prefix shell commands with RTK. No security or publication approval attestation.
## Expected output
Findings, required/optional distinction, evidence, changed files none, residual limitations. Review only currently complete 1.3/1.4; final reviewer owns all units later.
## Verification
Source/document inspection only. Tests not independently executed.
## Handoff format
Verdict, findings with evidence, required parent actions, unknowns and unchanged files.
