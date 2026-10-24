# Implementation Plan: contracts-evidence-prerequisite-delivery

**Publication authority:** earlier commit/push approvals apply to their historical actions. The 2026-10-24 frozen-fixture admission explicitly authorizes local milestone commits and as-needed agents, not source repairs, new dependencies, pushes, native probes, deployments or releases.


## Current physical domain layout
User requested domain-first organization on2026-10-22. Document DTO implementation is now private `src/documents/{behavioral_contract,evidence,verification,references,raw_objects}.rs`; `src/validator/mod.rs` preserves every existing public document import as a compatibility facade. Existing `manifest/`, `schema/` and `errors/` remain cohesive domains/support. Historical source paths below describe original task ownership; new edits use the current owner paths. No new public document namespace, aggregate framework, validator behavior or gate approval follows from this move. Shared repository AI guidance is canonical `.agents/rules/engineering.md` with relative client entrypoint symlinks.

## Overview

**Implementation continuation authorized (2026-10-22).** User first approved P0-A–F for planning and subsequently authorized remaining prerequisite units under their stated gates, with TDD and no commits/probes/pushes/releases. User attests initial-project publication history and approves document/catalog1.0.0 and crate0.1.0 as first MVP versions. Exact sha2=0.11.1/sha3=0.12.0/semver=1.0.28 pins, features, audited lock/notices and Rust>=1.88/version profile are subsequently approved; see the run dependency proposal/admission evidence. This bounded plan elaborates, not replaces, the stable canonical tasks in [tasks.md](tasks.md). BG-01 and Spec2UD-1–UD-5 remain OPEN. Partial evidence is recorded in the run; checkboxes do not assert whole-task completion.

Objective: deliver real schema/catalog/manifest/validation APIs, full required fixtures and compatibility evidence before Spec 2 can consume them. Types alone do not satisfy BG-01. This is substantial unfinished Spec 1 work, not a small execution-isolation patch. Risk-first preflight, then foundation-first typed/data surfaces, integrated validators, and real conformance/handoff. No unsandboxed worker or accepting conversion is introduced.

### Activation gates
- **G-CODE:** separate approval of an exact implementation unit and its RED/GREEN checkpoint scope. Planning approval is not this approval.
- **G-VERSION:** owner verifies external publication/distribution history and records target format/API/catalog versions. No local tags currently exist; that does not prove no external release. Breaking published required-field/semantic changes need MAJOR/migration review. Type-only unit 1.2 may avoid format constants, but does not waive G-VERSION for format-bearing work.
- **G-DEPS:** parent reviews needed crypto/version tooling, exact versions/licenses, lockfile impact and current APIs before use. No hand-rolled crypto, undocumented transitive imports or silent dependency changes. Existing pins lack direct SHA-2/SHA-3/SemVer dependencies; no version is invented here. Existing TOML parser/writer transitives have spec-1.1.0 tags; test TOML 1.0 compatibility rather than infer it from the direct version label.
- **G-TOOLS:** approved locked dependencies available in an isolated Cargo cache, build/test tools and coverage approach established. A failed download/setup is not RED. Native Kiro/SMT is unavailable; broad legacy document-validator failures remain explicit rather than silently waived.
- **G-NATIVE:** separate approved environment/actions for any real network-disabled conformance, host/adversarial qualification or live provider resources. Current user authorization is qualification plans only. Mocks/loader callbacks do not prove native offline or isolation.
- **G-SCHEDULE:** approve the honest deeper dependency chain before broad implementation. Three parent stages below are coordination groups, not a claim of leaf depth three. Unit 3.2 is a scheduling container requiring concrete bounded fixture-batch expansion before dispatch.

### Common execution and verification contract
Every code unit below: write focused tests first; validate intended missing/incorrect implementation RED (not syntax/dependency/setup failure), checkpoint RED on the active approved branch, minimally implement only owned files, rerun the **same** target GREEN, checkpoint GREEN; optional refactor only while green. No artificial failure is needed for test/data-only additions that already pass and do not change production behavior; label those honestly, not as a fabricated RED/GREEN cycle.

