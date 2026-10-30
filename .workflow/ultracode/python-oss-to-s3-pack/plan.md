# Python OSS-to-S3 pack specification

## Authorized workload construction — 2026-10-30

After the admission review, the user authorized a purpose-built Python
document-storage application in this repository, local code/tests only.
[Reference-workload eval contract](reference-workload-eval.md) defines this
continuation before implementation and supersedes the admission-only write
scope for the named new application. No cloud/commit/qualification authority
is added. Parent remains sole writer; two additional bounded worker turns
(public SDK lookup, independent Python review), no recursive delegation.

## Implementation admission continuation — 2026-10-30

The earlier sections below describe the 2026-10-22 documentation run, not the
current checkout. Current baseline: `56300de5dd7ab778909b41b02411ea8d087a28ad`,
branch `feat/execution-isolation-runtime`, initially clean tracked working tree.
The user now requests task implementation. Admission must honor the task graph;
missing workload/owner decisions and upstream contracts cannot be invented.

- Objective: verify the presently executable scope, correct stale upstream
  readiness statements and record exact remaining task prerequisites.
- Sole writer/integrator: parent. Owned paths: the four existing Spec 3 Markdown
  documents and this existing workflow root. No new competing run root.
- Forbidden: changes to Specs 1/2, runtime/code/schema/manifest/dependency/license
  files, credentials, qualification answers, fixtures, service/native/model
  requests, user agent infrastructure, commits, pushes or publication.
- Assumption to verify: missing approved workload/SDK/corpus/environment inputs
  continue to block dependent implementation. No optional AI profile is admitted.
- Team: `upstream_interfaces` (`rust_reviewer`) inspects actual Rust interfaces;
  `pack_admission` (`python_reviewer`) inspects decision artifacts/dependencies.
  Both are read-only packets, no recursive delegation. Role files and live
  catalog specify `gpt-6.1-sol/high`, read-only defaults; actual model execution
  and effective sandbox enforcement are not independently attested.
- Limits: two dispatches total, ten minutes per packet, one parent integration
  pass, at most one bounded repair; no further implementation/review wave.
- Risk: local documentation; no public API change. Stop downstream work if
  required approvals, interfaces or qualifying evidence are absent.
- Evals defined before edits: independently rerun selected offline catalog,
  schema, manifest and result tests (exit 0); run the existing `check(doc)`
  structural/dependency grader and its five negative controls (zero errors,
  each mutation detected); retain 67 MISSING criteria, five unresolved UDs,
  three open BGs, all 17 unchecked leaf tasks and the six-level/37-edge graph;
  inspect diff/HEAD/scope (only owned files, no whitespace errors).
- Historical `check_docs.py` entrypoint has obsolete snapshot-file/branch
  assertions. Do not edit that historical baseline or claim it passes; isolate
  its existing document-check function for the current document-only check.
- Evidence: append a dated admission report and state continuation. No TDD RED,
  coverage percentage, native conformance or owner gate closure is claimed for
  documentation-only changes. No checkpoint commits are authorized.

## Goal
Keep the specs-only branch appropriately named and produce Spec 3 requirements, adversarial analysis, conditional design and gated tasks under `.kiro/specs/python-oss-to-s3-pack/`.

## Success criteria
Canonical artifacts cover all ten requested groups, six acceptance gates, five open decisions, exact measurement denominators, evaluation integrity and upstream dependencies. Every requirement has verification and MISSING evidence. No fabricated workload, versions, corpus, environment or runtime evidence. SDK claims are checked against current primary documentation. Design/tasks are fully traceable with truthful prerequisites.

## Current context
Branch already `spec/framework-specs`, HEAD fb21cc6, tracking the same named remote ref; no rename needed. Its delta from master contains only four Spec 2 documents. Actual contracts crate remains stubs, and Spec 2 remains conditional. Pre-existing execution-isolation workflow directory and research inputs are not owned by this run.

## Constraints
Write only this run root and the new Spec 3 directory. No runtime implementation, fixture application, actual manifest/contract instances, dependency installation, cloud access, credentials, licensing changes, changes to Specs 1/2, commits, pushes or publication. Generic read-only Context7/primary-document requests allowed; no repository content sent externally.

## Risk level
Medium documentation change with security-sensitive future migration implications. Preserve unresolved decisions; production transfer/cutover/source deletion remain out of scope.

## Approval gates
The user requests the full documentation sequence and explicitly requires unresolved facts to remain unresolved. Conditional downstream artifacts follow that scope, not implementation approval. The user explicitly approved six-level leaf dependencies within three parent waves for Spec 3. Future workload creation, credentials, live-service tests and publishing require their own authorization.

## Mode
Delegated, parent sole writer. Native collaboration available; generic agents with inherited model/effort. Observed model/effort, authentication and client profile UNVERIFIED; no separate CLI session used to attest them. Read-only workers are behaviorally restricted, not claimed sandbox-isolated.

## Work packets
01 source/measurement boundary review; 02 requirements QA; 03 integrated design/task review. No recursive delegation. Parent retains drafting, primary-source verification and integration.

## Eval contract
- Outcome: source-grounded conditional spec, not a qualified migration pack.
- Shared surface: parent-owned spec text; all agents read-only.
- Checks: four skill validators, document/graph/coverage checks, existing scaffold tests and baseline/scope comparison.
- Blockers: unsourced SDK semantics, hidden upstream prerequisites, invented choices, biased denominators or unsupported security guarantees.
- Handoff: exact findings, citations, commands/exits and limitations.

## Integration policy
Separate user intent, current source, primary API facts and proposed designs. Correct disproven premises explicitly. One bounded parent repair cycle after integrated review; normal phase-gate drafting is not runtime implementation.

## Verification plan
Define graders before drafts. Verify ten numbered groups with atomic EARS; required sections; five UD rows; six AC gates; three metric formulae, zero-denominator and adjudication rules; REQ/DES/TASK links; graph cycles/ownership. Inspect unfamiliar validators before execution. Native Kiro/SMT and real-service qualification are not available in this docs run.

## Completion criteria
Requested branch state and artifacts verified, scoped checks pass, review findings resolved or explicitly decision-gated, unchanged existing work. No green document check may change requirement evidence from MISSING.
