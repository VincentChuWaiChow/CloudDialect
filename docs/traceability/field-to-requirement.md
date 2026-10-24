# Field-to-requirement traceability

## Scope and source authority

This is the field-level table required by NFR-2.1 and canonical Task 25, not a
migration acceptance decision. It covers **148 declared JSON instance paths**
across five schemas, **40 nested TOML manifest paths**, and a **14-entry Rust API
appendix**. Container declarations and conditional-only fields are included;
shared local schema references are expanded at each consuming instance path.
Requirement mappings identify the introducing obligation and relevant constraints,
not a claim that every requirement's verification method is complete.

Primary normative sources are the Spec 1 [requirements](../../.kiro/specs/contracts-evidence-foundation/requirements.md),
[design](../../.kiro/specs/contracts-evidence-foundation/design.md),
[canonical tasks, Task 25](../../.kiro/specs/contracts-evidence-foundation/tasks.md),
[implementation plan, unit 3.5](../../.kiro/specs/contracts-evidence-foundation/implementation-plan.md),
and [P0-A–F amendments](../../.kiro/specs/contracts-evidence-foundation/p0-contract-amendments.md).
Some historical design/amendment paragraphs still describe proposals or earlier
stubs; the following checked-in declarations, not those historical delivery
claims, define the finite paths tabulated here:

- [Behavioral contract schema](../../catalog/schemas/behavioral-contract/1.0/schema.json),
  [evidence record schema](../../catalog/schemas/evidence-record/1.0/schema.json),
  [verification result schema](../../catalog/schemas/verification-result/1.0/schema.json).
- [Catalog index schema](../../catalog/schemas/catalog-index/1.0/schema.json) and
  [extension registry schema](../../catalog/schemas/extension-registry/1.0/schema.json).
- [Manifest declarations](../../crates/clouddialect-contracts/src/manifest/types.rs),
  [document references and store port](../../crates/clouddialect-contracts/src/documents/references.rs),
  [verification/result projections](../../crates/clouddialect-contracts/src/documents/verification.rs),
  and [phase results](../../crates/clouddialect-contracts/src/errors/phase.rs).

### Path and open-container interpretation

`[*]` means each array entry; `contracts.*` means each operation-keyed contract
reference, not wildcard resource permission. Scalar array entries do not invent
an additional field declaration. `$schema` rows refer to optional **instance**
schema selectors, not the schema document's dialect declaration. `$defs`, `$ref`,
`if`, `then`, `else`, and `patternProperties` are schema machinery, not extra
instance fields; fields introduced through them are included at their instance
paths. In particular, evidence provenance/reasoning and obligation alternatives
are not omitted just because they occur in conditional branches.

`inputs` and `outputs` are opaque embedded JSON Schema objects. Resource
`configuration` is also an open object. Their author-supplied keys cannot be
exhaustively enumerated as finite CloudDialect fields. TOML `[x]` retains arbitrary
TOML values, including integer/datetime types; it is not an alternative capability
declaration. The table stops at these declared container boundaries rather than
fabricating child fields or pretending an embedded schema is a policy grant.

The three primary document schemas admit root qualified extension keys through
`patternProperties`: an ASCII letter followed by at least one ASCII alphanumeric
or hyphen and a colon. This pattern defines a namespace family, not a finite
field list. Req 10.1, Req 10.2 and Req 10.3 define grammar, exact encoded JSON value
preservation and case-insensitive reserved families `core`, `core-*`, `cdc`,
`cdc-*`; a reserved key warns verbatim rather than becoming authority. The catalog
index and registry schemas have their own root openness; do not infer the primary
schemas' root closure or extension pattern on every nested object. All five kinds
have explicit `required_extensions` (Req 10.4): unknown mandatory semantics block
separately even without payload. Registry ownership or optional extension content
does not establish recognition, authentication, or acceptance (Req 10.5).

## Behavioral contract: operation, resource and obligation declarations

The resource-dimension mapping follows Req 2.2 and the owner-approved RC-1
[reconciliation decisions](../../.workflow/ultracode/execution-isolation-implementation/contract-reconciliation-decisions.md):
each ordering/consistency/retry/failure dimension has a known declaration or an
explicitly discriminated unresolved assumption. Coexisting known and unknown
information does not silently resolve the unknown part.

