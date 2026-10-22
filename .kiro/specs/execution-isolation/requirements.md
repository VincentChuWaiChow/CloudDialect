# Requirements Document

## Introduction

**Execution & Isolation is Spec 2 of 3.** It defines the operational boundary between CloudDialect's trusted Rust control plane and untrusted repositories, dependencies, build scripts, transformation packs, language workers, and candidate test execution. Spec 1 (`contracts-evidence-foundation`) owns document formats and validation contracts; Spec 3 will supply Python OSS-to-S3 transformation logic.

CloudDialect is source-available under CloudDialect Community License 1.0. Commercial use requires a separate executed license. This specification does not change the licensing model.

**Status: conditional specification, not implementation-ready.** All requirement evidence is `MISSING`. Documentation validation is not implementation or security evidence. The five mandated architecture decisions remain unresolved. The user authorized conditional design and decision-gated tasks on 2026-10-22; this is not approval of any mechanism.

**Source hierarchy:** the supplied execution-isolation brief defines intent and includes research pages 12–13 excerpts; checked-in Spec 1 documents define the intended dependency; current source defines what exists. The full research PDF was not independently extracted in this run. The source snapshot is `ad2b4bd0823c1a092e9a331c69a600ecad96bb6c`.

## Glossary

- **Core / control plane:** trusted Rust coordinator that owns policy, identity binding, accounting, verifier control, and state transitions; it does not execute candidate code in its own privilege context.
- **Worker:** an untrusted process and all descendants used for analysis, transformation, builds, or test execution, regardless of language.
- **Isolation boundary:** enforceable separation between a worker execution domain and host/control-plane resources; a process boundary alone is not an isolation guarantee.
- **Tenant:** an operator-authenticated administrative owner; **project** is the separate data-isolation domain within that owner. Every access is scoped to both, including projects belonging to one tenant.
- **Run / attempt:** a logical execution and one of its bounded tries. Each retry has a distinct attempt identity; a new attempt does not reset the run's cumulative budget.
- **Capability:** an explicitly authorized operation on a bounded resource, data class, and destination for a specific identity, project, and execution purpose.
- **Execution profile:** approved, measured configuration specifying finite limits and tested environment support. Values and enforcement technology are unresolved, not default-unlimited.
- **Execution budget:** remaining cumulative run allowances, including CPU time, wall time, memory accounting, process/output/storage bounds, attempts, and authorized external operation/model-use quotas where enabled.
- **Admitted input identity:** immutable digest of the input artifact set known before launch, including dependencies and configuration relevant to the requested operation.
- **Candidate identity:** immutable identity plus content digest of the resulting artifact set, bound to its originating input and attempt after production and frozen before verification. A preexisting candidate used for verification already has this identity; a new transformation output does not have a knowable digest before launch.
- **Trusted acceptance control:** protected selection of obligations, tests, oracles, expected outcomes, and final decision; distinct from the sandbox that executes candidate-linked tests.
- **Checkpoint:** durable, validated control-plane metadata at a declared transition, not an untrusted process-memory image or permission to replay an operation.
- **Audit record:** attributable metadata linking inputs, plans, patches, tests, reviewers, execution identities, policy decisions, and outcomes without embedding secrets.
- **Terminal blocker:** a terminal attempt outcome with an explicit reason and recovery instructions; no promotion to accepted or automatic continuation in that attempt.
- **Offline mode:** no outbound network traffic from the run's workers or run-serving model, credential, telemetry, resolver, and audit paths; unrelated host traffic is outside the measurement scope.
- **Qualification suite:** versioned, independently maintained adversarial fixtures and observation procedures executed on declared deployment profiles. Passing is scoped evidence, not proof against all attacks.

## User Roles

| Role | Goal |
|---|---|
| Framework core | Launch and supervise workers without delegating authority to them. |
| Pack authors | Declare required capabilities and compatibility without granting themselves permissions. |
| Operators | Configure supported environments, trusted identities, credentials, data flows, and operational policies. |
| Security reviewers and auditors | Inspect linked evidence, bounded guarantees, and residual risks. |
| Future language-worker implementers | Integrate Python, Go, Java, or other workers through one enforced boundary. |

