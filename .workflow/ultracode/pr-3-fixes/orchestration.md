# Orchestration
## Parent critical path
R3 nested duplicate handling, integrate source/diff/spec checks, serialize full checks and final report.
## Packets
01catalog owns schema/catalog.rs plus tests/catalog_loading.rs,tests/catalog_extension_roundtrip.rs.02diagnostics owns validator/fidelity.rs,manifest/validator.rs plus tests/fidelity_mapping.rs,tests/manifest_semantic.rs.03review read-only integrated diff.
## Delegation
2writers now,1reviewer later; parent also writes disjoint structural/raw duplicate code/tests.
## Agents
Native spawn_agent, inherited model/effort; no recursive dispatch.
## Delegation limits
3cumulative runs, maximum3open workers,2waves,1consolidated integration repair,60minutes.
## Wait points
Only after parent independent work.
## Fallback
Unspawnable packet handled locally with reduced independent evidence.
## Verification order
Baseline -> targeted RED/GREEN -> integrated tests -> final review -> at most1repair -> all checks/scope.
