# Requirements Document

**Later publication action approval:** the user explicitly requested git commit and git push for this work. Those actions are now authorized; the earlier no-commit/no-push implementation limit is historical. Native probes, deployment and releases remain unauthorized.


## Introduction

CloudDialect is a community framework for cloud workload transformation and migration. This document defines the behavioral contract and evidence schemas that form the foundation layer for all CloudDialect transformation activity — Spec 1 of 3 in a sequenced set.

The purpose of this spec is to specify the versioned, machine-readable schemas and supporting rules that pack authors, the framework core, verifier implementers, tool integrators, and auditors all depend on. Spec 2 (Execution and Isolation) and Spec 3 (Python OSS-to-S3 Migration Pack) both consume definitions established here.

This spec does not define Rust CLI behavior, worker isolation, transformation logic, production cutover, or commercial service delivery. It does not alter CloudDialect's licensing model.

**Current continuation status (2026-10-22):** user approved P0-A–F for implementation planning, then separately authorized the remaining Spec 1 prerequisite units in [implementation-plan.md](implementation-plan.md), TDD and no commits/probes/pushes/releases. User attests this is a new project and approves document/catalog **1.0.0** and crate API **0.1.0** as the initial MVP versions. This is not an independent external-publication audit or security-gate approval. Exact encoded extension JSON value bytes are required; raw_value on the existing pinned serde_json is separately approved.

**Delivery is partial.** DTOs, schemas, catalog inventory, frozen snapshot/loading, structural JSON validation, local TOML rules and candidate hash equality have bounded verification. JSON semantic/completeness validation, full mandatory-blocker integration, exhaustive conformance and delivered downstream compatibility remain unfinished; scoped fidelity/TOML semantic delivery is recorded in the current report. See the [current run report](../../../.workflow/ultracode/execution-isolation-implementation/final-report.md) for actual evidence and outstanding work. Canonical task checkmarks are conservative; BG-01 and Spec 2 UD-1–UD-5 remain OPEN.

**Evidence status convention used in this document:**
Every requirement carries one of four evidence status labels reflecting where each requirement stands in the development lifecycle:
- `MISSING` — no schema, fixture, test, or implementation artifact exists yet.
- `SPECIFIED BUT INCOMPLETE` — a draft schema exists but known gaps or unresolved decisions remain.
- `IMPLEMENTED BUT UNVERIFIED` — an artifact exists but has not been validated against conformance fixtures or integration tests.
- `VERIFIED WITHIN A DECLARED SCOPE` — passing conformance fixtures and integration tests exist for a stated set of inputs and environments.

The labels below retain the original full-conformance baseline, not a claim that no implementation exists now. Current bounded unit evidence and remaining gaps are recorded in the linked delivery plan/run report; partial artifact delivery does not verify an entire requirement.

---

## Glossary

- **Behavioral Contract**: A versioned JSON document that records the externally observable behavior of one operation or component — its inputs, outputs, side effects, identity, permissions, configuration, resource dependencies, retry/ordering/consistency behavior, and the evidence and unresolved assumptions behind each claim.
- **Evidence Record**: A versioned JSON document that records an observation or inference about a source or target workload, including the provenance of that observation, its classification (observed / inferred / unknown), and any unresolved assumptions it carries.
- **Verification Result**: A versioned JSON document produced by an independently controlled verifier after running a declared set of tests against a transformation candidate. It records test identities, outcomes, and the artifact identities used.
- **Transformation Pack (Pack)**: A versioned collection of transformations targeting a specific migration scenario (e.g., Alibaba OSS Python SDK to AWS S3 Python SDK). A pack is identified and described by its manifest.
- **Pack Manifest**: A TOML document that declares a transformation pack's identity, supported source and target environments, operation coverage, behavioral contract references, assumptions, limitations, schema references, and toolchain requirements.
- **Observed fact**: A claim derived directly from source artifacts (code, configuration, logs, infrastructure definitions) through analysis tools, with provenance recorded. Not inferred or assumed.
- **Inferred fact**: A claim derived by reasoning from observed facts. The reasoning chain must be recorded. An inferred claim may be wrong and must not be promoted to observed without additional evidence.
- **Unknown**: A claim that cannot be established from available evidence. Must be recorded explicitly; must not be silently omitted or filled by generated text.
- **Structural validation**: Checking that a document instance conforms to its declared JSON Schema. Structural validity does not imply semantic correctness or migration safety.
- **Semantic validation**: Checking that references within and across documents are consistent — for example, that every contract reference in a manifest resolves to an existing contract with the declared identifier, and that enum values have defined meanings in context.
- **Behavioral verification**: Running independently controlled tests (not generated by the patch-writing agent) against a candidate transformation and recording outcomes in a Verification Result. Passing structural and semantic validation never, by itself, establishes behavioral correctness.
- **Conformance Fixture**: A shared test artifact containing input instances and expected validation outcomes used to verify that multiple implementations (Rust validator, language workers, CI tooling) agree on schema interpretation.
- **Bundled Schema Catalog**: The set of CloudDialect JSON Schema documents distributed with a release and used as the sole trusted source for schema resolution during validation. Arbitrary network retrieval during validation is not permitted.
- **Document-format version**: A version identifier embedded in every CloudDialect JSON document that identifies the schema version against which the document must be validated. Distinct from the JSON Schema dialect version and from the pack version.
- **JSON Schema dialect version**: The version of the JSON Schema specification (Draft 2020-12) used to author and interpret CloudDialect schemas. Distinct from document-format versions.
- **Transformation pack version**: A semantic version assigned to a specific pack release. Distinct from the schema version of the documents the pack produces or consumes.
- **Acceptance gate**: A project-level verifiable requirement that a candidate migration result must satisfy before it is accepted. Each acceptance gate is referenced in this document by its identifier (AC-01, AC-02, AC-07, AC-09).
- **Behavioral fidelity classification**: A per-obligation label drawn from the closed enumeration: `equivalent-within-contract` / `adaptation-required` / `architectural-change-required` / `unsupported` / `unknown`. Records whether a source behavioral obligation is satisfiable at the target. Manifests use a simplified set: `full` / `partial` / `unsupported` (see Requirement 5).
- **SBOM**: Software Bill of Materials — a machine-readable inventory of dependencies and their licenses included with each CloudDialect release.
- **Extension namespace**: A namespace prefix claimed by a pack author or tool integrator to scope extension fields in CloudDialect documents. A namespace prefix consists of ASCII alphanumeric characters and hyphens (starting with a letter, minimum two characters), followed by a colon separator when used in a qualified extension key. Namespace prefixes do not establish publisher identity or trust; they are used for collision avoidance only.
- **Reserved namespace**: A namespace prefix family reserved exclusively for CloudDialect core use. Reserved families are `core`, `core-*`, `cdc`, and `cdc-*`. Reserved-prefix matching is case-insensitive. Community extension prefixes MUST NOT begin with a reserved family prefix.