## Scope and Non-Goals

In scope: worker lifecycle and communication; filesystem/network/resource isolation; execution permissions; secrets; injection-resistant external enforcement; tenant/project data separation; behavioral verification orchestration; audit and interruption/recovery; qualification and operational documentation for these controls.

Out of scope: changing Spec 1 document formats or reimplementing its crate; transformation algorithms or the OSS-to-S3 pack; production data transfer, traffic cutover, source-resource deletion, deployment execution, commercial service delivery, and claims of universal sandbox security. Deploy/delete are distinct permission categories, not implemented execution paths in this spec. Operator-host compromise and hardware side channels are residual risks, not silently covered guarantees.

## Cross-Spec Dependencies and Readiness

| Consumed contract | Intended use | Verified repository state / blocker |
|---|---|---|
| `StructuralResult`, `SemanticResult` | Gate documents and reference integrity before dispatch and output promotion. | Types absent from module-stub source; Spec 1 implementation prerequisite. |
| `BehavioralResult`, `BehavioralOutcome` | Complete behavioral verification after Spec 1's deferred phase. | Types absent; Spec 1 design includes `test_results`, tasks omit it and enumerate `Deferred`, `EvidenceComplete`, `Blocked`. Neither completeness nor deferred means behavioral success. |
| `CatalogUrlLoader` | Use Spec 1's offline schema validation, if needed. | Not implemented; no new loader or network schema resolution here. |
| `ValidatorError`, `BlockingGap` | Carry dependency validation diagnostics and unresolved obligations. | Not implemented; do not invent exported variants or incompatible fields. |
| Pack manifest / verification result documents | Consume versioned capability declarations, identities, evidence, and approvals. | Capability declaration representation and behavioral outcome integration need upstream agreement. No competing schema is defined here. |

**BG-01 — upstream integration gate:** before executable implementation, the Spec 1 owner must deliver the consumed APIs and reconcile its design/task disagreement, capability declaration representation, behavioral outcome mapping, verification configuration/failure-detail representation, and trusted schema-catalog override contract. Record the accepted version and compatibility tests. Optional manifest extensions cannot silently acquire security authority. Any required upstream format change belongs to Spec 1 under separate review. Spec 2 does not silently add a `Passed` enum variant or interpret `EvidenceComplete` as accepted. Execution blockers remain separate from document-validation error categories.

References: `../contracts-evidence-foundation/design.md` (result types, offline loader, deferred phase); `../contracts-evidence-foundation/tasks.md` tasks 8–15; repository-root path `crates/clouddialect-contracts/src/lib.rs` contains stubs only at the baseline.

## Requirements

Criterion `N.M` means criterion M under Requirement N. Stable aliases `REQ-001` through `REQ-010` support cross-artifact traceability. Bracketed error names below are semantic blocker categories, **not selected IPC wire error codes**.

### Requirement 1: Worker Process Model

**ID:** REQ-001

**User Story:** As the framework core, I want a language-independent supervised worker lifecycle so that untrusted analysis and transformation cannot become uncontrolled host execution.

**Verification method:** integration test and adversarial test (handshake, malformed output, inherited handles, forked descendants, crash, and host capability probes); review of supported environment matrix.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. WHEN an execution request is admitted, THE Core SHALL bind the worker and its descendants to the authenticated tenant, project, run, attempt, admitted input identity, pack/toolchain identity, and approved policy revision before executing worker-controlled code.
2. IF a required control or compatible execution profile is unavailable, THEN THE Core SHALL reject launch with `[ISOLATION_UNAVAILABLE]` rather than run without isolation.
3. WHEN a worker is launched, THE Core SHALL expose only explicitly authorized process inputs, environment entries, handles, and communication channels.
4. WHEN a worker establishes communication, THE Core SHALL require an explicitly supported protocol version before accepting operation messages.
5. IF a message is malformed, oversized relative to the approved profile, out of sequence, duplicated, or bound to another attempt, THEN THE Core SHALL reject it with `[PROTOCOL_VIOLATION]` without executing the requested operation.
6. WHEN a worker exits, crashes, disconnects, or is terminated, THE Core SHALL account for the entire descendant execution domain before releasing its resource reservation.
7. IF descendant termination or reaping cannot be confirmed within the approved cleanup deadline, THEN THE Core SHALL quarantine the execution domain with `[CLEANUP_UNCONFIRMED]`.
8. THE Core SHALL NEVER treat process exit status, worker log text, or a worker-reported success field as an acceptance decision.
9. THE Core SHALL expose the same authorization and isolation requirements to every supported worker language.
10. WHEN a worker produces candidate artifacts, THE Core SHALL bind their immutable output identity to the admitted input and originating attempt before verification begins.
11. WHEN a worker exits, crashes, disconnects, or another attempt-ending event occurs, THE Core SHALL initiate descendant termination under the approved cleanup deadline.