| Schema | Field path | Requirement | Notes |
|---|---|---|---|
| behavioral-contract | `$schema` | Req 1.2, Req 1.3, Req 9.3 | Optional exact catalog selector; unknown or mismatched URI cannot silently fall back. |
| behavioral-contract | `behavioral_notes` | Req 2.5 | Service-specific differences outside base behavior fields, with evidence-linked notes. |
| behavioral-contract | `behavioral_notes[*].description` | Req 2.5 | Human-readable behavioral difference, such as integrity-checksum or delivery semantics. |
| behavioral-contract | `behavioral_notes[*].evidence_ref` | Req 2.5, Req 8.3 | Required note evidence ID; semantic validation resolves it independently of structural success. |
| behavioral-contract | `behavioral_notes[*].note_type` | Req 2.5 | Open note-type vocabulary; it is not a closed provider-specific enum. |
| behavioral-contract | `format_version` | Req 2.1, Req 6.1, Req 6.2 | Document-format SemVer, distinct from dialect and pack version; syntax is not support admission. |
| behavioral-contract | `id` | Req 2.1, Req 6.5 | Pack-local contract identity also follows the global lowercase UUID v4 instance scheme. |
| behavioral-contract | `inputs` | Req 2.1 | Opaque embedded input JSON Schema object, not an enumerated CloudDialect child vocabulary. |
| behavioral-contract | `obligations` | Req 2.3, Req 2.4 | Behavioral obligations with fidelity and evidence-or-assumption alternatives; declaration is not verification. |
| behavioral-contract | `obligations[*].behavioral_fidelity` | Req 2.3 | Five-value detailed fidelity vocabulary; distinct from the manifest three-value summary. |
| behavioral-contract | `obligations[*].evidence_refs` | Req 2.4, Req 8.3 | Nonempty evidence-ID alternative; scoped semantic resolution is separate from parsing. |
| behavioral-contract | `obligations[*].obligation_id` | Req 2.4, Req 3.6 | Identifies the obligation for evidence linkage and a located blocking-gap report. |
| behavioral-contract | `obligations[*].unresolved_assumption` | Req 2.4, Req 11.4 | Explicit unknown-evidence alternative, mutually exclusive with evidence_refs in this schema. |
| behavioral-contract | `obligations[*].unresolved_assumption.assumption_id` | Req 2.4 | Machine-readable identifier for the unresolved obligation assumption. |
| behavioral-contract | `obligations[*].unresolved_assumption.description` | Req 2.4, Req 11.4 | What is unknown; completeness retains this text as a blocking gap. |
| behavioral-contract | `obligations[*].unresolved_assumption.reason` | Req 2.4 | Why the information could not be established; not generated replacement evidence. |
| behavioral-contract | `operation_identity` | Req 2.1 | Source operation boundary and wrapper-chain declaration. |
| behavioral-contract | `operation_identity.call_site` | Req 2.1 | Identifies the operation entry point; not an authenticated execution receipt. |
| behavioral-contract | `operation_identity.wrapper_chain` | Req 2.1 | Ordered wrapper names; when declared the array is nonempty. |
| behavioral-contract | `outputs` | Req 2.1 | Opaque embedded output JSON Schema object; no behavioral execution is proved. |
| behavioral-contract | `required_extensions` | Req 2.6, Req 10.4, Req 10.5 | Optional unique namespace-prefix list; an unknown mandatory declaration separately blocks, never unlocks acceptance. |
| behavioral-contract | `resource_dependencies` | Req 2.2 | Dependencies with identity, permissions, configuration and per-dimension behavioral coverage. |
| behavioral-contract | `resource_dependencies[*].configuration` | Req 2.2 | Open resource configuration object; arbitrary configuration keys are not finite schema paths. |
| behavioral-contract | `resource_dependencies[*].identity` | Req 2.2 | Declared dependency identity, not caller or resource authentication. |
| behavioral-contract | `resource_dependencies[*].known_behavior` | Req 2.2 | Known ordering/consistency/retry/failure declarations; each unknown dimension needs an assumption. |
| behavioral-contract | `resource_dependencies[*].known_behavior.consistency` | Req 2.2 | Nonempty known consistency statement; declaration is not observed-provider proof. |
| behavioral-contract | `resource_dependencies[*].known_behavior.failure` | Req 2.2 | Nonempty known failure statement, covering the failure dimension under RC-1. |
| behavioral-contract | `resource_dependencies[*].known_behavior.ordering` | Req 2.2 | Nonempty known ordering statement; RC-1 separately permits a matching unresolved ordering assumption. |
| behavioral-contract | `resource_dependencies[*].known_behavior.retry` | Req 2.2 | Nonempty known retry statement, covering the retry dimension under RC-1. |
| behavioral-contract | `resource_dependencies[*].permissions` | Req 2.2 | Required permission names; describing permissions never grants them. |
| behavioral-contract | `resource_dependencies[*].unresolved_assumptions` | Req 2.2, Req 11.4 | Explicit unknown resource behavior; completeness projects these gaps against declared obligations. |
| behavioral-contract | `resource_dependencies[*].unresolved_assumptions[*].assumption_id` | Req 2.2 | RC-1 nonempty machine-readable resource-assumption identifier. |
| behavioral-contract | `resource_dependencies[*].unresolved_assumptions[*].behavior` | Req 2.2 | RC-1 closed discriminator: ordering, consistency, retry or failure; not guessed from free text. |
| behavioral-contract | `resource_dependencies[*].unresolved_assumptions[*].description` | Req 2.2, Req 11.4 | Nonempty statement of the resource unknown; carried as blocking assumption text. |
| behavioral-contract | `resource_dependencies[*].unresolved_assumptions[*].reason` | Req 2.2 | Nonempty explanation of why the resource behavior remains unknown. |
| behavioral-contract | `side_effects` | Req 2.1 | Declared externally observable side effects; an empty declaration is representable. |
| behavioral-contract | `side_effects[*].description` | Req 2.1 | Human-readable statement of the externally visible effect. |
| behavioral-contract | `side_effects[*].evidence_ref` | Req 2.1, Req 8.3 | Optional supporting evidence ID; when present the semantic phase resolves the evidence through the scoped store. |