---

## User Roles

| Role | Interaction with this spec's outputs |
|---|---|
| **Pack author** | Writes TOML manifests; references behavioral contracts from a pack; must understand manifest schema, versioning rules, and capability declaration format. |
| **Framework core maintainer** | Owns and evolves JSON Schemas; sets version and compatibility policy; approves schema changes. |
| **Verifier implementer** | Consumes evidence records and verification result schemas; must not conflate structural validity with behavioral correctness. |
| **Tool integrator** | Parses manifests and validates contracts in Rust core, language workers, and CI tooling; needs machine-readable, validatable documents and offline resolution. |
| **Evaluator / auditor** | Inspects evidence bundles; needs evidence classification and provenance to be explicit and human-readable; needs traceability between obligations and verification outcomes. |

---

## Scope

**In scope:**
- Published JSON Schema documents for behavioral contracts, evidence records, and verification results.
- TOML manifest format for transformation packs.
- Document identifier and versioning rules.
- Compatibility policy between schema versions.
- Evidence classification semantics (observed / inferred / unknown) at the fact/claim level.
- Validation boundary definitions (structural / semantic / behavioral).
- Offline schema resolution rules.
- Extension mechanism for schema fields and manifest sections.
- Conformance fixture format (unresolved — see Section 9).
- Traceability between requirements and acceptance gates AC-01, AC-02, AC-07, AC-09.

**Out of scope:**
- Rust CLI implementation.
- Worker isolation and execution environment.
- Python OSS-to-S3 transformation logic.
- Production data transfer, traffic cutover, source-resource deletion.
- Commercial service delivery.
- Performance benchmarks or SLA thresholds (no basis for these numbers yet; recorded as unresolved).

---

## Requirements

### Requirement 1: JSON Schema Authoring and Distribution

**User Story:** As a framework core maintainer, I want all CloudDialect document types to be defined by JSON Schema Draft 2020-12 documents so that validation is unambiguous, machine-executable, and independently reproducible across implementations.

**Traceability:** AC-01, AC-07, AC-09

#### Acceptance Criteria

1. THE Schema_Catalog SHALL contain a JSON Schema Draft 2020-12 document for each of the following document types: behavioral contract, evidence record, and verification result. Each schema document MUST declare `"$schema": "https://json-schema.org/draft/2020-12/schema"` and provide a unique `"$id"` URI within the CloudDialect schema namespace.
   - Verification method: Structural review of catalog contents confirming the presence of `$schema` and `$id` fields; validation with two independent Draft 2020-12 validators.
   - Evidence status: `MISSING`

2. WHEN a CloudDialect JSON document is validated, THE Validator SHALL resolve all `$ref` and `$schema` URIs exclusively from the Bundled Schema Catalog distributed with the release; arbitrary network retrieval during validation is not permitted.
   - Verification method: Conformance fixture asserting that validation succeeds with network access disabled.
   - Evidence status: `MISSING`

3. WHEN a CloudDialect JSON document contains a `$ref` or `$schema` URI that cannot be resolved from the Bundled Schema Catalog, THE Validator SHALL reject the document with a diagnostic error identifying the unresolvable URI and SHALL NOT perform partial validation.
   - Verification method: Conformance fixture with a document declaring an unresolvable `$ref`, asserting the error message contains the unresolvable URI and no partial validation result is returned.
   - Evidence status: `MISSING`

4. THE Schema_Catalog SHALL be distributed as part of every CloudDialect release artifact, with each schema file included at a stable, versioned path following the pattern `schemas/{document-type}/{major}.{minor}/schema.json`.
   - Verification method: Release artifact inventory review confirming all schema files follow the path pattern.
   - Evidence status: `MISSING`

5. WHEN a new schema version is published, THE Schema_Catalog SHALL retain prior schema versions at their original stable paths for the duration of the compatibility window declared in the version compatibility policy (Requirement 6).
   - Verification method: Structural review of catalog between consecutive release artifacts.
   - Evidence status: `MISSING`

6. THE Schema_Catalog SHALL include a machine-readable index document that: (a) is itself schema-valid against a schema included in the catalog, (b) lists all included schema identifiers, their document-format versions, and their catalog-relative paths.
   - Verification method: Conformance fixture asserting the index validates against its own schema and lists all expected entries.
   - Evidence status: `MISSING`

---

### Requirement 2: Behavioral Contract Document Format

**User Story:** As a pack author and verifier implementer, I want a standardized behavioral contract format so that I can record source operational behavior with sufficient fidelity for a verifier to determine whether a migration candidate satisfies it.

**Traceability:** AC-01, AC-02

#### Acceptance Criteria

1. THE Behavioral_Contract_Schema SHALL require each contract document to declare: a document identifier that is unique within the pack it belongs to, its document-format version, the source operation identity (including the call site and every wrapper in the call chain up to the operation boundary, with at least one wrapper entry required when a wrapper chain exists), input schema, output schema, and externally visible side effects.
   - Verification method: Conformance fixture asserting documents missing any of these fields fail structural validation.
   - Evidence status: `MISSING`

2. THE Behavioral_Contract_Schema SHALL require each contract document to declare resource dependencies including: identity, permissions required, and configuration parameters; for each dependency, any ordering, consistency, retry, or failure behavior that is known MUST be declared, and any such behavior that is not known MUST be recorded as an unresolved-assumption entry on that dependency.
   - Owner-approved RC-1 binding (2026-10-22): each resource-specific unresolved assumption requires `behavior` in `ordering | consistency | retry | failure` and nonempty `assumption_id`, `description`, `reason`. Every dependency covers all four dimensions by a nonempty `known_behavior.<dimension>` value or a matching populated assumption; both may coexist for partial knowledge. Generic evidence/obligation assumptions remain unchanged. Nonempty means decoded string length at least one, not proof the claim is true.
   - Verification method: Conformance fixture with example operations having non-trivial resource dependencies, including one dependency where retry behavior is declared and one where it is recorded as an unresolved assumption.
   - Evidence status: `MISSING`