### Requirement 2: Filesystem and Network Isolation

**ID:** REQ-002

**User Story:** As an operator, I want repository and build execution confined to declared resources so that malicious input cannot reach host data or unauthorized endpoints.

**Verification method:** adversarial test and integration test with forbidden-file sentinels, path/link races, metadata/service probes, socket/descriptor escape attempts, and run-scoped network observation.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. THE Execution_Boundary SHALL deny worker filesystem access outside the approved project inputs, toolchain material, private writable workspace, and explicitly granted test resources.
2. THE Execution_Boundary SHALL protect approved inputs, toolchain material, and acceptance-control assets from worker writes.
3. IF traversal, symlinks, hard links, mount aliasing, inherited descriptors, devices, or host-control sockets would escape the authorized resource set, THEN THE Execution_Boundary SHALL deny the access.
4. THE Execution_Boundary SHALL deny network communication unless authorized by an explicit destination, operation, and purpose policy.
5. WHEN network access is authorized, THE Execution_Boundary SHALL enforce the destination restriction across DNS resolution, redirects, proxies, local services, metadata endpoints, and descendant processes.
6. WHILE offline mode is active, THE Execution_Boundary SHALL permit zero outbound network traffic attributable to the run, including run-serving control-plane dependencies.
7. IF an offline request requires an external service, THEN THE Core SHALL return `[OFFLINE_DEPENDENCY]` before the external call.
8. THE Execution_Boundary SHALL apply confinement to analysis, dependency resolution, build scripts, toolchain startup, and verifier-side candidate execution as untrusted executable activity.
9. IF host observation detects an isolation violation, THEN THE Core SHALL invalidate that attempt with `[ISOLATION_VIOLATION]`.

### Requirement 3: Resource Limits, Budgets, and Cancellation

**ID:** REQ-003

**User Story:** As an operator, I want measured finite execution allowances so that malicious or stalled workers cannot consume unbounded resources or outlive cancellation.

**Verification method:** unit test, integration test, and adversarial test at configured boundaries; reference-workload measurement review before choosing values.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. IF an execution profile lacks measured, approved, positive finite CPU-time, memory, wall-clock, process-count, output-size, workspace-size, attempt-count, or cleanup-deadline limits, THEN THE Core SHALL block execution with `[BUDGET_UNCONFIGURED]`.
2. WHERE external operations or model use are enabled, THE Core SHALL require explicit finite call/usage allowances before authorizing them.
3. WHEN work is reserved for an attempt, THE Core SHALL charge its resource reservation against the remaining run budget before dispatch.
4. THE Core SHALL account for descendants, parallel workers, verifier execution, and retries within cumulative run accounting rather than independent resettable allowances.
5. IF the next operation would exceed a discrete allowance or an enforced runtime limit is reached, THEN THE Core SHALL stop further dispatch with `[BUDGET_EXHAUSTED]`.
6. WHEN the control plane accepts cancellation, THE Core SHALL prohibit new operations for that run.
7. WHEN cancellation or a runtime limit stops an active attempt, THE Core SHALL initiate termination of its full execution domain under the approved cleanup deadline.
8. IF cancellation, budget exhaustion, or a failed control check is ordered before acceptance commit, THEN THE Core SHALL prohibit acceptance for that attempt.
9. THE Core SHALL preserve consumed-budget accounting across checkpoint recovery and retries.
10. IF reliable budget accounting cannot be recovered, THEN THE Core SHALL return `[ACCOUNTING_UNKNOWN]` rather than reset the allowance.