## Evidence: classification, provenance and unresolved support

Conditional branches require observed provenance, inferred reasoning or unknown assumptions; none authenticates observations.

| Schema | Field path | Requirement | Notes |
|---|---|---|---|
| evidence-record | `$schema` | Req 1.2, Req 1.3, Req 9.3 | Optional exact admitted schema URI, not runtime network retrieval. |
| evidence-record | `claim` | Req 3.1 | Claim being recorded; text does not itself establish an observed fact. |
| evidence-record | `classification` | Req 3.1, Req 3.2 | Explicit observed/inferred/unknown value; neither omission nor implicit default is allowed. |
| evidence-record | `format_version` | Req 3.1, Req 6.1, Req 6.2 | Explicit document-format SemVer; independent of tool, pack and schema dialect versions. |
| evidence-record | `id` | Req 3.1, Req 6.5 | Globally unique lowercase UUID v4 evidence identity; no version ambiguity may be hidden by lookup. |
| evidence-record | `provenance` | Req 3.1 | Observed-branch provenance object; conditional declaration is included in the inventory. |
| evidence-record | `provenance.analysis_tool_identity` | Req 3.1 | Analysis tool claimed for observed provenance; a tool name is not an attestation. |
| evidence-record | `provenance.source_artifact_identity` | Req 3.1 | Source artifact claimed for observed provenance; authenticity requires independent evidence. |
| evidence-record | `reasoning_chain` | Req 3.1, Req 3.3 | Inferred-branch array requires at least one reasoning step; inference does not become observation. |
| evidence-record | `required_extensions` | Req 3.5, Req 10.4, Req 10.5 | Unknown mandatory namespace blocks separately; optional provenance extensions cannot override evidence classification. |
| evidence-record | `unresolved_assumptions` | Req 3.1, Req 3.4, Req 3.6 | Unknown classification requires a nonempty list; unresolved support can block mandatory obligations. |
| evidence-record | `unresolved_assumptions[*].assumption_id` | Req 3.1, Req 3.4 | Machine-readable identifier of the remaining evidence gap. |
| evidence-record | `unresolved_assumptions[*].description` | Req 3.1, Req 3.4, Req 3.6 | States the unresolved gap; retained in obligation-specific blocking reports. |
| evidence-record | `unresolved_assumptions[*].reason` | Req 3.1, Req 3.4 | Explains why the gap remains; no fabricated reasoning fills an unknown. |

## Verification: immutable artifact declarations, tests and approval references

P0-C adds exact configuration and execution bindings. Req 4.8 consistency checks do not authenticate origins, approval authority, or the protected obligation set.

