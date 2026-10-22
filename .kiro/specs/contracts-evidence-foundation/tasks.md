# Implementation Plan: CloudDialect Contracts & Evidence Foundation

**Later publication action approval:** the user explicitly requested git commit and git push for this work. Those actions are now authorized; the earlier no-commit/no-push implementation limit is historical. Native probes, deployment and releases remain unauthorized.


## Current physical domain layout
User requested domain-first organization on2026-10-22. Document DTO implementation is now private `src/documents/{behavioral_contract,evidence,verification,references,raw_objects}.rs`; `src/validator/mod.rs` preserves every existing public document import as a compatibility facade. Existing `manifest/`, `schema/` and `errors/` remain cohesive domains/support. Historical source paths below describe original task ownership; new edits use the current owner paths. No new public document namespace, aggregate framework, validator behavior or gate approval follows from this move. Shared repository AI guidance is canonical `.agents/rules/engineering.md` with relative client entrypoint symlinks.

## Overview

This plan builds the `clouddialect-contracts` Rust crate, its bundled JSON Schema catalog, conformance fixture suite, and supporting documentation. It covers Spec 1 of 3; Spec 2 (Execution & Isolation) and Spec 3 (Python OSS-to-S3 Migration Pack) both depend on artifacts produced here.

**Current continuation status (2026-10-22):** user approved P0-A–F for implementation planning, then separately authorized the remaining Spec 1 prerequisite units in [implementation-plan.md](implementation-plan.md), TDD and no commits/probes/pushes/releases. User attests this is a new project and approves document/catalog **1.0.0** and crate API **0.1.0** as the initial MVP versions. This is not an independent external-publication audit or security-gate approval. Exact encoded extension JSON value bytes are required; raw_value on the existing pinned serde_json is separately approved.

**Delivery is partial.** DTOs, schemas, catalog inventory, frozen SHA-256 snapshot identity and embedded-only schema loading have bounded verification; full validators, artifact hash verification, conformance and delivered downstream compatibility remain unfinished. See the [current run report](../../../.workflow/ultracode/execution-isolation-implementation/final-report.md) for actual evidence and outstanding work. Canonical task checkmarks are conservative; BG-01 and Spec 2 UD-1–UD-5 remain OPEN.

`BehavioralResult` full implementation, the Rust CLI binary, worker isolation, and Python transformation logic are explicitly out of scope.

---

## Task Dependency Graph

```json
{
  "waves": [
    { "id": 0, "tasks": ["1"] },
    { "id": 1, "tasks": ["2", "3", "4", "5", "6", "9"] },
    { "id": 2, "tasks": ["7", "15"] },
    { "id": 3, "tasks": ["8"] },
    { "id": 4, "tasks": ["10"] },
    { "id": 5, "tasks": ["11", "12", "16", "17"] },
    { "id": 6, "tasks": ["13", "18", "19"] },
    { "id": 7, "tasks": ["14", "20", "21", "23", "24"] },
    { "id": 8, "tasks": ["22", "25"] },
    { "id": 9, "tasks": ["26"] }
  ]
}
```

---

## Bounded implementation progress (2026-10-22)
Implemented/checked prerequisite units:1.2–1.11, resource DTO amendment1.4R and2.1–2.3 in the [delivery plan](implementation-plan.md). This includes resource-specific dimension bindings and all16 known/unknown coverage combinations per dependency, in addition to earlier DTO/schema/catalog work; it is not all validators or full conformance. Canonical checkmarks below remain conservative until every task obligation passes; see [latest resource evidence](../../../.workflow/ultracode/execution-isolation-implementation/resource-contract-report.md), [catalog evidence](../../../.workflow/ultracode/execution-isolation-implementation/catalog-delivery-report.md) and [contract decisions](../../../.workflow/ultracode/execution-isolation-implementation/contract-reconciliation-decisions.md).

## Tasks

P0 graph correction: task 17's typed fidelity function depends on task 15 and P0-E store/types from task 9, not task 13. It precedes task 13, which integrates it. Fixture cases are written before their implementation units even though full fixture-suite delivery is grouped later; documentation approval is not valid runtime RED/GREEN evidence.
All five JSON instance schemas permit optional nonempty `$schema`, but trusted DocumentKind plus supported format_version selects the actual admitted schema. If provided, the URI must resolve and match that selection. Use full SemVer 2.0.0 grammar (including prerelease/build) consistently; syntax validity and supported-version policy are separate.

- [x] 1. Initialize `clouddialect-contracts` Rust crate scaffold
  - Create the directory `crates/clouddialect-contracts/` at the repository root.
  - Write `crates/clouddialect-contracts/Cargo.toml` as a library crate (no `[[bin]]` section) with the following pinned dependencies (exact versions to be resolved at implementation time from crates.io, keeping Cargo.lock committed):
    - `boon` — JSON Schema Draft 2020-12 validation (`UrlLoader` trait, MIT/Apache-2.0)
    - `toml` — TOML 1.0 parsing with serde integration (MIT/Apache-2.0)
    - `include_dir` — compile-time directory embedding (MIT/Apache-2.0)
    - `uuid` — UUID v4 generation with `v4` feature (MIT/Apache-2.0)
    - `serde` with `derive` feature (MIT/Apache-2.0)
    - `serde_json` (MIT/Apache-2.0)
    - `regex` — reserved-namespace detection (MIT/Apache-2.0)
  - Create `crates/clouddialect-contracts/src/lib.rs` with module declarations: `pub mod schema; pub mod validator; pub mod manifest; pub mod errors;` and top-level re-exports of public types.
  - Create stub `mod.rs` files for each declared module: `src/schema/mod.rs`, `src/validator/mod.rs`, `src/manifest/mod.rs`, `src/errors/mod.rs`.
  - Create `crates/clouddialect-contracts/tests/conformance.rs` as an empty integration test file (populated in Task 22).
  - Design reference: Section 10 (Rust Crate Structure).
  - _Requirements: Req 1, Req 8, NFR-1_

  **Acceptance criteria:**
  - `cargo check` passes on the skeleton crate with no errors.
  - The module tree matches the layout in design Section 10.
  - No `[[bin]]` section is present in `Cargo.toml`.

- [ ] 2. Write behavioral contract JSON Schema
  - Create `catalog/schemas/behavioral-contract/1.0/schema.json`.
  - The schema must declare `"$schema": "https://json-schema.org/draft/2020-12/schema"` and `"$id": "https://schemas.clouddialect.io/behavioral-contract/1.0.0/schema.json"`.
  - Required top-level fields (all in `required` array): `id`, `format_version`, `operation_identity`, `inputs`, `outputs`, `side_effects`, `resource_dependencies`, `obligations`.
  - `id`: `type: string`, `format: uuid` — UUID v4, hyphenated lowercase.
  - `format_version`: full SemVer 2.0.0 pattern from design, including valid prerelease/build syntax; reject leading-zero core/numeric-prerelease and empty suffixes. Supported versions remain a separate compatibility check.
  - `operation_identity`: object with required `call_site` string; optional `wrapper_chain` array of strings.
  - `inputs`, `outputs`: objects (JSON Schema fragments).
  - `side_effects`: array of objects, each requiring a `description` string, optional `evidence_ref` string.
  - `resource_dependencies`: array of objects, each requiring `identity`, `permissions` (array of strings), `configuration` (object); optional `known_behavior` object with nonempty `ordering`, `consistency`, `retry`, `failure` strings; optional `unresolved_assumptions` array referencing the resource-specific assumption definition. Each such assumption requires nonempty `assumption_id`, `description`, `reason` and `behavior` in `ordering | consistency | retry | failure`. Every dependency must cover each dimension by a known declaration or matching populated assumption; both may coexist. Do not impose this discriminator on generic evidence/obligation assumptions (owner-approved RC-1).
  - `obligations`: array of objects, each requiring `obligation_id` string and `behavioral_fidelity` (enum: `equivalent-within-contract`, `adaptation-required`, `architectural-change-required`, `unsupported`, `unknown`); each obligation must satisfy a `oneOf`: either has `evidence_refs` (array, minItems 1) or has `unresolved_assumption` ($ref to `#/$defs/unresolved_assumption`).
  - `behavioral_notes`: optional array; each entry requires `note_type` (string), `description` (string), `evidence_ref` (string).
  - Extension mechanism: add `"patternProperties": { "^[a-zA-Z][a-zA-Z0-9\\-]+:": {} }` and `"unevaluatedProperties": false`.
  - Define `$defs.unresolved_assumption`: object requiring `assumption_id`, `description`, `reason` (all strings).
  - Design reference: Section 4.1. DR-6 for extension mechanism.
  - _Requirements: Req 1.1, Req 2.1, Req 2.2, Req 2.3, Req 2.4, Req 2.5, Req 2.6_

  **Acceptance criteria:**
  - The schema file is valid JSON and contains `$schema` and `$id` fields (Req 1.1).
  - A document missing any field in the `required` array fails validation (Req 2.1).
  - A document with an obligation missing both `evidence_refs` and `unresolved_assumption` fails validation (Req 2.4).
  - A document with `behavioral_fidelity` value `"invalid-value"` fails validation (Req 2.3).
  - Every resource covers all four behavior dimensions independently; missing, unrelated or unbound assumptions cannot cover a dimension or another dependency. All16 known/unknown combinations and partial knowledge with both declarations must validate; malformed declared entries must still fail (Req 2.2, approved RC-1).
  - A document with an unknown community extension key (e.g., `"acme:custom": "value"`) passes validation (Req 2.6, 10.2).