### Requirement 4: Permission Model and Supply-Chain Admission

**ID:** REQ-004

**User Story:** As a pack author, I want declared capabilities checked against operator authority so that a manifest describes needs without granting privileges.

**Verification method:** unit test, adversarial test, and review for policy intersection, revoked grants, immutable identities, signature/provenance failures, malicious signed packs, and dependency substitution.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. WHEN a pack requests a capability, THE Core SHALL limit the effective grant to the intersection of its validated declaration, authenticated caller authority, operator policy, project scope, execution mode, and remaining budget.
2. IF a declaration, identity binding, policy revision, or required approval is absent or invalid, THEN THE Core SHALL deny the operation with `[PERMISSION_DENIED]`.
3. THE Core SHALL distinguish propose, approve, deploy, and delete permissions without implication from one permission to another.
4. THE Core SHALL NEVER grant approve, deploy, or delete authority to candidate-controlled workers through repository content or model output.
5. WHEN a grant or approval is used, THE Core SHALL validate its phase-specific binding at the authorization boundary: admitted input and execution context for pre-candidate operations, or frozen candidate and protected verification identity for candidate-specific operations and approvals, together with project, operation, policy revision, and validity period.
6. WHEN operator policy is revoked or narrowed during execution, THE Core SHALL stop operations that no longer have authorization.
7. WHEN a pack or toolchain is admitted, THE Core SHALL verify immutable content identity, permitted version, provenance, declared capabilities, and signing authority against operator policy before execution.
8. THE Core SHALL NEVER treat a valid signature as permission to bypass isolation, behavioral tests, or capability checks.
9. IF a dependency or executable differs from its admitted immutable identity, THEN THE Core SHALL block its use with `[ARTIFACT_IDENTITY_MISMATCH]`.
10. WHEN an execution release is qualified, THE Release_Owner SHALL provide signed release evidence, a dependency inventory, and verifiable build provenance for the tested executable set.

### Requirement 5: Credential Scoping and Secret Handling

**ID:** REQ-005

**User Story:** As an operator, I want credentials limited to declared test operations so that repository or model compromise cannot expose general cloud authority.

**Verification method:** adversarial test and integration test with synthetic canary secrets, forbidden resource operations, expiry/revocation, log/export leakage, and worker capability inspection.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. THE Core SHALL NEVER include credential values in model prompts, retrieval context, model-visible tool output, checkpoints, audit payloads, or exported evidence.
2. IF a worker has no approved credential-dependent operation, THEN THE Execution_Boundary SHALL expose no credential material or credential-acquisition capability to it.
3. WHERE an approved test operation needs credentials, THE Credential_Authority SHALL restrict their authority to the bound tenant, project, test resources, operations, and approved lifetime.
4. THE Execution_Boundary SHALL deny access to ambient host credentials, credential agents, metadata services, other workers' secrets, and undeclared credential providers.
5. IF credentials expire, are revoked, or cannot be verified as scoped, THEN THE Core SHALL block the dependent operation with `[CREDENTIAL_UNAVAILABLE]`.
6. WHEN an attempt terminates or is cancelled, THE Core SHALL invalidate its credential-use authorization before any retry.
7. WHEN a retry needs credentials, THE Core SHALL require a fresh authorization decision rather than reuse checkpointed credential material.
8. THE Core SHALL exclude secret-bearing raw stdout, stderr, crash dumps, and diagnostics from persistent or external channels unless processed by the approved secret-handling policy.
9. THE Credential_Authority SHALL NEVER grant production deploy/delete or cross-project access as a consequence of permission to run a test.

### Requirement 6: Prompt Injection and Data Egress Control

**ID:** REQ-006

**User Story:** As a security reviewer, I want authority enforced outside model context so that malicious repository text cannot redefine policy or acceptance.