| Schema | Field path | Requirement | Notes |
|---|---|---|---|
| verification-result | `$schema` | Req 1.2, Req 1.3, Req 9.3 | Optional exact admitted selector; unknown or mismatched URI rejects without fallback. |
| verification-result | `acceptance` | Req 4.4, Req 4.8 | Optional approval declaration with bound references; presence is not authenticated acceptance. |
| verification-result | `acceptance.approval_evidence` | Req 4.4, Req 4.6, Req 4.8 | P0-C reference to the independently checked immutable approval-evidence artifact; declaration is not provenance or authority. |
| verification-result | `acceptance.approval_evidence.content_hash` | Req 4.4, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the independently checked immutable approval-evidence artifact bytes; mismatch invalidates acceptance. |
| verification-result | `acceptance.approval_evidence.content_hash.algorithm` | Req 4.4, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `acceptance.approval_evidence.content_hash.value` | Req 4.4, Req 4.6, Req 4.8 | Lowercase hex digest of the independently checked immutable approval-evidence artifact bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `acceptance.approval_evidence.identity` | Req 4.4, Req 4.6, Req 4.8 | Nonempty identity of the independently checked immutable approval-evidence artifact; compare against independent trusted context. |
| verification-result | `acceptance.approver_identities` | Req 4.4, Req 4.8 | At least one distinct nonempty approver claim; independent authority, expiry and revocation checks remain required. |
| verification-result | `acceptance.timestamp` | Req 4.4, Req 4.8 | Calendar-valid normalized UTC Z time; schema annotation alone is insufficient. |
| verification-result | `acceptance.verification_result` | Req 4.4, Req 4.8 | P0-C exact DocumentRef to this result; no self-containing document digest is required. |
| verification-result | `acceptance.verification_result.format_version` | Req 4.4, Req 4.8 | Must equal this result document-format version, not a pack or policy version. |
| verification-result | `acceptance.verification_result.id` | Req 4.4, Req 4.8 | Must equal this verification result ID; matching is consistency, not approver authentication. |
| verification-result | `binding` | Req 4.1, Req 4.8 | P0-C execution/artifact/obligation declaration; trusted expected context must be checked independently. |
| verification-result | `binding.attempt_id` | Req 4.1, Req 4.8 | Declared attempt identity; semantic validity alone does not prove replay resistance. |
| verification-result | `binding.dependencies` | Req 4.1, Req 4.6, Req 4.8 | P0-C reference to the immutable dependency artifact or exact member inventory; declaration is not provenance or authority. |
| verification-result | `binding.dependencies.content_hash` | Req 4.1, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the immutable dependency artifact or exact member inventory bytes; mismatch invalidates acceptance. |
| verification-result | `binding.dependencies.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `binding.dependencies.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Lowercase hex digest of the immutable dependency artifact or exact member inventory bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `binding.dependencies.identity` | Req 4.1, Req 4.6, Req 4.8 | Nonempty identity of the immutable dependency artifact or exact member inventory; compare against independent trusted context. |
| verification-result | `binding.input` | Req 4.1, Req 4.6, Req 4.8 | P0-C reference to the immutable input artifact or exact composite-member inventory; declaration is not provenance or authority. |
| verification-result | `binding.input.content_hash` | Req 4.1, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the immutable input artifact or exact composite-member inventory bytes; mismatch invalidates acceptance. |
| verification-result | `binding.input.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `binding.input.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Lowercase hex digest of the immutable input artifact or exact composite-member inventory bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `binding.input.identity` | Req 4.1, Req 4.6, Req 4.8 | Nonempty identity of the immutable input artifact or exact composite-member inventory; compare against independent trusted context. |
| verification-result | `binding.obligation_ids` | Req 4.1, Req 4.8 | Nonempty unique declared obligation set; protected mandatory selection cannot be replaced by this candidate-controlled list. |
| verification-result | `binding.oracle` | Req 4.1, Req 4.6, Req 4.8 | P0-C reference to the immutable expected-outcome/oracle artifact; declaration is not provenance or authority. |
| verification-result | `binding.oracle.content_hash` | Req 4.1, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the immutable expected-outcome/oracle artifact bytes; mismatch invalidates acceptance. |
| verification-result | `binding.oracle.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `binding.oracle.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Lowercase hex digest of the immutable expected-outcome/oracle artifact bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `binding.oracle.identity` | Req 4.1, Req 4.6, Req 4.8 | Nonempty identity of the immutable expected-outcome/oracle artifact; compare against independent trusted context. |
| verification-result | `binding.policy` | Req 4.1, Req 4.6, Req 4.8 | P0-C reference to the immutable policy artifact, not a policy name resolved afresh; declaration is not provenance or authority. |
| verification-result | `binding.policy.content_hash` | Req 4.1, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the immutable policy artifact, not a policy name resolved afresh bytes; mismatch invalidates acceptance. |
| verification-result | `binding.policy.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `binding.policy.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Lowercase hex digest of the immutable policy artifact, not a policy name resolved afresh bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `binding.policy.identity` | Req 4.1, Req 4.6, Req 4.8 | Nonempty identity of the immutable policy artifact, not a policy name resolved afresh; compare against independent trusted context. |
| verification-result | `binding.project_id` | Req 4.1, Req 4.8 | Opaque claimed project binding; not authorization to query another project. |
| verification-result | `binding.run_id` | Req 4.1, Req 4.8 | Declared run identity; trusted run/freshness comparison is outside syntax checks. |
| verification-result | `binding.suite` | Req 4.1, Req 4.6, Req 4.8 | P0-C reference to the immutable protected test-suite artifact; declaration is not provenance or authority. |
| verification-result | `binding.suite.content_hash` | Req 4.1, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the immutable protected test-suite artifact bytes; mismatch invalidates acceptance. |
| verification-result | `binding.suite.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `binding.suite.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Lowercase hex digest of the immutable protected test-suite artifact bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `binding.suite.identity` | Req 4.1, Req 4.6, Req 4.8 | Nonempty identity of the immutable protected test-suite artifact; compare against independent trusted context. |
| verification-result | `binding.tenant_id` | Req 4.1, Req 4.8 | Opaque claimed tenant binding, never a way for an input to choose authenticated tenant scope. |
| verification-result | `candidate` | Req 4.1, Req 4.2, Req 4.6, Req 4.8 | P0-C reference to the transformation candidate; declaration is not provenance or authority. |
| verification-result | `candidate.content_hash` | Req 4.1, Req 4.2, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the transformation candidate bytes; mismatch invalidates acceptance. |
| verification-result | `candidate.content_hash.algorithm` | Req 4.1, Req 4.2, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `candidate.content_hash.value` | Req 4.1, Req 4.2, Req 4.6, Req 4.8 | Lowercase hex digest of the transformation candidate bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `candidate.identity` | Req 4.1, Req 4.2, Req 4.6, Req 4.8 | Nonempty identity of the transformation candidate; compare against independent trusted context. |
| verification-result | `configuration` | Req 4.1, Req 4.6, Req 4.8 | P0-C reference to the exact immutable secret-free configuration artifact; declaration is not provenance or authority. |
| verification-result | `configuration.content_hash` | Req 4.1, Req 4.6, Req 4.8 | Algorithm/value digest claim binding the exact immutable secret-free configuration artifact bytes; mismatch invalidates acceptance. |
| verification-result | `configuration.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Shared approved five-algorithm vocabulary; candidate algorithm requirement is Req 4.2; other artifacts reuse the P0-C hash representation. |
| verification-result | `configuration.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Lowercase hex digest of the exact immutable secret-free configuration artifact bytes; semantic checks enforce the algorithm-specific length, not authenticity. |
| verification-result | `configuration.identity` | Req 4.1, Req 4.6, Req 4.8 | Nonempty identity of the exact immutable secret-free configuration artifact; compare against independent trusted context. |
| verification-result | `format_version` | Req 4.1, Req 6.1, Req 6.2 | Verification document-format SemVer; distinct from pack/toolchain versions. |
| verification-result | `id` | Req 4.1, Req 6.5 | Unique lowercase UUID v4 result identity; required acceptance self-reference must match it. |
| verification-result | `pack` | Req 4.1, Req 4.6, Req 4.8 | Versioned pack artifact with immutable digest; a version label alone does not identify executable bytes. |
| verification-result | `pack.content_hash` | Req 4.1, Req 4.6, Req 4.8 | P0-C pack executable/content digest claim; independent equality/freshness checks remain required. |
| verification-result | `pack.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Approved shared digest algorithm for the pack bytes; a hash is not a signature. |
| verification-result | `pack.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Canonical lowercase hex pack digest; semantic checks enforce exact algorithm-specific length. |
| verification-result | `pack.id` | Req 4.1 | Nonempty pack identity used in the verification record, not authenticated publisher identity. |
| verification-result | `pack.version` | Req 4.1, Req 6.1 | Full SemVer pack version, including valid suffixes; not this verification document-format version. |
| verification-result | `required_extensions` | Req 4.5, Req 10.4, Req 10.5 | Unknown mandatory namespace separately blocks; optional extensions cannot rewrite a core outcome or approval. |
| verification-result | `tests` | Req 4.1, Req 4.7 | At least one test required structurally; an array is not proof those tests were executed. |
| verification-result | `tests[*].failure_description` | Req 4.3 | Closed structured failure object for fail/skip/error; legacy strings do not satisfy P0-C. |
| verification-result | `tests[*].failure_description.detail` | Req 4.3 | Required safe human-readable detail, 1–2000 Unicode characters; not raw logs or secret-bearing output. |
| verification-result | `tests[*].obligation_ids` | Req 4.1, Req 4.8 | Nonempty unique test links must belong to binding.obligation_ids; protected completeness remains a verifier duty. |
| verification-result | `tests[*].origin_identity` | Req 4.1, Req 8.4, Req 4.8 | Declared test origin; distinct independently controlled origin must be established outside self-reported strings. |
| verification-result | `tests[*].outcome` | Req 4.1, Req 4.3 | Closed pass/fail/skip/error vocabulary; a non-pass requires structured safe detail. |
| verification-result | `tests[*].test_id` | Req 4.1, Req 4.8 | Nonempty test identity; semantic checks reject duplicates across entries. |
| verification-result | `toolchain` | Req 4.1, Req 4.6, Req 4.8 | Versioned toolchain artifact with immutable digest; a version label alone does not identify executable bytes. |
| verification-result | `toolchain.content_hash` | Req 4.1, Req 4.6, Req 4.8 | P0-C toolchain executable/content digest claim; independent equality/freshness checks remain required. |
| verification-result | `toolchain.content_hash.algorithm` | Req 4.1, Req 4.6, Req 4.8 | Approved shared digest algorithm for the toolchain bytes; a hash is not a signature. |
| verification-result | `toolchain.content_hash.value` | Req 4.1, Req 4.6, Req 4.8 | Canonical lowercase hex toolchain digest; semantic checks enforce exact algorithm-specific length. |
| verification-result | `toolchain.id` | Req 4.1 | Nonempty toolchain identity used in the verification record, not authenticated publisher identity. |
| verification-result | `toolchain.version` | Req 4.1, Req 6.1 | Full SemVer toolchain version, including valid suffixes; not this verification document-format version. |

