# EVAL DEFINITION: execution-isolation-spec

Defined before specification edits, 2026-10-22.

## Assumptions
The user authorized conditional design/tasks without selecting five open mechanisms. No runtime exists. Requirement evidence remains MISSING even after this document passes checks.

## Capability evals
1. Requirements use canonical headings, sequential IDs and one SHALL per criterion; each requirement declares verification and MISSING evidence.
2. All ten groups, six complete threat rows, five unresolved decisions and three acceptance gates exist.
3. No invented sandbox selection, IPC wire schema, credential delivery, audit backend/retention duration, or resource-limit numbers.
4. Design elements trace to requirements; tasks trace to both; graph is acyclic and same-wave ownership disjoint.
5. Missing Spec 1 types, evidence-completeness/acceptance distinction, and native-analysis limitations are explicit.
6. Adversarial rubric: descendant escape, inherited capabilities, forged/stale evidence, policy widening, cross-project leakage, checkpoint failure, audit failure, cancellation/acceptance race, malicious signed packs.

## Regression evals
- Existing seven scaffold checks pass.
- Existing tracked files outside owned paths unchanged; research input hashes unchanged.
- License and Spec 1 documents unchanged; no runtime implementation or dependency changes.

## Graders
Inspected skill validation scripts; run-local structural/graph checker; bounded independent model reviews; Git diff and SHA-256 baseline. Human decisions remain required for open architecture gates. Do not report pass@k or pass^k from a single documentation check.

## Report
PASS for the conditional documentation scope. Four skill validators and the stronger run-local grader pass; nine grader sensitivity tests and seven existing scaffold tests pass. Baseline research hashes and unrelated tracked files are preserved. Exact commands/exits are in `check-results.json`; findings, approvals and limitations are in `final-report.md`.

Three independent reviews completed; a fourth supplied a partial finding, integrated by the parent before final checks. No final independent post-repair sign-off or native Kiro/SMT run is claimed. UD-1 through UD-5/BG-01 remain open, all requirement implementation evidence remains MISSING. Document checks do not prove AC-05, AC-06 or AC-08 implementation. Statistical pass@k/pass^k were not measured.