**Verification method:** adversarial test with injections through source, documentation, dependencies, retrieved examples, tool output, and encoded results; unit test of egress decisions.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. THE Core SHALL treat repository content, retrieved material, candidate-produced tool output, and model output as untrusted data rather than authorization.
2. WHEN a model or worker proposes a tool call, THE Core SHALL authorize the concrete operation and resource outside the model context before executing it.
3. IF untrusted content requests secrets, broader permissions, weaker acceptance rules, or unauthorized endpoints, THEN THE Core SHALL deny the requested authority change with `[UNTRUSTED_AUTHORITY_REQUEST]`.
4. THE Core SHALL NEVER let a candidate or model modify protected acceptance tests, expected outcomes, test-selection rules, approver identity, or policy used to judge that candidate.
5. WHEN code, artifacts, prompts, retrieval examples, logs, or exports would leave the project boundary, THE Core SHALL require an explicit data-class, destination, model/provider routing, purpose, and retention policy authorization.
6. IF a model route or external processor is unavailable or lacks approved data-handling terms, THEN THE Core SHALL block transmission with `[EGRESS_DENIED]` instead of falling back to another provider.
7. THE Core SHALL record the applied routing and egress policy revision without persisting secret or disallowed source payloads.
8. THE Qualification_Suite SHALL include attacks that attempt exfiltration through otherwise valid tool requests, generated filenames, test outputs, and diagnostic channels.

### Requirement 7: Tenant and Project Data Isolation

**ID:** REQ-007

**User Story:** As an operator, I want separate project storage and retrieval boundaries so that one project's execution cannot read or poison another project's artifacts.

**Verification method:** adversarial test and integration test for same-tenant/different-project and different-tenant access, guessed identifiers, cache poisoning, exports, retention, and restored data.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. WHEN the Core reads, writes, lists, retrieves, caches, exports, or deletes project data, THE Core SHALL enforce the authenticated tenant/project scope independently of caller-supplied path or object identifiers.
2. THE Core SHALL partition private artifacts, workspace material, caches, retrieval indexes, migration examples, audit access, and checkpoints by tenant and project.
3. THE Core SHALL NEVER reuse project-private cached outputs or retrieval results across project boundaries, including projects owned by the same tenant.
4. WHERE immutable public toolchain or schema material is shared, THE Core SHALL restrict sharing to operator-admitted content that contains no project-derived data.
5. IF a cached object has mismatched identity, provenance, policy compatibility, or project scope, THEN THE Core SHALL reject it with `[CACHE_INVALID]`.
6. WHEN a project retention policy is configured, THE Core SHALL require explicit retention and deletion rules for each persisted data class, including backups, logs, exports, and retrieval indexes.
7. WHEN an authorized project-data deletion or expiry is applied, THE Core SHALL invalidate access to affected live objects and references under that policy.
8. IF retained backup or audit copies cannot yet be physically deleted under the configured policy, THEN THE Core SHALL report the retained classes, restricted access, and scheduled deletion condition rather than claim complete erasure.
9. WHEN data is restored, THE Core SHALL preserve project authorization and deletion restrictions before making restored objects accessible.
10. THE Core SHALL limit this spec's deletion behavior to framework-owned project data rather than source cloud resources.
11. IF the applicable data class lacks an approved retention/deletion policy, THEN THE Core SHALL deny persistence with `[RETENTION_UNCONFIGURED]`.
12. IF execution requires persistence denied by the retention policy gate, THEN THE Core SHALL block that execution before launch.

### Requirement 8: Behavioral Verification and Verifier Integrity

**ID:** REQ-008

**User Story:** As a security reviewer, I want independently controlled behavioral verification so that forged evidence or weakened tests cannot turn an unsafe candidate into an accepted result.