## Schema catalog: application inventory, support material and lifecycle

P0-D separates resolver support resources from application schemas; the catalog never becomes the project document store.

| Schema | Field path | Requirement | Notes |
|---|---|---|---|
| catalog-index | `$schema` | Req 1.6, Req 9.3 | Optional schema selector for the index itself; self-validation still uses admitted catalog material. |
| catalog-index | `format_version` | Req 1.6, Req 6.1, Req 6.2 | Index document-format version, distinct from individual listed schema versions. |
| catalog-index | `required_extensions` | Req 10.4, Req 10.5 | EX-1 declaration applies to the index too; catalog membership never recognizes unknown mandatory semantics. |
| catalog-index | `resolver_resources` | Req 1.2, Req 9.2, Req 9.5 | P0-D separate support-resource inventory; empty only with justified pinned-engine offline built-ins. |
| catalog-index | `resolver_resources[*].catalog_path` | Req 1.6, Req 9.4, Req 9.5 | Frozen catalog-relative support bytes; exact indexed lookup, not a live filesystem overlay. |
| catalog-index | `resolver_resources[*].schema_id` | Req 1.2, Req 9.2, Req 9.5 | Exact admitted meta-schema/vocabulary URI; never a host-prefix or network permission. |
| catalog-index | `schemas` | Req 1.6, Req 9.4 | Nonempty authoritative application-schema inventory; no private project documents or fake manifest schema. |
| catalog-index | `schemas[*].catalog_path` | Req 1.4, Req 1.6, Req 9.4 | Stable catalog-relative versioned path; loader admission rejects unsafe or unresolved paths. |
| catalog-index | `schemas[*].document_type` | Req 1.1, Req 1.6 | Five-kind application schema discriminator; catalog index and registry support the three primary document kinds. |
| catalog-index | `schemas[*].end_of_life_date` | Req 1.5, Req 7.1 | Date required for end-of-life-declared status; not automatic permission to remove compatibility support. |
| catalog-index | `schemas[*].format_version` | Req 1.6, Req 6.2 | Listed schema document-format SemVer, not the containing index version. |
| catalog-index | `schemas[*].schema_id` | Req 1.6, Req 9.4 | Exact application schema URI, unique across application and resolver resource sets. |
| catalog-index | `schemas[*].status` | Req 1.5, Req 7.1 | Current, supported or end-of-life-declared status records the retention/support lifecycle. |