- [ ] 3. Write evidence record JSON Schema
  - Create `catalog/schemas/evidence-record/1.0/schema.json`.
  - The schema must declare `"$schema": "https://json-schema.org/draft/2020-12/schema"` and `"$id": "https://schemas.clouddialect.io/evidence-record/1.0.0/schema.json"`.
  - Required top-level fields: `id`, `format_version`, `claim`, `classification`.
  - `id`: `type: string`, `format: uuid`.
  - `format_version`: SemVer pattern as in Task 2.
  - `claim`: `type: string`.
  - `classification`: enum `["observed", "inferred", "unknown"]`. The field is required; no default is permitted (Req 3.2).
  - Use `if/then/else` chaining to enforce conditional required fields:
    - If `classification == "observed"`: require `provenance` object with required `source_artifact_identity` and `analysis_tool_identity` strings.
    - If `classification == "inferred"`: require `reasoning_chain` array with `minItems: 1`, items as strings.
    - If `classification == "unknown"` (else branch): require `unresolved_assumptions` array with `minItems: 1`; each item requires `assumption_id`, `description`, `reason` strings.
  - Extension mechanism: same `patternProperties` + `unevaluatedProperties: false` pattern as Task 2.
  - Design reference: Section 4.2. DR-6 for extension mechanism.
  - _Requirements: Req 1.1, Req 3.1, Req 3.2, Req 3.3, Req 3.4, Req 3.5_

  **Acceptance criteria:**
  - A document with no `classification` field fails validation (Req 3.2).
  - An `inferred` record with an empty `reasoning_chain` array fails validation (Req 3.3).
  - An `unknown` record with an absent `unresolved_assumptions` field fails validation (Req 3.4).
  - An `observed` record with `provenance` present passes validation (Req 3.1).
  - A document with a valid community extension field passes validation (Req 3.5).

- [ ] 4. Write verification result JSON Schema
  - Create `catalog/schemas/verification-result/1.0/schema.json`.
  - The schema must declare `"$schema": "https://json-schema.org/draft/2020-12/schema"` and `"$id": "https://schemas.clouddialect.io/verification-result/1.0.0/schema.json"`.
  - Required top-level fields: `id`, `format_version`, `candidate`, `pack`, `toolchain`, `configuration`, `binding`, `tests`, exactly as the P0-C design §4.3 schema.
  - `candidate`: object requiring `identity` (string) and `content_hash` object; `content_hash` requires `algorithm` (enum: `["sha-256", "sha-384", "sha-512", "sha3-256", "sha3-512"]`) and `value` (string).
  - `pack`/`toolchain`: nonempty id, SemVer version and required content_hash; configuration is an ArtifactRef to exact immutable secret-free bytes, not inline configuration secrets.
  - `binding`: P0-C tenant/project/run/attempt references, immutable input/dependencies/suite/oracle/policy ArtifactRefs and nonempty unique mandatory obligation_ids. Document strings are not authentication.
  - `tests`: nonempty array; each entry requires test_id, nonempty unique obligation_ids, closed pass/fail/skip/error outcome and origin_identity. Non-pass requires `failure_description: { "detail": "safe text" }`, detail length 1–2000 Unicode characters; unknown failure-object fields or a legacy string fail.
  - `acceptance`: optional; requires approver identities, UTC timestamp, matching verification_result DocumentRef and approval_evidence ArtifactRef. Structural validity never proves approval authenticity or acceptance. Semantic checks enforce exact digest lengths/UTC/reference/obligation/test uniqueness; Spec 2 compares protected expected identities and observations.
  - Extension mechanism: same `patternProperties` + `unevaluatedProperties: false` as Task 2.
  - Design reference: Section 4.3.
  - _Requirements: Req 1.1, Req 4.1, Req 4.2, Req 4.3, Req 4.4, Req 4.5, Req 4.7_

  **Acceptance criteria:**
  - A document with an empty `tests` array fails validation (Req 4.7).
  - A fail/skip/error entry without structured failure_description.detail, with empty/overlong detail, or with a string/unknown object key fails validation (Req 4.3).
  - A document with `algorithm: "md5"` in `content_hash` fails validation (Req 4.2).
  - A document with an `acceptance` object missing `approver_identities` fails validation (Req 4.4).
  - A document with a valid community extension field passes validation (Req 4.5).
  - Missing configuration/binding/pack-toolchain digests or test obligation links fail; positive fixture includes all required P0-C fields. Required-field/semantic format migration reviewed before publication.

- [ ] 5. Write catalog index JSON Schema
  - Create `catalog/schemas/catalog-index/1.0/schema.json`.
  - The schema must declare `"$schema": "https://json-schema.org/draft/2020-12/schema"` and `"$id": "https://schemas.clouddialect.io/catalog-index/1.0.0/schema.json"`.
  - Required top-level fields: `format_version`, `schemas`, `resolver_resources`.
  - `schemas`: array with `minItems: 1`; each entry requires `schema_id` (string, the `$id` URI), `document_type` (enum: `["behavioral-contract", "evidence-record", "verification-result", "catalog-index", "extension-registry"]`), `format_version` (string), `catalog_path` (string), `status` (enum: `["current", "supported", "end-of-life-declared"]`).
  - When `status == "end-of-life-declared"`, `end_of_life_date` (string, `format: date`) is required — enforce with `if/then`.
  - `resolver_resources`: P0-D separate array of exact schema_id/catalog_path support mappings; validate uniqueness across both URI sets and admitted path/material completeness. Empty only with verified offline engine built-ins. Do not overload application document_type with fake pack-manifest or private instance records.
  - Design reference: Section 4.5.
  - _Requirements: Req 1.6, Req 9.4_

  **Acceptance criteria:**
  - A document missing `schemas` fails validation.
  - A schema entry with `status: "end-of-life-declared"` but no `end_of_life_date` fails validation.
  - The catalog index schema itself is referenced in `catalog/index.json` (after Task 7).

- [ ] 6. Write extension registry schema and seed files
  - Create `catalog/schemas/extension-registry/1.0/schema.json` defining the structure of the extension registry document. The registry object requires `format_version` (string) and `extensions` (array, may be empty). Each extension entry requires `namespace_prefix` (string), `owner` (string), `purpose` (string), `registration_date` (string, `format: date`).
  - Create `catalog/extension-registry.json` as a valid, initially empty registry: `{ "format_version": "1.0.0", "extensions": [] }`.
  - Design reference: Section 3.1, Req 10.6.
  - _Requirements: Req 10.6_

  **Acceptance criteria:**
  - `catalog/extension-registry.json` validates against the extension registry schema.
  - The file is present in every release artifact (verified by presence in repository at the correct path).
  - An entry with a missing required field fails validation against the registry schema.