**Verification method:** adversarial test, unit test, and integration test with Spec 1 contract fixtures; seeded critical defects, forged results, assertion removal, stale evidence, incomplete tests, and candidate substitution.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. WHEN execution consumes a contract document or worker-produced evidence, THE Core SHALL require successful Spec 1 structural and semantic validation for its immutable identity before acting on it.
2. IF structural or semantic validation fails, THEN THE Core SHALL block the dependent action with the consumed `ValidatorError` diagnostics.
3. WHEN behavioral verification begins, THE Verifier_Control SHALL bind the exact candidate, dependencies, toolchain, configuration, mandatory obligations, protected test set, expected outcomes, policy, and attempt into the verification identity.
4. THE Verifier_Control SHALL keep acceptance control outside candidate-writable execution domains while applying worker isolation to all candidate-linked test execution.
5. IF mandatory evidence is absent, unknown, failed, skipped, deferred, stale, fabricated, or from a different verification identity, THEN THE Verifier_Control SHALL return a non-accepting result identifying every affected obligation through `BlockingGap` or the agreed upstream diagnostic mapping.
6. THE Verifier_Control SHALL NEVER interpret `StructuralResult`, `SemanticResult`, `BehavioralOutcome::Deferred`, or `BehavioralOutcome::EvidenceComplete` alone as behavioral acceptance.
7. WHEN all mandatory checks complete, THE Verifier_Control SHALL produce a `BehavioralResult` using the reconciled Spec 1 API without altering Spec 1 schemas in this spec.
8. WHEN acceptance is requested, THE Core SHALL require independently authenticated approval bound to the complete current verification identity in addition to passing behavioral evidence and the acceptance-commit safety checks.
9. IF any bound artifact or protected rule changes before acceptance commit, THEN THE Core SHALL invalidate the result with `[EVIDENCE_IDENTITY_MISMATCH]`.
10. THE Qualification_Suite SHALL reject or block every seeded critical defect in its declared AC-05 suite.
11. IF the consumed API cannot represent the required non-accepting outcomes or verification linkage, THEN THE Core SHALL block integration with `[UPSTREAM_CONTRACT_UNRESOLVED]`.
12. THE Qualification_Suite SHALL include a valid reference candidate that completes the declared acceptance path as a positive control against vacuous reject-all qualification.
13. THE Verifier_Control SHALL authenticate the origin and integrity of behavioral observations independently of candidate-controlled result text, runner imports, or exit codes.

### Requirement 9: Audit Records and Operational Evidence

**ID:** REQ-009

**User Story:** As an auditor, I want attributable execution records so that I can reconstruct decisions without trusting worker claims or receiving secrets.

**Verification method:** integration test and review of provenance joins, append failures, access/tamper detection, secret exclusion, restore exercises, and environment qualification reports.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. WHEN an attempt is admitted, THE Core SHALL durably record its identity and initial authorization before worker execution begins.
2. WHEN a security-relevant transition occurs, THE Core SHALL record its attributable input identities, plan and patch references, test/evidence references, reviewer/approver identity where applicable, execution identity, policy revision, budget disposition, timestamp, and outcome or denial reason.
3. THE Audit_Control SHALL prevent workers from modifying or deleting acceptance-control audit records.
4. WHEN an audit record is read or restored, THE Audit_Control SHALL detect missing links or integrity failures before treating it as valid decision evidence.
5. IF required audit persistence fails, THEN THE Core SHALL prohibit new dispatch and acceptance until durability is re-established or the attempt is terminally blocked with `[AUDIT_UNAVAILABLE]`.
6. THE Core SHALL permit containment, revocation, and termination actions even when audit storage is unavailable.
7. THE Audit_Control SHALL enforce project-scoped access and the operator-approved retention/deletion policy without persisting credential values.
8. WHEN an execution profile is published as qualified, THE Release_Owner SHALL include an adversarial-suite report identifying environment, executable identities, tested threat scope, results, and residual risks.
9. THE Release_Owner SHALL provide tested restore procedures, failure/cancellation/cleanup runbooks, and a named compatibility owner for each supported environment.

### Requirement 10: Failure Behavior, Checkpoints, and Safe Interruption

**ID:** REQ-010

**User Story:** As the framework core, I want explicit checkpoint and retry semantics so that interruption never becomes fabricated success or duplicated side effects.

**Verification method:** failure-injection integration test at every declared checkpoint and both sides of external-effect/acceptance boundaries; unit test of state transitions and retry classification.

**Evidence status:** `MISSING`

#### Acceptance Criteria

