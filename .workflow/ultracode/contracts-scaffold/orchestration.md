# Orchestration

## Parent critical path
Inspect spec, resolve dependencies, define tests, verify RED, scaffold, verify GREEN, integrate, publish.

## Packets
- 01-spec-review: read-only task/design and scaffold risk review.
- 02-final-review: read-only integrated diff and acceptance review.

## Delegation
One discovery wave and one review wave; no write-capable workers or recursive fan-out.

## Agents
Native generic agents, inherited parent settings; specialist/model/effort identity not independently observable.

## Delegation limits
At most 3 cumulative runs including follow-ups, 2 open workers, 2 waves, 1 repair, 90 minutes total.
Initial consumed runs: 0. No worker requires security isolation; instructions constrain behavior only.

## Wait points
Spec findings before integration; final review before GREEN commit/push.

## Fallback
If agents unavailable, record concrete failure and perform bounded review in parent without claiming independence.

## Verification order
Structural RED -> RED commit -> implementation -> structural GREEN -> Cargo checks -> review -> GREEN commit -> remote PR proof.
