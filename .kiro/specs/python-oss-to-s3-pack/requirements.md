# Requirements Document

## Introduction

This is Spec 3 of 3: the **CloudDialect Python OSS-to-S3 Migration Pack** (`python-oss-to-s3-pack`). It defines a deterministic, reviewable migration of a bounded Python application's Alibaba OSS SDK (`oss2`) usage to AWS S3 SDK (`boto3`) usage. It exercises Spec 1's contracts/validation and Spec 2's execution boundary; it does not implement or replace either foundation.

**Status: conditional specification, NOT implementation-ready.** The reference workload, SDK versions, corpus, AI scope and real-service environment are unresolved. Every requirement and criterion has evidence status `MISSING`. Document review is not implementation, behavioral, security or real-service evidence. The branch is specification-only.

CloudDialect remains source-available under CloudDialect Community License 1.0. Commercial use requires a separate executed license; this specification changes neither condition.

## Glossary

| Term | Definition |
|---|---|
| Reference workload | Selected, licensed, non-toy Python application with actual OSS use, immutable source identity and at least three distinct OSS operation types. No application has yet been selected. |
| Call-site inventory | Source locations, possible SDK targets, wrappers, configuration/resource dependencies, confidence and unresolved references for the complete declared source tree. Static possibility is not proof of a runtime target. |
| Deterministic recipe | Versioned rule producing the same candidate bytes and configuration changes from the same approved inputs, without a model. |
| Obligation | Independently reviewable source behavior to preserve or explicitly adapt, including inputs, outputs, side effects, permissions and failure behavior. |
| Qualification corpus | Frozen, reviewed set of eligible tasks, annotated discovery cases and known-unsafe candidates, each with immutable identities and declared strata. It has not been assembled. |
| Accepted output | An immutable candidate digest for which the trusted acceptance path recorded acceptance under a particular frozen suite/rule/environment identity. A worker success message is not acceptance. |
| False acceptance | An accepted output independently found to violate at least one required obligation; criticality is assigned before evaluation, not after observing a failure. |
| Pack maintainer | Owner of workload maintenance, contracts, recipes, compatibility declarations and requalification. Not automatically the independent evaluator. |
| Independent oracle | Expected behavior and adjudication controlled outside the patch-generating path, with reviewer identity/provenance and no candidate write authority over acceptance assets. |
| Evidence bundle | Export binding baseline, candidate diff, inventory/findings, contracts/evidence, verification, metrics, limitations and gaps by immutable identities. |
| Qualification | Acceptance of one pack/version for its declared scope and environment after all mandatory gates, not a claim of general migration safety. |
| Unsupported / blocked / rejected | Respectively: outside declared applicability; prerequisites/evidence/authorization unavailable; tested violation of a required obligation. Logical pack outcomes; upstream serialization is not yet agreed. |

## User Roles

| Role | Goal |
|---|---|
| Pack maintainer | Maintain bounded support and repeatable recipes; requalify changes. |
| Pack user | Assess applicability from inspectable evidence rather than broad compatibility claims. |
| Framework maintainer | Discover and close Spec 1/2 interface gaps without pack-local substitutes. |
| Evaluator / auditor | Independently reproduce results and challenge acceptance, coverage and measurements. |

## Scope and Non-Goals

In scope: workload selection documentation; Python static analysis; populated operation contracts; deterministic code/configuration recipes; optional evidence-gated AI; independently controlled verification; evidence export; a valid TOML pack manifest; bounded qualification and reproducibility.

Out of scope: schema/validator infrastructure, isolation implementation, a second language or direction, general OSS/S3 API coverage, production data transfer, traffic cutover, source-resource deletion, commercial delivery. A future approved test may delete only its explicitly designated synthetic objects; that is not authority to delete migration source resources. This documentation run creates no reference application, runtime code, contract instances, manifest instance or cloud resources.

The six-stage workflow is: establish source baseline → identify dependencies/behavior → record gaps → propose transformation → verify candidate → export diff/findings/results/limitations. Gaps do not become observed facts when they pass through a later stage.

Candidate operation types are upload (`put_object`), download (`get_object`), deletion (`delete_object`), URL signing (`sign_url` / `generate_presigned_url`) and bucket existence (`does_bucket_exist` / `head_bucket`). These are examples, not an approved scope or a promise that the calls are behaviorally equivalent. UD-1 determines the subset; every actual call pattern must be accounted for.

## Upstream Dependencies and Evidence Boundaries