`C=crates/clouddialect-contracts/Cargo.toml`. Target `name` below means the future command `rtk cargo test --manifest-path "$C" --frozen --test name`; target names below define the planned checks; implemented targets are recorded in the run. Parent specifies per-run CARGO_HOME/CARGO_TARGET_DIR in approved /tmp directories; never mutate global Cargo configuration. `--frozen` is locked + offline and requires provisioned dependencies. Context7 current [Cargo docs](https://doc.rust-lang.org/cargo/commands/cargo-test.html) support target selection; [fetch options](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html) define frozen. Commands here define checks, not evidence of their execution; see the run for dated results.

After focused GREEN, every code unit also requires current full crate tests/check, fmt check, clippy with warnings denied, scope diff, unchanged unrelated work, linked requirement reread, and measured >=80% coverage for implemented behavior with exclusions disclosed. Missing required tooling/checks stays UNVERIFIED. Final BG-01 needs real consumer/conformance evidence, not just test counts. Separate user implementation approval now applies to the bounded prerequisite units, not gated runtime/release work.

File lists below are relative to the contracts crate unless prefixed `catalog/` or `docs/`. Parent alone edits shared lib.rs/mod.rs registration, Cargo.toml/Cargo.lock, common test support and catalog index. Workers, if later separately authorized, may edit only named private files; registration/lock/index writes serialize. Do not add public placeholders for unmet dependencies. Estimates are conditional engineering effort per bounded unit, not elapsed delivery promises; split before dispatch if >4h, >5 files or too many independent concepts. Runtime/security functionality remains Spec 2 scope.

## Verified progress (2026-10-22)
Units1.2–1.11, the approved resource DTO amendment1.4R, and2.1–2.3 passed bounded checks; checkmarks below refer only to those unit scopes. RC-1 now closes the resource-dimension representation/schema gap through typed bindings and per-dependency coverage tests. Whole canonical tasks remain conservative where obligations extend beyond a leaf; full conformance, runtime and BG-01 remain unfinished. [Latest resource evidence](../../../.workflow/ultracode/execution-isolation-implementation/resource-contract-report.md), [catalog evidence](../../../.workflow/ultracode/execution-isolation-implementation/catalog-delivery-report.md), and [approved contract decisions](../../../.workflow/ultracode/execution-isolation-implementation/contract-reconciliation-decisions.md).

## Tasks

### Owner-approved EX-1 declaration leaves
- [x] EX1-a Existing DTOs: `src/documents/{behavioral_contract,evidence,verification}.rs`, `src/schema/catalog.rs`, `tests/required_extension_models.rs` (five files). Explicit optional string-vector fields, omission/empty retention, raw payload preservation, original-byte catalog typed decoding to reject duplicate core declarations; no recognition grant or new registry DTO.
- [x] EX1-b Primary schema declarations: three primary schema.json files and `tests/required_extensions.rs` (four files). Optional unique ASCII prefixes, no colon/whitespace, declaration-only valid, malformed types/duplicates invalid; actual closed production adapter. Existing primary root closure unchanged.
- [x] EX1-c Catalog/registry declarations: their two schema.json files and `tests/required_extension_catalogs.rs` (three files). Same field constraints and declaration-only validity; preserve unrelated support-root openness. Inventory/registration is not implemented recognition.

These are bounded prerequisites to unit2.6, not completion of mandatory-extension blocker logic. All require actual RED/GREEN/full checks and shared final resource/extension review. Requirement/design/task prose and matching properties are parent-owned documentation reconciliation; prior non-EX design differences stay explicit. Scope approval and75-minute/18-reviewer/one-repair limits carry the existing resource tranche, not a reset.

Verified declaration delivery: [EX-1 report](../../../.workflow/ultracode/execution-isolation-implementation/extension-declaration-report.md). Three RED/GREEN pairs, twelve new integration tests and packet18 bounded static PASS; current full suite136integration+2unit+7compile-fail doctests passes. Unit2.6 remains unchecked: no actual mandatory-extension blocker engine or trusted-recognition implementation is delivered by these declarations.

- [ ] 1. Resolve prerequisites and freeze shared data contracts
  - [ ] 1.1 Record activation and tooling dispositions (read-only/draft)
    - Spec1: 1/8/23/26 prerequisite review. Requirements: 1.2, 6.2, 7.2, 9.5, 12.3.
    - Owned: run `results/activation-gates.json`, dependency/license proposal and evidence notes only; no manifest/lock change.
    - Checks: code authority; publication/version disposition; locked cache/toolchain; exact adapter API; crypto/version dependency proposal; TOML compatibility risk; coverage and native scope. Record unresolved gates, never infer approval.
    - Done: attributable evidence/dispositions supplied; each affected downstream unit remains closed while its activation gate is unresolved. Existing source-only verification is not compilation. Estimate: 1–2h. Dependencies: none. No production RED/GREEN applies.
  - [x] 1.2 Phase/error result foundation — recommended first implementation slice
    - Spec1: task 9 subset. Requirements: 8.1–8.2, 11.1. Gates: G-CODE and G-TOOLS; no format version constants or new dependency.
    - Owned: `src/errors/diagnostics.rs`, `src/errors/phase.rs`, `src/errors/mod.rs`, `src/lib.rs`, `tests/phase_results.rs` (five files; shared registrations parent-owned).
    - Cases: required diagnostic fields; category labels; structural warning/error vectors; distinct structural/semantic outcomes; no accepted/behavioral result fields; no cross-phase conversion. Target: `phase_results`; missing API imports are intended compile-time RED only after dependencies resolve.
    - Done: real diagnostics + structural/semantic results and exports, same-target GREEN/full regression; no BehavioralResult placeholder, validation algorithm or migration acceptance. Estimate: 2–3h. Dependencies: 1.1.
  - [x] 1.3 Immutable reference and injected store ports
    - Spec1: task 9 P0-E subset. Requirements: 8.3, 8.7, 9.5.
    - Owned: `src/validator/references.rs`, `src/validator/mod.rs`, `src/lib.rs`, `tests/reference_contracts.rs`.
    - Shared prerequisite allocation: define canonical P0-C ContentHash (algorithm: String, value: String) here because StoredDocument consumes it; 1.5 reuses it. No crypto function or new dependency, and no invented HashAlgorithm enum.
    - Cases: DocumentKind is explicit; DocumentRef exact version; StoredDocument bytes/digest identity; missing versus safe store error; exact/ambiguous evidence lookup; caller-bound scope contract without ambient fallback. Target: `reference_contracts`.
    - Done: P0-E types/trait compile with representative consumer; no backend or claim that an untrusted store is authenticated by its trait. Estimate: 2–3h. Dependencies: 1.2.
  - [x] 1.4 Source contract/evidence models
    - Spec1: tasks 2/3/9/13 model subset. Requirements: 2.1–2.5, 3.1–3.6, 8.3.
    - Owned: `src/validator/source_documents.rs`, `src/validator/raw_objects.rs`, `src/validator/mod.rs`, `src/lib.rs`, `tests/source_document_models.rs` (five paths after the approved exact-value clarification).
    - Approved clarification 2026-10-22: preserve encoded extension JSON value bytes, not merely decoded values. User separately approved raw_value on pinned serde_json; the feature/requirement disposition is a parent-owned prerequisite slice, not a new crate/version choice. Shared direct raw-object decoding avoids Serde flatten normalization; freeform object members use raw values too. 1.5 reuses this decoder. Unit estimates must be rechecked if additional lexical syntax beyond value bytes is requested.
    - Cases: all required source fields; observed provenance; inferred reasoning; unknown assumptions; obligation/reference alternatives; extension preservation. Target: `source_document_models`.
    - Done: BehavioralContract/EvidenceRecord and nested models used by later store/completeness APIs exist; no fabricated references or acceptance. Split if model scope exceeds four hours. Estimate: 3–4h. Dependencies: 1.3.
  - [x] 1.4R Approved resource-assumption DTO amendment
    - Spec1: tasks2/9 model subset, requirement2.2, owner-approved RC-1. Separate from generic assumptions and schema completeness.
    - Owned: `src/documents/behavioral_contract.rs`, `src/documents/mod.rs`, `src/validator/mod.rs`, `src/lib.rs`, `tests/source_document_models.rs` (five files).
    - Cases: required closed dimension enum and resource-specific assumption payload, root/validator type identity, missing/duplicate fields, exact encoded extension values, generic evidence/obligation assumptions unchanged. Target: `source_document_models`; missing API RED checkpoint before code, same-target GREEN/full checks. Existing resource fixture gains its explicit behavior binding, not removal.
    - Done: typed resource binding and intentional vector element-type amendment; parsing does not prove per-dimension coverage or nonempty text. No new dependencies, accepting conversions or native probes. Schema enforcement is unit1.7. Conditional estimate1–2h, dependencies1.4 plus current owner decision.
  - [x] 1.5 Verification/projection models
    - Spec1: task 9 remaining P0-A/C. Requirements: 4.1–4.8, 8.6.
    - Owned: `src/validator/verification_types.rs`, `src/validator/mod.rs`, `src/errors/mod.rs`, `src/lib.rs`, `tests/verification_models.rs`.
    - Cases: exact P0-C fields; TestResult/failure object; immutable binding refs; three non-accepting BehavioralOutcomes; empty completeness projection; mismatched projection cannot acquire acceptance; approval/origin strings are claims. Target: `verification_models`.
    - Done: reconciled public result and document types compile; no execution or approving conversion. Estimate: 3–4h. Dependencies: 1.3.
  - [x] 1.6 Freeze bounded schema-test support
    - Spec1: tasks 2–6/22 test support only. Requirements: 1.1–1.3, NFR-1.
    - Owned: `tests/support/mod.rs`, `tests/schema_harness.rs`.
    - Cases: known-good schema/instance; deliberate mismatch; malformed JSON; unknown URI never retrieves; local references; explicit format-assertion capability exposed. Target: `schema_harness`.
    - Done: reusable test-only in-memory/deny-unknown resolver with actual pinned Boon signatures and positive/negative controls; no default filesystem/network loader. Not a production CatalogSnapshot or native offline proof. Estimate: 2–3h. Dependencies: 1.2.
  - [x] 1.7 Behavioral-contract schema
    - Spec1: task 2 and its focused fixtures. Requirements: 2.1–2.7, 10.2.
    - Owned: `catalog/schemas/behavioral-contract/1.0/schema.json`, `tests/behavioral_schema.rs`. Path version is conditional on G-VERSION, not permission to overwrite a published format.
    - Cases: required fields; wrapper/resource behavior; fidelity enum; evidence-or-assumption; notes; qualified extensions; optional declared schema selector. Target: `behavioral_schema`.
    - Done: real schema plus intended rejection tests pass through frozen support; rich release fixtures still scheduled in 3.2. Estimate: 2–4h. Dependencies: 1.6. Gate: G-VERSION.
  - [x] 1.8 Evidence-record schema
    - Spec1: task 3 and focused fixtures. Requirements: 3.1–3.6, 10.2.
    - Owned: `catalog/schemas/evidence-record/1.0/schema.json`, `tests/evidence_schema.rs` (version conditional).
    - Cases: explicit classification; observed provenance; nonempty inferred reasoning; unknown assumptions; invalid enum; extension preservation. Target: `evidence_schema`.
    - Done: real schema rejects missing evidence without inventing observations. Estimate: 2–3h. Dependencies: 1.6. Gate: G-VERSION.
  - [x] 1.9 Verification-result schema
    - Spec1: task 4 and focused fixtures. Requirements: 4.1–4.8, 8.6.
    - Owned: `catalog/schemas/verification-result/1.0/schema.json`, `tests/verification_schema.rs` (version conditional).
    - Cases: immutable required refs; nonempty tests/obligations; fail/skip/error structured detail and Unicode bounds; SemVer suffixes; hash shape; normalized UTC-Z shape; approval ref shape versus authentication. Target: `verification_schema`.
    - Done: schema matches model/approved format; semantic/authenticity checks explicitly remain for 2.9/Spec 2. Estimate: 3–4h. Dependencies: 1.5, 1.6. Gate: G-VERSION.
  - [x] 1.10 Catalog-index and extension-registry schemas
    - Spec1: tasks 5/6. Requirements: 1.6, 9.4–9.5, 10.6.
    - Owned: `catalog/schemas/catalog-index/1.0/schema.json`, `catalog/schemas/extension-registry/1.0/schema.json`, `catalog/extension-registry.json`, `tests/catalog_schemas.rs` (versions conditional).
    - Cases: self-index fields; exact five application types; resolver_resources; EOL date conditional; empty registry; malformed namespace entry. Target: `catalog_schemas`.
    - Done: application/support resource representation and empty valid registry, no fake manifest JSON Schema or project instances. Estimate: 3–4h; split index/registry if beyond bound. Dependencies: 1.6. Gate: G-VERSION.
  - [x] 1.11 Manifest and capability types
    - Spec1: task 15. Requirements: 5.1–5.8, 10.5.
    - Owned: `src/manifest/types.rs`, `src/manifest/mod.rs`, `src/errors/mod.rs`, `src/lib.rs`, `tests/manifest_types.rs` (five files, shared registrations parent-owned).
    - Deliver canonical task-9 ManifestValidationResult and ManifestOutcome here, with exact outcome/structural_errors/semantic_errors/warnings fields; unit 2.7 consumes real types, not placeholders. This does not broaden first slice 1.2.
    - Cases: required sections; explicit empty/nonempty capabilities; closed actions; exact fields; unsupported operation retained; manifest-result/error/warning shapes; extensions do not become grants. Target: `manifest_types`.
    - Done: P0-B declarations and local-validation result types, not effective policy/credentials/dispatch. Estimate: 2–4h. Dependencies: 1.3, 1.5. Gate: G-VERSION.

- [ ] 2. Implement the actual validation boundary
  - [x] 2.1 Populate and validate catalog index
    - Spec1: task 7. Requirements: 1.6, 9.2/9.4.
    - Owned: `catalog/index.json`, `src/schema/catalog.rs`, `src/schema/mod.rs`, `tests/catalog_index.rs`.
    - Cases: five app entries; self-schema; exact URI uniqueness; missing/duplicate/path escape; support entries or proved built-ins; relocated immutable bytes. Target: `catalog_index`.
    - Done: catalog inventory matches real files and required support resources; unknown engine built-ins are not silently assumed. Estimate: 2–3h. Dependencies: 1.7, 1.8, 1.9, 1.10.
  - [x] 2.2 Frozen snapshot and digest
    - Spec1: task 8 subset. Requirements: 9.1–9.5; P0-D.
    - Owned: `src/schema/snapshot.rs`, `src/schema/mod.rs`, `src/lib.rs`, `tests/catalog_snapshot.rs`; parent separately owns any approved dependency/lock change.
    - Cases: exact P0-D digest vectors; path sort/length encoding; duplicate/index alias; overflow; mutation after construction; ambient variables irrelevant. Target: `catalog_snapshot`.
    - Done: immutable admitted parts + deterministic digest; constructor never asserts signer authority. If dependencies and these four files exceed five changed files, separate the approved manifest/lock update before dispatch. Estimate: 3–4h. Dependencies: 1.3, 2.1. Gate: G-DEPS (SHA-256).
  - [x] 2.3 Actual resolver and embedding
    - Spec1: task 8 completion. Requirements: 1.2–1.3, 9.1–9.5.
    - Owned: `src/schema/loader.rs`, `src/schema/embedded.rs`, `src/schema/mod.rs`, `src/lib.rs`, `tests/catalog_loading.rs`.
    - Cases: embedded/explicit snapshot; parsed Value adapter; unknown/file/HTTP URI deny; pinned-engine built-ins; both ambient overrides ignored; format assertions enabled; malformed schema maps safe errors. Target: `catalog_loading`.
    - Done: real checksum-source-matching adapter before compilation; no byte-returning stub, ambient file loader or mock-only qualified-offline claim. Estimate: 3–4h. Dependencies: 2.2.
  - [x] 2.4 Version grammar/compatibility helper
    - Spec1: task 11 logic subset. Requirements: 6.1–6.5, 7.1–7.4.
    - Owned: `src/validator/version.rs`, `tests/version_compatibility.rs`. Test-includes the exact private source; production registration in mod.rs belongs with its actual structural consumer2.5, not an unused public placeholder.
    - Cases: full SemVer suffixes; invalid leading-zero/empty IDs; supported/unsupported major; minor-newer warning; distinct pack/document/dialect spaces; safe unsupported-security-version handling. Target: `version_compatibility`.
    - Done: explicit compatibility dispositions, no implicit format fallback. Estimate: 2–3h. Dependencies: 1.2. Gates: G-VERSION, G-DEPS (version/range strategy).
  - [x] 2.5 Structural JSON entrypoint
    - Spec1: task 10 and task 11 integration. Requirements: 1.2–1.3, 4.7, 8.1–8.2, 11.1–11.2.
    - Owned consumer: `src/validator/structural.rs`, `src/validator/mod.rs`, `src/lib.rs`, `src/schema/snapshot.rs` (crate-private immutable entry accessor), `tests/structural_validation.rs` (five files). Distinct prerequisite2.5U owns two internal-reuse lines in documents/{raw_objects,mod}.rs to share existing RawFields duplicate-key rejection; no new public type/parser or decoding behavior. Validate initially exposes only the implemented structural method; semantic methods arrive with their real leaf, never a success/panic stub.
    - Cases: trusted DocumentKind; absent/known/wrong schema URI; malformed input offset; missing field; real version disposition; safe paths/diagnostics; no semantic/behavior acceptance. Target: `structural_validation`.
    - Done: actual schemas + loader, no shape inference or unresolved external references. Estimate: 3–4h. Dependencies: 2.3, 2.4.
    - Bounded initial-MVP delivery: [version/structural report](../../../.workflow/ultracode/execution-isolation-implementation/version-structural-report.md). All five current kinds, trusted selection and version/root-projection boundaries are tested; opaque/external/dynamic/changed-$id root projection scopes conservatively fail rather than guess. Exact versions still use the actual compiler. This explicit limitation is not an owner-approved waiver of future arbitrary-schema compatibility. Canonical tasks10/11 remain conservative pending their larger extension/conformance obligations.
  - [ ] 2.6 Extensions and mandatory blockers
    - Spec1: task 12. Requirements: 10.1–10.7.
    - Owned: `src/validator/extensions.rs`, `src/validator/structural.rs`, `tests/extension_validation.rs` (structural integration serialized after 2.5).
    - Cases: reserved exact/upper/mixed families; invalid grammar; unknown optional preserved; unknown mandatory blocks; extension cannot override core; collisions confer no trust. Target: `extension_validation`.
    - Done: structural warnings remain non-blocking while mandatory security semantics never become permissive; no acceptance bit. Estimate: 3–4h. Dependencies: 2.5, 1.10.
    - Bounded delivery2.6a: scanner/structural integration and test-only document fixtures (five named files in the current wave eval),7actual integration tests. Bounded2.6b: separate private mandatory_extensions.rs and exact-source test target,5tests; no trusted implemented namespace. This helper is deliberately not registered until real semantic consumer2.9 exists. Full2.6 stays unchecked: private tests are not public end-to-end mandatory enforcement; catalog/registry optional-extension round-trip conformance also remains unestablished.
  - [x] 2.7 TOML/local manifest validation
    - Spec1: task 16. Requirements: 5.1–5.8, 11.3.
    - Owned: `src/manifest/validator.rs`, `src/manifest/mod.rs`, `src/lib.rs`, `tests/manifest_validation.rs`, `tests/manifest_api.rs` (parent-only local trait registration/test).
    - Cases: all eight local rule families; version/range validity; inline body rejected; missing toolchain; capability grammar/duplicates; unknown fields; explicit TOML 1.0-compatible valid syntax. Target: `manifest_validation`.
    - Done: declarations never dispatch or resolve external authority; local pass does not imply references/fidelity checked. Do not require rejecting all later TOML syntax unless the approved requirement actually says that. Estimate: 3–4h. Dependencies: 1.11, 2.3, 2.4.
    - Verified bounded initial MVP:12local rule groups plus1real trait-dispatch test; exact snapshot ownership, no ambient catalog. Schema refs are syntax-checked locally, not resolved/matched to compatibility bounds. Full SemVer syntax/core-u64 support and Cargo comparator/prerelease ranges follow approved profile. Req11.3 source-artifact coverage remains future work, not claimed by this TOML-only API. See [wave report](../../../.workflow/ultracode/execution-isolation-implementation/extension-manifest-hash-report.md).
  - [x] 2.8 Scoped fidelity function
    - Spec1: task 17, before task 13 integration. Requirements: 5.2, 8.3/8.7.
    - Owned: `src/validator/fidelity.rs`, `src/validator/mod.rs`, `tests/fidelity_mapping.rs`.
    - Cases: full/equivalent; full/adaptation deny; partial/architectural deny; missing exact contract; wrong identity/version/type; store outage; unsupported operation retained. Target: `fidelity_mapping`.
    - Done: real typed/validated source documents through injected store, not catalog instance search. Estimate: 2–3h. Dependencies: 1.4, 2.5, 2.7.
    - Owner-approved initial API amendment: final validator:&Validator argument on standalone fidelity and future completeness helpers. Private exact reference/hash/schema/mandatory resolver now has actual fidelity/TOML consumers; no implicit embedded/default catalog. CatalogIndex now publicly retains extensions:RawObject through the existing duplicate-aware raw decoder; initial document version unchanged, Rust struct-literal change explicitly approved.
  - [x] 2.9 JSON/TOML semantic entrypoints and verification linkage
    - Spec1: task 13. Requirements: 4.8, 8.3/8.7, 11.1.
    - Owned: `src/validator/semantic.rs`, `src/validator/mod.rs`, `src/manifest/validator.rs`, `tests/semantic_validation.rs`.
    - Cases: exact references; corrupt/wrong-type/hash mismatch; unique tests/obligation membership; UTC/date validity; matching approval-result ref; missing/ambiguous/outage; no cross-project/ambient fallback. Target: `semantic_validation`.
    - Done: separate actual JSON/TOML ports, faithful safe errors; document strings still do not authenticate observations/approval. Split verification-link rules into a second named unit before dispatch if >4h or >8 independent test groups; no silent overrun. Estimate: conditional 4h slice. Dependencies: 1.5, 2.6, 2.8. Gate: G-DEPS for required hash checks.
    - Bounded2.9M TOML consumer delivered: src/manifest/{validator,mod}.rs plus tests/manifest_semantic.rs, shared crate-private reference visibility parent-owned. Actual local-gated method/trait checks exact references and fidelity, including explicit unsupported/orphan references, using owned snapshot. Full2.9 remains unchecked: JSON semantic method, evidence traversal and verification linkage are not delivered by this TOML leaf.
    - Semantic continuation (2026-10-23), [current report](../../../.workflow/ultracode/execution-isolation-implementation/semantic-continuation/final-report.md):
      - [x] 2.9V Implemented private P0-C verification consistency with focused RED/GREEN: all ten digest positions/five algorithms, unique tests, obligation membership and exact approval self-reference; real structural/calendar checks plus fixed core semantic date-time assertions prevent relaxed-catalog bypass.
      - [x] 2.9S Implemented actual JSON Validator/Validate semantic entrypoint, all-five-kind mandatory blockers and exact scoped evidence resolution for obligations, side effects and notes. Preserves located source diagnostics, reports safe referring paths for store failures and uses the explicit active snapshot.
      - [x] 2.9T Integrated that contract/evidence boundary into fidelity and every TOML binding, including unsupported/orphan declarations, without refetching a contract for the second decision. Three new behavior regressions were RED before the correction.
      - [x] Independent integrated reviewer28 found no new actionable defect in the scoped semantic/policy diff and independently passed36selected tests against68unchanged source hashes. Parent reran the full implemented tree after preserving the pending completeness draft outside Cargo discovery; full tests/check/fmt/Clippy/coverage passed. Unit2.9/canonical task13 is delivered within that scope, not completeness, exhaustive conformance or BG-01. Reviewer confirmed a pre-existing unresolvable-schema URI diagnostic blocker for future conformance; no second source repair was authorized. Earlier2.9M absence statements above describe its historical delivery, not current source.
  - [x] 2.10 Non-accepting completeness and artifact staleness
    - Reopened after reviewer29 and repaired 2026-10-24 with explicit two-file approval: actual RED `03738e4` (2 gaps instead of 4), minimal branch move/GREEN `0396883` (10 completeness tests). Direct assumptions survive root blockers without store access; 6 artifact-hash tests and the full suite also pass. The proposed patch had prior static review 30; repaired-tree execution is parent-verified only. Full corpus/fresh coverage/native/consumer evidence is not supplied by this unit.
    - Spec1: task 14. Requirements: 3.6, 4.6, 8.6, 11.4.
    - Owned: `src/validator/behavioral.rs`, `src/validator/mod.rs`, `src/lib.rs`, `tests/evidence_completeness.rs`.
    - Cases: observed/inferred completeness not acceptance; unknown/missing/corrupt source gaps; all named obligation diagnostics; empty verification projection; every supported hash algorithm with known vectors/tamper; no approving conversion. Target: `evidence_completeness`.
    - Done: real state/data checks and crypto verification, never executed behavioral-test success. If hash algorithms make the unit exceed bounds, split a dedicated artifact-hash helper/test slice before dispatch. Estimate: conditional 3–4h. Dependencies: 1.4, 1.5, 2.9. Gate: G-DEPS (all five declared hash algorithms).
    - Bounded2.10H split recorded before worker22 dispatch: `src/validator/artifact_hash.rs`, `tests/artifact_hash.rs`, parent-only `src/validator/mod.rs`/`src/lib.rs` registration. Dependencies for this independent hash-only leaf are existing VerificationResult DTO and approved sha2/sha3 pins, not unfinished semantic/completeness orchestration. All five declared algorithms and ten independently checked empty/abc vectors, malformed/tamper/immutability controls pass6tests. Public helper checks candidate digest equality only; no acceptance/provenance/other-artifact claim. Full2.10 stays unchecked.
    - Bounded2.10B delivered after explicit owner empty-scope decision: helper/export and9focused tests, missing-API RED→GREEN and full checks. Invalid source/valid zero obligations return Deferred/no store; semantic mandatory blockers stay separate. Resource assumptions map to every obligation, even alongside a root blocker. No verification/test projection. Parent-only audit: reviewer28predates this implementation, allocation exhausted. Earlier2.10H completeness-absence wording is historical. Full conformance/downstream/native proof remains unfinished.

- [ ] 3. Complete conformance and upstream handoff
  - [x] 3.1 Bounded phase-aware conformance runner
    - Current verified delivery2026-10-24:11runner controls and101canonical fixtures pass, including actual prefix/wrong-expectation/zero-selection controls. The historical seed alone executed1fixture; later separately approved admissions delivered the current corpus. Full frozen Cargo nowpasses288Rusttests(including7doctests)plus101fixtureoracles. This completes only boundedrunner3.1, not task22/full104-case/native/consumer qualification. Fresh coverage remains unverified without matching LLVM tools.
    - Spec1: task 22 runner subset. Requirements: 8.1/8.6/8.7, 10.7.
    - Owned: `Cargo.toml` (parent only), `tests/conformance.rs`, `tests/support/fixtures.rs`, `tests/conformance_controls.rs` (four files; metadata-only test registration does not require lock changes).
    - Configure `[[test]] name = "conformance"`, `path = "tests/conformance.rs"`, `harness = false`. The test-only main delegates to the fixture runner, interprets the trailing directory prefix itself, rejects invalid/escaping selectors and exits nonzero on zero discovered/selected fixtures. No user-facing Rust CLI or production worker is added. Default libtest name filtering is **not** directory selection and is not used for this target.
    - Cases: explicit kind/phase dispatch; exact outcome/boolean mapping; wrong expected result detected; zero selected/discovered fixtures; actual prefix inclusion/exclusion; killable per-fixture timeout; missing/extra diagnostics. Target: default `conformance_controls` for runner controls, then custom `conformance -- positive` and a guaranteed-empty prefix whose nonzero exit is required evidence. Record observed fixture counts; a Cargo exit with zero tests/fixtures is never GREEN.
    - Done: positive and deliberately failing expectation controls, no thread-only timeout pretending to terminate a stuck evaluator. Harness execution is not a production candidate sandbox. Full canonical task 22 remains incomplete until every 3.2 batch is delivered. Estimate: 3–4h. Dependencies: 2.9, 2.10.
  - [ ] 3.2 Fixture inventory and bounded batches — scheduling container, NOT a dispatch unit
    - [x] 3.2.1 Approved seed: `fixtures/positive/evidence-record/inferred/input.json` and `expected.json`, phase structural/kind evidence-record/outcome pass with empty errors/warnings/blockers. Exact reviewed bytes admitted in `408af0d`; its initial default/positive count1 and empty-selector rejection were verified. Subsequent separately approved deliveries follow; the seed alone did not complete any other batch.
    - [x] 3.2.2–3.2.8 Positive batches: 12 exact pairs admitted in `04253f5`; with the seed, all13 positives and task18 content pass. Independent packet31 found no actionable issue.
    - [x] 3.2.9–3.2.21 Negative batches: 23 exact pairs admitted in `88e638a`; 22 Fail/one newer-MINOR Warn, source-derived exact diagnostics and task19 checks pass. Independent packet32 found no actionable issue.
    - [x] 3.2.36–3.2.37 Round-trip batches: three exact pairs admitted in `6e91805`; actual typed raw-value byte comparisons pass. This is the round-trip subset, not full task21.
    - [x] 3.2.38–3.2.40 Non-native integration batches: three pairs admitted in `b0fdd1b`; Blocked/semantic Fail/hash-false stay distinct and non-accepting. No real backend/native claim.
    - [x] 3.2.22–3.2.35 Adversarial batches:26owner-approved exact pairs delivered; all8namespace classes and7P0profiles satisfy the approved local-nontrust scope. Actual full frozen suite and each new exact-prefix selector passed per batch; canonical task20is complete within that scope.
    - [x] 3.2.53/3.2.55/3.2.56 P0verification consistency:5exact pairs already delivered and full-suite verified; see core-preservation-2026-10-24/results/batch-*.json in the current workflow run.
    - [x] 3.2.42–3.2.45,3.2.48–3.2.52,3.2.54,3.2.57–3.2.60 Integration full batches:27exact store/digest/UTC/version pairs already delivered; all14full groups have passing complete frozen suites and reachable milestone commits in integration-admission-2026-10-24/results/.
    - [x] Supporting unresolvable-ref case from3.2.41delivered and verified; this is only half the batch. The network-disabled sibling remains absent/gated, so3.2.41is NOTcomplete.
    - Current corpus101/current inventory104:57fullbatchesplusonepartialbatch delivered; three native/backend/trusted-setup cases remain gated. Every admitted batch stays at most two directories/four files and has recorded complete frozen-suite/prefix evidence. Fresh coverage/MSRV, legacy-checker repair, traceability and consumer/native handoff remain incomplete. See the [current adversarial admission report](../../../.workflow/ultracode/execution-isolation-implementation/conformance-continuation/adversarial-admission-2026-10-24/final-report.md); parentcontainer3.2staysunchecked.
    - Spec1: tasks 18–21 and task 22 full-suite completion. Requirements: all their existing mappings plus P0-A–F.
    - Required inventory: 13 positive fixtures; 15 baseline + eight named P0 negatives; every eight adversarial namespace class with both correct-handling/rejection cases plus P0 authority attacks; three round-trip types; five baseline integration families plus named P0 scoped-store/snapshot/identity cases. Do not replace any with easier smoke cases.
    - Before dispatch, expand the canonical list into uniquely numbered `3.2.N` batches, each owning at most two concrete fixture directories (input + expected = at most four files), with paths, exact phase/kind, oracle and prerequisites recorded. Cases nested under a named canonical fixture class still count; no hidden unbounded directory ownership. One inventory owner prevents missing or duplicated cases.
    - Each batch estimate <=2h only after enumeration; grader is target `conformance -- <approved-prefix>` followed by the complete suite. Tests/data-only additions may already pass; no fabricated RED. Any production fix needs a fresh actual RED checkpoint and explicit owned-code scope.
    - Done: exhaustive versioned inventory, all seeded controls and full suite pass with no skipped/missing fixtures. Existing document/Python checks do not substitute. Dependencies: 3.1. G-NATIVE applies to actual network-disabled cases; unavailable evidence remains blocked. This container is **not implementation-ready until expanded/approved**.
  - [x] 3.3 Compatibility and format migration policy
    - Spec1: task 23. Requirements: 6.1–6.4, 7.1–7.4, NFR-3.
    - Owned: `docs/compatibility-policy.md`, `tests/compatibility_policy.rs`.
    - Cases: three version spaces; 12-month window; breaking P0 migration; legacy record non-acceptance; EOL/revision matrix; unsupported security version. Target: `compatibility_policy`.
    - Done: concrete approved versions/publication disposition, not an invented no-release assumption. Estimate: 2–3h. Dependencies: 2.4, 2.5. Gate: G-VERSION.
    - Delivered locally2026-10-23: policy plus3actual public-API/source-profile tests, owner-attested first-MVP history explicitly qualified, real prerelease manifest matrix, root-only fallback/security non-acceptance. Reviewer28included this snapshot; parent full checks passed. No publication, deployed-pack qualification, migration release or native proof claimed.
  - [x] 3.4 Extension/capability convention
    - Spec1: task 24. Requirements: 5.8, 10.1–10.6.
    - Owned: `docs/extension-convention.md`, `tests/extension_policy.rs`.
    - Cases: grammar/families; verbatim warnings; registry ownership not identity; optional data preservation; mandatory blockers; no `[x]` capability or acceptance override. Target: `extension_policy`.
    - Done: published-field semantics match real validator/manifest behavior. Estimate: 2–3h. Dependencies: 2.6, 2.7.
    - Delivered locally2026-10-23: convention plus4public-phase/round-trip/TOML tests and full integrated checks. Parent-only source audit, not independently reviewed:28agent allocation exhausted. No new production semantics, namespace assignment, registry authentication, release packaging or capability grant. The local adversarial corpus is now delivered by3.2.22–3.2.35; remaining3.2native/backend/setup cases and release packaging stay open.
  - [ ] 3.5 Field/gate traceability
    - Spec1: task 25. Requirements: NFR-2, P0-A–F.
    - Owned: `docs/traceability/field-to-requirement.md`, `docs/traceability/gate-to-requirement.md`, `tests/traceability.rs`.
    - Cases: every schema/manifest field; new P0 references; all five canonical gates; no orphan field; actual result/fixture links; no false native proof. Target: `traceability`.
    - Done: exhaustive traceability, not only one entry per schema. Estimate: 2–3h. Dependencies: 2.9, 2.10.
  - [ ] 3.6 Real downstream compatibility handoff
    - Spec1: task 26 dependency review portion; Spec 2 TASK-006/BG-01. Requirements: 8.1–8.7, 9.1–9.5, 12.3, NFR-1.
    - Owned: `tests/execution_consumer.rs`, `docs/upstream-compatibility.md`, dependency/license evidence notes in the run, and repository-root `THIRD-PARTY-NOTICES.md` only if actual approved dependency changes require it (never a crate-local duplicate).
    - Cases: actual imports/signatures/schemas; non-accepting mappings; catalog identities; manifest capability validation vs grant; two-validator/TOML interoperability; complete fixture/coverage/lint evidence; missing native checks block handoff. Target: `execution_consumer`, followed by complete crate/conformance and required approved-environment checks; no mere documentation-only handoff.
    - Done: exact immutable delivered revision/version/API/schema/catalog identities, dependency/license inventory, current full tests and upstream/core owner acceptance. No execution crate is created; no BG-01 closure by types-only or mocks. Estimate: 3–4h after prerequisites. Dependencies: 3.2, 3.3, 3.4, 3.5. Gates: G-NATIVE and all actual evidence requirements.
  - [ ] 3.7 Release-only work — deliberately deferred, not BG-01 closure proof
    - Spec1: remaining task 26: release tooling, actual SBOM/archive/signing and offline verification, publication procedures.
    - No implementation/publication unit is admitted here. Before dispatch, separately plan exact tool versions, license syntax, <=5-file slices, key custody, artifact identities and verification environment. No placeholder public key or CLI exit substitutes for signed provenance.
    - Done only with actual reviewed release evidence and separate relevant approvals; no credentials/signing/push/release operation is authorized by this prerequisite plan. Dependencies: 3.6. Estimate: UNESTIMATED until release scope/tools/custody known; not a bounded leaf and not counted as completed by prerequisite delivery.

## Notes

- Canonical-task checkboxes remain conservative. Record partial verified units in the run; do not mark a whole task complete until every obligation passes.
- Historical first implementation slice was **1.2 after relevant G-TOOLS/code dispositions**: five named source/test files, existing dependencies only, real phase/error models and RED/GREEN checkpoints; no schema/catalog, full manifest, crypto implementation, worker or production actions. No authorization is assumed.
- Earlier planning source verification confirmed pinned Boon API signatures only; subsequent scoped compilation/date/deny-loader checks are recorded in the run, not native offline qualification. Official 0.6.1 archive SHA-256 matches committed Cargo.lock; evidence under the run's results/planning-boon-api.json. Public archive GET required scoped network escalation; no package installed/executed. An isolated locked cache was subsequently provisioned and frozen builds passed; no global Cargo configuration was changed.
- Full-conformance and release batches remain explicitly gated planning containers, not disguised four-hour leaves. Expand them before broad execution approval; do not mechanically dispatch an entire container to a worker.
- Useful private-file parallel candidates: schema units 1.7 || 1.8 || 1.9 || 1.10 after prerequisites; documentation 3.3 || 3.4 || 3.5 when their own dependencies are satisfied. Parent registrations/index/lock/Cargo cache and coverage outputs serialize; no same-resource concurrency guarantee is inferred. A 30% parallelism target is not used to weaken real dependencies or duplicate verification.

## Task Dependency Graph

Parent coordination stages: 1 foundation, 2 validators, 3 qualification/handoff. The following **real unit prerequisites** are binding; activation gates apply additionally. 3.2/3.7 require their stated expansion/approval and are not leaf dispatches. The planning audit computes/discloses the deeper graph; no false depth-three or execution-readiness claim.

```json
{
  "1.1": [],
  "1.2": ["1.1"],
  "1.3": ["1.2"],
  "1.4": ["1.3"],
  "1.5": ["1.3"],
  "1.6": ["1.2"],
  "1.7": ["1.6"],
  "1.8": ["1.6"],
  "1.9": ["1.5", "1.6"],
  "1.10": ["1.6"],
  "1.11": ["1.3", "1.5"],
  "2.1": ["1.7", "1.8", "1.9", "1.10"],
  "2.2": ["1.3", "2.1"],
  "2.3": ["2.2"],
  "2.4": ["1.2"],
  "2.5": ["2.3", "2.4"],
  "2.6": ["2.5", "1.10"],
  "2.7": ["1.11", "2.3", "2.4"],
  "2.8": ["1.4", "2.5", "2.7"],
  "2.9": ["1.5", "2.6", "2.8"],
  "2.10H": ["1.5"],
  "2.10": ["1.4", "1.5", "2.9", "2.10H"],
  "3.1": ["2.9", "2.10"],
  "3.2": ["3.1"],
  "3.3": ["2.4", "2.5"],
  "3.4": ["2.6", "2.7"],
  "3.5": ["2.9", "2.10"],
  "3.6": ["3.2", "3.3", "3.4", "3.5"],
  "3.7": ["3.6"]
}
```

## Completion criteria

Planning approval alone completes none of this delivery. BG-01 becomes eligible for closure only after actual compatible implementation, exhaustive fixtures, exact artifact versions and required upstream/core owner/evidence checks. Spec 2 additionally needs all its five decisions, measured finite budgets and separately approved native qualification. The original end-to-end execution-isolation objective remains unchanged and unfinished.