3. THE Behavioral_Contract_Schema SHALL require that each behavioral obligation within a contract carries a `behavioral_fidelity` field drawn from the enumerated set: `equivalent-within-contract`, `adaptation-required`, `architectural-change-required`, `unsupported`, `unknown`.
   - Verification method: Conformance fixture asserting values outside the enumerated set fail structural validation; fixture includes one example per enum value.
   - Evidence status: `MISSING`

4. THE Behavioral_Contract_Schema SHALL require each behavioral obligation to reference at least one evidence record or declare its evidence status as `unknown` with an unresolved-assumption entry that contains, at minimum: a machine-readable assumption identifier, a human-readable description of what is unknown, and the reason the information could not be determined.
   - Verification method: Conformance fixture asserting a contract with an obligation lacking both an evidence reference and a fully-populated unresolved-assumption entry fails structural validation.
   - Evidence status: `MISSING`

5. THE Behavioral_Contract_Schema SHALL permit recording of behavioral differences not covered by base schema fields — including service-specific semantics such as object integrity checksum behavior or message delivery conditions — in a `behavioral_notes` section where each entry declares, at minimum: a note type drawn from an open enumeration, a human-readable description, and an associated evidence reference.
   - Verification method: Conformance fixture demonstrating a contract capturing S3 ETag checksum semantics and a contract capturing Pub/Sub exactly-once delivery conditions both validate successfully, including one entry per contract with a non-standard note type.
   - Evidence status: `MISSING`

6. THE Behavioral_Contract_Schema SHALL define an extension mechanism by which fields not defined in the base schema version declared in the document may be added to a contract document, such that a document containing only such extension fields still passes structural validation under the base schema.
   - Verification method: Conformance fixture asserting that a document containing an unknown extension field still passes structural validation under the base schema.
   - Evidence status: `MISSING`

7. WHEN a contract document is structurally valid, THE Validator SHALL NOT set a pass, accepted, or verified flag on any migration candidate treating structural validity alone as sufficient to indicate migration correctness or behavioral equivalence.
   - Verification method: Review of validator documentation and behavioral test asserting structural validation does not set a pass, accepted, or verified flag on the migration candidate.
   - Evidence status: `MISSING`

---

### Requirement 3: Evidence Record Document Format

**User Story:** As an evaluator and verifier implementer, I want evidence records that explicitly classify every claim as observed, inferred, or unknown, with provenance, so that I can assess the reliability of migration correctness claims without depending on AI-generated assumptions.

**Traceability:** AC-02, AC-07

#### Acceptance Criteria

1. THE Evidence_Record_Schema SHALL require each evidence record to declare: a unique document identifier, its document-format version, the claim being recorded, a classification drawn from the set `observed` / `inferred` / `unknown`, the provenance of the claim (source artifact identity and analysis tool identity for observed; reasoning chain for inferred), and any unresolved assumptions.
   - Verification method: Conformance fixture asserting documents missing any of these fields fail structural validation.
   - Evidence status: `MISSING`

2. THE Evidence_Record_Schema SHALL NOT permit omitting the classification field or defaulting it implicitly; every evidence record MUST carry an explicit classification value.
   - Verification method: Conformance fixture asserting a record with no classification field fails structural validation.
   - Evidence status: `MISSING`

3. WHEN an evidence record carries classification `inferred`, THE Evidence_Record_Schema SHALL require the `reasoning_chain` field to be present and contain at least one reasoning step.
   - Verification method: Conformance fixture asserting an `inferred` record with an absent or empty `reasoning_chain` array fails structural validation.
   - Evidence status: `MISSING`

4. WHEN an evidence record carries classification `unknown`, THE Evidence_Record_Schema SHALL require the `unresolved_assumptions` field to be present and contain at least one assumption entry describing what would be needed to resolve the gap.
   - Verification method: Conformance fixture asserting an `unknown` record with an absent or empty `unresolved_assumptions` array fails structural validation.
   - Evidence status: `MISSING`

5. THE Evidence_Record_Schema SHALL define an extension mechanism allowing future provenance fields to be added without breaking validation; documents containing extension fields conforming to the documented namespace SHALL pass structural validation without error and SHALL have their extension fields preserved unchanged by any conforming parser.
   - Verification method: Conformance fixture asserting that a record with a valid extension field passes structural validation and that the extension field is present and unmodified after round-trip parsing.
   - Evidence status: `MISSING`

6. WHEN a mandatory behavioral obligation references an evidence record with classification `unknown`, THE Validator SHALL return a structured blocking-gap report where each entry identifies: the affected obligation identifier, and the text of the unresolved assumption; the Validator SHALL NOT permit acceptance of the affected transformation until the gap is resolved.
   - Verification method: Integration test asserting the blocking-gap report contains the required fields and acceptance is prevented.
   - Evidence status: `MISSING`

---

### Requirement 4: Verification Result Document Format

**User Story:** As an evaluator and tool integrator, I want verification result documents to record exactly which artifacts were tested, which tests ran, and what the outcomes were, so that an independent reviewer can reproduce or audit the verification claim.

**Traceability:** AC-02, AC-07

#### Acceptance Criteria

1. THE Verification_Result_Schema SHALL require each result document to declare: a unique document identifier, its document-format version, the candidate identity and content hash, pack/toolchain identities with versions and content hashes, test identities and outcomes with obligation links, an immutable secret-free configuration artifact reference and the execution/input/dependency/suite/oracle/policy binding specified by P0-C.
   - Verification method: Conformance fixture asserting documents missing any of these fields fail structural validation.
   - Evidence status: `MISSING`

2. THE Verification_Result_Schema SHALL require the content hash of the transformation candidate to be recorded using a hash algorithm name drawn from a schema-enumerated list of approved algorithms.
   - Verification method: Conformance fixture asserting a result document with a hash algorithm name absent from the enumerated list fails structural validation.
   - Evidence status: `MISSING`

