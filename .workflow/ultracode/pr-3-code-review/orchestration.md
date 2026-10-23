# Orchestration
## Parent critical path
Freeze scope, independently inspect scripts/shared flows, run offline tests and reproduce candidate bugs.
## Packets
01-schema-version, 02-manifest-resolution, 03-model-extensions; disjoint read ownership plus necessary shared spec context.
## Delegation
One read-only wave, three sidecars; fresh request, no implementation work.
## Agents
Native collaboration spawn_agent, inherited default model/effort.
## Delegation limits
3 cumulative runs, no follow-up dispatches or recursion, zero repairs, total 40 minutes from 17:56:49 UTC (deadline18:36:49UTC); worker 25minutes maximum.
## Wait points
Only wait after independent parent checks or candidate repros.
## Fallback
If spawn unavailable review affected slice locally, mark reduced coverage.
## Verification order
Sources/specs -> offline suite -> adversarial repros -> integrate -> hash/status guard.