| Gate | Required upstream deliverable | Current evidence / consequence |
|---|---|---|
| BG-01 | Spec 1's five bundled schemas, offline `CatalogUrlLoader`, structural/semantic validation and manifest validation | Current crate contains module stubs, not these working APIs. Intended calls include `validate_structural()`, `validate_semantic()` and `ManifestValidator::validate_manifest()`; exact signatures/result mappings must be reconciled with Spec 1 before integration. |
| BG-02 | Spec 1 operation applicability and explicit `unsupported` outcome | Manifest validity alone cannot decide applicability for arbitrary analyzed call sites. An upstream-owned contract must define operation/version/pattern context, exact SDK constraint representation and the unsupported response; no invented overload or pack-local replacement validator. Current proposed operation entries do not define dedicated SDK constraint fields. |
| BG-03 | Spec 2 execution/IPC, completed `BehavioralResult`, verifier ownership and audit linkage | Spec 2 remains conditional with five mechanism decisions open. Reconcile its Spec 1 integration gate, result fields/status vocabulary, capability declaration and artifact/audit identity interfaces before binding this pack. |

The five catalog schemas are behavioral contract, evidence record, verification result, pack manifest and catalog index; their paths/versions are obtained from the approved bundled index, not guessed here. Static/semantic validity, evidence completeness and a completed `BehavioralResult` are not synonymous with behavioral acceptance. Spec 1's proposed result/design and tasks disagree on `test_results`; full configuration/oracle/policy/attempt bindings are not yet represented consistently. Unknown optional extensions cannot become authoritative acceptance fields. Expected API names above are consumed specifications, not claims of available Rust or Python exports. Interface gaps are changes for the owning upstream spec, not permission to edit it in this run.

## Source Corrections and Documentation Evidence

**The brief's universal claim that oss2 presigned expiry uses milliseconds is not supported by the inspected SDK source.** `oss2.Bucket.sign_url(..., expires, ...)` documents seconds; boto3's `ExpiresIn` also documents seconds. Do not introduce a milliseconds-to-seconds conversion merely to match that premise. Wrapper behavior, allowed ranges, rounding, signing configuration and credential lifetime still need testing against UD-2's exact versions. If an application really supplies milliseconds, its contract must show that source evidence and the explicit adaptation.

Documentation checked 2026-10-22 (moving upstream documentation is NOT a compatibility pin):

