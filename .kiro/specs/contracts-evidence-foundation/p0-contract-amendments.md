# P0 — proposed upstream contract reconciliation

**Status: planning baseline approved; scoped prerequisite implementation partially delivered.** On 2026-10-22 the user approved P0-A–F for planning, subsequently authorized prerequisite implementation and initial MVP versions, and later requested domain organization plus git commit/push. DTO/schema/inventory evidence now exists; full validators, conformance and downstream compatibility remain unfinished. BG-01 and Spec 2 UD-1–UD-5 remain OPEN. No native probes or releases are authorized.

**Approval provenance:** planning, code, exact extension-value preservation/raw_value, first MVP versions and crypto/SemVer dependency/profile decisions were separate user approvals. See [implementation-plan.md](implementation-plan.md) and the [current run report](../../../.workflow/ultracode/execution-isolation-implementation/final-report.md). These approvals are not native qualification, authenticated approval evidence or BG-01 closure.

## Proposed resolutions for review

| Item | Proposed resolution | Requirement / task |
|---|---|---|
| P0-A result contract | Keep Deferred/EvidenceComplete/Blocked, reconcile `test_results` and `verification_result`; no accepting variant or conversion. | 8.1/8.6; tasks 9/14 |
| P0-B capabilities | Required `[capabilities]` with versioned typed requirements outside `[x]`; declarations never grant authority. | 5.8; tasks 15/16 |
| P0-C verification record | Required immutable configuration/binding, pack/toolchain digests, obligation-to-test links, structured failure object and bound approval references. | 4.1/4.3/4.4/4.8; tasks 4/9/13 |
| P0-D catalog admission | Embedded snapshot or explicit immutable parts supplied by trusted caller; neither environment variable selects runtime trust. | 9.5; task 8 |
| P0-E reference APIs | `errors/` modules and root re-exports; project-scoped DocumentStore separate from schema catalog; explicit JSON and TOML semantic entrypoints. | 8.3/8.7; tasks 9/13/17 |
| P0-F compatibility | Record approved format/API/catalog revision and previous publication state before adopting changed required fields. | 6.2/7.2/NFR-3; tasks 23/25 |

## A. Public results — canonical contract, DTO exports delivered

`errors/` owns result/error types; root re-exports the public API. Existing task-9 StructuralResult/SemanticResult/ValidatorError/ValidatorWarning definitions are canonical: both result error vectors use ValidatorError, structural results include warnings. No new execution-error category is added to ValidatorError.

```rust
pub enum BehavioralOutcome { Deferred, EvidenceComplete, Blocked }
pub struct BehavioralResult {
    pub candidate_id: String,
    pub outcome: BehavioralOutcome,
    pub verification_result: Option<DocumentRef>,
    pub test_results: Vec<TestResult>,
    pub blocking_gaps: Vec<BlockingGap>,
}
pub struct DocumentRef { pub id: String, pub format_version: String }
pub struct FailureDescription { pub detail: String }
pub enum TestOutcome { Pass, Fail, Skip, Error }
pub struct TestResult {
    pub test_id: String,
    pub obligation_ids: Vec<String>,
    pub outcome: TestOutcome,
    pub origin_identity: String,
    pub failure_description: Option<FailureDescription>,
}
```

TestResult exactly projects a validated verification document test entry. Non-pass outcomes require `failure_description.detail` of 1–2000 Unicode characters; an object with unknown keys or a legacy string does not satisfy it. Pass may omit failure_description. Detail is approved safe diagnostic text, never raw stdout/stderr/secrets.

Spec 1 `check_evidence_completeness` sets `verification_result = None`, `test_results = []`, and Deferred (not evaluated), Blocked (gaps), or EvidenceComplete (known source-evidence gaps absent). Spec 2 may populate the result reference/test projection only from the same validated immutable VerificationResult. The projection must match that document exactly. Every missing/unknown/failed/skipped/stale/fabricated/mismatched mandatory obligation is represented by BlockingGap or an agreed upstream diagnostic; none is accepted. A document/result with no source-evidence gaps is still **not** proof of behavior.

No BehavioralOutcome authorizes acceptance, including EvidenceComplete with passing-looking test entries. Spec 2 independently validates observation origin, complete protected obligations/tests, current immutable bindings, approval authority/policy/budget/cleanup and durable ordered commitment. Serialized `origin_identity`, approver IDs or hashes are claims, not authentication receipts. No From/Into conversions between the three phase results.

