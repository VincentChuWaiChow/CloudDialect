# Result 07: model contract check

## Summary
Independent read-only reviewer `/root/model_contract_check`: **FAIL against literal extension-byte preservation; otherwise units 1.3/1.4 pass source inspection**. No writes or test execution by reviewer.

## Required finding
source_documents.rs stores extension values as serde_json::Value; source_document_models.rs asserts JSON-value equality. Design property 2 and requirement 10.2 require byte-for-byte preservation. Escapes, whitespace and object ordering are not retained (e.g. encoded \u0061 can serialize as a). A passing value round-trip is not literal-byte proof.

## Parent disposition
Accepted, not waived. User clarification requested: exact encoded extension-value bytes with approval of the existing serde_json raw_value feature, versus explicitly amended decoded-value semantics. No answer recorded yet. No feature/dependency/manifest change made. Units 1.4/1.5 source DTOs remain partial with respect to this guarantee; no full-conformance claim. Original source bytes remain separate in StoredDocument.

## Passed checks
Exact fields/labels, nested/freeform representation, DTO-versus-validation distinction, caller-scoped object-safe DocumentStore, missing/error distinction, shared ContentHash String fields. No other required finding.

## Verification and limits
Read-only source/spec comparison only; tests not independently rerun. Seventh cumulative sidecar explicitly authorized. No follow-up. Final integrated reviewer remains reserved as eighth.
