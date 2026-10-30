# Requirements Analysis: python-oss-to-s3-pack

## Method and Gate

**Historical review: 2026-10-22.** The findings and source lines below describe
that snapshot. The [2026-10-30 admission refresh](../../../.workflow/ultracode/python-oss-to-s3-pack/results/admission-2026-10-30.md)
supersedes its upstream-absence findings, not its qualification safeguards:
Spec 1 validators and result/binding declarations now exist. In particular,
F-02's stub diagnosis and F-04's absent `test_results` representation are no
longer current. Inventory-aware applicability, authenticated Spec 2 execution,
owner handoff and the five pack decisions remain unresolved. The actual fifth
catalog schema is extension registry, not the separately validated TOML pack
manifest. Requirements/design now reflect this delivered boundary. No pack
criterion, UD/BG or leaf task is closed by the refresh.

Manual adversarial analysis following `analyze-requirements`, preceded by the requirements-engineering structural validator. Two bounded read-only native agents reviewed upstream/evaluation boundaries and requirement counterexamples; the parent verified and integrated findings. Native Kiro Analyze Requirements and SMT were **not run**. No claim of formal consistency/completeness is made.

Pre-analysis: canonical headings, ten numbered groups, user stories, glossary, named outcomes, NEVER clauses, 67 atomic-SHALL criteria, verification/evidence lines and five explicit open decisions. The structural validator passes 67 criteria with zero warnings. Manual readiness dimensions pass for structure, role/scope, quantified boundaries, named failure paths, evidence provenance, measurement definitions, traceability and source attribution. Implementability and runtime evidence do not pass: UD/BG prerequisites are unresolved. This is sufficient for **conditional documentation only**, not the ordinary implementation-readiness gate.

**Disposition: proceed to conditional design and gated tasks.** No unresolved contradiction is knowingly hidden; the open choices and unavailable upstream contracts remain explicit blockers. All requirement evidence remains `MISSING`.

## Findings and Resolution

| ID / class | Counterexample or source conflict | Resolution / requirement |
|---|---|---|
| F-01 Conflict: expiry units | Blindly divide oss2 expiry by 1000 because the brief says milliseconds; emitted S3 URLs then expire prematurely. | Primary oss2 source and boto3 API both document seconds (DOC-01/02). Corrected premise; REQ-003.6 requires exact-version/wrapper boundary evidence rather than inventing a universal limitation. |
| F-02 Completeness: absent foundation | Treat proposed crate types or Spec 2 IPC as delivered and import imaginary APIs. | Actual `src/lib.rs` and module stubs contain no validators. BG-01/BG-03 block integration; REQ-008.5 prohibits substitutes. |
| F-03 Completeness: applicability input | A manifest-only validator passes while source contains an operation omitted from that manifest. | BG-02 requires upstream inventory/pattern/version context and authoritative SDK constraint representation; REQ-008.3 tests both omitted and explicitly unsupported operations. |
| F-04 Conflict: result vocabulary | Treat `EvidenceComplete` as accepted, or silently omit design `test_results` because upstream tasks omitted it. | Reconcile upstream result fields and full baseline/configuration/oracle/policy/attempt bindings through BG-03. REQ-006.6 and REQ-010 preserve separate acceptance authority. |
| F-05 Accepted scenario: favourable denominators | Remove failed tasks, duplicate successful retries, omit unreviewed accepted outputs or erase an earlier false acceptance after repair. | Measurement Definitions and REQ-009 freeze units/rules, retain immutable history and disclose uncertainty; critical historical false acceptance disqualifies the version (REQ-006.7). |
| F-06 Ambiguity: heldout | Public availability or a newly copied fixture is labelled proof that no model trained on it. | UD-1/UD-4 distinguish model training exclusion from repository-lineage separation. Unknown training provenance blocks AI qualification; model-free path uses no model and needs no training claim. REQ-001.5, REQ-009.2. |
| F-07 Accepted scenario: reject-all | Reject every unsafe candidate and claim a perfect verifier without accepting anything valid. | Independent positive control required (REQ-006.8), in addition to frozen critical negatives; zero denominators are N/A. |
| F-08 Accepted scenario: laundered test origin | Patch writer generates tests, then transfers ownership to another person. | REQ-006.1 prohibits patch-path origin for suite/oracle assets and records authorship/review; transfer of control cannot cure prohibited origin. |
| F-09 Accepted scenario: unclassified dynamic call | Unknown reflective target never receives a mandatory label, so only known calls are checked. | REQ-002.7 blocks acceptance for potentially relevant unresolved behavior until independently discharged. This is stricter than merely exporting the unknown. |
| F-10 Accepted scenario: no working reference migration | An unrelated hand-written positive control passes while every pack-produced reference candidate fails. | REQ-006.9 requires accepted deterministic pack output for the complete declared reference workload, with real-service evidence. No general success percentage threshold invented. |
| F-11 Conflict: crash versus completed result | Require a completed result even when its trusted linkage is lost. | REQ-006.6 applies with intact result/audit linkage; REQ-010.4 handles loss without fabricating a completed accepted result. |
| F-12 Ambiguity: real versus purpose-built fixture | A toy demo is relabelled as a real application, or a bounded search is asserted to prove no suitable public app exists. | UD-1 preserves substantive functionality, licensing, timed review and independent selection evidence. Purpose-built creation/commit is a future explicit gate, not performed by this specs-only request. |
| F-13 Design loopback: circular AI admission | No provider calls until proven benefit, but no comparison can prove benefit without calls. | Final integrated review: DES-005 / ADR-004 and TASK-013 now distinguish explicitly approved bounded experiments from qualified-profile inclusion. REQ-005 unchanged. |
| F-14 Design loopback: inventory bootstrap | Scope requires analyzer inventory while analyzer requires approved scope. | Final integrated review: DES-001 consumes a distinct frozen manual SelectionInventory; core reconciles analyzer differences before transformation. TASK-006/015 test bootstrap and reconciliation; oracle answers stay protected. |
| F-15 Metric ambiguity: attempt context | Dedup key accidentally includes attempt ID, making identical retries inflate output denominators. | Parent final review: Measurement Definitions explicitly separate stable acceptance context from full audit binding; DES-009 and TASK-012 retain both identities. |