## B. Capability declaration

Required TOML table: `[capabilities]`, `format_version = "1.0.0"`, `requirements` array (explicitly empty is valid). This version belongs to the document-format version space, not pack version or an authorization policy revision. Each requirement has exactly these required nonempty fields: `id`, `operation_id`, `action`, `resource`, `purpose`. IDs are unique within the declaration and operation_id must refer to an operation in this manifest.

```toml
[capabilities]
format_version = "1.0.0"
[[capabilities.requirements]]
id = "write-test-store"
operation_id = "upload_object"
action = "write"
resource = "test-object-store"
purpose = "behavioral-test"
```

Actions form the closed set `read`, `write`, `execute`, `network`, `model`, `propose`, `approve`, `deploy`, `delete`. Unknown version/action/field, duplicate IDs or unknown operation_id fail manifest validation. `resource` is an opaque logical alias, not an interpreted path, URL, wildcard or account authority. Operator policy must resolve it to concrete authorized resources/destinations/operations; absent resolution denies execution admission. No wildcard or prefix expansion is implied.

The declaration describes requests only, even for approve/deploy/delete. Those permissions never imply each other. Candidate-controlled workers can never receive approve/deploy/delete authority; Spec 2 has no deploy/delete execution path. `[x]` cannot replace, extend or override this table. A syntactically valid declaration requesting a prohibited capability may be inspected but is denied by admission; schema parsing alone never dispatches it.

Proposed types: `CapabilityDeclaration { format_version: String, requirements: Vec<CapabilityRequirement> }`; `CapabilityRequirement { id: String, operation_id: String, action: CapabilityAction, resource: String, purpose: String }`; CapabilityAction has exactly the nine actions above, serialized lowercase. PackManifest has a required `capabilities: CapabilityDeclaration` field. Unknown nested capability fields are rejected rather than silently ignored.

## C. Verification document and binding

The exact proposed JSON Schema is in design §4.3. Required top-level fields are `id`, `format_version`, `candidate`, `pack`, `toolchain`, `configuration`, `binding`, `tests`.

- `candidate`, `configuration`, and binding's `input`, `dependencies`, `suite`, `oracle`, `policy` are ArtifactRef objects: nonempty `identity` plus `content_hash { algorithm, value }`. Configuration is a reference to **exact immutable secret-free bytes**, not inline secrets or an unspecified canonicalized JSON hash. Composite dependency/input/suite artifacts enumerate their exact members; Spec 2 freezes their identities.
- Pack/toolchain keep `id`/`version` and gain required content_hash. Version strings alone do not identify executable bytes.
- Binding also requires `tenant_id`, `project_id`, `run_id`, `attempt_id` (nonempty opaque references) and nonempty unique `obligation_ids`. These are compared against trusted expected context, not used to authenticate caller scope. Policy/suite/oracle are immutable references, not names resolved afresh at acceptance time.
- Every test entry requires nonempty `test_id`, nonempty unique `obligation_ids`, outcome and origin_identity. The semantic layer checks unique test IDs and that all test obligation IDs belong to binding's set. The trusted verifier additionally compares that set and executed tests with the protected mandatory obligation/test selection; a candidate cannot define its own smaller set.
- Non-pass tests require the structured failure object above. Optional `acceptance` records at least one approver identity, UTC timestamp, exact `verification_result: DocumentRef`, and `approval_evidence: ArtifactRef`. The result reference must match this document's id/version. Approval evidence must be independently checked and bound to the immutable verification identity, not to a self-referential digest of a document containing its own approval. Spec 2 owns authentication, expiry/revocation and final commit.
- `format: date-time` is not sufficient by itself: the proposed timestamp representation is normalized UTC ending in Z; semantic validation must reject invalid dates/non-Z timestamps and unsupported formats under the declared validator profile. All digest values must be lowercase hexadecimal of the algorithm's exact length; digest equality is not origin authentication.

Future fixtures must cover missing bindings/configuration/digests, duplicate or unmapped tests/obligations, fail/skip/error without structured detail, altered configuration, old attempt/policy/suite/oracle, forged approval, stale/missing observations and a valid reference. Structural schema success alone never closes these semantic/trust checks.

Proposed Rust projection (all fields map directly to design §4.3; deserialization alone establishes no authority):