- [ ] 7. Populate `catalog/index.json`
  - Create `catalog/index.json` listing all five schema documents from Tasks 2–6. This file must itself be valid against the catalog index schema (`catalog/schemas/catalog-index/1.0/schema.json`).
  - Each entry in the `schemas` array must specify: `schema_id` (the `$id` URI of the schema), `document_type`, `format_version: "1.0.0"`, `catalog_path` (e.g., `schemas/behavioral-contract/1.0/schema.json`), and `status: "current"`.
  - The five application schemas are behavioral-contract, evidence-record, verification-result, catalog-index and extension-registry. No pack-manifest JSON Schema exists; TOML manifests use programmatic validation. Bundle any additional Draft 2020-12 meta-schema/vocabulary resources required by the actual engine as resolver support material, not project document instances.
  - Design reference: Section 3.1.
  - _Requirements: Req 1.6, Req 9.4_

  **Acceptance criteria:**
  - `catalog/index.json` is valid JSON and validates against `catalog/schemas/catalog-index/1.0/schema.json`.
  - All five schema documents in Tasks 2–6 are represented in the index.
  - Each `catalog_path` entry points to an existing file relative to the catalog root.

- [ ] 8. Implement `CatalogUrlLoader` and catalog embedding
  - Create `crates/clouddialect-contracts/src/schema/catalog.rs`: define `CatalogIndex` and `CatalogEntry` structs deserializable from `catalog/index.json` using `serde_json`.
  - Create `crates/clouddialect-contracts/src/schema/embedded.rs`: use `include_dir::include_dir!` macro to embed the entire `catalog/` directory at compile time into a static. Expose a function `get_embedded_schema(uri: &str) -> Option<&'static [u8]>` that looks up a URI in the embedded catalog index and returns the corresponding schema bytes.
  - Planning source inspection verified checksum-matched boon 0.6.1: UrlLoader::load returns serde_json::Value or boxed error; Compiler::new() plus use_loader(Box<dyn UrlLoader>), with enable_format_assertions available. Preserve/recheck exact source evidence before coding; compile/offline/date behavior remains unverified. Do not implement the former byte-returning loader wording.
  - Create `crates/clouddialect-contracts/src/schema/loader.rs` with CatalogSnapshot and CatalogUrlLoader using exact P0-D constructors/immutable digest encoding. Embedded or explicitly supplied frozen parts only; neither historical environment variable is read. Resolve exact admitted index URIs, reject traversal/duplicate/missing entries and prohibit runtime network/live-filesystem fallback.
  - Create `crates/clouddialect-contracts/src/schema/mod.rs` re-exporting CatalogSnapshot and CatalogUrlLoader, plus root public re-exports. No public network-loader injection.
  - Write unit tests in `src/schema/loader.rs` (in `#[cfg(test)]` module):
    - Test: validation succeeds with network disabled (mock network as unavailable).
    - Test: an unresolvable URI returns an error containing the URI string (Req 1.3, 9.3).
    - Test: a valid schema URI resolves to non-empty bytes.
    - Test: explicit admitted snapshot bytes work; hostile CLOUDDIALECT_SCHEMA_CATALOG/CLOUDDIALECT_CATALOG_DIR values have no effect. Verify immutable digest vectors and missing/tampered/duplicate/traversing paths and offline meta-schema resolution.
  - Design reference: Section 3.1, 3.4 (CatalogUrlLoader), DDR-05.
  - _Requirements: Req 1.2, Req 1.3, Req 9.1, Req 9.2, Req 9.3, Req 9.4_

  **Acceptance criteria:**
  - All unit tests pass with `cargo test`.
  - The loader never calls any networking code.
  - Unresolvable URI error contains the exact URI string (Req 1.3).
  - Explicit snapshot construction is exercised; environment selection is absent and native zero-network evidence is reported separately from mocked unavailable-network tests.

- [ ] 9. Define error types and result types
  - Create `crates/clouddialect-contracts/src/errors/mod.rs` with the following public types:
    - `ErrorCategory` enum variants: `Structural`, `Semantic`, `Version`, `UnresolvableReference`, `Unparseable`, `UnsupportedVersion`, `UnsupportedMajorVersion`, `Unsupported`, `BlockingGap`.
    - `ValidatorError` struct: `document_id: Option<String>`, `format_version: Option<String>`, `category: ErrorCategory`, `field_path: Option<String>`, `description: String`, `byte_offset: Option<usize>`.
    - `StructuralOutcome` enum: `Pass`, `Warn`, `Fail`.
    - `SemanticOutcome` enum: `Pass`, `Fail`.
    - `BehavioralOutcome` enum: `Deferred`, `EvidenceComplete`, `Blocked`.
    - `StructuralResult` struct: `document_id: Option<String>`, `format_version: Option<String>`, `outcome: StructuralOutcome`, `errors: Vec<ValidatorError>`, `warnings: Vec<ValidatorWarning>`.
    - `SemanticResult` struct: `document_id: Option<String>`, `outcome: SemanticOutcome`, `errors: Vec<ValidatorError>`.
    - `BehavioralResult` struct: `candidate_id: String`, `outcome: BehavioralOutcome`, `verification_result: Option<DocumentRef>`, `test_results: Vec<TestResult>`, `blocking_gaps: Vec<BlockingGap>`; P0-A projection invariants are mandatory and all outcomes are non-accepting.
    - Define P0-A/C/E DocumentKind, DocumentRef, StoredDocument, DocumentStore, TestResult/TestOutcome, FailureDescription and verification document/binding/artifact/hash types exactly matching design §4.3; root re-exports public types. Keep execution errors separate from ValidatorError. Structural/semantic results remain task-9 fields, not separate StructuralError/SemanticError wrappers.
    - `ManifestValidationResult` struct: `outcome: ManifestOutcome`, `structural_errors: Vec<ValidatorError>`, `semantic_errors: Vec<ValidatorError>`, `warnings: Vec<ValidatorWarning>`.
    - `ValidatorWarning` struct: `field_name_verbatim: String`, `description: String`.
    - `BlockingGap` struct: `candidate_id: String`, `obligation_id: String`, `assumption_text: String`.
    - `ManifestOutcome` enum: `Pass`, `Fail`.
  - No `From`/`Into` implementations between `StructuralResult`, `SemanticResult`, and `BehavioralResult` — these types are non-interchangeable (Req 8.1).
  - Design reference: Section 3.4.
  - _Requirements: Req 8.1, Req 8.2, Req 11.1_

  **Acceptance criteria:**
  - `cargo check` passes on the error module.
  - No `From` or `Into` trait implementations exist between `StructuralResult`, `SemanticResult`, and `BehavioralResult`.
  - `StructuralResult` has no field whose name contains "semantic", "behavioral", "accepted", or "correct" (Req 8.2).
  - Consumer signatures agree with P0-E; every BehavioralOutcome is non-accepting; no verification reference/empty test projection in completeness results; projection mismatch is rejected. Type presence alone does not close BG-01.

- [ ] 10. Implement `validate_structural()`
  - Create `crates/clouddialect-contracts/src/validator/structural.rs`.
  - Implement a Validator owning an explicitly constructed CatalogSnapshot (not an ambient unit default), exposing:
    ```rust
    pub fn validate_structural(&self, kind: DocumentKind, document: &[u8]) -> StructuralResult
    ```
  - Implementation steps:
    1. Parse `document` as JSON. If parsing fails, return `StructuralResult` with `outcome: Fail` and a `ValidatorError` with `category: Unparseable` and `byte_offset` set to the approximate failure position.
    2. Extract `format_version` from the parsed document (if present). If absent or not a string, record a structural error and continue.
    3. Select exact admitted catalog schema by trusted DocumentKind plus supported format_version. Optional instance `$schema` must resolve and match that selection; absent is valid, unknown/mismatched blocks. All five instance schemas permit the optional property. Never infer kind from attacker-controlled shape.
    4. Install the snapshot-backed adapter using the actually verified pinned boon compiler API, then compile. No guessed constructor or fallback resolver.
    5. Validate the parsed document against the compiled schema using `boon`. Collect all errors as `ValidatorError` instances with `field_path` in dot-notation.
    6. Scan all field keys for extension prefixes (keys containing `:`). Pass prefixes to the reserved-namespace checker (implemented in Task 12). Collect any warnings.
    7. Return `StructuralResult` with aggregated errors, warnings, and the appropriate outcome.
  - Add `validate_structural` to the `Validate` trait in `src/validator/mod.rs`.
  - Write unit tests:
    - Test: valid minimal behavioral contract passes.
    - Test: document with missing required field fails with `category: Structural`.
    - Test: non-JSON bytes return `category: Unparseable`.
    - Test: Property 1 — valid document serialized back and re-validated produces same `outcome` (schema round-trip, Req 2.1, 3.1, 4.1).
  - Design reference: Section 3.4, Key Interaction 5.1.
  - _Requirements: Req 1.2, Req 2, Req 3, Req 4, Req 8.1, Req 11.1, Req 11.2_

  **Acceptance criteria:**
  - A valid minimal behavioral contract passes with `outcome: Pass`.
  - A document with a missing required field returns `outcome: Fail` and a `ValidatorError` identifying the field path (Req 11.1).
  - Non-JSON input returns `category: Unparseable` with a byte offset (Req 11.1).
  - `StructuralResult` contains no field implying semantic or behavioral correctness (Req 8.2).
  - Property 1 round-trip test passes.
  - Caller DocumentKind with absent instance `$schema` succeeds for valid input; unknown/mismatched URI, wrong kind and missing version fail without heuristic or network fallback. Prerelease/build versions are representable; unsupported ones still fail compatibility policy.