| Ref | Primary source | What it supports / limitation |
|---|---|---|
| DOC-01 | [oss2 API source](https://github.com/aliyun/aliyun-oss-python-sdk/blob/master/oss2/api.py) (`Bucket.sign_url`) | `expires` is documented in seconds. Context7 lookup returned v2/async projects, not oss2; those were rejected and the actual SDK source used. |
| DOC-02 | [boto3 generate_presigned_url](https://docs.aws.amazon.com/boto3/latest/reference/services/s3/client/generate_presigned_url.html) and [boto3 signing guide](https://github.com/boto/boto3/blob/develop/docs/source/guide/s3-presigned-urls.rst) | `ExpiresIn` uses seconds; a supported API name is not proof of equivalent signing behavior. Context7 `/boto/boto3` queried. |
| DOC-03 | [S3 object metadata](https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingMetadata.html) | ETag is not universally content MD5; user-metadata key handling needs explicit consideration. Not a measured claim about the reference workload. |
| DOC-04 | [LibCST metadata](https://github.com/instagram/libcst/blob/main/docs/source/metadata.rst) | Qualified names are possible names, with source-position metadata available. Context7 `/instagram/libcst` queried; dynamic behavior remains unresolved. |

These references inform requirements; they are not populated Spec 1 evidence records or passing pack tests. All evidence statuses below remain `MISSING`.

## Unresolved Decisions

| ID | Status | Required decision / closure evidence | Blocks |
|---|---|---|---|
| UD-1 | UNRESOLVED — highest priority | Select and document the reference application's immutable identity, license permission, actual functionality, timed inventory review (<60 minutes), ≥3 operation types and holdout provenance. Resolves Spec 1 UD-1. If no suitable public application exists, explicitly authorize creation and later commit of a purpose-built, non-toy reference application. None is selected or created here. | Scope, operation contracts, recipes and workload-dependent criteria (MISSING). |
| UD-2 | UNRESOLVED | After workload inspection, approve exact oss2/boto3 compatibility constraints, resolved toolchain/dependency versions and behavior evidence. No unbounded or unspecified compatibility advertisement. | SDK-specific analysis, mapping, manifest and tests (MISSING). |
| UD-3 | UNRESOLVED | Independent review of corpus composition, representativeness, annotations, obligation criticality, eligibility, budgets and acceptance rules before use. | Discovery measurements, comparison and qualification (MISSING). |
| UD-4 | UNRESOLVED | Decide whether any model is used, for which bounded cases, training/example exclusion evidence, allowed inputs, comparator and measurable benefit rule. Model-free mode requires no model training claim; public availability is NOT proof of holdout for a model. | AI-enabled profile; deterministic path remains mandatory (MISSING). |
| UD-5 | UNRESOLVED | Designate real-service accounts/environment identities, authorized synthetic resources, scoped access references, cleanup/budget ownership and permission-denial test plan. Record references, NEVER secret values. | AC-04 real-service and final qualification (MISSING). |

Repository-lineage separation can be audited; exclusion from an opaque model's training data usually cannot. If it cannot be established, the AI-enabled profile is ineligible under this brief, not silently labelled held out. If the purpose-built fallback cannot meet the non-toy or holdout constraints, UD-1 remains open. A bounded selection attempt cannot prove no suitable public application exists; its search scope and reviewer decision must be recorded.

## Measurement Definitions

These definitions are normative for REQ-002 and REQ-009. Report per pack/version, frozen corpus, environment and approach, with named strata (ordinary eligible tasks, unsupported cases, unsafe mutants). Do not pool unlike strata or claim a population-wide success rate.

| Measurement | Numerator | Denominator / unit |
|---|---|---|
| Correctly accepted transformation rate | Eligible tasks whose designated output is accepted and independently judged to satisfy every declared obligation | All predeclared eligible tasks, including failed, blocked, abandoned and no-output tasks. One task contributes at most one numerator count. |
| False acceptance among accepted outputs | Accepted output identities independently found to violate at least one required obligation | All accepted output identities for that approach/corpus, including earlier accepted outputs later repaired or revoked. An output identity is task + approach + candidate digest + frozen acceptance context. |
| Unsafe-candidate escape rate | Known-unsafe candidate identities accepted by the verifier | All known-unsafe candidate identities actually submitted to the frozen verifier. Untested planned mutants are reported separately and block completion. |
| Discovery precision / recall | True-positive annotated dependency/pattern identities | Precision denominator: all reported positive identities (TP+FP). Recall denominator: all annotated actual identities (TP+FN). Match/dedup rules are frozen before evaluation; unresolved predictions remain visible and are not assumed TP. |

Task eligibility, designated-output selection (for example, first output within a frozen attempt budget), equivalence/dedup rules, run horizon, independent adjudicator, criticality and uncertainty policy are frozen BEFORE generating comparison outputs. The correctly accepted numerator requires both acceptance and independent correctness; correct-but-rejected outputs are separately counted, not called accepted. Retrying a digest cannot enlarge a denominator; a changed digest is a new output and never erases earlier acceptance. Frozen acceptance context means baseline/configuration/dependency/toolchain/pack/contract/suite/oracle/policy/rule/environment-profile identities; it excludes run/attempt identifiers, observation timestamps and per-attempt synthetic resource names. Full execution bindings still retain those excluded fields for audit. Repeated trials belong to separately declared trial strata, not hidden duplicates.

Every rate includes numerator, denominator, percentage (only when defined), unadjudicated count, exclusions with reasons, planned-but-untested count and limitations. A zero denominator is `N/A`, never 0% or 100%. Pending independent judgments make reported values provisional; false-acceptance reporting includes the possible range from confirmed violations through confirmed violations plus unadjudicated acceptances. Missing judgments or tests cannot produce qualification. A discovered corpus/oracle defect freezes the affected evaluation, preserves the old results and requires a new reviewed corpus/rule version; exclusions cannot be rewritten after outcomes are known to improve the score.

No numerical productivity or correctness threshold is invented here. Mandatory AC gates and zero observed critical false acceptances are necessary, not proof that no defects exist. UD-3/UD-4 must define any additional thresholds before comparison. Effort accounting includes setup, generation, review, diagnosis, repair and unsuccessful attempts; machine/tool usage and human time are reported separately.

## Requirements

### Requirement 1: Reference Workload and Scope Declaration

**ID:** REQ-001 — AC-01, AC-10. **User Story:** As a pack maintainer, I want a reviewable reference scope so that support claims have a checkable boundary.

#### Acceptance Criteria

1. WHEN a workload is proposed THE Pack SHALL record its immutable source identity, fixture-use license permission, non-toy functionality, actual OSS usage and complete inventory review duration of less than 60 minutes by one named reviewer.
   Verification: inspect selection record and timed review transcript. Evidence: `MISSING` (UD-1).
2. IF a proposed workload contains fewer than three distinct OSS operation types THEN THE Pack SHALL reject it as the reference workload.
   Verification: boundary fixtures with two and three operation types. Evidence: `MISSING` (UD-1).
3. WHEN support is declared THE Pack SHALL bind every supported operation and call pattern to the selected workload identity and exact approved source/target SDK version constraints.
   Verification: compare workload inventory, resolved versions and manifest declarations. Evidence: `MISSING` (UD-1/2).
4. IF no suitable public workload is selected THEN THE Pack SHALL retain UD-1 as unresolved pending an approved purpose-built non-toy application's creation, fixture license review and repository commit.
   Verification: missing-selection case cannot progress to qualification. Evidence: `MISSING` (UD-1).
5. WHERE a model is used THE Pack SHALL require evidence that the reference workload was excluded from that model's training and example data before qualifying that profile.
   Verification: provenance review including an opaque-training/unknown case that blocks qualification. Evidence: `MISSING` (UD-1/4).
6. WHEN scope is presented THE Pack SHALL limit its coverage claim to the declared reference workload, operations, call patterns, versions and tested environment.
   Verification: inspect manifest, report and reproduction instructions for identical boundaries. Evidence: `MISSING` (UD-1/2/5).

### Requirement 2: Source Baseline Analysis

**ID:** REQ-002 — AC-02, AC-03. **User Story:** As a pack user, I want an inspectable source inventory so that hidden dependencies cannot be mistaken for migrated behavior.

#### Acceptance Criteria

1. WHEN analysis begins THE Pack SHALL identify an immutable baseline containing the declared Python source/configuration tree, resolved dependency inventory and analysis-tool identity.
   Verification: change source/config/dependency/tool identity independently and detect a changed baseline. Evidence: `MISSING` (UD-1/2).
2. WHEN static analysis finishes THE Pack SHALL emit a complete declared-tree inventory with file/span, possible OSS target, wrapper chain, configuration/resource dependencies, classification and unresolved-reference entries.
   Verification: independently annotated imports, aliases, wrappers and configuration fixtures. Evidence: `MISSING` (UD-1/3).
3. IF a target or dependency cannot be established statically THEN THE Pack SHALL record it as unknown with source location and a reason instead of silently treating it as unrelated or supported.
   Verification: dynamic import, monkey-patching, reflective invocation and ambiguous alias fixtures. Evidence: `MISSING` (UD-3).
4. WHEN critical dependency and dangerous unsupported-pattern cases are evaluated THE Pack SHALL identify every predeclared critical annotation in the qualification corpus.
   Verification: frozen critical annotation set has zero missed identities. Evidence: `MISSING` (UD-3).
5. WHEN discovery quality is reported THE Pack SHALL publish precision, recall, raw counts and unresolved references for the entire annotated corpus using the Measurement Definitions.
   Verification: hand-calculated TP/FP/FN/unknown and zero-denominator fixtures. Evidence: `MISSING` (UD-3).
6. WHILE analyzing source THE Pack SHALL NEVER import or execute application code outside the Spec 2 execution boundary.
   Verification: import-side-effect fixture and denied-execution observer at the integration boundary. Evidence: `MISSING` (BG-03).
7. IF an unresolved target or dependency could affect declared migration behavior THEN THE Pack SHALL block candidate acceptance until independently reviewed evidence resolves it as covered or outside that behavior.
   Verification: ambiguous dynamic OSS call cannot be ignored merely because it lacks a mandatory-obligation label. Evidence: `MISSING` (UD-1/3).

### Requirement 3: Behavioral Contract Population

**ID:** REQ-003 — AC-02. **User Story:** As an evaluator, I want concrete operation obligations so that API-shaped rewrites are not confused with behavioral preservation.

#### Acceptance Criteria

1. WHEN an operation enters scope THE Pack SHALL provide its completed Spec 1 contract covering call/wrapper identity, inputs, outputs, side effects, resources, permissions, configuration, ordering, retry and failure behavior.
   Verification: required-dimension checklist and upstream structural/semantic validation. Evidence: `MISSING` (UD-1/2, BG-01).
2. WHEN an obligation is declared THE Pack SHALL attach at least one evidence record with observed/inferred/unknown provenance and a per-obligation fidelity classification from Spec 1's enumeration.
   Verification: remove each evidence link/classification and assert failure. Evidence: `MISSING` (BG-01).
3. WHEN a mandatory obligation is admitted for acceptance THE Pack SHALL link it to source evidence, an independently approved expected outcome and an identified verification method.
   Verification: traceability audit detects missing or generator-controlled oracle. Evidence: `MISSING` (UD-3).
4. WHEN a known source/target semantic difference is identified THE Pack SHALL record a `behavioral_notes` entry with its concrete effect, evidence reference and required adaptation or declared limitation.
   Verification: known-difference fixtures cannot omit notes or evidence. Evidence: `MISSING` (UD-1/2).
5. WHERE an operation exposes or relies on ETag integrity behavior THE Pack SHALL explicitly document that S3 ETag is not universally a content MD5 checksum and verify the workload's integrity obligation independently of that assumption.
   Verification: critical ETag-as-MD5 mutant and applicable object/encryption/upload-mode cases. Evidence: `MISSING` (UD-1/2/3).
6. WHERE presigned URLs are in scope THE Pack SHALL document version-specific units, precision/range, wrapper conversions and credential-lifetime behavior using primary evidence rather than assuming oss2 expiry is milliseconds.
   Verification: approved expiry boundaries, conversion mutant and expired/denied access tests. Evidence: `MISSING` (UD-1/2/5).
7. WHERE object metadata is in scope THE Pack SHALL record known key/value normalization, encoding and retrieval differences as evidenced notes and unknown differences as unresolved assumptions.
   Verification: case/encoding/round-trip fixtures for the selected workload's metadata obligations. Evidence: `MISSING` (UD-1/2).
8. IF a mandatory obligation has unknown fidelity or unresolved behavior needed for acceptance THEN THE Pack SHALL block acceptance even when its contract is schema-valid.
   Verification: structurally valid contract with unresolved mandatory behavior. Evidence: `MISSING` (BG-01).

### Requirement 4: Deterministic Transformation

**ID:** REQ-004 — AC-01, AC-04, AC-10. **User Story:** As a pack user, I want rule-based proposals so that the reference migration is reproducible without AI.

#### Acceptance Criteria

1. WHEN the deterministic path runs THE Pack SHALL produce its proposal without any model call or model-produced runtime decision.
   Verification: run with model transport unavailable and observe zero model requests. Evidence: `MISSING` (UD-1/2, BG-03).
2. WHEN identical baseline, recipe, contract, scope and toolchain identities are supplied THE Pack SHALL produce byte-identical candidate source/configuration and diff artifacts.
   Verification: two clean executions compare digests; timestamps/run identifiers remain outside candidate/diff bytes. Evidence: `MISSING` (UD-1/2).
3. WHEN a recipe transforms a supported call pattern THE Pack SHALL associate every code/configuration edit with its source location, recipe identity and affected obligation identifiers.
   Verification: approved positive/negative transformation fixtures and edit-to-obligation trace checks. Evidence: `MISSING` (UD-1/2).
4. WHEN generating a candidate THE Pack SHALL preserve the original baseline without in-place source or configuration modification.
   Verification: source digests unchanged after success, cancellation and transformation failure. Evidence: `MISSING` (BG-03).
5. IF the actual baseline or declared compatibility differs from the admitted identities THEN THE Pack SHALL refuse the transformation before applying edits.
   Verification: stale source, lockfile and incompatible SDK fixtures. Evidence: `MISSING` (UD-2).
6. IF the candidate still contains an unaccounted mandatory OSS dependency or requires an unsupported pattern THEN THE Pack SHALL expose a non-accepted partial proposal with the unresolved inventory entries.
   Verification: mixed supported/unsupported application retains explicit gap and no accepted status. Evidence: `MISSING` (UD-1/3).

### Requirement 5: Optional AI Assistance

**ID:** REQ-005 — AC-04, AC-10. **User Story:** As a maintainer, I want evidence-gated optional AI so that cost and uncertainty are justified rather than assumed useful.

#### Acceptance Criteria

1. IF UD-4 is unresolved or AI is disabled THEN THE Pack SHALL keep model-assisted execution unavailable while preserving the deterministic path.
   Verification: unresolved/disabled profiles trigger no model requests. Evidence: `MISSING` (UD-4).
2. WHERE AI assistance is proposed THE Pack SHALL require an approved bounded case set, model/configuration identity, holdout evidence, allowed data scope and predeclared benefit rule before comparison.
   Verification: omit each approval field and block comparison. Evidence: `MISSING` (UD-4).
3. WHEN AI benefit is evaluated THE Pack SHALL compare against the deterministic baseline under identical frozen eligibility, obligations, acceptance rules and effort accounting.
   Verification: mismatched-corpus/rule/budget comparison is invalid, with raw totals preserved. Evidence: `MISSING` (UD-3/4).
4. IF the approved comparison shows no measurable benefit THEN THE Pack SHALL exclude AI from the qualified profile.
   Verification: zero/negative-benefit and uncertain-result cases cannot justify enabling AI. Evidence: `MISSING` (UD-4).
5. WHERE a model proposes a patch THE Pack SHALL treat it as an untrusted candidate with no authority to change the independent oracle, acceptance rules, credentials or verdict.
   Verification: oracle-edit and forged-result adversarial cases through Spec 2. Evidence: `MISSING` (BG-03).
6. WHERE external model processing is enabled THE Pack SHALL use only Spec 2's approved egress/data policy without transmitting credentials or silently falling back to another provider.
   Verification: disallowed-data/provider and credential-canary denial tests. Evidence: `MISSING` (UD-4, BG-03).

### Requirement 6: Independently Controlled Verifier Integration

**ID:** REQ-006 — AC-02, AC-04, AC-05. **User Story:** As an evaluator, I want a verifier outside the patch path so that self-authored tests cannot authorize their own patch.

#### Acceptance Criteria

1. WHEN verification is prepared THE Pack SHALL bind a suite and oracle assets authored outside the patch-writing path, with recorded origin/review provenance and independent control, to the exact baseline, candidate, contract, rules and environment identities.
   Verification: identity substitution, patch-writer-controlled and patch-writer-generated-then-approved suite fixtures are refused. Evidence: `MISSING` (UD-3, BG-03).
2. WHEN a candidate is accepted THE Pack SHALL require passing results for every mandatory functional, permission-denial and failure-behavior test under the frozen acceptance rules.
   Verification: fail, skip, absent and interrupted required tests each prevent acceptance. Evidence: `MISSING` (UD-3/5).
3. IF only local mocks, emulators or offline fixtures ran THEN THE Pack SHALL label their evidence local-only rather than integration-verified against real services.
   Verification: local-only report cannot satisfy AC-04's real-service gate. Evidence: `MISSING` (UD-5).
4. WHEN a real-service integration claim is made THE Pack SHALL include observed evidence from the declared source and target environments, scoped test-resource identities and authorized access references.
   Verification: independent review of real OSS/S3 receipts, denial outcomes and cleanup evidence without secret values. Evidence: `MISSING` (UD-5).
5. WHEN a critical seeded defect is submitted THE Pack SHALL reject or block the candidate without recording acceptance.
   Verification: every frozen critical mutant, including integrity, expiry, metadata, missing-denial and oracle-tampering cases when applicable. Evidence: `MISSING` (UD-3).
6. WHEN verification terminates with intact trusted result/audit linkage THE Pack SHALL produce an upstream-compatible completed `BehavioralResult` without interpreting evidence completeness or worker exit success as acceptance.
   Verification: upstream conformance fixtures for complete, failed and blocked outputs; missing/forged linkage follows Requirement 10 instead. Evidence: `MISSING` (BG-01/03).
7. IF any critical false acceptance is observed THEN THE Pack SHALL mark that pack/version unqualified regardless of later repair or aggregate success rate.
   Verification: historical critical false acceptance survives repair and invalidates qualification; a revised version requires fresh evaluation. Evidence: `MISSING` (UD-3).
8. WHEN verifier integrity is qualified THE Pack SHALL demonstrate acceptance of an independently confirmed valid positive control as well as rejection or blocking of every critical negative control.
   Verification: a reject-all verifier fails the positive control even with zero unsafe escapes. Evidence: `MISSING` (UD-3, BG-03).
9. WHEN the pack/version is qualified THE Pack SHALL demonstrate an accepted deterministic pack-produced transformation of the complete declared reference workload with every mandatory obligation verified, including declared real-service evidence.
   Verification: an unrelated positive control, all-abstention run or locally passing partial reference transformation cannot qualify the pack. Evidence: `MISSING` (UD-1/2/3/5).

### Requirement 7: Evidence Bundle Export

**ID:** REQ-007 — AC-02, AC-10. **User Story:** As a pack user, I want a portable evidence bundle so that I can review what changed and what remains unproven.

#### Acceptance Criteria

1. WHEN exporting a run THE Pack SHALL include baseline/candidate identities, diff, analysis findings, obligation/evidence links, test identities/outcomes, measurements, limitations and unresolved gaps.
   Verification: bundle inventory and cross-reference validation for success and non-success runs. Evidence: `MISSING` (BG-01/03).
2. WHEN an artifact is linked in a bundle THE Pack SHALL bind its immutable identity to the run/attempt and audit references supplied by Spec 2.
   Verification: swap same-named artifacts between attempts and reject linkage. Evidence: `MISSING` (BG-03).
3. WHEN preparing human-readable findings THE Pack SHALL distinguish observed facts, inferences and unknowns using their evidence provenance.
   Verification: inferred/unknown fixtures remain labelled after export. Evidence: `MISSING` (BG-01).
4. WHEN exporting artifacts THE Pack SHALL exclude credential values, usable signed URLs, secret-bearing configuration and unauthorized source data under the declared export policy.
   Verification: canary scanning and disallowed-artifact tests; denied export leaves no distributable bundle. Evidence: `MISSING` (UD-5, BG-03).
5. IF required artifacts are missing, corrupt or identity-mismatched THEN THE Pack SHALL label the export incomplete and non-qualifying rather than silently dropping the gaps.
   Verification: delete/alter each mandatory artifact and inspect status. Evidence: `MISSING` (BG-01/03).
6. WHEN another engineer reproduces qualification THE Pack SHALL provide versioned instructions and non-secret inputs that reproduce the declared workflow, verdicts and measurement counts within the documented environment constraints.
   Verification: independent engineer run with identities and outcome comparison; signed URL bytes and service timestamps are not reproducibility targets. Evidence: `MISSING` (UD-1/2/3/5).

### Requirement 8: Pack Manifest

**ID:** REQ-008 — AC-01, AC-02. **User Story:** As a framework maintainer, I want a valid manifest so that applicability and contract references use the shared validation path.

#### Acceptance Criteria

1. WHEN the pack manifest is prepared THE Pack SHALL populate Spec 1 Requirement 5's TOML format with pack identity/version, source/target constraints, operation entries, contract references, schema/toolchain declarations, limitations and namespaced extensions only under `[x]`.
   Verification: upstream positive and malformed-manifest conformance fixtures. Evidence: `MISSING` (UD-1/2, BG-01).
2. WHEN loading the pack THE Pack SHALL validate the manifest and referenced documents through Spec 1 structural/semantic validation and offline bundled-catalog resolution.
   Verification: offline catalog success plus unresolved schema/contract reference failures. Evidence: `MISSING` (BG-01).
3. IF an analyzed operation, version or call pattern falls outside declared coverage THEN THE Pack SHALL obtain an explicit `unsupported` applicability result through the upstream-owned manifest validation contract.
   Verification: undeclared operation, mismatched version and unsupported pattern integration fixtures. Evidence: `MISSING` (BG-02).
4. WHEN support fidelity is declared THE Pack SHALL keep Spec 1 manifest labels (`full`, `partial`, `unsupported`) distinct from per-obligation fidelity and candidate acceptance.
   Verification: partial-support manifest cannot accept a candidate with an unmet mandatory obligation. Evidence: `MISSING` (BG-01/02).
5. IF a required validation/applicability API or result mapping is absent or incompatible THEN THE Pack SHALL block integration with an upstream gap record instead of implementing a substitute validation authority.
   Verification: missing-API/result-field fixture and owning-spec issue linkage. Evidence: `MISSING` (BG-01/02/03).

### Requirement 9: Qualification Corpus and Measurements

**ID:** REQ-009 — AC-03, AC-04, AC-05, AC-10. **User Story:** As an auditor, I want reproducible denominators and independent judgments so that reported success cannot hide unsafe acceptance.

#### Acceptance Criteria

1. WHEN an approach comparison is requested THE Pack SHALL freeze corpus identities, eligibility, obligations, criticality, acceptance rules, output-selection/dedup rules, attempt budgets and independent adjudication ownership before generating comparison outputs.
   Verification: freeze precedes first comparison output; later mutations invalidate the comparison. Evidence: `MISSING` (UD-3/4).
2. WHEN assembling the corpus THE Pack SHALL record independent representativeness review and repository-lineage separation between development examples and evaluation workloads.
   Verification: lineage leak and missing-review cases block qualification. Evidence: `MISSING` (UD-1/3).
3. WHEN reporting qualification THE Pack SHALL compute all three required rates with the units, denominators, zero-denominator and pending-adjudication behavior in Measurement Definitions.
   Verification: hand-computed datasets with failures, no output, duplicate attempts, repaired acceptances, unsafe mutants and unknown judgments. Evidence: `MISSING` (UD-3).
4. WHEN attempts are reviewed or repaired THE Pack SHALL retain their original acceptance events and charge setup, generation, review, diagnosis, repair and unsuccessful effort to the declared approach.
   Verification: append-only history reconstruction reproduces totals after a repair. Evidence: `MISSING` (UD-3/4).
5. WHEN measurements are published THE Pack SHALL include raw counts, exclusions, unknowns, untested cases, uncertainty and corpus-specific limitations alongside any percentages.
   Verification: report missing each disclosure fails qualification reporting. Evidence: `MISSING` (UD-3).
6. IF an evaluation changes eligibility, oracle or acceptance rules after observing outcomes THEN THE Pack SHALL require a new reviewed evaluation version while preserving the invalidated version's history.
   Verification: post-result exclusion/oracle edit cannot improve the original score. Evidence: `MISSING` (UD-3).
7. WHEN a pack, recipe, dependency/SDK, contract, suite, rule or relevant environment identity changes THE Pack SHALL invalidate reuse of qualification outside the unchanged declared scope pending requalification.
   Verification: identity-change matrix blocks stale qualification. Evidence: `MISSING` (UD-2/3/5).
8. WHILE comparison outputs are generated THE Pack SHALL keep evaluation expected outcomes, annotations and mutant answers outside the patch-generating path's readable and writable inputs.
   Verification: input-manifest audit and attempted oracle access are denied; accidental exposure invalidates the affected comparison. Evidence: `MISSING` (UD-3/4, BG-03).

### Requirement 10: Failure and Unsupported Behavior

**ID:** REQ-010 — AC-01, AC-02, AC-04, AC-05. **User Story:** As a pack user, I want explicit non-success outcomes so that partial or unavailable evidence never looks like a completed migration.

#### Acceptance Criteria

1. IF applicability is unsupported THEN THE Pack SHALL report the operation/pattern, source location and declared scope without applying an undocumented fallback.
   Verification: unsupported inputs retain diagnostic context and produce no accepted output. Evidence: `MISSING` (BG-02).
2. IF mandatory evidence, approvals, upstream interfaces or real-service qualification inputs are missing THEN THE Pack SHALL report a blocked outcome with the missing dependency and next required action.
   Verification: one missing prerequisite per gate; none silently defaults to success. Evidence: `MISSING` (UD-1/2/3/4/5, BG-01/02/03).
3. IF transformation is partial or verification fails THEN THE Pack SHALL preserve the reviewable proposal and limitations as non-accepted output subject to export policy.
   Verification: partially rewritten source and failing test cannot yield acceptance. Evidence: `MISSING` (BG-03).
4. IF execution is cancelled, exceeds the approved Spec 2 budget or loses trusted audit/result linkage THEN THE Pack SHALL terminate the attempt without acceptance under Spec 2 cleanup and recovery rules.
   Verification: cancellation, timeout and missing-audit integration faults. Evidence: `MISSING` (BG-03).
5. WHILE reporting or retrying a run THE Pack SHALL NEVER promote unsupported, blocked, rejected, stale or incomplete evidence to accepted based solely on retry success, model text, structural validity or worker status.
   Verification: forged-success and stale-replay cases retain their non-acceptance. Evidence: `MISSING` (BG-01/02/03).
6. WHEN retry or repair is explicitly authorized THE Pack SHALL create a new linked attempt using Spec 2's remaining shared budget without mutating prior terminal history.
   Verification: new attempt identity, cumulative budget and preserved failed/accepted history. Evidence: `MISSING` (BG-03).

## Acceptance Gate Traceability

| Gate | Requirements / minimum qualifying evidence | Current status |
|---|---|---|
| AC-01 Declared support | REQ-001, REQ-004, REQ-008, REQ-010; exact scope/versions plus tested explicit unsupported results | MISSING |
| AC-02 Traceable behavior | REQ-002, REQ-003, REQ-006, REQ-007, REQ-008, REQ-010; every mandatory obligation → source evidence → approved expected outcome → verification | MISSING |
| AC-03 Measured discovery | REQ-002, REQ-009; all frozen critical annotations detected; whole-corpus precision/recall/counts/unknowns | MISSING |
| AC-04 Behavioral verification | REQ-004, REQ-005, REQ-006, REQ-009, REQ-010; mandatory functional/denial/failure tests and declared real-service evidence | MISSING |
| AC-05 Verifier integrity | REQ-006, REQ-009, REQ-010; critical mutants rejected/blocked, no observed critical false acceptance | MISSING |
| AC-10 Reproducibility and utility | REQ-001, REQ-004, REQ-005, REQ-007, REQ-009; independent engineer reproduces bounded results; AI benefit evidenced if enabled | MISSING |

REQ-001 through REQ-010 each have evidence status **MISSING**. Closing a documentation review does not close UD/BG gates, select values, or authorize execution. `analysis.md` records the adversarial gate; downstream design/tasks are conditional until the decisions and upstream evidence exist.
