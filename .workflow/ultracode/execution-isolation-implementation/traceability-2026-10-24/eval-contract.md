# Eval contract
## Goal
Exhaustive source-bound traceability, not decorative one-row-per-schema mapping.
## Success criteria
Exact complete field keys, no duplicate/orphan rows, valid requirement IDs; allfivecanonicalgate requirementsets andactualfieldreferences. Explicit no-native/acceptance/BGclosure boundary and immutablelocalconformance link.
## Integration surfaces
45producesfield-to-requirement.md;46producesgate-to-requirement.md;parenttests/traceability.rs consumesboth. Primarytables: Schema|Field path|Requirement|Notes andGate|Description|Requirements|Schema fields. Usebackticksaroundfieldkeys, Req N.N/NFR-N.N tokens, no literalunescapedpipesinsidecells. Manifestpathsusearrays[*]/contracts.*. Opaqueinput/schema/TOMLvaluesareleafcontainers,notfabricatedfinitefieldlists. Conditionalproperties/localrefsunionacrossfiveJSONschemas.
## Downstream consumers
ParentTDDtestvalidatesagainstcurrentJSONSchema/Rustmanifestsource,notworkflowinventory;currentdocsbuildersmayusefield-inventory.json onlyasauthoringaid. Gatefieldsfullyqualifiedschema.path; rust-api appendixkeysprovidedininventoryareverifiableports,notgrants.
## Required checks
ExecutedREDbeforewriters; same-targetGREEN; dropped/duplicate/orphanfield,invalidrequirement,missinggate/unknownfield/wronggate-mappingcontrolsfail; fullintegratedchecks andindependentreview.
## Deliverables
Onlytwodocs,onenewintegrationtest,relatedstatus/workflow; no productionchange.
## Blocking conditions
Unresolvedrequirementmappingneedsownerdecision;nosource/schema/APIchangesorfalseBGproof. Oneconsolidatedrepairandhard18:15:58UTC.