3. WHEN any test in a verification run has an outcome other than `pass`, THE Verification_Result_Schema SHALL require the result document to record for that test: the test identifier, the observed outcome value, and `failure_description` as an object with a required safe human-readable `detail` string of 1–2000 Unicode characters and no unknown object fields.
   - Verification method: Conformance fixture asserting a result with a non-passing outcome but missing or empty failure description fails structural validation.
   - Evidence status: `MISSING`

4. WHEN a result document carries an acceptance decision, THE Verification_Result_Schema SHALL require: at least one approver identity, a normalized UTC Z approval timestamp, a reference matching this verification document's id/version, and immutable approval-evidence reference whose authenticity and verification binding must be checked independently of document content.
   - Verification method: Conformance fixture asserting a result carrying an acceptance decision but missing approver identity or timestamp fails structural validation.
   - Evidence status: `MISSING`

5. THE Verification_Result_Schema SHALL define an extension mechanism by which fields not defined in the base schema version may be added to a result document without modifying the base schema, such that a document containing only such extension fields still passes structural validation under the base schema without modification.
   - Verification method: Conformance fixture asserting a result document with an unknown extension field passes structural validation under the unmodified base schema.
   - Evidence status: `MISSING`

6. WHEN the content hash of an artifact referenced in a verification result does not match the hash recorded in that result at acceptance time, THE Verifier SHALL report the acceptance result as invalid and SHALL NOT treat it as a valid acceptance for any downstream decision.
   - Verification method: Integration test asserting that modifying a referenced artifact causes the Verifier to report the acceptance result as invalid and to block downstream acceptance decisions.
   - Evidence status: `MISSING`

7. THE Verification_Result_Schema SHALL require the test-identities array to be present and contain at least one entry; a result document declaring zero tests run SHALL fail structural validation.
   - Verification method: Conformance fixture asserting a result document with an empty or absent test-identities array fails structural validation.
   - Evidence status: `MISSING`

8. WHEN a verification document is consumed, THE Semantic_Validator SHALL reject duplicate test identities, unmapped obligation identities, invalid hash encodings, non-UTC timestamps and inconsistent result references without interpreting a structurally valid binding, origin string or approval string as authenticated authority.
   - Verification method: Positive and seeded-negative fixtures for P0-C; Spec 2 separately compares protected expected context/obligations and authenticates observations/approval before acceptance.
   - Evidence status: `MISSING`

---

### Requirement 5: Transformation Pack Manifest Format

**User Story:** As a pack author and tool integrator, I want a TOML manifest format for transformation packs so that I can declare a pack's identity, coverage, assumptions, and references in a human-writable, machine-parseable file.

**Traceability:** AC-01, AC-09

#### Acceptance Criteria

1. THE Pack_Manifest_Schema SHALL define the required structure for a TOML pack manifest, requiring at minimum: a pack identifier formatted as a dot-separated reverse-domain-name string unique within the CloudDialect pack registry namespace; a semantic version conforming to SemVer 2.0.0; a human-readable name and description; the source cloud provider and service; the target cloud provider and service; the supported source language and its version range expressed as a SemVer 2.0.0 range; and schema references for the contract, evidence, and verification result documents the pack produces.
   - Verification method: Structural review of the manifest schema definition and conformance fixture asserting documents missing required fields are rejected by the manifest validator.
   - Evidence status: `MISSING`

2. THE Pack_Manifest_Schema SHALL require the manifest to declare all operations the pack supports, with each operation entry listing: the operation identifier; the `behavioral_fidelity` classification drawn from the closed enumeration `full` / `partial` / `unsupported`; any assumptions the transformation relies on; and any known limitations.
   - Verification method: Conformance fixture asserting a manifest with an operation entry missing required fields or containing a `behavioral_fidelity` value outside the defined enumeration fails manifest validation.
   - Evidence status: `MISSING`

3. THE Pack_Manifest_Schema SHALL require the manifest to reference behavioral contracts using a contract-reference entry containing only a document identifier and a document-format version conforming to SemVer 2.0.0; manifests SHALL NOT include the content of the referenced contract document within the manifest file itself.
   - Verification method: Conformance fixture asserting a manifest containing the body of a contract document inline (rather than a reference entry) fails manifest validation; fixture asserting a manifest with a valid contract-reference entry passes.
   - Evidence status: `MISSING`

4. THE Pack_Manifest_Schema SHALL require the manifest to declare the minimum CloudDialect schema-catalog version the pack is compatible with, and the maximum version the pack has been validated against, both expressed as SemVer 2.0.0 strings.
   - Verification method: Conformance fixture asserting a manifest missing either compatibility version field fails manifest validation.
   - Evidence status: `MISSING`

5. THE Pack_Manifest_Schema SHALL require the manifest to declare all toolchain dependencies required to execute the pack, with each entry stating the dependency name and its version constraint expressed as a SemVer 2.0.0 range; IF the pack declares any operation with `behavioral_fidelity` set to `full` or `partial`, THEN THE Manifest_Validator SHALL reject a manifest whose toolchain section is absent or contains zero entries.
   - Verification method: Conformance fixture asserting a manifest with an empty or missing toolchain section fails manifest validation when the pack declares at least one executable operation.
   - Evidence status: `MISSING`

6. THE Pack_Manifest_Schema SHALL include an extension section in the manifest, keyed under the reserved TOML table name `[x]`, that allows additional machine-readable fields to be added by pack authors; fields defined under `[x]` SHALL NOT conflict with any top-level key defined by the core schema.
   - Verification method: Conformance fixture asserting a manifest with a populated `[x]` section passes manifest validation.
   - Evidence status: `MISSING`

7. WHEN a manifest declares an operation with `behavioral_fidelity` value `unsupported`, THE Manifest_Validator SHALL accept the manifest and SHALL NOT silently omit the unsupported operation declaration; the entry SHALL remain accessible through the same operation-listing interface as all other operation entries.
   - Verification method: Conformance fixture asserting the unsupported operation is present and returned when the full operation list is retrieved after manifest parsing.
   - Evidence status: `MISSING`

