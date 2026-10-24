# Acceptance-gate traceability

This is the local gate matrix required by NFR-2.2 and
[Task 25](../../.kiro/specs/contracts-evidence-foundation/tasks.md). The
[Acceptance Gate Traceability table](../../.kiro/specs/contracts-evidence-foundation/requirements.md#acceptance-gate-traceability)
is authoritative for gate IDs, purposes and requirement sets. The
[Traceability Artifacts design](../../.kiro/specs/contracts-evidence-foundation/design.md#traceability-artifacts)
defines this table's four columns. The companion
[field-to-requirement table](field-to-requirement.md) provides exhaustive field
coverage; the gate rows select relevant declared fields, not every field in a
document. A mapping is not a passing-gate assertion.

Field keys below qualify JSON instance paths with their document kind and TOML
paths with `manifest`. `[*]` denotes an array element; `contracts.*` denotes an
operation-keyed contract reference. These are documentation path conventions,
not permission wildcards.

| Gate | Description | Requirements | Schema fields |
|---|---|---|---|
| AC-01 | Each transformation declares a versioned manifest, supported source/toolchain/catalog versions, operations, fidelity, assumptions and limitations. Schema/catalog versions and extension policy support compatibility checks; unsupported inputs require explicit validation results, not inferred support. | Req 1, Req 5, Req 6, Req 7, Req 10, Req 11, NFR-1 | `manifest.pack.id`, `manifest.pack.version`, `manifest.source.language_version_range`, `manifest.toolchain[*].version_range`, `manifest.operations[*].id`, `manifest.operations[*].behavioral_fidelity`, `manifest.operations[*].assumptions`, `manifest.operations[*].limitations`, `manifest.compatibility.catalog_min`, `manifest.compatibility.catalog_max`, `manifest.contracts.*.format_version`, `manifest.schema_refs.contract_schema_version`, `manifest.schema_refs.evidence_record_schema_version`, `manifest.schema_refs.verification_result_schema_version`, `manifest.capabilities.format_version`, `behavioral-contract.format_version`, `catalog-index.schemas[*].format_version`, `catalog-index.schemas[*].status`, `catalog-index.schemas[*].end_of_life_date`, `behavioral-contract.required_extensions`, `extension-registry.extensions[*].namespace_prefix`, `manifest.x` |
| AC-02 | Mandatory behavioral obligations link to classified source evidence and declared verification identities, suite/oracle and approval references. Unresolved mandatory obligations block acceptance; declaring an expected-outcome reference or a verification method is not proof that either was approved or executed. | Req 2, Req 3, Req 4, Req 8, Req 11, NFR-2 | `behavioral-contract.obligations[*].obligation_id`, `behavioral-contract.obligations[*].behavioral_fidelity`, `behavioral-contract.obligations[*].evidence_refs`, `behavioral-contract.obligations[*].unresolved_assumption`, `behavioral-contract.resource_dependencies[*].unresolved_assumptions`, `evidence-record.id`, `evidence-record.claim`, `evidence-record.classification`, `evidence-record.provenance.source_artifact_identity`, `evidence-record.provenance.analysis_tool_identity`, `evidence-record.reasoning_chain`, `evidence-record.unresolved_assumptions`, `verification-result.binding.obligation_ids`, `verification-result.binding.suite`, `verification-result.binding.oracle`, `verification-result.tests[*].test_id`, `verification-result.tests[*].obligation_ids`, `verification-result.tests[*].origin_identity`, `verification-result.acceptance.approval_evidence` |
| AC-05 | Protected tests, results and candidate identity must resist weakening, forgery, substitution and extension-based acceptance bypass. Mandatory-extension declarations and namespace metadata support local rejection checks; payloads, registry owner strings and apparently valid bindings cannot establish verifier integrity or override core acceptance rules. | Req 10 | `behavioral-contract.required_extensions`, `evidence-record.required_extensions`, `verification-result.required_extensions`, `catalog-index.required_extensions`, `extension-registry.required_extensions`, `extension-registry.extensions[*].namespace_prefix`, `extension-registry.extensions[*].owner`, `manifest.x`, `verification-result.binding.suite.content_hash`, `verification-result.binding.oracle.content_hash`, `verification-result.candidate.content_hash` |
| AC-07 | Every accepted result must record immutable input/dependency, candidate, pack/toolchain, configuration, protected test/outcome and approval bindings. Local consistency and hash checks can reject malformed or mismatched claims; trusted observations and current context must separately invalidate altered or stale acceptance attempts. | Req 3, Req 4, Req 8 | `evidence-record.provenance`, `verification-result.id`, `verification-result.format_version`, `verification-result.binding.input`, `verification-result.binding.dependencies`, `verification-result.candidate.identity`, `verification-result.candidate.content_hash`, `verification-result.pack.id`, `verification-result.pack.version`, `verification-result.pack.content_hash`, `verification-result.toolchain.id`, `verification-result.toolchain.version`, `verification-result.toolchain.content_hash`, `verification-result.configuration.identity`, `verification-result.configuration.content_hash`, `verification-result.binding.tenant_id`, `verification-result.binding.project_id`, `verification-result.binding.run_id`, `verification-result.binding.attempt_id`, `verification-result.binding.policy`, `verification-result.binding.suite`, `verification-result.binding.oracle`, `verification-result.binding.obligation_ids`, `verification-result.tests[*].test_id`, `verification-result.tests[*].obligation_ids`, `verification-result.tests[*].origin_identity`, `verification-result.tests[*].outcome`, `verification-result.tests[*].failure_description.detail`, `verification-result.acceptance.approver_identities`, `verification-result.acceptance.timestamp`, `verification-result.acceptance.verification_result`, `verification-result.acceptance.approval_evidence` |
| AC-09 | Qualifying releases include dependency/license inventories, SBOMs and verifiable artifact provenance; tampering and unapproved permission escalation must fail policy checks. Catalog identity, distribution paths, format compatibility and requested capabilities are declared inputs, not proof of release signing, license screening or authorization. | Req 1, Req 6, Req 7, Req 12, NFR-1 | `catalog-index.format_version`, `catalog-index.schemas[*].schema_id`, `catalog-index.schemas[*].catalog_path`, `catalog-index.schemas[*].format_version`, `catalog-index.schemas[*].status`, `catalog-index.schemas[*].end_of_life_date`, `catalog-index.resolver_resources[*].schema_id`, `catalog-index.resolver_resources[*].catalog_path`, `manifest.pack.version`, `manifest.compatibility.catalog_min`, `manifest.compatibility.catalog_max`, `manifest.capabilities.requirements[*].action`, `manifest.capabilities.requirements[*].resource` |

## Extension and artifact boundaries

The design's illustrative AC-05 row mentions `patternProperties`. That is JSON
Schema metadata, not an instance field, so it is deliberately not a field key in
the matrix. The schemas' root pattern and open-root policy permit extension
members; namespace spelling and registry entries do not authenticate a publisher.
Opaque extension values have no invented finite child-field inventory. Their
exact encoded JSON value bytes must be preserved; that preservation is not a
policy grant. `required_extensions` can introduce mandatory blockers, while
manifest `[x]` cannot replace typed capability declarations or override core
validation and acceptance rules. See the
[extension convention](../extension-convention.md).

For AC-09, release dependency/license inventories, SBOM generation, asymmetric
signatures, published verification keys and verified release archives are
separate release artifacts under Req 12. They are not fields in these instance
schemas. A catalog path, schema identifier, version or content digest alone does
not demonstrate artifact provenance, signer authenticity, successful license
screening or a distributed release. These two traceability documents are local
documentation; release inclusion and publication have not been established by
their existence.

## P0 amendments and non-accepting integration

The [P0-A–F contract amendments](../../.kiro/specs/contracts-evidence-foundation/p0-contract-amendments.md)
refine the representations and ports supporting the canonical gates; they do
not create additional acceptance gate IDs or expand the canonical requirement
sets above. In particular:

- **P0-A / Req 8.6:** structural, semantic and behavioral phase results remain
  distinct. `Deferred`, `EvidenceComplete` and `Blocked` are non-accepting;
  evidence completeness is not a behavioral pass, even with passing-looking
  serialized tests. There is no approving conversion between phase results.
- **P0-B / Req 5.8:** typed, versioned capability requirements live outside
  manifest `[x]`. A parsed `action` and logical `resource` alias are requests,
  never effective capability grants. Admission still needs independent operator
  policy and concrete authorized-resource resolution.
- **P0-C / Req 4.8:** immutable configuration, artifact/binding digests,
  obligation-to-test links, structured failure detail and bound approval
  references support AC-02/AC-07. Serialized origin/approver identities, hashes
  and UTC timestamps are declarations, not trusted verifier observations or
  authenticated approvals. Protected mandatory test selection, current trusted
  tenant/project/run/attempt/policy context, freshness and approval authority
  must be checked independently before acceptance.
- **P0-D / Req 9.5:** catalog construction admits embedded or explicit immutable
  snapshot bytes; a snapshot digest is identity evidence, not signer trust.
  Caller-controlled repository/environment data cannot establish catalog
  authority. Local loader checks do not prove actual network-disabled or
  trusted environment-variable setup.
- **P0-E / Req 8.7:** project-scoped `DocumentStore` resolution and explicit JSON
  and TOML structural/semantic entrypoints are ports, not an implemented secure
  backend or caller authentication. The trusted host must bind store scope;
  synthetic store tests cannot prove real cross-project denial. The companion
  field table separately lists these Rust API surfaces rather than treating
  method names as serialized instance fields.
- **P0-F:** approved initial document/catalog 1.0.0 and crate API 0.1.0 versions
  are owner-provided initial-MVP decisions, not independent publication-history
  verification or downstream compatibility acceptance. Discovery of prior
  distribution requires the specified migration/version review; parsing a
  version does not establish supported semantics.

## Evidence scope and open qualification

The [local 101-case conformance report](../../.workflow/ultracode/execution-isolation-implementation/conformance-continuation/adversarial-admission-2026-10-24/final-report.md)
belongs to immutable Git revision
`f526e677668eee9abeb5bdd937119b4b87ee3688`. It reports 101 admitted canonical
fixtures, local structural/semantic checks and bounded runner controls. The
relative report link follows the working tree; the recorded Git revision fixes
the cited snapshot. Its synthetic/local scope is not real downstream consumer
qualification, authenticated upstream/core owner acceptance, release proof or
native isolation. The
[current run report](../../.workflow/ultracode/execution-isolation-implementation/final-report.md)
tracks later work without retroactively changing that snapshot's scope.

Three qualification/setup cases remain absent and gated: observed
network-disabled validation (the unadmitted half of batch 3.2.41), real
same-tenant cross-project backend denial (3.2.46), and trusted snapshot
environment setup (3.2.47). No mock, declaration, schema fixture or documentation
row substitutes for actual approved native/backend/snapshot setup and its
observed result.

**BG-01 remains open.** Spec 2 UD-1–UD-5 and its runtime qualification remain
open. The
[BG-01 closure record](../../.kiro/specs/execution-isolation/decisions/BG-01-upstream.md#closure-record-required)
still requires actual upstream/core owner approvals, exact delivered immutable
API/schema/catalog identities and real consumer/compatibility evidence.
Traceability and local conformance alone do not supply those approvals or close
any acceptance, runtime, native or release gate.