1. THE Core SHALL classify attempt states as intermediate (`prepared`, `running`, `verifying`, `committing`, `stopping`) or terminal (`accepted`, `rejected`, `blocked`, `cancelled`) with explicit permitted transitions.
2. THE Core SHALL persist declared checkpoints at admission, dispatch intent, worker completion, verification completion, and acceptance commit boundaries without serializing worker secrets.
3. WHEN recovery reads a checkpoint, THE Core SHALL verify its tenant/project scope, integrity, candidate and evidence identities, policy applicability, budget accounting, and external-effect disposition before resumption.
4. IF a checkpoint is corrupt, incomplete, incompatible, or insufficient to determine side effects, THEN THE Core SHALL produce `[RECOVERY_BLOCKED]` rather than assume an operation did not happen.
5. WHEN a retry is authorized for a classified transient failure, THE Core SHALL create a new attempt within the remaining run budget using current policy and fresh credential authorization.
6. THE Core SHALL NEVER automatically retry validation failures, permission denials, unsupported environments, identity mismatches, revoked credentials, or unresolved external write outcomes.
7. WHEN an interrupted operation has a reconciled idempotent outcome, THE Core SHALL resume only from the declared checkpoint whose prerequisites remain valid.
8. IF a retry or resumption changes any verification-bound identity, THEN THE Core SHALL invalidate prior behavioral success for that identity.
9. THE Core SHALL order cancellation, revocation, budget exhaustion, and acceptance commit through one authoritative decision sequence.
10. WHEN a terminal state is recorded, THE Core SHALL prohibit reopening that attempt; recovery requires a separately authorized attempt except that an already accepted run is read back rather than replayed.
11. THE Qualification_Suite SHALL demonstrate safe resumption or an explicit terminal blocker for failure injection at every declared checkpoint.
12. THE Core SHALL NEVER promote incomplete verification, uncertain cleanup, or an uncommitted acceptance intent to accepted state.
13. WHEN cancellation is ordered after durable acceptance commit, THE Core SHALL report the already committed outcome without replaying or retroactively changing that attempt.

## Unresolved Decisions

All five items below are **UNRESOLVED**, not implied defaults. Decision records need evidence, owner approval, affected-criterion review, and downstream revalidation before implementation activation.

| ID | Unresolved decision | Required decision evidence / owner | Gate |
|---|---|---|---|
| UD-1 | Isolation mechanism: namespaces, seccomp-bpf, containers, WebAssembly, or combination not selected. | Security/operations owners compare boundary strength, Linux/macOS/CI portability and support, maintenance cost, failure modes, adversarial results. Unsupported environments must fail closed. | REQ-001, REQ-002, REQ-003, REQ-005, REQ-008 |
| UD-2 | Worker IPC: versioned JSON-RPC over stdio is a research recommendation only; message schema, framing, compatibility rules, and wire error codes not defined. | Core/worker owners specify bounded parsing, correlation, startup negotiation, cancellation, backpressure, and security tests. | REQ-001, REQ-003, REQ-006, REQ-010 |
| UD-3 | Credential injection/delivery mechanism not selected. | Security/operations owners evaluate environment, mounted files, proxy, or alternatives against least privilege, expiry/revocation, observability and leak tests. | REQ-005, REQ-006, REQ-010 |
| UD-4 | Audit persistence backend and initial retention policy not selected. | Operations/security owners define durability, tamper evidence, project authorization, outage behavior, restore, deletion, and external/offline compatibility. Related persisted-data retention classes need explicit policy too. | REQ-007, REQ-009, REQ-010 |
| UD-5 | Resource-limit values not established. | Core/operations owners measure a reference workload and adversarial saturation before approving CPU, memory, wall, process, storage, output, attempt, cleanup, and enabled external-use budgets; units and accounting semantics must be explicit. | REQ-001, REQ-003, REQ-010 |

No numerical production defaults, security-strength claims for unselected technology, or invented public API outcomes are introduced. Configured-boundary tests are specified now; numerical qualification and platform claims remain gated. BG-01 is an additional upstream readiness blocker, not a resolved architecture choice.

## Threat Model

Trust assumptions: operator-authorized control-plane code, identity source, policy/signing roots, acceptance-control assets, and host enforcement are trusted. A malicious tenant, pack, dependency, repository, model response, or worker may control all candidate input and execution inside its domain. Compromise of trusted roots requires a different incident response; signatures do not make a pack trusted code.

