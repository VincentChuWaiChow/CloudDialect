---
description: CloudDialect repository engineering and domain organization rules
inclusion: always
alwaysApply: true
---

# CloudDialect AI engineering rules

This is the canonical repository rule. Edit this file, not its symlink entrypoints.
It applies to every agent and delegated packet working in this repository;
system/developer instructions and explicit user scope take precedence.

## Domain-first organization

- Group files by business responsibility, not by generic technical categories
  such as a global `models/`, `services/` or `utils/` directory. Keep a domain's
  types, rules and tests together when they share an owner and reason to change.
- In the contracts crate, contract/evidence/verification document declarations
  belong to the document domain; transformation-pack declarations belong to
  `manifest/`; catalog/schema admission belongs to `schema/`; validation
  orchestration belongs to `validator/`. Keep diagnostics in `errors/`.
- Split document models by business vocabulary (behavioral contract, evidence,
  verification). Do not mix unrelated document families in one growing file.
- Keep the shared kernel small: only genuinely shared identities, diagnostics
  and serialization support. Do not duplicate shared types between domains.
- Domain code must not depend on CLI, deployment or worker implementations.
  Use explicit ports only when the approved contract requires them. Do not add
  repositories, aggregates, services or abstraction layers merely to claim DDD.
- Preserve the public API when moving implementation files: use compatibility
  reexports of the same definitions, not duplicate structs or wrapper types.
  A public-path or schema change requires explicit migration/version review.
- Prefer one file until a domain needs multiple cohesive files. Do not create
  empty domain scaffolding or reorganize unrelated specifications/research.

## Evidence, scope and safety

- Read relevant requirements, design and tasks together before changing code.
  State assumptions; distinguish facts, inference and unverified evidence.
- Use TDD for behavior changes: demonstrate a relevant failing test, implement
  the minimum fix, then rerun the same test and integrated checks. For a pure
  move, establish passing tests before and after; do not fabricate RED.
- Preserve exact extension JSON value bytes and all non-accepting phase-result
  boundaries. A parsed declaration is not a policy grant or authenticated proof.
- Keep Spec 2 UD-1–UD-5/BG-01 open until their actual approvals and required
  evidence exist. No mock, schema test or folder layout proves native isolation.
- Do not commit, push, publish, deploy or run native sandbox/provider probes
  without current explicit user authorization for that action. Preserve unrelated
  work and never read or print credentials, tokens or authentication stores.
- Always prefix shell commands with `rtk`; use `rtk proxy` for unfiltered output.
  Follow the user's RTK guidance without copying personal configuration here.

## Documentation and delegation

- Use Context7 for current library/framework/SDK/API/CLI/cloud documentation:
  first `resolve-library-id`, then `query-docs` with the selected ID and concrete
  question. Verify material version-specific claims against pinned source.
  General business logic and read-only code review do not require a docs lookup.
- Default to delegate useful independent work on non-trivial tasks. Bound each
  packet's ownership, checks, elapsed time, cumulative dispatches and repairs;
  keep one integration owner. No recursive or hidden fan-out. If tools or limits
  prevent delegation, state that boundary rather than inventing agent activity.
- Workers must not revert others' edits. Serialize shared exports, manifests,
  lockfiles and external mutations. Integrate and verify the combined tree.
- Report actual commands/results and skipped checks. Symlink readability proves
  rule availability, not that every AI client loaded or obeyed it at runtime.