Final integrated review was one read-only pass followed by one consolidated parent repair and deterministic revalidation. Environment approval is an independent outer envelope, not a worker grant; verifier admission intersects it with actual frozen test needs and blocks incompatible scope. This preserves truthful parallelism without adding authority or closing UD-5.

## Historical Upstream Evidence Reviewed — 2026-10-22

| Source | Evidence / implication |
|---|---|
| `crates/clouddialect-contracts/src/lib.rs:3-9` and four module files | Scaffold, no implemented public validation surfaces. |
| Spec 1 `design.md:40,274-278,672-768,1410-1411` | Five planned schemas; proposed result and manifest interfaces. |
| Spec 1 `tasks.md:190-193,342-377` | Result field discrepancy; manifest operation representation and coverage checks need reconciliation. |
| Spec 1 `requirements.md:409,447-448` | Unknown extensions cannot determine acceptance; unsupported source operations must be identified. |
| Spec 2 `design.md:7`; `requirements.md:240-250,307-309` | Conditional execution design; binding, positive/negative controls and unresolved mechanism choices. |

Line references record the inspected checkout; upstream revision changes require re-review. No edits to Specs 1/2 were made. DOC-01 through DOC-04 primary links and Context7 provenance are in `requirements.md`.

## Adversarial Scenario Disposition

| Scenario | Required outcome |
|---|---|
| Two operation types; exactly 60-minute inventory review | Reject selection (≥3 types, strictly <60 minutes). |
| Unselected workload, missing SDK pins or unavailable upstream API | Block affected implementation/qualification; do not fill defaults. |
| Dynamic alias possibly reaches OSS; tests for known calls pass | Block until independent scope evidence resolves it. |
| Valid contract with unknown mandatory behavior | Block acceptance despite valid schema. |
| Unsupported operation absent from manifest | Explicit upstream unsupported applicability response; no silent no-op. |
| Verified local mock tests but no service receipts | Local-only evidence; cannot satisfy real-service qualification. |
| Generator-authored tests later independently approved | Reject as qualifying suite; independent origin required. |
| One critical unsafe candidate accepted then repaired | Preserve event and disqualify same pack/version. |
| All candidates rejected | Positive control and reference demonstration fail qualification. |
| No eligible tasks or no accepted outputs | N/A rates; missing qualifying reference evidence still blocks. |
| Accepted output awaiting adjudication | Provisional metric plus unknown count/range; no qualification. |
| Export contains live URL/token | Deny distributable export; retain only authorized diagnostic metadata. |
| Worker returns success after cancellation/audit loss | Non-accepted terminal attempt; no trusted result fabrication. |

## Deliberately Unresolved / Out of Scope

UD-1 workload (highest priority), UD-2 SDK versions, UD-3 corpus, UD-4 AI and UD-5 real-service environment remain unresolved. BG-01/02/03 require upstream owner decisions and executable compatibility evidence. Their decision owners and closure criteria are in requirements; downstream documentation may show proposed logical interfaces but cannot claim pins, schemas or protocols exist.

No generic migration success rate, measured productivity gain, real-service test result, sandbox guarantee or implementation evidence is claimed. Commercial licensing, runtime implementation, production data movement/cutover/deletion and publication remain outside this run.