```rust
pub struct ContentHash { pub algorithm: String, pub value: String }
pub struct ArtifactRef { pub identity: String, pub content_hash: ContentHash }
pub struct VersionedArtifact {
    pub id: String, pub version: String, pub content_hash: ContentHash,
}
pub struct VerificationBinding {
    pub tenant_id: String, pub project_id: String,
    pub run_id: String, pub attempt_id: String,
    pub input: ArtifactRef, pub dependencies: ArtifactRef,
    pub suite: ArtifactRef, pub oracle: ArtifactRef, pub policy: ArtifactRef,
    pub obligation_ids: Vec<String>,
}
pub struct AcceptanceRecord {
    pub approver_identities: Vec<String>, pub timestamp: String,
    pub verification_result: DocumentRef, pub approval_evidence: ArtifactRef,
}
pub struct VerificationResult {
    pub id: String, pub format_version: String,
    pub candidate: ArtifactRef, pub pack: VersionedArtifact,
    pub toolchain: VersionedArtifact, pub configuration: ArtifactRef,
    pub binding: VerificationBinding, pub tests: Vec<TestResult>,
    pub acceptance: Option<AcceptanceRecord>,
}
```

## D. Trusted catalog source

Proposed public boundary (ordinary constructors, not security attestations): `CatalogSnapshot::embedded() -> Result<CatalogSnapshot, ValidatorError>`, `CatalogSnapshot::from_parts(index: Vec<u8>, schemas: BTreeMap<String, Vec<u8>>) -> Result<CatalogSnapshot, ValidatorError>`, `CatalogUrlLoader::new(snapshot: CatalogSnapshot) -> CatalogUrlLoader`, `Validator::new(snapshot: CatalogSnapshot) -> Validator` and `ManifestValidator::new(snapshot: CatalogSnapshot) -> ManifestValidator`. Snapshot owns frozen bytes and exposes a content digest for comparison with operator-admitted identity; no live filesystem/network lookup during validation.

Proposed snapshot digest encoding: SHA-256 of UTF-8 domain separator `CloudDialectCatalogSnapshot/v1` followed by a zero byte, then all entries (including index.json) sorted by UTF-8 path bytes. For each entry concatenate unsigned 64-bit big-endian path-byte length, path bytes, unsigned 64-bit big-endian content-byte length, exact content bytes. Reject overflow, absolute/traversing/duplicate paths and unresolved index entries. This is a proposal requiring fixture vectors and approval, not measured/implemented hashing.

Trusted host code may snapshot an approved directory and call from_parts after provenance/admission checks. The library reads neither `CLOUDDIALECT_CATALOG_DIR` nor `CLOUDDIALECT_SCHEMA_CATALOG`; caller environment/repository data is not catalog authority. No public injection of a network loader. URI resolution is exact admitted index lookup, not a host-prefix permission. The bundled schema set includes the Draft 2020-12 meta-schema/vocabularies required by the selected engine; no implicit engine network fallback.

Catalog index application document types are exactly behavioral-contract, evidence-record, verification-result, catalog-index, extension-registry. A TOML manifest has no fake JSON Schema entry. Required `resolver_resources` is a separate array of `{ schema_id, catalog_path }` entries for meta-schema/vocabulary support, possibly empty only if the pinned engine demonstrably resolves all such resources offline internally. Exact resolver URI keys must be unique across both lists; every external `$ref` is available from admitted support entries or proved built-ins, never a network fallback. Catalog contains schemas/extension registry, **not private contract/evidence instances**.

## E. Project-scoped document references and semantic entrypoints

```rust
pub struct StoredDocument {
    pub reference: DocumentRef,
    pub content_hash: ContentHash,
    pub bytes: Vec<u8>,
}
pub trait DocumentStore {
    fn get(&self, id: &str, format_version: Option<&str>)
        -> Result<Option<StoredDocument>, ValidatorError>;
}
pub trait Validate {
    fn validate_structural(&self, kind: DocumentKind, document: &[u8]) -> StructuralResult;
    fn validate_semantic(&self, kind: DocumentKind, document: &[u8], store: &dyn DocumentStore) -> SemanticResult;
}
pub trait ValidateManifest {
    fn validate_manifest(&self, toml_bytes: &[u8]) -> ManifestValidationResult;
    fn validate_manifest_semantic(&self, toml_bytes: &[u8], store: &dyn DocumentStore) -> SemanticResult;
}
```

