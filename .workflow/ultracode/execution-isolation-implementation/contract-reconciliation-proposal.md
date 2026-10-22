# Remaining format-contract decisions — proposal, not approved

Historical recommendation text below. The user has since selected VC-1 root-only projection and approved the RC-1/EX-1 contracts; see [the attributable decision record](contract-reconciliation-decisions.md). That does not implement the amendments or approve every unasked version edge case.

Static packet15 identified representation/policy gaps. Catalog snapshot/resolver work can proceed independently; do not silently repair these by making required behavior narrower or more permissive. Initial MVP versions were previously approved, not a waiver of migration/publication checks. These recommendations do not authorize runtime probes or close BG-01.

## VC-1: explicit forward-MINOR structural mode
Req7.3 requires same-MAJOR/newer-MINOR MAJOR.0.0 base-schema processing with non-blocking warnings; rejecting all newMINORs would violate it. P0 trusted kind/schema dispatch still applies. Recommended first-MVP policy:
- Trusted per-kind stable baseline1.0.0. Preserve original declaredversion/bytes; record separate effective admitted base schema. Stable newerMINOR uses admitted MAJOR.0.0, returns Warn, never acceptance.
- Explicit compatibility projection excludes newly added root fields only from the base-check view and emits exactpath warnings; keep original bytes intact. Known fields/types/required fields still validated. Do not recursively strip arbitrary keys, configuration maps or JSONSchema fragments. For unsupported nested typed-core additions, fail UnsupportedVersion rather than silently pretending coverage. Root-only MVP is an explicit requested restriction; choose typed-nested support instead if fullnested additions are required now.
- Absent instance$schema valid; presentURI must resolve and match effective selected admittedschema. A future/unlisted URI still fails, regardless ofMAJOR.
- Known admitted versions retain normal strict schema behavior; unindexed same-MINOR PATCH/prereleases unsupported. Build metadata ignored for precedence but exact references remainexact. Valid syntax beyond u64core range UnsupportedVersion, never InvalidFormat.
- Internal helper needs full trusted supported-version/profile, not onlysupportedMAJOR. Add UnsupportedVersion disposition; do not expose a misleading Supported verdict for unindexedpatch/prerelease. Document policy/fixtures explicitly and reconcile Req7.3 scope onapproval.
Sources: requirements6.4/7.3/7.4, tasks11/negativeversionfixtures, P0-E, closedroot schemas.

## RC-1: resource behavior coverage
Each resource dependency must cover ordering/consistency/retry/failure. Add resource-specific assumption discriminator behavior with exactly thosefour labels, retaining assumption_id/description/reason andextensions. Do not impose discriminator on evidence/obligation assumptions. For eachdimension require either nonemptyknown_behavior.<dimension> or at leastone complete matchingassumption; allowboth for partialknowledge. An unrelated/generalassumption cannot coverall dimensions. Enforce per-resource structurally. Sources: Req2.2; existingresource/model/schema lacks binding. No NLP/identifierguessing.

## EX-1: explicit mandatory-extension core field
Allfive caller-selected JSONdocumentkinds gain optional root required_extensions: array of unique existinggrammar namespaceprefixes (ASCIIletter start, >=2letters/digits/hyphens, no colon); absent/empty meansnone. Preservefield explicitly in actualtypedDTOs ratherthan unknown-extension bags. Keepunrelatedrootopenness unchanged. An otherwise-valid document withunknownmandatoryprefix remains structurallyvalid but separatelyblocked fromacceptance, even withoutmatchingpayload. Recognition derives onlyfrom trustedimplementedsemantics (MVPsupportsnone unlessactuallyimplemented), nevernamespace registration/claimedownership/attackerpayload. Exactprefix strings and existing reserved-family case-insensitivity are preserved; no registry-conferred grant or acceptancebit. Sources: Req10.4, designmandatoryblocker, task12 anddeclaration-only adversarialfixture.

Approvalrequested: policy/data-contract recommendations only for affected laterprerequisite implementation. No fabricated upstream/core acceptance or immutable deliveredrevision, and no assumption that featurebranch publication is a signed formatrelease. Record userdecision before altering normative schemas/DTOs.
