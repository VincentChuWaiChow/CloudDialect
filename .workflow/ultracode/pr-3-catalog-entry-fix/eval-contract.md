# Eval contract
## Goal
Preserve exact unknown JSON entry values, not merely decoded equality.
## Success criteria
Regression fails from field loss before code edit; succeeds with exact escapes/whitespace/number spelling/null/opaque duplicate payloads afterwards. Decoded duplicate member names fail before inventory acceptance.
## Integration surfaces
CatalogEntry serde try_from RawFields -> nested CatalogIndex -> from_parts -> Serialize -> valid re-admission.
## Downstream consumers
Current public CatalogEntry literals require extensions field; all in-repo uses checked. No registry DTO/schemas or trusted extension recognition changed.
## Required checks
Focused RED/GREEN, full Rust/Python/fmt/check/Clippy, independent static diff review, preservation and coverage if practical.
## Deliverables
Two scoped crate files plus local task evidence.
## Blocking conditions
Schema/dependency/version policy change needed, unapproved external mutation, source drift, failed sole repair or elapsed cap.