- [ ] 11. Implement version compatibility logic
  - Create `crates/clouddialect-contracts/src/validator/version.rs`.
  - Implement `check_version_compatibility(format_version: &str, supported_major: u64) -> VersionCheckResult` where `VersionCheckResult` is an internal enum: `Supported`, `MinorNewer(String)`, `UnsupportedMajor(String)`, `InvalidFormat`.
  - Logic:
    - Parse `format_version` as SemVer (MAJOR.MINOR.PATCH). If not parseable, return `InvalidFormat`.
    - If MAJOR matches `supported_major` exactly and MINOR == supported MINOR: `Supported`.
    - If MAJOR matches and MINOR > supported MINOR: `MinorNewer(format_version.to_string())` — non-blocking warning (Req 7.3).
    - If MAJOR != `supported_major`: `UnsupportedMajor(format_version.to_string())` — blocking error (Req 7.4, 6.4).
  - Integrate into `validate_structural()`:
    - `MinorNewer` → append a `ValidatorWarning` to the result; continue validation using MAJOR.0.0 base schema.
    - `UnsupportedMajor` → return `StructuralResult` with `outcome: Fail` and a `ValidatorError` with `category: UnsupportedMajorVersion` containing the unsupported version, error category string, and a reference to compatibility documentation.
    - `InvalidFormat` → return `StructuralResult` with `outcome: Fail` and `category: Version`.
  - Write unit tests:
    - Test: same MAJOR/MINOR → `Supported`.
    - Test: same MAJOR, higher MINOR → `MinorNewer`.
    - Test: higher MAJOR → `UnsupportedMajor`.
    - Test: Property 7 — unsupported MAJOR produces `category: UnsupportedMajorVersion` and no partial validation output.
  - Design reference: Section 3.4 (version compatibility), Section 7 (Error Taxonomy).
  - _Requirements: Req 6.4, Req 7.3, Req 7.4_

  **Acceptance criteria:**
  - A document with a MINOR-newer version within the same MAJOR passes structural validation with a non-blocking warning identifying the version difference (Req 7.3).
  - A document with an unsupported MAJOR returns `category: UnsupportedMajorVersion` with all three required fields (Req 7.4).
  - Property 7: `UnsupportedMajorVersion` result contains no partial validation output.

- [ ] 12. Implement reserved-namespace detection and extension enforcement
  - Create `crates/clouddialect-contracts/src/validator/extensions.rs`.
  - Implement a compiled static regex (use `once_cell::sync::Lazy` or `std::sync::OnceLock`) matching the reserved namespace family: `^(core|core-[a-z0-9-]*|cdc|cdc-[a-z0-9-]*)$` with case-insensitive flag (Req 10.3).
  - Implement `scan_extension_fields(document: &serde_json::Value) -> (Vec<ValidatorWarning>, Vec<ValidatorError>)`:
    - Walk all keys in the document object (top level only; behavioral documents are not deeply nested objects with arbitrary extension keys).
    - For each key containing `:`, extract the prefix (everything before the first `:`).
    - Check the prefix is valid ASCII extension prefix grammar (`^[a-zA-Z][a-zA-Z0-9\-]+$`; minimum 2 chars, ASCII only). If not valid grammar, `unevaluatedProperties: false` in the schema will have already rejected it; no additional action needed.
    - If the prefix matches the reserved-namespace regex: emit a `ValidatorWarning` with `field_name_verbatim` set to the exact key as it appears in the document and `description` set to `"prefix '<prefix>' is reserved for core use"`. Structural validation continues (Req 10.3).
  - Implement `check_mandatory_extensions(document: &serde_json::Value, validator: &CatalogUrlLoader) -> Vec<ValidatorError>`:
    - Look for a `required_extensions` array in the document (Req 10.4).
    - For each prefix in `required_extensions`, check whether the validator recognizes it (i.e., it maps to a known catalog schema or registered namespace). If unrecognized, emit a `ValidatorError` with `category: BlockingGap` and description identifying the unrecognized mandatory extension.
  - Integrate `scan_extension_fields` into `validate_structural()` (after schema validation).
  - Write unit tests:
    - Test: Property 5 — reserved namespace warning is non-blocking — `CORE:field`, `Core-Validation:field`, `cdc:field`, `CDC:field`, `Cdc-Evidence:field` each produce a warning and `outcome` is still `Pass` (Req 10.3).
    - Test: a document with an unrecognized mandatory extension produces a blocker and cannot be accepted (Req 10.4, Property 6).
    - Test: a community extension field `acme:custom` produces no warning and passes structural validation (Req 10.2).
    - Test: Property 8 — all five reserved-prefix variants produce exactly one warning with the verbatim field name.
  - Design reference: Section 3.4 (Reserved-namespace detection), Key Interaction 5.4.
  - _Requirements: Req 10.2, Req 10.3, Req 10.4, Req 10.5_

  **Acceptance criteria:**
  - Property 8: `CORE:field`, `Core-Validation:field`, `cdc:field`, `CDC:field`, and `Cdc-Evidence:field` all produce a warning; none cause `outcome: Fail` (Req 10.3).
  - A valid community extension field `acme:custom` passes structural validation without warnings (Req 10.2).
  - Property 6: a mandatory unknown extension produces a blocker; document cannot be marked accepted (Req 10.4).
  - An extension field attempting to override a core verification result field does not affect the core outcome (Req 10.5).

- [ ] 13. Implement `validate_semantic()`
  - Create `crates/clouddialect-contracts/src/validator/semantic.rs`.
  - Implement:
    ```rust
    pub fn validate_semantic(&self, kind: DocumentKind, document: &[u8], store: &dyn DocumentStore) -> SemanticResult
    pub fn validate_manifest_semantic(&self, toml_bytes: &[u8], store: &dyn DocumentStore) -> SemanticResult
    ```
  - JSON entrypoint resolves contract evidence refs via P0-E DocumentStore; TOML entrypoint checks exact contract id/version and fidelity through that store. Schema catalog never contains project instances or resolves their identities. Use actionable UnresolvableReference/semantic diagnostics with field paths.
  - Both entrypoints perform corresponding structural/local validation before reference checks. Validate returned reference identity/version/type/content hash/bytes/schema; missing/ambiguous/corrupt/unavailable/cross-project results block. No ambient filesystem, cross-project or network fallback.
  - Verification JSON semantic checks include unique test IDs, obligation membership, exact hash encoding, UTC timestamp and matching approval-result reference. Trusted expected context/obligation completeness, observation and approval authentication remain Spec 2 obligations, not inferred from a document's binding claims.
  - Integrate fidelity mapping cross-document check (from Task 17) into this phase.
  - Write unit tests:
    - Test: a manifest referencing a non-existent contract by identifier returns `SemanticResult` with `outcome: Fail` and `category: UnresolvableReference` (Req 8.3).
    - Test: a valid document with all references resolvable returns `SemanticResult` with `outcome: Pass`.
    - Test: Property 4 — a document that passes structural validation may independently fail semantic validation (`SemanticResult` is distinct from `StructuralResult`; no shared fields implying correctness).
  - Design reference: Section 3.4, Key Interaction 5.2.
  - _Requirements: Req 8.2, Req 8.3_

  **Acceptance criteria:**
  - A manifest with a non-existent contract reference returns `SemanticResult { outcome: Fail }` with an error identifying the unresolvable reference (Req 8.3).
  - `SemanticResult` has no field implying behavioral correctness (Req 8.2).
  - Property 4: `validate_structural` pass and `validate_semantic` fail can coexist on the same document without contradiction.

