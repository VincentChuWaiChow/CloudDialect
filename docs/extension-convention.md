# Extension convention — initial MVP

This convention implements Spec1 requirements5.8 and10.1–10.6. It describes
declarations and non-accepting validation, not runtime permissions or acceptance.
See the [compatibility policy](compatibility-policy.md) for format changes.

## Names and reserved families

A namespace prefix must match the **whole-string ASCII** grammar
`[A-Za-z][A-Za-z0-9-]+`: an ASCII letter followed by ASCII letters, digits or
hyphens, with **minimum total length two**. A colon separates the prefix from
the remaining qualified key: `acme:field-name`. The initial validator checks
the prefix before the first colon; this convention does not impose a new suffix
grammar. One-character prefixes, digit starts, spaces, underscores and Unicode
lookalikes are invalid. Root qualified extension keys are scanned on the original
input, including when newer-MINOR root projection is used.

The reserved namespace families are `core`, `core-*`, `cdc`, and `cdc-*`.
Reserved-prefix matching is **case-insensitive**, within the ASCII grammar:
`core:`, `core-validation:`, `cdc:`, `cdc-evidence:`, `CORE:`,
`Core-Validation:`, `CDC:` and `Cdc-Evidence:` are reserved examples.
The family suffix may be empty (`core-:` is reserved); `corex:` is not in
these families. Community spelling is preserved, not normalized by this rule.

Each reserved qualified key produces a warning with the **verbatim field name**
and a description that its prefix is reserved for core use. That warning alone
does not fail structural validation or establish authority. New namespaces
within the existing reserved families may be introduced **without modifying
prefix-validation logic**. Changes altering previously permitted namespace
usage require a **compatibility review**; a breaking change must follow the
MAJOR/migration policy rather than silently invalidate an existing format.

## Registry, ownership and collisions

[`catalog/extension-registry.json`](../catalog/extension-registry.json) is the
authoritative extension registry document for recorded community claims. Each
entry contains `namespace_prefix`, `owner` (claimed owner), `purpose`, and
`registration_date`. The initial registry is empty; do not invent registrations
or dates. Every release artifact must include the registry, even when empty;
this document does not prove release packaging has occurred.

Record proposed ownership and resolve conflicting claims through registry
review before recording an authoritative assignment. Do not silently replace a
conflicting owner's entry or interpret two payload claims as interchangeable
semantics. Prefixes alone do not establish publisher identity or trust.
Registry membership is not authenticated ownership, implemented recognition,
an automatic collision detector, a policy grant, or a reason to clear a blocker.
The MVP validator does not select trusted extension semantics from owner claims.

## Optional JSON payloads and exact preservation

A valid unknown community extension at a schema-permitted location must not
fail structural validation merely because it is unknown. Existing closed typed
objects remain closed; a root convention is not permission to add arbitrary
nested fields. Freeform core inputs/outputs and extension payloads remain opaque,
not a place to discover new core document references or security declarations.

Conforming typed JSON round-trips preserve the qualified key and **exact encoded
JSON value bytes**, including escapes, numeric spelling, and whitespace/member
ordering inside that value. Decoded-value equality alone is insufficient.
Document/key ordering and whitespace outside extension values are not promised.
Preservation does not execute, recognize, authenticate or endorse the payload.
Duplicate decoded object keys are rejected, not silently collapsed.

Unknown optional data must not change core obligations, security policy,
verification decisions or migration acceptance. For example,
`acme:verification-status: "accepted"` is inert data and cannot override a core
result. A schema or semantic pass never grants acceptance or runtime capability.

## Mandatory JSON declarations

All five JSON document kinds allow optional root `required_extensions` as an
array of **unique decoded namespace prefixes without a colon**, using the same
ASCII/minimum-length grammar. Omission or `[]` means no mandatory declaration;
malformed types, prefixes or duplicate strings fail structurally. Uniqueness is
by decoded string; only reserved-family matching is case-insensitive.

A declaration alone requires recognition; a payload is not required. An unknown
mandatory prefix remains structurally valid but the separate semantic phase
reports an explicit `BlockingGap` identifying `required_extensions[index]` and
the namespace. The current MVP implements **no trusted extension namespaces**,
including reserved ones. Registry/catalog membership, a claimed owner,
`recognized: true` or `accepted: true` payloads cannot clear that blocker.
Recognition requires actual trusted implemented semantics and reviewed activation.
Mandatory-extension checks do not authenticate evidence or execute behavior.

## TOML extension and capability boundary

Pack-manifest custom data belongs in the optional `[x]` table. TOML typed values
are retained; exact source-byte preservation is the JSON-value guarantee above,
not a claim about TOML formatting. A direct `[x]` key cannot shadow a core
top-level key. Nested opaque values are not scanned as core declarations.

Only the mandatory core `[capabilities]` declaration carries requested needs;
it is validated separately. `[x]` cannot supply a missing capabilities table,
change core requested capabilities, grant credentials or authorize execution.
Even a valid core capability declaration is a request, never effective policy.

## Evidence boundary

`extension_policy`, `extension_validation`, `required_extensions`,
`semantic_validation` and model round-trip tests check these bounded source
rules. The complete versioned adversarial conformance corpus, authenticated
publisher/registry governance, release packaging and native execution isolation
require separate evidence. This document does not close BG-01 or Spec2 gates.