8. THE Pack_Manifest_Schema SHALL require the P0-B `[capabilities]` table containing its explicit supported format version and typed requirement list, distinct from `[x]`, without treating declarations as grants or implying propose/approve/deploy/delete authority from another action.
   - Verification method: Fixtures for explicit empty requirements, valid typed requirement, missing/unknown versions/actions/fields, duplicate IDs, unknown operation references and extension escalation; Spec 2 admission denies unresolved resources and prohibited worker authority.
   - Evidence status: `MISSING`

---

### Requirement 6: Document Identifier and Versioning Rules

**User Story:** As a framework core maintainer and tool integrator, I want unambiguous versioning rules for all three version spaces so that validators, pack authors, and CI tooling can determine compatibility without guessing.

**Traceability:** AC-01, AC-09

#### Acceptance Criteria

1. THE Framework_Core SHALL define and document three distinct version spaces: (a) JSON Schema dialect version (currently Draft 2020-12), (b) document-format version (embedded in each CloudDialect JSON document instance), and (c) transformation pack version (semantic version of a pack release). These three version spaces MUST NOT be conflated in any schema, manifest, or validator interface; each MUST be carried in a distinct, named field.
   - Verification method: Review of schema and manifest definitions confirming each version space has a distinct field name and documentation.
   - Evidence status: `MISSING`

2. THE Framework_Core SHALL assign each published CloudDialect JSON Schema document a document-format version following SemVer 2.0.0 (MAJOR.MINOR.PATCH), with documented semantics for each change type: MAJOR for breaking changes to required fields or validation behavior, MINOR for backward-compatible additions, PATCH for non-normative corrections.
   - Verification method: Review of version changelog and conformance fixture exercising a MINOR addition against the prior-version schema.
   - Evidence status: `MISSING`

3. THE Framework_Core SHALL publish a compatibility matrix declaring, for each document-format version, the range of validator and pack versions that can produce or consume it, expressed as SemVer 2.0.0 range constraints.
   - Verification method: Review of published compatibility matrix confirming version ranges are expressed in SemVer range notation.
   - Evidence status: `MISSING`

4. WHEN a document instance declares a document-format version that the active Validator does not support, THE Validator SHALL return an error response containing: the unsupported version identifier string, the error category `unsupported-version`, and a reference to the compatibility documentation; THE Validator SHALL NOT silently accept or silently ignore the document.
   - Verification method: Conformance fixture asserting a document with an unsupported document-format version produces an error containing all three required fields.
   - Evidence status: `MISSING`

5. THE Framework_Core SHALL assign each CloudDialect document instance a globally unique identifier at creation time using an identifier scheme that: produces identifiers with at least 122 bits of entropy, documents the format (e.g., UUID v4 in hyphenated lowercase string form), and guarantees uniqueness across all instances created by conforming implementations.
   - Verification method: Review of identifier specification; integration test asserting no two independently created documents share an identifier in a corpus of 10,000 generated instances.
   - Evidence status: `MISSING`

---

### Requirement 7: Compatibility Policy

**User Story:** As a pack author and tool integrator, I want a published compatibility policy so that I can determine whether updating a schema version will break my existing pack manifests or document instances.

**Traceability:** AC-01, AC-09

#### Acceptance Criteria

1. THE Framework_Core SHALL publish a written compatibility policy that defines: what constitutes a breaking change to a schema; what constitutes a backward-compatible change; a minimum support window of at least 12 calendar months for a given document-format version after its successor is published; and the process for declaring a version end-of-life.
   - Verification method: Review of published compatibility policy document confirming the 12-month minimum window is stated.
   - Evidence status: `MISSING`

2. WHEN a new MAJOR document-format version is released, THE Framework_Core SHALL provide a documented migration guide describing how documents at the prior version are updated to the new version.
   - Verification method: Review of migration guide accompanying any MAJOR version release.
   - Evidence status: `MISSING`

3. WHEN a validator encounters a document at a MINOR version newer than the validator's supported version within the same MAJOR version, THE Validator SHALL process the document using the MAJOR.0.0 base schema and SHALL surface a non-blocking warning identifying the newer minor version and the fields it did not validate.
   - Owner-approved VC-1 initial profile: exact admitted stable versions are supported by precedence (build metadata ignored). Newer stable MINOR uses an actually admitted MAJOR.0.0 base and root-only additions with exact-path warnings; original bytes/version remain intact. Unsupported nested additions are not projected away, while existing freeform/open base-schema data remains valid. Same-MINOR unindexed PATCH, prerelease and valid core-u64 overflow are UnsupportedVersion; malformed syntax remains InvalidFormat in the helper. Trusted kind and exact admitted URI selection remain mandatory.
   - Verification method: Conformance fixture asserting a document with a MINOR-newer version within the same MAJOR is processed without a validation error but with a recorded warning identifying the version difference.
   - Evidence status: `MISSING`

4. WHEN a validator encounters a document at a MAJOR version it does not support, THE Validator SHALL reject the document with an error response containing: the unsupported MAJOR version, the error category `unsupported-major-version`, and a reference to the compatibility documentation; THE Validator SHALL NOT partially validate the document.
   - Verification method: Conformance fixture asserting a document at an unsupported MAJOR version produces an error containing all three required fields and no partial validation result.
   - Evidence status: `MISSING`

---

### Requirement 8: Validation Boundary Definitions

**User Story:** As a verifier implementer and evaluator, I want the three validation boundaries — structural, semantic, and behavioral — to be formally defined and enforced as separate phases so that a structurally valid document is never presented as evidence of migration correctness.

**Traceability:** AC-02, AC-07

#### Acceptance Criteria

1. THE Framework_Core SHALL formally document the distinction between structural validation, semantic validation, and behavioral verification in a dedicated specification section, with: the inputs, outputs, and pass/fail semantics for each phase defined; each phase independently invocable; and each phase returning a distinct result type (`StructuralResult`, `SemanticResult`, `BehavioralResult`) that is not interchangeable with the others.
   - Verification method: Review of specification section; conformance fixture asserting each phase returns only its own result type.
   - Evidence status: `MISSING`

2. WHEN structural validation passes, THE Validator interface SHALL NOT expose a `StructuralResult` value that carries a field implying semantic correctness or behavioral correctness of the migration candidate.
   - Verification method: Review of Validator API surface confirming `StructuralResult` has no semantic or behavioral correctness fields; integration test asserting structural validation alone does not produce a `SemanticResult` or `BehavioralResult`.
   - Evidence status: `MISSING`

