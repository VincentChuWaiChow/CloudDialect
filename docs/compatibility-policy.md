# CloudDialect format compatibility policy

## Initial MVP and evidence boundary

The maintainer attests that this is a new project and approves document/catalog
**1.0.0** and crate API **0.1.0** as the first MVP versions. This is an owner
publication-history statement, not an independent registry audit or a release
receipt. No publication, successor or end-of-life date is invented here.

The source-tested format profile is deliberately narrow:

| Validator crate API | JSON document format | Catalog format | Pack-manifest fixture version |
|---|---|---|---|
| `=0.1.0` | `=1.0.0` | `=1.0.0` | `=0.1.0-rc.1` |

These are SemVer comparator constraints, not a promise about unbuilt future
versions. The pack column describes the actual local-validation example
`0.1.0-rc.1+build.1`; it qualifies declaration-format handling only. No deployed
transformation pack, worker, behavioral equivalence or Spec 2 compatibility
handoff is certified. Other syntactically valid pack versions are not rejected
merely for being absent from this evidence table, but are not thereby qualified.

## Three distinct version spaces

1. **JSON Schema dialect:** schema documents declare Draft 2020-12 in their
   `$schema` field. A document instance's optional `$schema` instead selects its
   exact admitted CloudDialect schema URI; it is not its document-format version.
2. **Document-format version:** JSON instances carry `format_version`. Schema
   catalog entries also identify that format explicitly. Catalog format and
   typed capability-declaration format have their own named version fields;
   neither is inferred from a pack release.
3. **Transformation pack version:** manifests carry `pack.version`; verification
   records carry the tested pack's `pack.version`. Neither selects a schema
   dialect nor establishes artifact identity. Pack toolchains/language ranges
   use the approved Cargo comparator and prerelease rules.

The Rust crate API version is separately recorded in Cargo.toml. It is not an
additional spelling of any of these three document/pack version spaces.

## Change classification and migration

- **MAJOR / breaking:** a change to required fields, field types, permitted enum
  values or validation semantics that makes a previously valid document invalid,
  or reinterprets a security/behavioral claim incompatibly. Do not hide it in a
  MINOR/PATCH release. Removing enum alternatives can break existing documents.
- **MINOR / backward-compatible:** optional additions or broadened optional enum
  choices that keep prior valid documents valid without changing their meaning.
  Backward schema compatibility is not unlimited forward-reader compatibility:
  an old closed enum may still reject a newly introduced value.
- **PATCH:** non-normative corrections such as descriptions/typos, not changes to
  required data or validation behavior.

Every MAJOR release must include a migration guide from the prior format,
identifying changed fields/semantics, examples and positive/negative migration
fixtures. No universal automatic migrator is promised. P0 capability, binding,
configuration and structured-failure requirements define this initial MVP; if
applied to a previously distributed older format they require MAJOR/migration
review. Legacy records missing them fail current validation, never acquire
acceptance through a warning or an optional extension.

## Validator 0.1.0 admission profile

- Exact admitted stable document versions are supported by precedence; build
  metadata is ignored for precedence, not for exact DocumentRef identity.
- A newer stable MINOR may use an actually admitted MAJOR.0.0 base. Only unknown
  root additions are projected for that check, with exact-path warnings; original
  bytes/version remain intact. Unsupported nested additions are rejected, while
  existing freeform/open base-schema data remains valid. Opaque projection
  scopes fail closed rather than guessing.
- Unindexed same-MINOR PATCH versions, prereleases and valid core integers above
  u64 are `unsupported-version`. Malformed SemVer is a version-format error;
  an unsupported MAJOR is `unsupported-major-version`.
- An explicit admitted schema URI must match the trusted kind/version selection.
  There is no environment, filesystem or network fallback. Unknown mandatory
  extension semantics block separately, even after structural success.

Full SemVer syntax remains representable in documents and manifests; grammar
validity is not a support promise. Structural/semantic success and
EvidenceComplete/Deferred are all non-accepting.

## Support window and end of life

Retain each published document-format version and its original stable catalog
paths for **at least 12 calendar months from the publication date of its successor
version**. This is a calendar-month rule, not a fixed 365-day substitution. Keep
the predecessor supported longer if another recorded policy requires it.

To declare EOL, update its catalog/index.json entry's `status` to
`end-of-life-declared`, set `end_of_life_date` no earlier than that minimum window,
and announce the successor, dates and migration guide in the changelog. Record
the actual successor publication evidence before calculating eligibility; a Git
commit date or an absent local tag is not publication proof. Missing dates or
evidence block EOL rather than inventing them. Do not remove retained schema
material prematurely. No initial-MVP EOL has been declared by this document.

Normative scope: [requirements 6–7 and NFR-3](../.kiro/specs/contracts-evidence-foundation/requirements.md),
[approved prerequisite delivery profile](../.kiro/specs/contracts-evidence-foundation/implementation-plan.md).