## Extension registry: namespace governance, not trust

Req 10.6 requires release inclusion; listing a namespace or claimed owner is not implementation recognition or authenticated ownership.

| Schema | Field path | Requirement | Notes |
|---|---|---|---|
| extension-registry | `$schema` | Req 1.6, Req 9.3, Req 10.6 | Optional catalog-resolved registry schema selector; distribution obligation is separate. |
| extension-registry | `extensions` | Req 10.6 | Known community namespace listings; explicitly empty registry is allowed at initial release. |
| extension-registry | `extensions[*].namespace_prefix` | Req 10.1, Req 10.6 | ASCII namespace prefix without colon; registration does not authenticate publisher identity. |
| extension-registry | `extensions[*].owner` | Req 10.1, Req 10.6 | Claimed namespace owner for governance/collision handling, not a trusted principal. |
| extension-registry | `extensions[*].purpose` | Req 10.6 | Documented purpose of the registered namespace; no implied implemented semantics. |
| extension-registry | `extensions[*].registration_date` | Req 10.6 | Registry registration date; schema date syntax is not external registration evidence. |
| extension-registry | `format_version` | Req 6.1, Req 6.2, Req 10.6 | Registry document-format SemVer, not namespace policy authority. |
| extension-registry | `required_extensions` | Req 10.4, Req 10.5 | Registry declarations can themselves block; registry entries never grant namespace recognition. |

## Transformation pack: supported coverage and requested capabilities

Paths follow the actual nested Rust manifest declarations. Req 5.8/P0-B requests stay distinct from effective policy and from the opaque [x] table.

| Schema | Field path | Requirement | Notes |
|---|---|---|---|
| manifest | `capabilities` | Req 5.8 | P0-B required typed request table outside [x]; declaration grants nothing. |
| manifest | `capabilities.format_version` | Req 5.8, Req 6.1 | Explicit supported capability-declaration format version, not a permission-policy revision. |
| manifest | `capabilities.requirements` | Req 5.8 | Explicit typed request list; an explicitly empty list is valid, absence is not a silent default. |
| manifest | `capabilities.requirements[*].action` | Req 5.8 | Closed request action; propose/approve/deploy/delete never imply one another or grant worker authority. |
| manifest | `capabilities.requirements[*].id` | Req 5.8 | Nonempty request identifier, unique within this declaration. |
| manifest | `capabilities.requirements[*].operation_id` | Req 5.8 | Must name an operation declared by this manifest; not a cross-project operation selector. |
| manifest | `capabilities.requirements[*].purpose` | Req 5.8 | Nonempty declared reason for the request, not proof of necessity or approved authority. |
| manifest | `capabilities.requirements[*].resource` | Req 5.8 | Opaque logical resource alias; no path, URL or wildcard expansion, with concrete authorization left to trusted admission. |
| manifest | `compatibility` | Req 5.4, Req 6.3 | Catalog compatibility declaration; neither a whole release compatibility proof nor owner acceptance. |
| manifest | `compatibility.catalog_max` | Req 5.4 | Maximum catalog version the pack declares validated; the string is not qualification evidence. |
| manifest | `compatibility.catalog_min` | Req 5.4 | Minimum compatible catalog version in SemVer syntax. |
| manifest | `contracts` | Req 5.3, Req 8.3 | Operation-keyed exact contract references; no inline behavioral contract content. |
| manifest | `contracts.*.format_version` | Req 5.3, Req 8.3 | Exact contract document-format SemVer; not a floating newest-version lookup. |
| manifest | `contracts.*.id` | Req 5.3, Req 8.3 | Exact contract document identifier resolved only through the supplied project-scoped store. |
| manifest | `operations` | Req 5.2, Req 5.7 | Supported-operation declarations retain unsupported entries rather than silently dropping them. |
| manifest | `operations[*].assumptions` | Req 5.2 | Optional assumptions relied on by the transformation; no automatic evidence promotion. |
| manifest | `operations[*].behavioral_fidelity` | Req 5.2, Req 5.7, Req 2.3 | Closed full/partial/unsupported summary; semantic fidelity mapping checks the referenced detailed contract obligations. |
| manifest | `operations[*].id` | Req 5.2, Req 11.3 | Operation identity used for coverage and explicit unsupported diagnostics. |
| manifest | `operations[*].limitations` | Req 5.2 | Optional known limitations; not silently removed to claim full coverage. |
| manifest | `pack` | Req 5.1 | Pack metadata table; parsing never authenticates the pack publisher. |
| manifest | `pack.description` | Req 5.1 | Human-readable scope/description; not evidence that transformations work. |
| manifest | `pack.id` | Req 5.1 | Dot-separated reverse-domain pack identifier; not a UUID document-instance ID. |
| manifest | `pack.name` | Req 5.1 | Human-readable pack name. |
| manifest | `pack.version` | Req 5.1, Req 6.1 | Full SemVer pack release version; not catalog or document-format version. |
| manifest | `schema_refs` | Req 5.1, Req 6.1 | Separate format-version references for the produced JSON document families. |
| manifest | `schema_refs.contract_schema_version` | Req 5.1, Req 6.1 | Behavioral contract document-format version, resolved against the active catalog. |
| manifest | `schema_refs.evidence_record_schema_version` | Req 5.1, Req 6.1 | Evidence record document-format version; not an evidence instance reference. |
| manifest | `schema_refs.verification_result_schema_version` | Req 5.1, Req 6.1 | Verification result document-format version, distinct from pack/toolchain versions. |
| manifest | `source` | Req 5.1 | Supported source provider/service/language environment declaration. |
| manifest | `source.cloud` | Req 5.1 | Declared source cloud provider, not live account or provider qualification. |
| manifest | `source.language` | Req 5.1 | Supported source programming language. |
| manifest | `source.language_version_range` | Req 5.1 | Source language SemVer range; syntax support does not establish a real workload match. |
| manifest | `source.service` | Req 5.1 | Declared source service. |
| manifest | `target` | Req 5.1 | Target cloud/service declaration; no deployment or cutover authority. |
| manifest | `target.cloud` | Req 5.1 | Declared target provider. |
| manifest | `target.service` | Req 5.1 | Declared target service. |
| manifest | `toolchain` | Req 5.5 | Dependency list must be nonempty for full/partial operations; absent decoding to empty is not unconditional validity. |
| manifest | `toolchain[*].name` | Req 5.5 | Required toolchain dependency name; not executable bytes or a publisher attestation. |
| manifest | `toolchain[*].version_range` | Req 5.5 | Full SemVer dependency constraint checked locally; actual runtime compatibility remains separate. |
| manifest | `x` | Req 5.6, Req 5.8 | Opaque optional TOML extension table; core-key shadowing rejects and extensions cannot replace typed capability requests. |

