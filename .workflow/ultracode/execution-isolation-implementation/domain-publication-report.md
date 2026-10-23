# Domain organization and shared AI rules

## Outcome
The contracts crate now groups behavioral-contract, evidence and verification declarations under private `src/documents/`, with genuinely shared reference/raw-serialization support there. Existing `manifest/`, `schema/` and `errors/` remain cohesive; `validator/` is the compatibility facade. No public imports or DTO/serialization behavior changed. This is domain-oriented organization, not an invented aggregate/repository framework. Physical scope is the stated narrow crate-only default; unrelated specifications/research were not relocated.

Canonical rule: `.ai/rules/engineering.md`. Six relative Git symlinks expose it as `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, `.kiro/steering/engineering.md` and `.cursor/rules/engineering.mdc`. All links resolve locally and are recorded with Git mode120000. Current client-path conventions were checked against official docs/Context7; no native AI-client activation or enforcement is claimed. No global agent configuration changed.

## Verification
94 Rust integration tests plus6 compile-fail doctests pass. All six conversion failures are E0277, not broken imports. Positive type-identity tests pass before/after the move across every existing export facade.11 Python checks pass (7 scaffold,2 rule,2 domain-layout); frozen check, fmt, Clippy with warnings denied and staged whitespace checks pass. See results/domain-final-check-results.json. The first staged audit caught whitespace in command logs; text presentation was normalized, with raw copies/hashes retained in results/evidence-text-normalization.json. No test results or source/value bytes were changed.

LLVM tools match Rust1.99.1/LLVM23.1.1. Emitted production mappings:198/204 lines (97.06%),19/22 functions (86.36%),262/276 regions (94.93%). Tests/dependencies excluded; unmapped derive-only DTOs are not assigned a percentage, and no branch/full-spec/native coverage is claimed. Reproduce with `rtk proxy python3 .workflow/ultracode/execution-isolation-implementation/measure-domain-coverage.py` using the documented isolated cache/tools.

## Team
Latest request used packet11 read-only layout mapping and packet12 static integration review (PASS, no required findings). Earlier packet09 implemented manifest types; packet10 found schema gaps repaired only after explicit additional approval.12 cumulative sidecars, zero recursive/followup turns. Generic inherited model/effort unverified; task restrictions are behavioral, not distinct sandbox isolation. Parent independently ran combined checks. Review did not execute tests or attest native behavior.

## Commits and publication
User explicitly authorized commit/push. Code commit `fd8400a6ea50746bb72701f2db4c66191d35a934`, AI-rule commit `3926eaca88ea8862743194a214e7da681ccce3b9`; workflow/approval status evidence is a separate final commit. Configured SSH signing was retained and signature blocks exist; signer identity verification is UNVERIFIED because no allowedSignersFile is configured. Do not confuse signed bytes with independently verified owner identity. Historical RED/GREEN logs predate commit authorization and are not retroactive checkpoint commits.

Normal push targets only `origin/feat/execution-isolation`, never force/main/master. The final push and independent remote-SHA comparison occur after this report's evidence commit; the conversation and `/tmp/clouddialect-publication-receipt.json` hold the live receipt, avoiding an endlessly self-changing commit hash. Research files remain unchanged/untracked and excluded.

## Remaining scope
Original execution-isolation objective remains incomplete. Requirement2.2 needs a precise per-resource behavior/assumption model; full validators, snapshot/digest/version helpers and exhaustive conformance/handoff remain unfinished. BG-01 and UD1–UD5 remain OPEN; no native probes, deployment or releases were performed or authorized. See prerequisite-catalog-report.md for prior implementation evidence and explicit limitations.