3. WHEN semantic validation is performed, THE Semantic_Validator SHALL check both of the following through an explicitly injected project-scoped DocumentStore, separate from the schema catalog: (a) every manifest contract reference resolves by exact identifier/version; and (b) every contract evidence reference resolves uniquely by exact identifier to a structurally valid immutable evidence record.
   - Verification method: Integration test with a manifest referencing a non-existent contract by identifier, asserting a `SemanticResult` containing a resolution-failure error is returned.
   - Evidence status: `MISSING`

4. WHEN behavioral verification is performed, THE Verifier SHALL execute only tests whose origin identity is distinct from the software process or automated system that produced the transformation candidate; THE Verifier SHALL record each test's origin identity in the Verification Result.
   - Verification method: Review of Verifier documentation confirming origin identity recording; behavioral test asserting the Verification Result includes an origin identity field for each test.
   - Evidence status: `MISSING`

5. THE Framework_Core SHALL document that a structurally valid document never, by itself, establishes migration correctness; this statement MUST appear explicitly in the schema documentation for each document type.
   - Verification method: Review of schema documentation for each document type confirming the statement is present.
   - Evidence status: `MISSING`

6. THE Framework_Core SHALL keep BehavioralOutcome::Deferred, BehavioralOutcome::EvidenceComplete and BehavioralOutcome::Blocked non-accepting, with the P0-A result/reference/test projection contract and no accepting conversion from structural, semantic or evidence-completeness results.
   - Verification method: API/fixture checks for all three outcomes, empty completeness projection, mismatched/stale verification projection and unknown/failed/skipped mandatory evidence; Spec 2 owns authenticated behavioral execution and final acceptance.
   - Evidence status: `MISSING`

7. THE Validator SHALL expose separate JSON and TOML semantic entrypoints receiving the P0-E DocumentStore, with trusted explicit DocumentKind for JSON, failing on missing, ambiguous, corrupt, wrong-type or unavailable referenced records rather than consulting ambient paths or another project.
   - Verification method: JSON/TOML signature consumer tests and store fixtures with exact-version success and all named failure modes; no network or cross-project fallback.
   - Evidence status: `MISSING`

---

### Requirement 9: Offline Schema Resolution

**User Story:** As a tool integrator, I want all schema resolution to be satisfiable from the bundled catalog without network access so that validation can run in air-gapped environments and CI pipelines with restricted egress.

**Traceability:** AC-01, AC-09

#### Acceptance Criteria

1. THE Validator SHALL resolve all schema URIs used during document validation exclusively from the Bundled Schema Catalog; it SHALL NOT make outbound network requests to resolve schemas.
   - Verification method: Conformance fixture asserting validation completes successfully with outbound network access fully disabled.
   - Evidence status: `MISSING`

2. THE Bundled Schema Catalog SHALL contain every schema and sub-schema required to validate a behavioral contract, evidence record, verification result, and pack manifest without network access.
   - Verification method: Catalog completeness review against all `$ref` URIs in all published schemas; end-to-end validation test executed in a network-isolated environment.
   - Evidence status: `MISSING`

3. IF a document's declared `$schema` URI cannot be resolved from the Bundled Schema Catalog, THEN THE Validator SHALL return an error identifying the unresolvable URI and SHALL NOT silently fall back to a different schema version.
   - Verification method: Conformance fixture with a document declaring an unknown `$schema` URI, asserting the error message contains the unresolvable URI and no fallback validation occurs.
   - Evidence status: `MISSING`

4. THE Schema_Catalog index (Requirement 1.6) SHALL be the authoritative document mapping schema identifiers to catalog-relative file paths; validators SHALL NOT use hard-coded absolute file paths.
   - Verification method: Review of Validator implementation confirming all schema path lookups use the catalog index; integration test asserting validation succeeds after relocating the catalog to a non-default path when the index is updated accordingly.
   - Evidence status: `MISSING`

5. THE Validator SHALL select schema material only from an embedded or explicitly supplied immutable CatalogSnapshot admitted by trusted caller policy, without reading environment-selected overrides or exposing network-resolver replacement through its public API.
   - Verification method: Embedded/explicit snapshot positive fixtures, tampered/index/path/meta-schema failures and hostile environment variables having no effect; actual pinned engine adapter API must be verified before implementation.
   - Evidence status: `MISSING`

---

### Requirement 10: Extension Mechanism

**User Story:** As a pack author and future tool integrator, I want a documented, stable extension mechanism for all CloudDialect document types so that I can add domain-specific fields without forking the core schemas or breaking existing validators — and as a verifier implementer, I want unknown extensions to be unable to silently influence acceptance decisions or security policies.

**Traceability:** AC-01, AC-05

#### Acceptance Criteria

1. THE Framework_Core SHALL publish an extension convention document specifying all of the following:
   - The namespace prefix grammar: an ASCII letter followed by zero or more ASCII alphanumeric characters or hyphens, minimum total length two characters, with a colon separator when the prefix is used in a qualified extension key (e.g., `acme:field-name`).
   - The reserved namespace families: `core`, `core-*`, `cdc`, and `cdc-*`. Reserved-prefix matching is case-insensitive: `CORE:`, `Core-Validation:`, `CDC:`, and `Cdc-Evidence:` are all reserved.
   - The list of currently reserved prefixes with concrete examples: `core:`, `core-validation:`, `cdc:`, `cdc-evidence:`.
   - The governance rule that new namespaces within existing reserved families MAY be introduced without modifying prefix-validation logic, and that changes altering previously permitted namespace usage require a compatibility review.
   - The ownership and collision-handling policy: extension namespace ownership SHALL be documented in the extension registry; prefixes alone SHALL NOT establish publisher identity or trust.
   - Verification method: Review of extension convention document confirming all six elements are present and the reserved-prefix list is consistent with the grammar.
   - Evidence status: `MISSING`

