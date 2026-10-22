# Owner contract decisions — 2026-10-22

Source: current user replies to the three questions referencing [the proposal](contract-reconciliation-proposal.md). These are attributable owner choices, not native qualification or independent publication evidence. No code implementing these amendments is included in catalog leaves2.2/2.3.

## VC-1 — approved projection scope
Exact answer: “Root-level additions only; warn with exact paths and reject unsupported nested additions”. Preserve original declared version and bytes; use a separate base-check view, never recursively strip arbitrary maps or embedded JSON Schema fragments. The newer-MINOR path remains non-accepting. This reply selects projection scope; other proposed edge-case version dispositions must still be reconciled with the approved version profile and exact catalog dispatch before implementation. Do not silently treat this answer as blanket approval of every unasked policy detail.

Subsequent owner answer: “Use this initial version-support profile”. Support exact admitted stable versions; apply the approved root-only fallback for newer stable MINOR versions; ignore build metadata for precedence; unindexed PATCH/prerelease and u64-overflow versions are UnsupportedVersion. Malformed SemVer remains the helper's InvalidFormat disposition, while full valid suffixes stay representable. This now authorizes the remaining version-support profile for implementation; no version helper or structural integration is claimed implemented yet.

## RC-1 — approved resource coverage contract
Exact answer: “Use the discriminator and per-dimension coverage rule”. Resource-specific unresolved assumptions require `behavior` in `ordering | consistency | retry | failure` while retaining their existing assumption payload. Every dependency covers each dimension by a nonempty known declaration or a populated matching unresolved assumption. Both may coexist for partial knowledge. Generic evidence/obligation assumptions do not acquire this discriminator. No free-text guessing or ID convention substitutes for the binding.

Next implementation: reconcile requirement/design/task model text, add the resource-specific DTO/round-trip regressions and compatibility exports, then schema constraints with actual assertion RED/GREEN. Existing resource fixtures must become explicitly dimension-complete, not be deleted or loosened.

Subsequent RC-1 delivery: typed DTO GREEN `79493fe`, schema GREEN `7f97567`; [bounded evidence and limitations](resource-contract-report.md). This closes the model/schema binding gap, not full canonical conformance or behavioral verification. VC-1/EX-1 implementation remains separate.

## EX-1 — approved mandatory-extension declaration
Exact answer: “Use this declaration and trusted-recognition rule for all five kinds”. Optional root `required_extensions` is an array of unique namespace prefixes in the existing ASCII grammar, without the colon. Omission/empty means none. Unknown mandatory prefixes create a separate blocker even with no extension payload; structural validity does not imply acceptance. Recognition requires trusted implemented semantics, not registry membership or claimed ownership. Preserve explicit DTO round-trips and existing root openness elsewhere. No prefix is implicitly recognized merely because this field exists.

Next implementation: amend all five schema/DTO contracts and design examples, then integrate the actual extension validator after structural dispatch. Declaration-only structural-pass/blocker fixtures remain mandatory. This decision does not confer runtime policy grants.

## Boundaries
Initial MVP versions remain document/catalog1.0.0 and crate0.1.0, based on the earlier owner attestation. No release or crate publication has been authorized; if prior distributed formats are discovered, migration/version review remains required. BG-01/UD1–UD5, native qualification and measured budgets remain open.