## Rust API appendix: ports, results and frozen schema context

These 14 named API keys supplement, rather than pretend to be fields of, a sixth JSON schema. P0-A result projections and P0-E resolution ports remain non-accepting.

| Schema | Field path | Requirement | Notes |
|---|---|---|---|
| rust-api | `DocumentRef.id` | Req 4.4, Req 5.3, Req 8.3 | P0-A/P0-E exact document identity; parsing a reference neither resolves nor authenticates it. |
| rust-api | `DocumentRef.format_version` | Req 5.3, Req 6.1, Req 8.3 | P0-E exact document-format version, distinct from pack/catalog/dialect versions. |
| rust-api | `StoredDocument.reference` | Req 8.3, Req 8.7 | P0-E claimed returned identity/version must match the request and decoded document; trait conformance is not authentication. |
| rust-api | `StoredDocument.content_hash` | Req 8.3, Req 8.7 | P0-E claimed digest must match exact returned bytes before reference consumption; hash equality is not owner provenance. |
| rust-api | `StoredDocument.bytes` | Req 8.3, Req 8.7 | Returned document bytes are checked for hash, expected kind, identity/version, structural validity and mandatory semantics before use; the public byte vector is not a backend immutability guarantee. |
| rust-api | `DocumentStore::get` | Req 8.3, Req 8.7 | P0-E injected project-scoped read port: exact contract version, uniquely resolved evidence ID, safe error versus missing; no ambient/cross-project fallback or backend authentication by trait. |
| rust-api | `Validator::validate_structural` | Req 8.1, Req 8.2, Req 8.5, Req 9.5 | P0-D/P0-E explicit DocumentKind and immutable snapshot return only StructuralResult; no accepting or semantic/behavioral conversion. |
| rust-api | `Validator::validate_semantic` | Req 8.1, Req 8.3, Req 8.7, Req 4.8 | P0-C/P0-E separate JSON entrypoint and trusted explicit kind/store; returns SemanticResult, not authenticated origin/approval or behavioral acceptance. |
| rust-api | `ManifestValidator::validate_manifest` | Req 5.1, Req 5.8, Req 8.1 | P0-B/P0-E TOML parse/local-rule diagnostics only; ManifestValidationResult does not establish external reference integrity or effective capabilities. |
| rust-api | `ManifestValidator::validate_manifest_semantic` | Req 8.3, Req 8.7, Req 5.3 | P0-E separate TOML semantic entrypoint with the same explicit store; checks exact references/transitive evidence/fidelity, never authorizes execution. |
| rust-api | `check_fidelity_mapping` | Req 2.3, Req 5.2, Req 8.3, Req 8.7 | P0-E explicit manifest/store/active-validator helper maps full/partial to detailed obligation fidelity; unsupported entries remain declarations and errors grant no authority. |
| rust-api | `check_evidence_completeness` | Req 3.6, Req 8.6, Req 11.4 | P0-A/P0-E non-accepting Deferred/EvidenceComplete/Blocked projection with verification_result=None and empty test_results; gaps retain candidate/obligation/assumption text. |
| rust-api | `verify_artifact_hash` | Req 4.2, Req 4.6 | Candidate exact-byte digest equality only; a boolean match is not provenance, origin, approval, freshness or acceptance. |
| rust-api | `CatalogSnapshot` | Req 1.2, Req 9.1, Req 9.4, Req 9.5 | P0-D embedded or explicitly supplied frozen indexed bytes and digest; constructor/digest is not signer admission, environment selection or network-loader authority. |

