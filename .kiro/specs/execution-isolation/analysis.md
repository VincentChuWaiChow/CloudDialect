# Requirements Analysis: execution-isolation

## Method and Gate Status

Manual adversarial analysis using the analyze-requirements checklist and three bounded native read-only reviews: upstream compatibility, threat coverage, then requirements QA. **Native Kiro Analyze Requirements / neurosymbolic SMT execution: UNVERIFIED, not invoked.** This report makes no solver-completeness claim.

**Gate result: suitable for user-authorized conditional design and decision-gated tasks only.** Requirements are not implementation-ready: UD-1 through UD-5 and BG-01 remain open. The user explicitly approved preserving these choices on 2026-10-22. An unconditional requirements-to-design approval would be false; downstream artifacts must carry these blockers.

## Pre-Analysis Audit

- Canonical title, Introduction, Glossary, Requirements, numbered requirement headings, user stories, and acceptance sections: present.
- All ten requirement groups have verification methods and `MISSING` evidence status.
- Atomic EARS criteria, named semantic blocker categories, NEVER clauses, and configured finite boundaries: present.
- Numerical resource values intentionally absent: UD-5 requires measurements first. Testability is parameterized by approved profiles, not evidence of measured qualification.
- Six threat rows contain assets, attacker capability, entry points, trust boundaries, controls, tests, and residual risk.
- Local lexical checks initially passed 100 criteria before the three fixes below; final counts/results are in the run report. The stronger grader also rejects empty/missing artifacts.

## Findings and Disposition

| Finding | Class / evidence | Resolution | Status |
|---|---|---|---|
| F-01 | Wrong factual premise: brief describes Spec 1 types as consumed, but source only has module stubs. | Cross-spec table and BG-01 require real implemented APIs and compatibility tests. No speculative re-exports. | Documented prerequisite; implementation blocked. |
| F-02 | Conflict: Spec 1 design has BehavioralResult.test_results; task 9 omits it; evidence-completeness outcomes are not behavioral success. | 8.6, 8.7, 8.11 prohibit false acceptance and require upstream reconciliation. Execution blockers remain separate. | BG-01 open; no local schema change. |
| F-03 | Incompleteness: manifest capability representation, verification configuration/structured failure details, and catalog override integrity disagree or are absent upstream. | BG-01 includes all four compatibility surfaces; optional extensions never silently grant authority. | Upstream amendment/delivery gate. |
| F-04 | Accepted unsafe scenario: candidate-linked runner forges stdout or poisons imports despite read-only test files. | 1.8, 8.4, 8.13 separate execution from trusted control and require independent observation provenance; T-04 tests forgery. | Requirements amended; enforcement qualification MISSING. |
| F-05 | Completeness: reject-all implementation could pass defect-rejection gate vacuously. | 8.12 requires a valid positive candidate to complete acceptance alongside negative controls. | Resolved in requirements. |
| F-06 | Race: cancellation and acceptance can both appear successful without ordering. | 3.8, 10.9, 10.12, 10.13 specify a commit order and already-committed readback. | Resolved semantically; implementation MISSING. |
| F-07 | Ambiguity: candidate identity included future output while 1.1 required its digest before transformation. | Glossary splits admitted input from candidate output identity; 1.1 uses input and 1.10 binds outputs before verification. | Resolved by targeted amendment. |
| F-08 | Completeness: crash/disconnect did not explicitly start descendant termination, so cleanup deadline might never begin. | Added 1.11 for every attempt-ending event; 1.6/1.7 preserve reservations and quarantine uncertainty. | Resolved by targeted amendment. |
| F-09 | Completeness: 7.6 only applied when retention policy existed, permitting persistence with no policy. | Added 7.11 denying persistence and 7.12 blocking dependent execution when policy absent. | Resolved by targeted amendment. |
| F-10 | Potential impossible guarantee: a credential-readable worker can exfiltrate through an authorized channel. | Require no credentials in model/control evidence, narrow effective authority, external egress checks; explicitly retain credential-consumer risk under T-05 and UD-3. No universal anti-exfiltration claim. | Bounded guarantee, residual risk explicit. |
| F-11 | Completeness: worker-only offline mode leaves core model/DNS/resolver/audit egress open. | Glossary, 2.6/2.7, and T-06 cover run-serving dependencies with zero outbound traffic; missing offline services block. | Resolved in scope. |
| F-12 | Residual F-07 ambiguity found by integrated reviewer: 4.5 still required a candidate digest for every grant use, including pre-transformation operations. | Made 4.5 phase-specific: input/context for pre-candidate operations; frozen candidate/protected verification identity for candidate-specific operations and approvals. Propagated into AdmissionPolicy and task 5.1 tests. | Resolved by parent; no independent post-fix sign-off claimed. |

## Convergence and Open Gates

Parent reviewed and integrated all three requirements-QA corrections before writing design. The fourth reviewer supplied F-12, then its runtime handle was absent on continuation; no final independent verdict was available. The parent completed the integrated audit and reran deterministic checks after the scoped correction. No known unresolved logical contradiction is accepted as implementation-ready. The deliberately unresolved mechanisms and upstream contracts are blockers, not implicit defaults or guesses.

| Gate | Owner decision needed | Allowed next work |
|---|---|---|
| UD-1 | Security/operations: supported environment and isolation mechanism | Comparative decision packet and qualification design only. |
| UD-2 | Core/worker maintainers: exact IPC/version/framing/errors | Protocol decision packet only, no invented wire schema. |
| UD-3 | Security/operations: scoped credential delivery | Threat comparison and synthetic leak-test plan only. |
| UD-4 | Operations/security: persistence and data-class retention | Durability/restore/deletion decision packet only. |
| UD-5 | Core/operations: measured budget values/accounting | Reference measurement plan; no invented defaults. |
| BG-01 | Spec 1 owner: delivered compatible contracts and schemas | Upstream reconciliation proposal; no modifications in this run. |

## Evidence Limits

Manual review and document validators establish only draft quality and traceability. AC-05, AC-06, AC-08, platform support, restore, build provenance, and actual isolation remain `MISSING`. Approved conditional documentation does not authorize implementation, cloud access, deployment, or publication.
