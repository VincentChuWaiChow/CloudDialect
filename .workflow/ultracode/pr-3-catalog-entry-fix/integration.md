# Integration
## Accepted
Minimal11-line production change reuses existing RawFields/raw_object_model for CatalogEntry, adds flattened raw extension map, keeps absent-date omission. Three new regressions plus all existing checks pass. Independent static reviewer found no actionable issue on exact final hashes.
## Rejected
No Value reconstruction of unknown values, duplicate-name acceptance, schema narrowing, new dependency/DTO namespace/trusted recognition or API compatibility claim.
## Conflicts
None. One parent implementation wave, one read-only review packet; zero repairs.
## Decisions
Initial-MVP public struct-literal callers must add extensions (an empty RawObject when none). No document/catalog schema/version change. Existing2Codexthreads not mutated; date was already fixed in0b3feab.
## Final changes
Only src/schema/catalog.rs and tests/catalog_extension_roundtrip.rs, plus this local run.
## Verification still needed
No remaining required local check. No remote commit/push/PR review clearance requested or performed.
## Remaining risks
External downstream literals require migration; no in-repo literal callers. This is not full-spec/native qualification. Codex remote rerun was still running at preflight and has not evaluated this unpublished correction.
