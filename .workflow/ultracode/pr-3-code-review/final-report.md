# PR 3 code review — final report

## Outcome
Request changes: five independently reproduced P2 correctness defects at frozen head10a4717c3b48fb544be568da37fd1eb42e5c4379 vs masterb3c89ff03fd03457e45808fff9d5c3b255407d9e. No production fixes or GitHub writes.

### R1 [P2] Reject nested schema resource identities that shadow indexed material
Location: `crates/clouddialect-contracts/src/schema/catalog.rs:227–233`.
Trigger: Index a support URI as false, then embed a nested $id with the same URI and type:object in the evidence schema and reference that URI.
Observed: CatalogSnapshot::from_parts succeeds; schema_bytes(support) returns false, yet validate_structural passes an evidence object using the nested resource.
Consequence: The exact admitted URI-to-material mapping is not authoritative during validation. This concerns inconsistent trusted catalog content, not an untrusted-instance takeover.
Recommended correction: Detect conflicting schema resource identities across indexed roots and nested schema resources before admitting/compiling the snapshot; preserve legitimate non-conflicting bundling.
Reproduction: `nested_resource_identity_shadows_indexed_bytes` in results/adversarial-probes.rs; output in results/adversarial-probes.txt.

### R2 [P2] Omit absent optional catalog dates when serializing
Location: `crates/clouddialect-contracts/src/schema/catalog.rs:50–51`.
Trigger: Admit the committed catalog index, serialize the CatalogIndex, then re-admit it with unchanged material.
Observed: Serialization emits end_of_life_date:null for every entry whose date was omitted; re-admission rejects the output because the property requires a string.
Consequence: The public catalog Serialize implementation produces schema-invalid bytes from the delivered valid catalog, breaking write/round-trip consumers.
Recommended correction: Add skip_serializing_if=Option::is_none and parse-serialize-revalidate coverage.
Reproduction: `admitted_catalog_serialization_fails_its_own_schema` in results/adversarial-probes.rs; output in results/adversarial-probes.txt.

### R3 [P2] Check ambiguous duplicate core fields below the document root
Location: `crates/clouddialect-contracts/src/validator/structural.rs:44–48`.
Trigger: Pass a contract containing operation_identity:{call_site:first,call_site:second}.
Observed: validate_structural returns Pass, while serde_json::from_slice::<BehavioralContract> on the identical original bytes rejects duplicate field call_site.
Consequence: The root-only duplicate guard leaves nested core data subject to last-value-wins interpretation and structural success is inconsistent with the typed document parser. This is not an acceptance or demonstrated authority bypass.
Recommended correction: Apply duplicate-aware checking at nested core-object boundaries without reconstructing or losing opaque extension-value bytes; add a validator/decoder consistency regression.
Reproduction: `nested_duplicate_core_fields_pass_structure_but_typed_decode_rejects` in results/adversarial-probes.rs; output in results/adversarial-probes.txt.

### R4 [P2] Address fidelity failures with the operation array index
Location: `crates/clouddialect-contracts/src/validator/fidelity.rs:91–93`.
Trigger: The first upload_object operation declares full, but its correctly resolved contract has an adaptation-required obligation.
Observed: The error path is operations["upload_object"].behavioral_fidelity, although operations is a Vec and the actual path is operations[0].behavioral_fidelity.
Consequence: Tools and IDE consumers cannot navigate the reported field; the current fidelity test asserts this incorrect path.
Recommended correction: Enumerate operations and emit the numerical index; retain the operation identity in safe descriptive context.
Reproduction: `fidelity_diagnostic_indexes_an_array_by_string` in results/adversarial-probes.rs; output in results/adversarial-probes.txt.

### R5 [P2] Retain referenced-document identity and path in resolution failures
Location: `crates/clouddialect-contracts/src/validator/fidelity.rs:40–46`.
Trigger: A referenced stored contract has a matching hash and identity but is missing operation_identity.
Observed: Its Structural diagnostic becomes pack-scoped contracts.upload_object.operation_identity with a generic Required field is missing description. That field cannot exist on ContractRef, and original document identity/version are overwritten.
Consequence: The consumer cannot identify the contract field needing repair. The same remapping exists for unsupported/orphan references in manifest/validator.rs:69-75.
Recommended correction: Keep the original referenced-document diagnostic, or point to the actual manifest .id reference and explicitly retain the referenced location in structured/safe context.
Reproduction: `contract_failure_is_reattributed_to_nonexistent_pack_field` in results/adversarial-probes.rs; output in results/adversarial-probes.txt.

## What changed
Only this review artifact directory. Existing two unrelated research files remain untracked.
## Verification
Fresh frozen/offline Rust suite: 215 integration + 2 unit + 7 compile-fail doctests pass; conformance target still zero. Fresh Python suite: 15 pass. Five adversarial probes confirm reported defects using actual built frozen production library. Read exact commands/exits in results/check-receipts.json. Default cache failed before successful use of existing isolated cache; no download.
## Final audit
Three read-only native reviewers, one wave, no recursion/follow-up dispatches or repairs. Parent inspected changed scripts/rules and cross-module paths, reproduced/deduplicated findings. Inherited model/effort and runtime isolation not independently attested; no cost/token-savings claim.
## Skipped checks
No fresh Clippy/fmt/coverage/MSRV, CI, historical full documentation graders, native/provider probes or release checks. This is code review, not a qualification/acceptance audit.
## Remaining risks
Bounded source/spec/test review is not exhaustive. Known unfinished downstream features and BG-01/UD gates stay explicit and open.
## Next useful step
Authorize a separate scoped fix tranche for R1–R5 with failing regressions, then re-review integrated results.
