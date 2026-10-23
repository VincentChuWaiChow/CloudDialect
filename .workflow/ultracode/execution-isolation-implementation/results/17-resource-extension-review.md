# Packet17 — static RC-1/EX-1 review

Native read-only `/root/resource_extension_review`, completed within8minutes. User explicitly approved two extra read-only reviewers to18cumulative,75minutes from the existing resource-tranche start, one consolidated repair, zero recursion/followups. This is dispatch17;18 remains reserved. No writes/tests/source execution/network/probes/commits. Inherited model/effort and independent sandbox isolation unverified.

**Bounded PASS — RC-1, no required repair.**
- Required closed resource discriminator/payload: behavioral_contract.rs40–60,89–99,150–154.
- Four per-dependency anyOf constraints allow known/unknown coexistence; global item validation still rejects malformed declarations: behavioral schema84–181,275–306.
- Generic assumptions unchanged; facades reexport the same definitions: behavioral_contract.rs30–38,104–112; documents/mod.rs9–13; validator/mod.rs4–10; lib.rs29–34.
- Tests cover all16 combinations, partial knowledge, independent dependencies, unrelated/duplicate-dimension noncoverage, malformed fields, generic assumptions and exact raw extension values: behavioral_schema.rs212–330; source_document_models.rs228–321.

EX-1 recommendations: explicitly add optional required_extensions to the four existing DTOs (three primary documents plus CatalogIndex), updating RawFields conversions; no new registry DTO. Identical optional unique-prefix array grammar across five schema roots, including whitespace/newline exclusion. Preserve unrelated root openness. Declaration-only documents structurally pass; separate future extension validation blocks unknown mandatory semantics even if registry ownership/payload claims exist. Inventory/registration is never implemented recognition or acceptance.

Limits: static only; no independent test/coverage rerun or native proof. Pre-existing non-RC whole-design parity MISS remains recorded, not fixed here. Parent owns final integrated verification/publication. No code changed while this frozen RC-1 review was running.
