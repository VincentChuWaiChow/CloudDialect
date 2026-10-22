# Implementation-planning eval — defined before plan/approval edits

## Outcome
Translate planning-approved P0-A–F into bounded implementation units without starting implementation or claiming delivered compatibility.

## Capability checks
- Approval notes state the exact user scope: implementation planning only; adoption/publication/implementation/native gates remain separate.
- Every unit has stable ID, canonical Spec 1 task mapping, explicit file ownership, dependencies, 3–8 focused cases, RED/GREEN target, scope/regression checks, estimate <=4h or mandatory split before dispatch, and observable done criteria.
- Shared lib/manifest/lock/index changes have one parent owner; no contradictory same-wave file writes or hidden prerequisite. Parent coordination waves may group a deeper leaf DAG only with its real depth disclosed; no falsified depth-3 claim.
- Coverage maps every canonical task 2–26 to delivery or explicitly deferred release-only work; no API-stub-only claim of BG-01 completion.
- First implementation slice is separately approvable and cannot be confused with authorization for the entire unfinished Spec 1/runtime.
- Pinned dependency/API evidence is source/checksum-bound where available; unavailable cache/network/docs remain UNVERIFIED. No download/install/execution fallback is silently assumed.

## Regression checks
Preserve all existing source, catalog absence, locks/licenses/research and earlier decision drafts. P0 focused document suite and seven scaffold tests remain passing; broader validator/date/semantic/native limitations remain recorded. Audit exact changed/new paths, checkmarks, links/JSON/fences and diff whitespace.

## Review and limits
One independent read-only reviewer after draft; fifth cumulative agent, no follow-up or recursive delegation. One consolidated planning repair; 35-minute tranche. Installed dependency source may be read or bounded public official archives downloaded to /tmp for checksum verification only (no install, build script or native probe). Any privilege exception must be scoped/reported.
