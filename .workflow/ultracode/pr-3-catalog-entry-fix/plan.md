# Catalog-entry raw extension fix
## Goal
Correct current Codex P2 nested CatalogEntry extension loss; keep already-fixed absent-date serialization covered. PR3 currenthead0b3feab, bot rerun still running at preflight; no additional findings visible.
## Success criteria
Unknown entry value bytes survive typed/inventory decode, serialize and re-admission; decoded duplicate entry names fail; no null dates introduced; public field/API impact documented.
## Current context
Tracked tree clean; preserve2prior untracked workflow runs and research files. Existing pinned serde/raw decoder, initial MVP crate0.1.0/doc/catalog1.0.0.
## Constraints
Parent writes only src/schema/catalog.rs and tests/catalog_extension_roundtrip.rs plus this run. No new dependency/feature/export/schema/version/namespace change, Git/GitHub mutation, downloads, probes, secrets or unrelated cleanup.
## Risk level
Medium behavior risk, explicit public struct-literal compatibility change.
## Approval gates
User explicitly says fix reported Codex findings; necessary new public CatalogEntry.extensions:RawObject announced before implementation. Existing struct literals must now provide extensions; search finds no repository literals. This is a bounded initial-MVP bug fix, not a backward-compatible Rust struct-literal claim or security-gate closure. No commits/pushes/PR-thread changes authorized in this task.
## Mode
Delegated: parent owns tightly coupled implementation; one independent read-only reviewer after GREEN, while parent runs full checks.
## Work packets
Parent minimal regression/fix;01-review integrated raw/duplicate/date/API behavior.
## Eval contract
Full eval-contract.md (public DTO data contract).
## Integration policy
One cumulative sidecar, no recursion/follow-up dispatches, one review wave, one bounded consolidated repair,30minute cap18:26:43–18:56:43UTC. Separate current-request correction, not reset of old implementation allocation.
## Verification plan
Exact behavior RED then GREEN; full frozen/offline tests/check/fmt/Clippy/Python; source/hash preservation; independent review; fresh emitted-line coverage if existing tools available.
## Completion criteria
Current reported defects addressed locally with evidence and honest API/remote-review limits. No claim that pending Codex rerun is complete or unresolved threads resolved.