- [ ] 14. Implement blocking-gap report and artifact hash staleness detection
  - Create `crates/clouddialect-contracts/src/validator/behavioral.rs`.
  - Implement `check_evidence_completeness(contract: &BehavioralContract, candidate_id: &str, store: &dyn DocumentStore) -> BehavioralResult` using P0-E scoped validated evidence records:
    - Iterate over all obligations in the contract.
    - For obligations with `evidence_refs`: for each referenced evidence record, check if the classification is `unknown`. If yes, append a `BlockingGap { candidate_id, obligation_id, assumption_text }` to the result.
    - For obligations with `unresolved_assumption`: always append a `BlockingGap`.
    - If any `BlockingGap` entries exist, return `BehavioralResult { outcome: BehavioralOutcome::Blocked, blocking_gaps, candidate_id }`. The transformation MUST NOT be promoted to accepted state.
    - If no gaps, return `BehavioralResult { outcome: BehavioralOutcome::EvidenceComplete, ... }`; set verification_result to None and test_results to []. Missing/corrupt/ambiguous/unavailable evidence creates non-accepting diagnostics, not assumed completeness.
  - Implement `verify_artifact_hash(result_doc: &VerificationResult, candidate_bytes: &[u8]) -> bool`:
    - Extract `candidate.content_hash.algorithm` and `candidate.content_hash.value` from the result document.
    - Compute the hash of `candidate_bytes` using the specified algorithm.
    - Return `true` if the computed hash matches the recorded value, `false` otherwise.
    - When `false`, the acceptance result is invalid and must not be used for any downstream decision (Req 4.6).
  - Note: BehavioralOutcome::Deferred means not evaluated; this task implements Blocked/EvidenceComplete completeness checks only. No outcome accepts, and Spec 2 may populate a test projection only from the same immutable validated verification document with independent observation checks.
  - Write unit tests:
    - Test: a contract where all obligations have `observed` or `inferred` evidence → `outcome: EvidenceComplete`.
    - Test: a contract where a mandatory obligation has `unknown` evidence → `outcome: Blocked` with a `BlockingGap` containing `obligation_id` and `assumption_text` (Req 3.6, 11.4).
    - Test: modified candidate bytes → `verify_artifact_hash` returns `false` (Req 4.6).
    - Test: Property 6 — mandatory unknown evidence produces a blocking-gap preventing acceptance.
  - Design reference: Section 3.4, Key Interaction 5.3.
  - _Requirements: Req 3.6, Req 4.6, Req 11.4_

  **Acceptance criteria:**
  - A mandatory obligation with `unknown` evidence produces `BehavioralResult { outcome: Blocked }` with a `BlockingGap` containing all three required fields (Req 3.6, 11.4).
  - Tampered candidate bytes cause `verify_artifact_hash` to return `false` (Req 4.6).
  - Property 6 test passes.

- [ ] 15. Define manifest struct definitions
  - Create `crates/clouddialect-contracts/src/manifest/types.rs`.
  - Define the following Rust structs, all derived with `serde::Deserialize` for TOML deserialization:
    - `PackManifest`: top-level struct with fields `pack: PackSection`, `source: SourceSection`, `target: TargetSection`, `schema_refs: SchemaRefsSection`, `compatibility: CompatibilitySection`, `capabilities: CapabilityDeclaration`, `operations: Vec<OperationEntry>`, `toolchain: Vec<ToolchainEntry>`, `contracts: std::collections::HashMap<String, ContractRef>`, `x: Option<toml::Value>`.
    - CapabilityDeclaration/CapabilityRequirement/CapabilityAction exactly per P0-B, with explicit supported format version, required list (empty allowed), exact fields, closed actions, unique IDs and known operation references; no `[x]` grants or unknown-field swallowing.
    - `PackSection`: `id: String`, `version: String`, `name: String`, `description: String`.
    - `SourceSection`: `cloud: String`, `service: String`, `language: String`, `language_version_range: String`.
    - `TargetSection`: `cloud: String`, `service: String`.
    - `SchemaRefsSection`: `contract_schema_version: String`, `evidence_record_schema_version: String`, `verification_result_schema_version: String`.
    - `CompatibilitySection`: `catalog_min: String`, `catalog_max: String`.
    - `OperationEntry`: `id: String`, `behavioral_fidelity: ManifestFidelity`, `assumptions: Option<Vec<String>>`, `limitations: Option<Vec<String>>`.
    - `ManifestFidelity` enum: `Full`, `Partial`, `Unsupported` — serde-renamed to lowercase strings `"full"`, `"partial"`, `"unsupported"`.
    - `ToolchainEntry`: `name: String`, `version_range: String`.
    - `ContractRef`: `id: String`, `format_version: String`.
  - Design reference: Section 3.3, 4.4.
  - _Requirements: Req 5.1, Req 5.2, Req 5.3, Req 5.4, Req 5.5_

  **Acceptance criteria:**
  - `cargo check` passes on the types module.
  - `ManifestFidelity` deserializes correctly from TOML strings `"full"`, `"partial"`, `"unsupported"`.
  - `PackManifest` fails to deserialize from TOML missing any required section.

- [ ] 16. Implement `validate_manifest()`
  - Create `crates/clouddialect-contracts/src/manifest/validator.rs`.
  - Implement:
    ```rust
    pub fn validate_manifest(&self, toml_bytes: &[u8]) -> ManifestValidationResult
    ```
  - Parse `toml_bytes` into `PackManifest` using the `toml` crate. If parsing fails, return `ManifestValidationResult` with `outcome: Fail` and a structural error with `category: Unparseable`.
  - Enforce all eight programmatic rules from design Section 3.3:
    1. `pack.id` must match the dot-separated reverse-domain-name grammar: `^[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)+$` (Req 5.1).
    2. `pack.version`, `compatibility.catalog_min`, `compatibility.catalog_max` must parse as SemVer 2.0.0 (Req 5.1, 5.4).
    3. Each `operations[*].behavioral_fidelity` must be one of `full`, `partial`, `unsupported` (Req 5.2).
    4. Each `contracts.*` entry must carry `id` (non-empty string) and `format_version` (SemVer); the entry must NOT include any key other than `id` and `format_version` (detecting inline contract body) (Req 5.3).
    5. If any operation has `behavioral_fidelity` of `full` or `partial`, `toolchain` array must be present and non-empty (Req 5.5).
    6. `[x]` fields are accepted; they must not shadow any top-level core key (Req 5.6).
    7. An operation with `behavioral_fidelity: unsupported` is retained and accessible through the operation listing interface; validate that it is not omitted (Req 5.7).
    8. Require P0-B capabilities table/version/list and validate exact typed fields/actions/IDs/operation references. Request declaration never grants authority; Spec 2 separately resolves resources and denies prohibited worker actions (Req 5.8).
  - Return errors as `ValidatorError` instances with appropriate `field_path` values.
  - Write unit tests:
    - Test each of the 8 rules independently with passing/failing cases; include empty explicit capabilities, unknown capability versions/actions/fields, duplicate IDs and `[x]` escalation.
    - Test: manifest with `full` operation and empty toolchain → `outcome: Fail`, error on `toolchain` field path (Req 5.5).
    - Test: manifest with `unsupported` operation → operation is present in the parsed result (Req 5.7).
    - Test: manifest with inline contract body → `outcome: Fail` (Req 5.3).
  - Design reference: Section 3.3.
  - _Requirements: Req 5.1, Req 5.2, Req 5.3, Req 5.4, Req 5.5, Req 5.6, Req 5.7, Req 11.3_

  **Acceptance criteria:**
  - Each of the 8 manifest validation rules has passing/failing tests; local validation does not claim external reference/fidelity checks from validate_manifest_semantic.
  - A manifest with a `full`/`partial` operation and an empty toolchain section returns `outcome: Fail` (Req 5.5).
  - An `unsupported` operation is present in the parsed result after validation (Req 5.7).
  - An inline contract body is rejected (Req 5.3).