2. WHEN a document contains an extension field whose qualified key prefix conforms to the documented namespace grammar and is not a reserved prefix, THE Validator SHALL treat the field as a valid unknown property, SHALL NOT fail structural validation because of its presence, and SHALL preserve the field unchanged during any serialization or round-trip operation performed by a conforming parser.
   - Verification method: Conformance fixture asserting (a) a document with a valid community extension field passes structural validation, and (b) its qualified key is preserved and its encoded JSON value is byte-for-byte identical after round-trip parse-and-serialize, including escapes, numeric spelling and whitespace/member ordering inside that value. User clarification approved 2026-10-22: document/key ordering and whitespace outside extension values are not prescribed; decoded-value equality alone is insufficient.
   - Evidence status: `MISSING`

3. WHEN a document contains a field whose qualified key prefix matches a reserved namespace (case-insensitive), THE Validator SHALL surface a warning that includes: the violating field name exactly as it appeared in the document, and a description stating that the prefix is reserved for core use; this warning SHALL NOT cause the document to fail structural validation.
   - Verification method: Conformance fixture asserting that each of `core:`, `CORE:`, `Core-Validation:`, `cdc:`, `CDC:`, and `Cdc-Evidence:` prefixes produces a warning containing the verbatim field name, and the document still passes structural validation.
   - Evidence status: `MISSING`

4. IF an extension field is declared as required for behavioral correctness or security enforcement in a document (i.e., the document explicitly declares the extension as mandatory), AND THE active Validator does not recognize that extension namespace, THEN THE Validator SHALL produce an explicit blocker identifying the unrecognized mandatory extension and SHALL NOT permit the document to be accepted or the associated transformation to be promoted.
   - Owner-approved EX-1 declaration (2026-10-22): all five JSON document kinds permit optional root `required_extensions`, an array of unique decoded ASCII namespace prefixes without a colon. Omission or empty means none; duplicate strings, malformed prefixes and non-array/non-string values fail structurally. The declaration alone is sufficient to require the separate blocker; a payload is not required. Recognition requires trusted implemented semantics, never catalog/registry membership, ownership or payload claims. A structurally valid unknown mandatory declaration remains structurally valid and separately blocks acceptance.
   - Verification method: Integration test asserting that a document declaring an unrecognized mandatory extension produces a blocker result and cannot be accepted.
   - Evidence status: `MISSING`

5. Unknown optional extension fields SHALL NOT silently influence mandatory behavioral obligations, security policies, verifier decisions, or migration acceptance outcomes. THE Validator SHALL NOT interpret an unrecognized extension field as modifying, overriding, or extending any core schema field's semantics.
   - Verification method: Adversarial fixture containing an extension field that attempts to override a core verification result field (e.g., `acme:verification-status: "accepted"`) — asserting the core verification outcome is unchanged and the extension field is ignored for acceptance purposes.
   - Evidence status: `MISSING`

6. THE Framework_Core SHALL publish an extension registry document listing all known community extension namespaces with their claimed owner, purpose, and registration date; the registry SHALL be present as a file in every release artifact and MAY be empty at initial release.
   - Verification method: Review of release artifact confirming the registry file is present; review of at least one registered namespace entry for completeness of required fields when the first community extension is registered.
   - Evidence status: `MISSING`

7. THE Validator's adversarial conformance suite SHALL include fixtures covering all of the following bypass and edge-case scenarios:
   - Reserved namespace impersonation via exact match (e.g., `core:field`).
   - Reserved namespace bypass via uppercase (e.g., `CORE:field`, `CDC:field`).
   - Reserved namespace bypass via mixed case (e.g., `Core-Validation:field`, `Cdc-Evidence:field`).
   - Namespace collision between two community extensions using the same prefix from different pack authors.
   - Invalid prefix formats: prefix beginning with a digit, prefix containing a space, prefix of length one.
   - Unknown optional extension round-tripping: field is present, unmodified, and non-influencing after round-trip.
   - Unknown mandatory extension attempting to unlock acceptance: verifier must block.
   - Extension attempting to override a core verification result field: core outcome is unchanged.
   - Verification method: All eight fixture classes are present in the adversarial suite; each class includes at least one passing (correctly handled) and one failing (correctly rejected) case.
   - Evidence status: `MISSING`

---

### Requirement 11: Failure Behavior and Actionable Errors

**User Story:** As a pack author and tool integrator, I want every validation and resolution failure to produce an actionable error so that I can diagnose and correct the problem without guessing.

**Traceability:** AC-01, AC-02

#### Acceptance Criteria

1. WHEN THE Validator encounters any validation failure — structural, semantic, or version-related — THE Validator SHALL return an error response that includes: the document identifier (if parseable from the document), the document-format version (if parseable), the failure category drawn from `{structural, semantic, version, unresolvable-reference}`, the specific field path expressed in dot-notation, and a human-readable description; IF the document is not parseable at all, THEN THE Validator SHALL return an error response containing the failure category `unparseable` and the byte offset or line number at which parsing failed.
   - Verification method: Conformance fixture exercising each failure category, including an unparseable document, and asserting all required error fields are present in each case.
   - Evidence status: `MISSING`

2. THE Validator SHALL NOT silently accept, silently ignore, or partially validate any document that fails structural or semantic validation.
   - Verification method: Conformance fixture asserting that a document with a known structural error is rejected with an error result and no acceptance is indicated.
   - Evidence status: `MISSING`

3. IF a pack manifest declares a supported operation range that does not cover the source artifact being analyzed, THEN THE Manifest_Validator SHALL return an explicit result with category `unsupported` that identifies the out-of-range operation; it SHALL NOT return a generic failure.
   - Verification method: Integration test with a source artifact using an operation outside the manifest's declared coverage, asserting the result category is `unsupported` and the out-of-range operation is identified.
   - Evidence status: `MISSING`

4. WHEN missing mandatory evidence blocks acceptance of a transformation (per Requirement 3.6), THE Validator SHALL return a blocking-gap report that identifies: the affected transformation candidate identifier, the specific obligation identifier lacking evidence, and the unresolved assumption text; this report SHALL prevent the transformation from being promoted to an accepted state.
   - Verification method: Integration test asserting the blocking-gap report contains the three required fields and that the transformation cannot be accepted until the gap is resolved.
   - Evidence status: `MISSING`

---

### Requirement 12: Release and Dependency Integrity

**User Story:** As a framework core maintainer and auditor, I want every CloudDialect release to include a dependency inventory, license list, and verifiable artifact provenance so that integrators can verify the supply chain and auditors can assess license compliance.

