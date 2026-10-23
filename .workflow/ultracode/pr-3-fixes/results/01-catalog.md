# Packet 01 — Catalog R1/R2

## Outcome
R1/R2 corrected within the three owned files. No dependencies, schemas, exports,
manifest/lockfile, versions, Git actions, acquisition, native probes or credentials
changed/accessed. Frozen caller material remains byte-for-byte unchanged.

## Changes
- `src/schema/catalog.rs`: omit absent `CatalogEntry.end_of_life_date`; reject
  duplicate indexed/nested schema resource identities using the pinned Boon
  engine's actual draft-aware resource discovery and URL normalization/joining.
- `tests/catalog_loading.rs`: regress application/support target collisions,
  absolute/normalized/relative IDs, support-origin collisions, all five supported
  drafts and an admitted custom meta-schema; preserve legitimate bundling and
  schema-looking `$id` instance data in `const`/`examples` (including draft4's
  ignored `$id` spelling).
- `tests/catalog_extension_roundtrip.rs`: parse/serialize/re-admit the initial
  catalog with absent dates and one explicit date; assert correct omission and
  retained explicit value.

## Design boundary
Public Boon 0.6.1 exposes `Compiler::add_resource/compile` and
`CompileError::DuplicateId`, but keeps its resource collector and URL joins
private. A temporary schema copy gains nonconflicting `definitions` sibling
schemas for the other indexed IDs (`id` + `$id` supports the pinned drafts).
This permits engine-native duplicate detection without reimplementing JSON
Schema traversal or URL joining. Only `DuplicateId` rejects admission; other
compilation errors remain the existing later schema-validity boundary. The
probe has a closed exact caller-material loader, never replaces frozen bytes,
and its compiled output is discarded. This is not complete schema compilation,
authentication, acceptance or native zero-network qualification evidence.

Pinned source inspected: existing isolated cache `boon-0.6.1/src/{compiler,
roots,root,draft,loader}.rs`. No network lookup or dependency fetch.

## TDD evidence
All commands use `CARGO_HOME=/tmp/clouddialect-phase-cargo` and the exclusive
`CARGO_TARGET_DIR=/tmp/clouddialect-pr3-catalog-target`.

1. RED before source edits:
   `rtk proxy cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml --frozen --offline --test catalog_loading --test catalog_extension_roundtrip`
   exit101: absent dates serialized as null; Cargo stopped after the first target.
   Raw output: `catalog-roundtrip-red.txt`.
2. Separate R1 RED before source edits:
   `rtk proxy cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml --frozen --offline --test catalog_loading`
   exit101: nested indexed-ID shadow admitted (13pass/1fail).
   Raw output: `catalog-loading-red.txt`.
3. GREEN same combined command after fixes plus supplementary dialect tests:
   exit0: catalog loading17pass, catalog extension round-trip5pass.
   Raw output: `catalog-green.txt`.
4. `rtk proxy rustfmt --edition 2024` on the three owned files: exit0.

An intermediate compile typo (`Result::as_deref` on non-Deref SchemaIndex) was
corrected within this implementation tranche before GREEN. No integration repair
allocation consumed by this worker. Parent owns full-suite/lint/coverage checks
and the independent final review; those were not claimed here.

## Limits
Identity probe behavior is pinned-engine evidence, not a general promise for a
future engine version. Inventory admission still does not certify that every
schema compiles: e.g. unresolvable meta-schema/invalid schema errors remain later
errors, as existing regression tests confirm. Prior unrelated files preserved.