- [ ] 17. Implement fidelity enum mapping semantic check
  - Add `crates/clouddialect-contracts/src/validator/fidelity.rs`.
  - Implement `check_fidelity_mapping(manifest: &PackManifest, store: &dyn DocumentStore) -> Vec<ValidatorError>` against the P0-E exact-version store. Dependencies: tasks 9 and 15; complete before task 13 integration.
    - For each operation in the manifest with `behavioral_fidelity: Full`: look up the referenced contract and verify that every obligation in that contract has `behavioral_fidelity: equivalent-within-contract`. If any obligation carries a different value, emit a `ValidatorError { category: Semantic, field_path: "operations[<id>].behavioral_fidelity", description: "manifest fidelity 'full' but contract obligation <obligation_id> has fidelity '<value>'" }`.
    - For each operation with `behavioral_fidelity: Partial`: verify no obligation carries `architectural-change-required`, `unsupported`, or `unknown`. Emit an error if found.
    - For `Unsupported` operations: no fidelity constraint on the contract.
  - Integrate this check into ManifestValidator::validate_manifest_semantic(toml_bytes, store); Validator::validate_semantic is JSON-only.
  - Write unit tests:
    - Test: manifest `full` referencing a contract where all obligations are `equivalent-within-contract` → no errors.
    - Test: manifest `full` referencing a contract where one obligation is `adaptation-required` → `SemanticResult` with `outcome: Fail` (DDR-06).
    - Test: manifest `partial` with `architectural-change-required` obligation → `SemanticResult` with `outcome: Fail`.
  - Design reference: Section 4.6, DDR-06.
  - _Requirements: Req 5.2, Req 2.3_

  **Acceptance criteria:**
  - Manifest `full` + contract obligation `adaptation-required` produces `SemanticResult { outcome: Fail }`.
  - Manifest `partial` + contract obligation `architectural-change-required` produces `SemanticResult { outcome: Fail }`.
  - Manifest `full` + all obligations `equivalent-within-contract` produces `SemanticResult { outcome: Pass }`.

- [ ] 18. Write positive conformance fixtures
  - Every verification fixture includes P0-C configuration/binding/digests/test-obligation links; every manifest fixture includes an explicit valid P0-B capabilities table, with both empty and nonempty positive declarations. A minimal pass is structural only, never acceptance.
  - Create the following fixture directories under `crates/clouddialect-contracts/fixtures/positive/`. Each fixture directory contains `input.json` (valid document) and `expected.json` (`{ "outcome": "pass", "phase": "structural", "errors": [], "warnings": [] }`).
  - Fixtures to create (one sub-directory each):
    - `behavioral-contract/minimal-valid` — all required fields, one obligation with `evidence_refs` (Req 2.1)
    - `behavioral-contract/with-known-behavior` — resource dependency with `known_behavior.retry` declared (Req 2.2)
    - `behavioral-contract/with-unresolved-assumption-on-dependency` — resource dependency with `unresolved_assumptions` present (Req 2.2)
    - `behavioral-contract/with-behavioral-notes` — two entries in `behavioral_notes`: one with S3 ETag checksum semantics, one with Pub/Sub exactly-once delivery (Req 2.5)
    - `behavioral-contract/with-extension-field` — one community extension key `acme:custom-field` (Req 2.6, 10.2)
    - `evidence-record/observed` — classification `observed` with `provenance` (Req 3.1)
    - `evidence-record/inferred` — classification `inferred` with `reasoning_chain` of one step (Req 3.3)
    - `evidence-record/unknown` — classification `unknown` with `unresolved_assumptions` of one entry (Req 3.4)
    - `verification-result/minimal` — all required fields, one test entry with `outcome: pass` (Req 4.1)
    - `verification-result/with-acceptance` — includes `acceptance` with `approver_identities` and `timestamp` (Req 4.4)
    - `pack-manifest/full-coverage` — all required sections, one `full` operation with a non-empty toolchain (Req 5.1, 5.2, 5.5). This fixture is `input.toml`, not `input.json`.
    - `pack-manifest/with-unsupported-operation` — one `unsupported` operation entry (Req 5.7). `input.toml`.
    - `pack-manifest/with-extension-section` — a `[x]` section with one key-value pair (Req 5.6). `input.toml`.
  - Design reference: Section 3.5 (Testing Strategy, positive fixtures table).
  - _Requirements: Req 2.1, Req 2.2, Req 2.5, Req 2.6, Req 3.1, Req 3.3, Req 3.4, Req 4.1, Req 4.4, Req 5.1, Req 5.6, Req 5.7_

  **Acceptance criteria:**
  - All 13 positive fixture `input.json`/`input.toml` files are valid against their respective schemas when validated by `validate_structural()` or `validate_manifest()`.
  - Each `expected.json` specifies `"outcome": "pass"`.
  - Property 4 (fidelity enum exhaustiveness): the `behavioral-contract/minimal-valid` and `behavioral-contract/with-behavioral-notes` fixtures cover at least three distinct `behavioral_fidelity` values collectively.

- [ ] 19. Write negative conformance fixtures
  - Add P0 cases: verification missing-binding, missing-configuration, nonpass-string-failure-description, missing-test-obligation; manifest missing-capabilities, unknown-capability-version, duplicate-capability-id and capability-unknown-field. Record exact structural/local-semantic phase and field paths.
  - Create the following fixture directories under `crates/clouddialect-contracts/fixtures/negative/`. Each contains `input.json` (invalid document) and `expected.json` specifying `"outcome": "fail"` with at least one error entry identifying the `category` and a `description_contains` substring.
  - Fixtures to create:
    - `behavioral-contract/missing-id` — `id` field absent; expected error: `category: structural`, `field_path: "id"` (Req 2.1)
    - `behavioral-contract/obligation-no-evidence-no-assumption` — obligation with neither `evidence_refs` nor `unresolved_assumption`; expected error: `category: structural` (Req 2.4)
    - `behavioral-contract/invalid-behavioral-fidelity` — obligation with `behavioral_fidelity: "invalid-value"`; expected error: `category: structural` (Req 2.3)
    - `evidence-record/missing-classification` — no `classification` field; expected error: `category: structural`, `field_path: "classification"` (Req 3.2)
    - `evidence-record/inferred-empty-reasoning-chain` — `classification: "inferred"`, `reasoning_chain: []`; expected error: `category: structural` (Req 3.3)
    - `evidence-record/unknown-empty-assumptions` — `classification: "unknown"`, `unresolved_assumptions: []`; expected error: `category: structural` (Req 3.4)
    - `verification-result/empty-tests-array` — `tests: []`; expected error: `category: structural` (Req 4.7)
    - `verification-result/nonpass-without-failure-description` — test entry with `outcome: "fail"` and no `failure_description`; expected error: `category: structural` (Req 4.3)
    - `verification-result/acceptance-missing-approver` — `acceptance` object with no `approver_identities`; expected error: `category: structural` (Req 4.4)
    - `verification-result/unknown-hash-algorithm` — `content_hash.algorithm: "md5"`; expected error: `category: structural` (Req 4.2)
    - `pack-manifest/missing-toolchain-for-full-operation` — one `full` operation, `[[toolchain]]` section absent; expected error (Req 5.5). `input.toml`.
    - `pack-manifest/invalid-behavioral-fidelity` — operation with `behavioral_fidelity = "full-featured"`; expected error (Req 5.2). `input.toml`.
    - `pack-manifest/inline-contract-body` — `contracts.op` containing an `obligations` array inline; expected error (Req 5.3). `input.toml`.
    - `version/unsupported-major` — `format_version: "99.0.0"`; expected: `outcome: fail`, `category: unsupported-major-version` (Req 7.4, 6.4)
    - `version/unsupported-minor-newer` — `format_version: "1.99.0"` where validator supports `1.0`; expected: `outcome: warn` with a version warning (Req 7.3)
  - Design reference: Section 3.5 (Testing Strategy, negative fixtures table).
  - _Requirements: Req 2.1, Req 2.3, Req 2.4, Req 3.2, Req 3.3, Req 3.4, Req 4.2, Req 4.3, Req 4.4, Req 4.7, Req 5.2, Req 5.3, Req 5.5, Req 6.4, Req 7.3, Req 7.4_

  **Acceptance criteria:**
  - All baseline 15 plus the eight P0 negative fixtures return the declared fail result (except version/unsupported-minor-newer, which retains its documented structural warning; it cannot authorize unknown security semantics).
  - Each `expected.json` error entry specifies a `category` that matches the actual error category returned.
  - Property 4 (fidelity enum exhaustiveness): `invalid-behavioral-fidelity` fixture confirms an out-of-enum value is rejected.

