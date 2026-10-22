# Orchestration

## Parent critical path
Verify base; define evals; write requirements; integrate review; write conditional design; write tasks; validate final artifacts.

## Packets
- 01-upstream: read-only compatibility review, no edits.
- 02-threats: read-only brief coverage and adversarial cases, no edits.
- 03-requirements: read-only requirements QA after draft.
- 04-integrated: read-only full artifact review after integration.

## Delegation
Two discovery sidecars, followed by one staged review wave (requirements then downstream). Parent retains decisions and all writes.

## Agents
Native collaboration agents; inherited model/effort requested, observed identity UNVERIFIED. No external MCP delegated; parent handles Context7. No secret or auth-store reads.

## Delegation limits
At most two simultaneously open workers, four cumulative worker turns, zero follow-up turns, one parent repair cycle, 60 minutes elapsed. No child fan-out. Token usage unknown. Replan with user if bounds exhausted.

## Wait points
Requirements gate before design; integrated review before final report.

## Fallback
If delegation unavailable, safe parent review only and report lack of independence. If security decisions cannot be closed, downstream artifacts remain conditional, as approved.

## Verification order
Structure → requirement QA → traceability → dependency ownership → existing regression → final scope and provenance.
