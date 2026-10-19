# Integration

## Accepted
Task 1 explicit `errors/` path; no invented public types in empty stubs.
Read-only spec review findings, including the pre-submission contributor-policy gate.
Independent final review: PASS for scaffold source; no code defects identified.
## Rejected
Root workspace and future validator behavior.
## Conflicts
Design diagram uses `error/`, task 1 uses `errors/`; task 1 governs this scaffold.
## Decisions
Use stdlib Python scaffold graders because the Rust modules contain no executable behavior.
Registry-selected stable versions, except `toml =0.9.9` (last release tagged spec-1.0.0).
Cargo's exact version requirements ignore build metadata; the lockfile grader normalizes that metadata.
The dependency set is unchanged from the RED contract; this normalization does not weaken exact pins.
All downloaded direct dependency manifests corroborate the recorded registry license expressions.
Context7 resolved official Cargo docs and supplied manifest/locked-command guidance. Its `boon`
search returned an unrelated demo parser; its TOML version query found no documentation. Those
results were not used as authority; crates.io metadata and downloaded crate source supplied evidence.
## Final changes
Library manifest/lockfile, four public empty module stubs, documented empty library root,
empty conformance integration target, seven structural graders, target/cache ignores, dependency
notices, task 1 checkbox, and workflow evidence. No later task source or unrelated files changed.
## Verification still needed
Remote PR proof after contributor confirmation; local required graders and independent review passed.
## Remaining risks
Scaffold proves build/layout only, not JSON Schema compliance or runtime offline safety.
Source shows `boon::UrlLoader::load` returns `serde_json::Value`, not schema bytes; `Compiler`
uses `use_loader` after construction. Reconcile those later task-8/design assumptions before coding them.
TOML's transitive parser/writer versions carry spec-1.1.0 tags; the direct crate's tag alone does
not establish rejection of TOML 1.1 syntax. Actual parser conformance remains for manifest tasks.
Contributor status is still unconfirmed; no PR publication until that question is answered.
