# Exact adversarial admission proposal — NOT APPROVED

## Frozen surface
- Preparation baseline:30862a88fad6687a11e50b9d60b62adb55388050.
- Contract:`contract.json`, SHA256 `7645881b9f0829e66cef5dbfb0d63df3eb448ef0a3d1dd387445496d3ead46c9`.
- Exactly26 pairs/52files under `drafts/adversarial/`, mapped one-for-one to `crates/clouddialect-contracts/fixtures/adversarial/` with identical suffixes/bytes.
- Existing bounded batches3.2.22–3.2.35, at most two directories/four data files each. No case deletion or smoke substitution.

## Actual readiness evidence
All26 original oracles match;65parent controls include26wrong-outcome,13repaired-valid,13old-fail-oracle rejects repair,7structural counterparts and6selection/composite checks. Temporary73is NOT canonical: current47unchanged. Independent reviewer38also executed65controls;37traced seven P0profiles.105parent content/hash/schema assertions pass;52draft/11source/192protected hashes unchanged. No expected-data repair.

## Proposed next scope (requires explicit owner approval)
New45-minute local-only clock starting on approval, not an extension of preparation deadline11:21:56UTC. Copy only these exact52files into canonical fixtures, verify each bounded batch with full frozen Cargo tests and actual selectors/zero/wrong-oracle controls, then locally commit milestones. Related inventory/task20supporting status only, accurate to existing local-profile requirements. Choose at most two useful read-only reviewers under existing as-needed delegation; no recursion/follow-up dispatches and one consolidated data-packaging repair maximum.

## Forbidden actions and stop conditions
No source/harness/schema/manifest/lock/API/dependency/version changes, network fetch, native/provider/backend/authentication probes, push/merge/release/deployment or runtime gate closure. Stop for source repair, contract ambiguity, unsupported diagnostic change or deadline. These data cases cannot complete Spec2 admission/authentication requirements.

## Expected result, not present state
If approved and independently verified: canonical73fixtures, task20local-profile corpus potentially complete within approved wording, Spec1task21/22/full downstream and all21Spec2tasks/gates still open. Five P0Pass cases deliberately demonstrate untrusted declarations/claims, not actual authority denial, freshness enforcement or authentication.