DocumentStore is supplied by the trusted core already bound to authenticated tenant/project; a document cannot select its scope/backend. Some(version) is required for contract references; None is allowed for globally unique evidence IDs and must return exactly one version or a diagnostic. None result means missing; Err means a safe resolution failure, never an ignored unavailable dependency. Validator checks returned identity/version/type, bytes/hash and structural validity before using it. No network fallback or search in another project. An unparseable input or failed referenced document cannot yield semantic pass.

DocumentKind is the closed enum BehavioralContract/EvidenceRecord/VerificationResult/CatalogIndex/ExtensionRegistry, selected by trusted caller purpose (or protected fixture metadata), never inferred from attacker-controlled document shape. Structural selection uses kind + supported format_version in the admitted catalog. Instances may optionally carry a nonempty `$schema` string; if present it must resolve and match the selected kind/version URI. A missing `$schema` does not prevent dispatch; an unknown or mismatched value blocks, with no network fallback. Referenced records are validated using the expected kind, not a self-declared kind.

`validate_semantic` accepts JSON only; `validate_manifest_semantic` validates TOML locally then checks exact contract references/fidelity through the same store. `validate_manifest` covers local manifest rules, not proof that external references resolve. `check_fidelity_mapping(manifest: &PackManifest, store: &dyn DocumentStore) -> Vec<ValidatorError>` and `check_evidence_completeness(contract: &BehavioralContract, candidate_id: &str, store: &dyn DocumentStore) -> BehavioralResult` consume this port. Missing/unknown/corrupt/ambiguous records block; no fabricated empty store or permissive default.

Phase-specific fixture expectations are exact: structural/semantic use pass/fail/warn where the relevant outcome enum supports them; behavioral completeness uses deferred/evidence-complete/blocked (the three BehavioralOutcome variants); artifact-hash uses `expected_match: true|false`, not an invented invalid enum. JSON fixtures declare DocumentKind; TOML fixtures select the manifest entrypoint. EvidenceComplete is never normalized to behavioral pass. The fixture runner must reject unsupported phase/outcome pairs and seeded incorrect expectations.

## F. Version and approval gate

The user attests this is an initial project and approves document/catalog **1.0.0** and crate API **0.1.0** as first MVP versions. This is user-provided publication history, not an independent external-publication audit. If prior distribution is later discovered, new required capability/binding/configuration fields or string-to-object failure_description changes require MAJOR/migration review; never relabel them a compatible MINOR/PATCH. Legacy MINOR-newer structural warning cannot authorize a declaration whose security semantics the core does not implement. No silent default capabilities or approving conversion for old records.

Pack/toolchain and document-format versions use full SemVer 2.0.0 syntax, including valid prerelease/build identifiers; compatibility policy separately chooses supported versions. A numeric-only regex must not make a valid manifest pack version unrepresentable in verification. Fixtures include 1.2.3-rc.1, 1.2.3+build.1, 1.2.3-0 and rejection of numeric prerelease 01, leading-zero core versions and empty suffixes. Schema syntax checks do not establish version support or range compatibility.

Review decisions needed: accept/revise P0-A–F, supported capability operations/resource-alias resolution, verifier/approval evidence mapping, snapshot encoding and publication/version disposition. Record owner identities, approved amendment/schema/API digests and compatibility fixtures. User authorization to write this draft is not those approvals.

## Evidence and limitations

Current source at 3b9cb6bd0ee82dbef3b731a35ce68fc778c4906a has only module stubs. No definitions above are current exports. P0 makes documentation internally reviewable, not integration-ready.

Context7 resolved/queried primary JSON Schema documentation; format is annotation by default and object/reference constraints do not authenticate observations. [Format](https://json-schema.org/understanding-json-schema/reference/type), [Draft 2020-12 validation](https://json-schema.org/draft/2020-12/json-schema-validation). Context7 could not supply the correct Boon project; docs.rs was inaccessible. During the planning continuation, the [official boon 0.6.1 archive](https://static.crates.io/crates/boon/boon-0.6.1.crate) was fetched read-only and its SHA-256 matched Cargo.lock (`baa187da765010b70370368c49f08244b1ae5cae1d5d33072f76c8cb7112fe3e`). Source verifies UrlLoader::load returns `Result<serde_json::Value, Box<dyn std::error::Error>>`, Compiler::new takes no loader argument, Compiler::use_loader installs a boxed loader, and enable_format_assertions enables format assertions. This corrects the earlier API-availability uncertainty only; no compile/behavior/offline/date conformance is proved. Exact paths/lines are in the run's `results/planning-boon-api.json`.