- [ ] 20. Write adversarial conformance fixtures
  - Add P0 declaration-versus-grant and verification-claim cases: `[x]` authority injection, prohibited worker actions, forged approver/origin strings and incomplete/replayed bindings. Distinguish manifest/schema rejection from Spec 2 admission/authentication tests; document-only or mocked fixtures cannot prove native control enforcement.
  - Create the following fixture directories under `crates/clouddialect-contracts/fixtures/adversarial/extension-namespace/`. Each directory contains `input.json` and `expected.json`.
  - Requirement 10.7 mandates exactly 8 fixture classes; each class must include at least one case where the validator handles it correctly ("correctly handled") and at least one where it correctly rejects ("correctly rejected"):
    1. `reserved-exact-match/` — document with `core:field`; expected: `outcome: pass`, one warning with `field_name_verbatim: "core:field"`.
    2. `reserved-uppercase-bypass/` — documents with `CORE:field` and `CDC:field`; expected: each produces a warning; `outcome: pass` (Req 10.3).
    3. `reserved-mixed-case-bypass/` — documents with `Core-Validation:field` and `Cdc-Evidence:field`; expected: each produces a warning; `outcome: pass`.
    4. `namespace-collision/` — two community extension fields using the same prefix `acme:field-a` and `acme:field-b` in one document; both extensions are preserved (no error, since collision is a registry concern, not a structural one). Provide a "correctly rejected" case where a reserved prefix is used by two documents claiming the same community prefix — demonstrate the validator does not grant trust based on prefix alone.
    5. `invalid-prefix-formats/` — three sub-cases: prefix starting with digit `1bad:field`, prefix containing space `bad field:val`, prefix of length one `a:field`; each must fail structural validation (Req 10.7).
    6. `unknown-optional-round-trip/` — document with valid community extension `acme:meta` present; expected: `outcome: pass`, extension field present and unmodified in round-trip (Req 10.2, Property 5).
    7. `mandatory-extension-blocks-acceptance/` — document with `required_extensions: ["unknown-ns"]`; expected: `outcome: pass` structurally but blocker emitted preventing acceptance (Req 10.4, Property 6).
    8. `override-core-field-attempt/` — document with `acme:verification-status: "accepted"` alongside a real core verification result; expected: core outcome unchanged, extension field does not influence acceptance (Req 10.5).
  - Design reference: Section 3.5 (adversarial fixtures table), Security Considerations (extension namespace impersonation), Req 10.7.
  - _Requirements: Req 10.2, Req 10.3, Req 10.4, Req 10.5, Req 10.7_

  **Acceptance criteria:**
  - All 8 fixture classes are present (Req 10.7).
  - Reserved-prefix fixtures produce a warning but `outcome: pass` — never `outcome: fail` due to the reserved prefix alone (Req 10.3).
  - Invalid-prefix-format fixtures fail structural validation (Req 10.7).
  - Mandatory unknown extension fixture produces a blocker (Req 10.4).
  - Core-field override attempt: core verification outcome is unchanged (Req 10.5).
  - Property 8 test passes for all five reserved prefix variants.

- [ ] 21. Write round-trip and integration fixtures
  - Add P0 scoped-store exact-version/missing/ambiguous/wrong-type/hash-mismatch/outage cases, same-tenant cross-project lookup denial, snapshot environment immunity and digest vectors; verification duplicate test IDs/unmapped obligations/invalid UTC/hash lengths/mismatched approval-result ref. Native project isolation and external observation/authentication remain Spec 2 qualification, not implied by these fixtures.
  - **Round-trip fixtures** — create under `crates/clouddialect-contracts/fixtures/round-trip/extension-field-preservation/`:
    - `behavioral-contract/` — `input.json` has a community extension key `acme:meta`; `expected.json` specifies `outcome: pass` and that the extension field is present byte-for-byte after round-trip. (Property 2, Req 3.5, 10.2)
    - `evidence-record/` — same pattern for evidence record.
    - `verification-result/` — same pattern for verification result.
  - **Integration fixtures** — create under `crates/clouddialect-contracts/fixtures/integration/`:
    - `blocking-gap/unknown-evidence-mandatory-obligation/` — unknown source evidence; expected phase behavioral, outcome blocked, obligation/assumption diagnostics. EvidenceComplete is never normalized to behavioral pass (Req 3.6, 11.4).
    - `semantic-resolution-failure/manifest-bad-contract-ref/` — exact contract ref absent from the injected DocumentStore; expected semantic fail, unresolvable-reference and contracts.upload_object.id field path (Req 8.3).
    - `artifact-hash-mismatch/tampered-candidate/` — declared verification candidate hash plus tampered bytes; expected phase artifact-hash, expected_match false, no invented invalid outcome (Req 4.6).
    - `offline-resolution/network-disabled/` — a valid behavioral contract; `expected.json` specifies `outcome: pass` with note that this fixture must pass in a network-isolated environment (Req 1.2, 9.1).
    - `offline-resolution/unresolvable-ref/` — document declaring `"$schema": "https://example.com/unknown-schema"`; `expected.json` specifies `outcome: fail`, `category: unresolvable-reference`, `description_contains: "https://example.com/unknown-schema"` (Req 1.3, 9.3).
  - Design reference: Section 3.5 (round-trip and integration fixtures tables).
  - _Requirements: Req 1.2, Req 1.3, Req 3.6, Req 4.6, Req 8.3, Req 9.1, Req 9.3, Req 11.4_

  **Acceptance criteria:**
  - Round-trip fixtures: extension field in `input.json` is present and byte-for-byte unchanged after parse-and-serialize (Property 2, Property 5).
  - Phase-specific expectations reject legacy behavioral pass/fail normalization and invented hash outcomes; a deliberate wrong expected_match fails the runner. SemVer fixture boundaries cover prerelease/build acceptance syntax and invalid identifiers.
  - Blocking-gap integration fixture: blocker contains all three required fields (`candidate_id`, `obligation_id`, `assumption_text`) (Req 11.4).
  - Semantic resolution failure fixture: `SemanticResult` identifies the unresolvable field path (Req 8.3).
  - Offline resolution fixture with unresolvable `$schema`: error description contains the exact URI (Req 1.3).

- [ ] 22. Implement conformance fixture runner
  - Planning elaboration: register the conformance integration target with `harness = false` and a test-only main/prefix parser as specified in implementation-plan unit 3.1. Cargo's default Rust-test name filter is not a fixture-directory filter; zero selected fixtures must fail. Parent owns Cargo.toml registration; no production binary/CLI is introduced.
  - Dispatch JSON vs TOML semantic phases through the exact P0-E entrypoints; supply explicit synthetic project-scoped store fixtures, never an ambient schema-catalog document search or live backend.
  - Create `crates/clouddialect-contracts/tests/conformance.rs` as an integration test binary.
  - The runner:
    1. Walks the `fixtures/` directory recursively.
    2. For each leaf directory containing both `input.json` (or `input.toml`) and `expected.json`:
       - Determines phase and trusted JSON DocumentKind/manifest selection from protected expected.json. P0-E outcome union: structural pass/warn/fail; semantic pass/fail; behavioral deferred/evidence-complete/blocked; artifact-hash expected_match boolean with no outcome field. Unsupported combinations fail.
       - Calls the appropriate validator method.
       - Compares actual output against `expected.json`:
         - Phase-specific outcome or artifact-hash expected_match: exact match; never normalize EvidenceComplete to behavioral pass.
         - `errors[*].category`: exact match.
         - `errors[*].field_path`: exact match when present.
         - `errors[*].description_contains`: substring match.
         - `warnings[*].field_name_verbatim`: exact match.
         - `blockers[*].obligation_id`: exact match.
         - `blockers[*].assumption_text_contains`: substring match.
       - Fails the test with a descriptive message (fixture path, expected vs. actual) on any mismatch.
    3. Enforces a per-fixture timeout of 5 seconds (Req 10.7 security consideration for recursive schema DoS).
    4. Supports filtering by directory prefix: `cargo test --test conformance -- positive` runs only fixtures under `fixtures/positive/`.
  - Write at least one meta-test asserting the runner itself fails when given a fixture whose `expected.json` claims `outcome: pass` but the validator returns `outcome: fail`.
  - Design reference: Section 3.5, Implementation Guidance (Conformance Fixture Tooling).
  - _Requirements: All requirements marked "Conformance fixture" as their verification method_

  **Acceptance criteria:**
  - `cargo test --test conformance` runs all fixtures and passes when all fixture inputs match their expected outputs.
  - A deliberate mismatch in one fixture causes the test to fail with a message identifying the fixture path and the discrepancy.
  - Filtering by directory prefix works: `cargo test --test conformance -- adversarial` runs only adversarial fixtures.
  - Per-fixture 5-second timeout prevents a recursive schema from blocking the suite.