**Traceability:** AC-09

#### Acceptance Criteria

1. THE Framework_Core SHALL include a machine-readable SBOM in CycloneDX JSON or SPDX JSON format in each release artifact, listing all direct and transitive dependencies of the schema toolchain and Validator, their versions, and their declared licenses.
   - Verification method: Review of release artifact confirming the SBOM is present in one of the two required formats and is parseable by a conforming CycloneDX or SPDX tool.
   - Evidence status: `MISSING`

2. THE Framework_Core SHALL include a cryptographic signature over the schema catalog release artifact using an asymmetric signing key whose public key is published in the CloudDialect repository; a consumer SHALL be able to verify the signature offline without contacting a third-party service.
   - Verification method: Review of release artifact for signature file; integration test asserting signature verification succeeds on the release artifact using the published public key.
   - Evidence status: `MISSING`

3. WHEN a dependency in the schema toolchain carries a license that is incompatible with CloudDialect's dual-licensing distribution model, THE Framework_Core SHALL produce a named incompatibility report identifying the dependency name, version, and license before including that dependency in a release.
   - Verification method: Review of dependency screening process documentation; SBOM review confirming no incompatible licenses are present in a qualifying release.
   - Evidence status: `MISSING`

---

## Non-Functional Requirements

### NFR-1: Interoperability

1. THE Behavioral_Contract_Schema, Evidence_Record_Schema, and Verification_Result_Schema SHALL be expressed in JSON Schema Draft 2020-12 such that any conforming Draft 2020-12 validator can execute structural validation against them.
   - Verification method: Structural review confirming no non-standard JSON Schema keywords are required; test with at least two independent Draft 2020-12 validators.
   - Evidence status: `MISSING`

2. THE Pack_Manifest_Schema SHALL be expressed in TOML 1.0 compatible syntax such that any conforming TOML 1.0 parser can read a valid manifest.
   - Verification method: Conformance fixture parsed by at least two independent TOML 1.0 parsers.
   - Evidence status: `MISSING`

### NFR-2: Traceability

1. THE Framework_Core SHALL maintain a traceability table mapping each published schema field to the requirement in this document that introduced it.
   - Verification method: Review of traceability table for completeness at each schema release.
   - Evidence status: `MISSING`

2. THE Framework_Core SHALL maintain a requirements traceability matrix linking each acceptance gate (AC-01, AC-02, AC-07, AC-09) to the requirements and schema fields that satisfy it.
   - Verification method: Review of traceability matrix for completeness.
   - Evidence status: `MISSING`

### NFR-3: Schema Stability

1. THE Framework_Core SHALL NOT change the semantics of a field in an existing MINOR release; semantic changes require a MAJOR version increment.
   - Verification method: Review of version changelog and schema diff for each release.
   - Evidence status: `MISSING`

---

## Unresolved Decisions

The following decisions are explicitly unresolved. Requirements that depend on these decisions are marked with the relevant item number. No implementation work for the affected areas may be finalized until the decision is recorded.

| # | Decision | Impact |
|---|---|---|
| UD-1 | **Reference workload identity**: The exact Python OSS application used to exercise the schemas in the first implementation has not been selected. Acceptance criteria requiring schema exercise against a real workload (particularly Requirement 2.5 and Requirement 9.2 end-to-end validation) depend on this selection. | Delays end-to-end integration test completion for Specs 2 and 3. |
| UD-2 | **JSON Schema tooling for Rust**: The specific Rust crate(s) used to validate JSON Schema Draft 2020-12 documents have not been selected. Library selection occurs in the design phase after compatibility and license verification. | Affects NFR-1 verification and Requirement 9.1 implementation. |
| UD-3 | **TOML parsing library**: The Rust crate used to parse TOML pack manifests has not been selected. Design phase decision after license and maintenance verification. | Affects NFR-2 verification and Requirement 5 implementation. |
| UD-4 | **Conformance fixture format**: The format for shared test fixtures used to validate Rust types and language-worker implementations against the schemas has not been decided. All requirements marked "Conformance fixture" as their verification method depend on this decision. | Blocks writing of any conformance fixture until resolved. |
| UD-5 | **Schema registry mechanism**: The exact structure of the bundled schema catalog and how it is distributed with releases (e.g., embedded in a binary, as a directory alongside the binary, as a separate downloadable archive) has not been decided. | Affects Requirement 1 (distribution), Requirement 9 (offline resolution), and Requirement 12 (provenance). |

---

## Acceptance Gate Traceability

| Gate | Description | Requirements |
|---|---|---|
| AC-01 | Every transformation has a versioned manifest with supported versions, operations, assumptions, and limitations. Unsupported inputs produce explicit results. | Req 1 (schema authoring and distribution, 6 criteria: 1.1–1.6), Req 5 (manifest format), Req 6 (versioning), Req 7 (compatibility policy), Req 10 (extension mechanism — convention document, valid-namespace pass-through, reserved-namespace warning, registry), Req 11 (failure behavior), NFR-1 |
| AC-02 | Every mandatory behavioral obligation links to source evidence, approved expected outcome, and verification method. Unresolved mandatory obligations block acceptance. | Req 2 (contract format), Req 3 (evidence format), Req 4 (verification result, 7 criteria: 4.1–4.7), Req 8 (validation boundaries), Req 11 (failure behavior), NFR-2 |
| AC-05 | Verifier integrity: attempts to weaken protected acceptance tests, forge results, substitute candidates, or bypass acceptance via extensions cannot obtain an accepted result. | Req 10 (extension mechanism — mandatory extension blockers, override prevention, adversarial fixtures) |
| AC-07 | Every accepted result records input, candidate, pack, toolchain, test identities, configuration, outcomes, and approvals. Altered or stale artifacts invalidate the acceptance result. | Req 3 (evidence format), Req 4 (verification result, 7 criteria: 4.1–4.7), Req 8 (validation boundaries) |
| AC-09 | Releases include dependency/license inventories, SBOMs, and verifiable artifact provenance. Tampered artifacts and unapproved permission escalation fail policy checks. | Req 1 (schema distribution, 6 criteria: 1.1–1.6), Req 6 (versioning), Req 7 (compatibility), Req 12 (release integrity), NFR-1 |
