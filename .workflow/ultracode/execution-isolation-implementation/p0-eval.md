# P0 eval — defined before Spec 1 edits

## Authorized outcome
Reconcile Spec 1's requirements/design/tasks into one concrete **proposed** upstream contract, ready for owner approval. The runtime goal is not complete and BG-01 remains OPEN.

## Capability checks
1. All three edited documents and amendment record explicitly state P0 draft/pending approval, absent API implementation and no gate closure.
2. BehavioralResult fields/outcomes agree across docs; Deferred/EvidenceComplete/Blocked are non-accepting, with no new accepting enum or automatic conversion. Spec 2 records behavioral observations without confusing completeness with acceptance.
3. Manifest declares a versioned, typed capability requirement outside `[x]`; exact matching and unknown-version/operation tests defined; declaration never grants authority.
4. Verification schema/API agrees on immutable execution/configuration/suite/oracle/policy bindings, obligation-to-test mapping, structured failures, external approval references and absence of credentials. JSON Schema validity/authentication are distinct.
5. Catalog injection is explicit from trusted admitted bytes, no ambient env selection; schema registry and project-scoped document reference store remain different. Public export/module names and semantic API agree.
6. Updated tasks include positive and adversarial fixtures for changed contracts, real dependency compatibility and format-version review before publication. Stable task numbering/checkmarks preserved.
7. Embedded JSON schema snippets parse as JSON with local `$ref`s resolvable; TOML examples parse; structural checks do not imply conformance/qualification. Independent review is bounded and evidence-backed.

## Regression checks
- All pre-P0 file hashes preserved except the four owned Spec 1 docs and run artifacts.
- Seven scaffold tests pass; Rust/source/lockfiles/licenses/user research unchanged.
- No runtime/native/coverage/approved gate claims; no tasks newly marked complete.
- Scope, new-file whitespace, local links and JSON/TOML parsing pass; record any pre-existing general-validator failure separately.

## Graders and limits
Parent source/diff audit, deterministic Markdown/code-fence/field checks and existing scaffold suite; one independent review. Existing generic validators may report pre-existing layout defects; do not reformat unrelated sections to game them. Native Kiro/SMT unavailable. No statistical pass@k/pass^k or runtime coverage claim.
P0 is documentation, so no runtime TDD RED/GREEN or checkpoint commits are represented. Future implementation still requires validated same-target RED/GREEN and branch-local checkpoints. No Rust implementation is authorized by P0.