| Surface | Asset | Attacker capability / entry point | Trust boundary | Required enforcement (technology unresolved) | Adversarial test requirement | Residual risk |
|---|---|---|---|---|---|---|
| Repository, dependency and build execution | Host files, core integrity, compute | Malicious build scripts, dependencies and forks executed during analysis/build/startup | Worker descendants → host/core/other execution domains | Launch-before-code confinement; restricted mounts/handles/network; cumulative budgets; confirmed cleanup (1.1–1.8, 2.1–2.9, 3.1–3.10) | T-01: traversal/link races, host sockets, fork bombs, resource/log flooding, startup scripts, descendants surviving parent exit | Kernel/runtime vulnerabilities and privileged-host compromise remain; only qualified profiles supported. |
| Prompt injection through code, docs, retrieval and tool output | Secrets, permissions, acceptance rules | Arbitrary instruction text or encoded payload entering model/tool context | Untrusted data → external policy and acceptance control | Concrete tool authorization, protected rules, route/egress policy (4.1–4.6, 5.1, 6.1–6.8) | T-02: request secrets, widen policy, weaken oracle, redirect a valid tool to unauthorized destinations, encode exfiltration in output | Semantic attacks within already-authorized data flows remain possible; prompt filters alone are not a boundary. |
| Transformation packs and toolchains | Executable identity, permission policy, release trust | Tampered or malicious correctly signed pack; poisoned dependency/version resolution | Distribution/input → admitted executable set | Immutable identities, authorized provenance/signing/version checks, same confinement for signed packs (4.7–4.10, 2.8) | T-03: digest tamper, disallowed signer/version, dependency substitution, malicious signed pack seeking escalation | Compromised signing roots or approved toolchain vulnerabilities remain; provenance is not correctness. |
| Verifier, test oracle and evidence | Acceptance integrity | Remove assertions, forge output, substitute candidate or stale results | Candidate/test execution → protected selection/oracle/results/approval | Independent acceptance control, identity binding, trusted observations, fresh approval, atomic decision ordering (8.1–8.11, 10.8–10.12) | T-04: assertion removal, fabricated pass, zero/partial test run, stale replay, mutation between check and acceptance; all seeded critical defects blocked/rejected | Incomplete oracle coverage can miss unseeded defects; passing does not prove universal equivalence. |
| Credentials and cloud access | Test credentials and cloud resources | Credential harvesting, unauthorized API use, cross-project access from worker/tool/model | Credential authority → scoped operation and project | No ambient credentials; purpose/resource/lifetime scope; egress limits; fresh retry authorization (5.1–5.9, 4.1–4.6) | T-05: host environment/metadata/agent probes, forbidden read/write, cross-project resource, expiry/revocation, model/log/checkpoint leak canaries | An authorized credential-bearing worker may misuse its granted operations; cloud policy and chosen mechanism must bound damage. |
| Source, artifacts, caches and model egress | Confidential project data and evidence | Guess IDs, poison cache/retrieval, export another project's data, leak via prompts/logs | Project A → project B, provider or export recipient | Authenticated project partitioning, cache provenance, explicit egress/retention, deletion-aware restore (6.5–6.8, 7.1–7.10, 9.7) | T-06: same-tenant/different-project and cross-tenant cache/index contamination; log/prompt/export leakage; stale restore after deletion | Approved recipients may retain data under their terms; backup erasure and side-channel limits remain explicit. |

## Acceptance Gate Traceability

| Gate | Meaning and requirement coverage | Required evidence | Current status |
|---|---|---|---|
| AC-05 | Verifier integrity: REQ-004, REQ-006, REQ-008, REQ-009, REQ-010; especially 8.10 | T-02/T-03/T-04 qualification: every seeded critical defect rejected or blocked; protected-test, forged-result, stale-evidence and substitution attacks | MISSING |
| AC-06 | Contained execution/data access: REQ-001 through REQ-007 and REQ-009 | T-01/T-02/T-03/T-05/T-06: no forbidden host files, credentials, network or other-project data; offline network observation shows zero run-attributable outbound traffic | MISSING |
| AC-08 | Predictable interruption: REQ-001, REQ-003, REQ-005, REQ-008, REQ-009, REQ-010 | Failure injection before/after each checkpoint and effect/commit boundary; safe resumption or explicit blocker; no stale acceptance/replay | MISSING |

Qualification reports must name their environment and observation limitations. No gate is passed merely by writing this specification or running document validators.
