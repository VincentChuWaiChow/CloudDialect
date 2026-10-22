# Packet11: bounded domain-layout analysis
## Objective
Read-only map current contracts Rust files to smallest coherent business-domain layout.
## Context
User requests domain ordering, shared AI rule symlinks, commit/push. Physical scope question pending; do not assume repo-wide relocation. Existing public module API paths must remain compatible.
## Sources
Crate source/tests and canonical Spec1 requirements/design/tasks.
## Ownership
None; parent owns writes.
## Do
Recommend minimal domains, exact moves and required compatibility reexports; identify macro/private-module/import/test constraints and DDD-overengineering risks.
## Do not
No edits, commands with side effects, network, credentials, subagents/followups, commits or push. All shell commands prefixrtk.
## Expected output
Concrete small mapping with evidence, pitfalls and constraints; <=10minutes.
## Verification
Read-only source inspection, no runtime enforcement claim.
## Handoff format
Recommended paths; API preservation; limitations.