- [ ] 23. Write compatibility policy document
  - Record P0 publication-history review and approved target format/API/catalog versions. New required capability/binding/configuration fields and structured failure objects cannot mutate a previously published format in MINOR/PATCH; require MAJOR/migration fixtures where applicable. Legacy structural warnings do not grant unsupported security semantics.
  - Create `docs/compatibility-policy.md`.
  - Document must include all of the following (Req 7.1):
    - Definition of a **breaking change** to a schema: any change to required fields, field types, enum values, or validation behavior that would cause a previously valid document to fail validation.
    - Definition of a **backward-compatible change**: adding optional fields, adding new enum values to optional fields, non-normative corrections (typos, descriptions).
    - Minimum support window: **12 calendar months** from the publication date of a successor version. State this explicitly.
    - End-of-life process: how to declare a version EOL (update `status` in `index.json` to `end-of-life-declared`, set `end_of_life_date`, announce in the changelog).
    - Migration guide commitment: for every MAJOR version release, a migration guide will be published describing how to update documents from the prior version (Req 7.2).
    - Three distinct version spaces: JSON Schema dialect version, document-format version, transformation pack version — with definitions and field names for each (Req 6.1).
  - Design reference: Section 3.4 (version compatibility), DDR-06 note on MAJOR requirement for semantic changes.
  - _Requirements: Req 7.1, Req 7.2, Req 6.1_

  **Acceptance criteria:**
  - The document states the 12-month minimum support window explicitly (Req 7.1).
  - Breaking vs. non-breaking change definitions are present.
  - The three version spaces are named with their respective field names (Req 6.1).
  - EOL process references updating `status` in `index.json`.

- [ ] 24. Write extension convention document
  - Create `docs/extension-convention.md`.
  - Document must include all six elements from Req 10.1:
    1. **Namespace prefix grammar**: ASCII letter followed by zero or more ASCII alphanumeric characters or hyphens, minimum total length two characters, colon separator when used in a qualified key (e.g., `acme:field-name`).
    2. **Reserved namespace families**: `core`, `core-*`, `cdc`, and `cdc-*`. State that reserved-prefix matching is **case-insensitive**.
    3. **Concrete reserved prefix examples**: `core:`, `core-validation:`, `cdc:`, `cdc-evidence:`.
    4. **Governance rule**: new namespaces within existing reserved families may be introduced without modifying prefix-validation logic; changes altering previously permitted namespace usage require a compatibility review.
    5. **Ownership and collision-handling policy**: extension namespace ownership is documented in `catalog/extension-registry.json`; prefixes alone do not establish publisher identity or trust.
    6. **Pointer to registry**: `catalog/extension-registry.json` is the authoritative extension registry file.
  - Design reference: DDR-08, Section 3.4 (reserved-namespace detection), Req 10.1.
  - _Requirements: Req 10.1_

  **Acceptance criteria:**
  - All six elements listed in Req 10.1 are present in the document.
  - The reserved prefix list is consistent with the regex in Task 12: `core`, `core-*`, `cdc`, `cdc-*`.
  - Case-insensitivity of reserved-prefix matching is stated explicitly.
  - The document links to `catalog/extension-registry.json`.

- [ ] 25. Write traceability tables
  - Include P0-A–F fields/APIs and new Req 4.8/5.8/8.6/8.7/9.5; attach owner approval and real consumer/conformance report at the delivered immutable revision before claiming BG-01 compatibility.
  - Create `docs/traceability/field-to-requirement.md` (NFR-2.1):
    - A Markdown table with columns: `Schema`, `Field path`, `Requirement`, `Notes`.
    - Cover every field in every JSON Schema document (Tasks 2–6) and every field in the TOML manifest struct (Task 15).
    - Include entries for: `behavioral-contract.id` → Req 2.1, 6.5; `behavioral-contract.format_version` → Req 2.1, 6.1, 6.2; `behavioral-contract.obligations[*].behavioral_fidelity` → Req 2.3; `evidence-record.classification` → Req 3.1, 3.2; `verification-result.tests[*].origin_identity` → Req 4.1, 8.4; `verification-result.candidate.content_hash.algorithm` → Req 4.2; and so on for all fields.
  - Create `docs/traceability/gate-to-requirement.md` (NFR-2.2):
    - A Markdown table with columns: `Gate`, `Description`, `Requirements`, `Schema fields`.
    - Cover all five gates: AC-01, AC-02, AC-05, AC-07, AC-09 using the Acceptance Gate Traceability table from the requirements document as the source.
  - Design reference: Release Integrity section (Traceability Artifacts), NFR-2.
  - _Requirements: NFR-2.1, NFR-2.2_

  **Acceptance criteria:**
  - `field-to-requirement.md` contains at least one entry per JSON Schema document and per manifest struct field.
  - `gate-to-requirement.md` covers all five acceptance gates (AC-01, AC-02, AC-05, AC-07, AC-09).
  - All requirement references match requirement IDs from the requirements document.

- [ ] 26. Add release tooling configuration and documentation
  - Create `deny.toml` at the repository root with a `[licenses]` section allowing `MIT`, `Apache-2.0`, `Apache-2.0 WITH LLVM-exception`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Unicode-DFS-2016` and denying `GPL-2.0`, `GPL-3.0`, `AGPL-1.0`, `AGPL-3.0`, `SSPL-1.0`, `LGPL-2.0`, `LGPL-2.1`, `LGPL-3.0` with `copyleft = "deny"` (Req 12.3).
  - Create `release/README.md` documenting:
    - **SBOM generation**: command `cargo cyclonedx --format json --output-file release/sbom.cdx.json` and what it produces (Req 12.1).
    - **Artifact signing**: commands for signing (`minisign -S ...`) and verifying (`minisign -V -p release/minisign.pub ...`) with explanation that verification requires no network access (Req 12.2).
    - **License screening**: command `cargo deny check licenses` and the consequence (named incompatibility report) if a deny-listed license is found (Req 12.3).
    - The pre-release gate sequence: (1) run `cargo deny check licenses`, (2) run `cargo test --test conformance`, (3) generate SBOM, (4) build release archive, (5) sign archive.
  - Create `release/minisign.pub` as a placeholder file with a comment indicating the actual public key is committed when the first release is cut (Req 12.2).
  - Design reference: Release Integrity section, DDR-07.
  - _Requirements: Req 12.1, Req 12.2, Req 12.3_

  **Acceptance criteria:**
  - `deny.toml` is present at repository root and contains both the allow and deny license lists (Req 12.3).
  - `release/README.md` documents all three release tooling procedures (SBOM, signing, license screening).
  - `release/minisign.pub` is present in the repository (Req 12.2).
  - `cargo deny check licenses` exits zero on the initial crate scaffold (no deny-listed licenses in bootstrap dependencies).

---


## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP iteration.
- Every task references specific requirements for traceability.
- `BehavioralResult` full implementation (running independently controlled tests) is Spec 2 scope; Tasks 14 and 22 implement the evidence-completeness check only (`Blocked` / `EvidenceComplete` outcomes).
- The conformance fixture runner (Task 22) must be implemented before the release tooling gate (Task 26) since `cargo test --test conformance` is part of the pre-release gate sequence.
- All JSON Schema documents use `unevaluatedProperties: false` combined with `patternProperties` for extension fields per DDR-06.
- UUID v4 (hyphenated lowercase) is the required identifier format for all CloudDialect JSON document `id` fields per DDR-07.