## P0 amendment and compatibility boundaries

- **P0-A**: Req 8.1 and Req 8.6 preserve phase-specific results and the exact
  reference/test projection. `Deferred`, `EvidenceComplete` and `Blocked` are all
  non-accepting; no structural, semantic or completeness result converts to
  accepted behavior. Spec 1 completeness performs no native test execution.
- **P0-B**: Req 5.8 introduces the typed capability paths above. Actions and opaque
  resources are requests only, not grants, credentials, dispatch, or transitive
  propose/approve/deploy/delete authority.
- **P0-C**: Req 4.1, Req 4.3, Req 4.4 and Req 4.8 introduce immutable bindings,
  digests, obligation links, safe failure objects and bound approval references.
  Independently checked observation origin, protected test selection, replay and
  approval policy are not established by structurally/semantically valid strings.
- **P0-D**: Req 9.5 introduces explicit immutable catalog admission, exact support
  lookup and snapshot identity. The library does not read either historical
  `CLOUDDIALECT_CATALOG_DIR` or `CLOUDDIALECT_SCHEMA_CATALOG` override; callers must
  still establish trusted provenance and compare admitted frozen identity.
- **P0-E**: Req 8.3 and Req 8.7 introduce the injected project-scoped `DocumentStore`
  and distinct JSON/TOML semantic entrypoints. Missing, ambiguous, corrupt,
  wrong-type or unavailable records fail closed; the Rust trait does not prove a
  real backend's tenant/project isolation or authenticate its implementation.
- **P0-F**: Req 6.2, Req 7.2 and NFR-3.1 require version/migration review for
  incompatible fields. Owner-attested initial MVP document/catalog **1.0.0** and
  crate API **0.1.0** are recorded in the linked amendments/implementation plan;
  that publication-history attestation is not an independent publication audit.
  If prior distribution is discovered, new required fields or a legacy
  string-to-object failure change require MAJOR/migration review, not a compatible
  MINOR/PATCH label. This table neither changes a version nor publishes artifacts.

API implementation anchors for the appendix are the
[JSON validation facade](../../crates/clouddialect-contracts/src/validator/mod.rs),
[manifest entrypoints](../../crates/clouddialect-contracts/src/manifest/validator.rs),
[reference resolver](../../crates/clouddialect-contracts/src/validator/document_resolution.rs),
[fidelity helper](../../crates/clouddialect-contracts/src/validator/fidelity.rs),
[completeness helper](../../crates/clouddialect-contracts/src/validator/behavioral.rs),
[candidate hash helper](../../crates/clouddialect-contracts/src/validator/artifact_hash.rs),
[verification consistency checks](../../crates/clouddialect-contracts/src/validator/verification_semantics.rs),
and [snapshot implementation](../../crates/clouddialect-contracts/src/schema/snapshot.rs).
Use the companion [gate-to-requirement matrix](gate-to-requirement.md) for all five
canonical gates; field presence is not gate satisfaction.

## Evidence at an immutable local baseline

The [current adversarial admission report](../../.workflow/ultracode/execution-isolation-implementation/conformance-continuation/adversarial-admission-2026-10-24/final-report.md)
records **101 admitted local conformance cases** at immutable local Git revision
`f526e677668eee9abeb5bdd937119b4b87ee3688`. That completed phase reports parent
288 Rust tests (including seven doctests), 11 runner controls and independent
bounded checks. Those are declared local scopes, not published artifacts,
authenticated behavior, real downstream consumer qualification or owner
acceptance. The report predates this traceability document and is not evidence
that this document's own regression tests have already passed.

Three of 104 inventoried cases remain gated, not silently counted as passing:

1. **Observed network-disabled validation** requires an actually constrained
   native environment; local offline catalog logic is not that observation.
2. **Real cross-project backend denial** requires a real authenticated scoped
   store/backend; synthetic ports cannot prove tenant isolation.
3. **Trusted snapshot/environment setup** requires the approved trusted setup and
   admission evidence; constructor/hash tests are not signer or setup policy.

**BG-01 remains open**. Spec 2 UD-1–UD-5, real consumer/core-owner acceptance,
native/provider/backend qualification, measured runtime budgets, and release
qualification remain separate and incomplete. No task is marked done here;
NFR-2.1 field coverage alone cannot close canonical conformance, release gates,
or delivered downstream compatibility.
