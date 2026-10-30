# UD-3 — scoped credential-use proposal

Status: **OPEN — no delivery mechanism/provider approved or tested**. TASK-003, 2026-10-22. Security/operations approvers not supplied. REQ-004/005/006/010; DES-005/006/010.

## Comparison
| Option | Benefit | Exfiltration/revocation limitation |
|---|---|---|
| Environment | Compatible with many SDKs | Recipient obtains reusable bytes and can copy/encode them; clearing environment later does not recall copies. |
| Private mounted file / descriptor | Less broad inheritance than ambient environment | Authorized reader can retain bytes; unlink/close is not provider revocation. |
| Narrow operation broker with opaque attempt-bound handle | Worker need not receive provider secrets; policy checked at each operation | Additional trusted service/API; must not become arbitrary HTTP/signing proxy; cannot retract in-flight effects. |

Recommendation (inference): evaluate broker-mediated synthetic test operations first, with no raw provider credentials or ambient fallback in workers. Compatibility limitation: a candidate requiring arbitrary direct SDK calls is not automatically supported; approve a bounded operation interface or block, never secretly inject broader credentials. Broker responses, provider errors and presigned URLs can themselves carry sensitive data/authority and need egress classification.

## Proposed operation contract — review fixture only

This provider-neutral proposal makes the broker recommendation reviewable; it
does **not** select a broker, provider, transport, credential mechanism or runtime
schema. The 2026-10-30 update is OFFLINE documentation/fixture review only; it
authorizes no external call. It is not a claim that a run can call an external
provider in offline mode: such a call must be blocked before dispatch.

All names below are synthetic, non-resolving resource identities, not endpoints,
accounts, tokens or existing resources. Proposed trusted context:

- Tenant `synthetic-tenant-a`; project `synthetic-project-a`; run
  `synthetic-run-01`; attempt `synthetic-attempt-01`; admitted input
  `synthetic-input-01`; purpose `synthetic-credential-qualification`;
  current policy revision `synthetic-policy-01` and fixture revision
  `synthetic-credential-fixture-01`.
- `R` is the **exact** resource identity
  `synthetic-tenant-a/synthetic-project-a/synthetic-run-01/synthetic-attempt-01/input/fixture-v1`,
  immutable version `synthetic-version-01`. Its only permitted response body is
  the 11 UTF-8 bytes `fixture-v1\n` (LF, no CR); SHA-256
  `c90b5a8d0d6cc502266b53f3b2b03bd97b35679f770f94b31563b468bda63543`.
- `W` is the **exact**, initially absent resource identity
  `synthetic-tenant-a/synthetic-project-a/synthetic-run-01/synthetic-attempt-01/output/candidate-v1`.
  `synthetic-payload-01` resolves in trusted fixture material to the 13 UTF-8
  bytes `candidate-v1\n`; SHA-256
  `e7100a0df24bd95895e76cc3f5c881f2243b7c30bf75b6da205cd825402b8cff`.
  This is a synthetic test object, not a grant to publish or accept a real
  transformation candidate; no future candidate digest is assumed known.

| Proposed operation / exact resource | Proposed handle authority | Required actual provider permission/constraint — **UNVERIFIED, provider not selected** |
|---|---|---|
| `read_fixture` / `R`, exact version above | At most two dispatches, 11 bytes per response; only request references `synthetic-read-01` and `synthetic-read-02`; success fields are only operation reference and digest-checked fixed fixture bytes | Read exactly that immutable object/version, with no list, sibling read, alternate version, redirect or metadata/credential-service access. Effective identity, resource and other policy interactions must be tested. |
| `create_candidate` / `W` | At most one dispatch, reference `synthetic-create-01`; only `synthetic-payload-01` (13 bytes); response fields are only operation reference plus `created`, `denied` or `unknown`, never raw provider error/URL | Create exactly that absent object with atomic no-overwrite semantics. No overwrite, delete, list, ACL/policy/metadata changes, arbitrary body, alternate destination or administration. If the selected provider cannot enforce the required create-only boundary, block/revise the proposal; a check-then-write is insufficient. |

The whole proposed run allowance is three external dispatches, at most 22 read
bytes and 13 create bytes. These tiny **fixture values are not measured or
approved UD-5 deployment budgets**. The selected profile still needs all other
finite limits. Object existence and fixture setup belong to a separately
approved trusted setup scope; these two worker operations do not create
namespaces, enumerate resources or clean up objects.

Logical requests contain exactly `operation`, `operation_ref`, `resource_ref`,
and (create only) `payload_ref`. Resolve R/W to the full identities above, not a
prefix match. Reject missing/extra/duplicate fields, mismatched references,
caller-supplied identity/policy/expiry, URL/verb/headers/body overrides, shell
commands and arbitrary signing. This is the logical operation payload, not the outer IPC envelope. Under the
UD-2 proposal it is frozen as admitted material resolved through
`core.tool.params.args.parameters_ref`; the operation registry binds
`operation_ref` to the exact payload schema and R/W resolution. These synthetic
names are logical fixture aliases, not valid `ref:` wire IDs. Digest-checked
responses likewise become admitted material referenced by `result_ref`, not
inline bytes added to the wire response. Core must check both layers; operation
contract approval and their end-to-end mapping remain outstanding. The opaque handle belongs to an authenticated
attempt channel/trusted context, not request text, prompts, logs or checkpoints.
Possessing/copying an identifier alone conveys no authority. A handle's narrow
allowlist cannot prove the backing provider principal lacks broader authority.

## Lifecycle proposal

1. Before **every** dispatch, authenticate the channel and independently bind
   tenant/project/run/attempt, admitted input, purpose, fixture revision,
   operation/resource/version/payload and current policy revision. Intersect
   declaration, caller authority, operator policy, scope, mode and remaining
   cumulative run budget. Missing, stale or unverifiable authority fails closed;
   no raw-credential, ambient-provider or direct-SDK fallback exists.
2. Fixture validity is the half-open synthetic interval
   `[2030-01-01T00:00:00Z, 2030-01-01T00:01:00Z)`: time equal to expiry is denied.
   These are test-clock inputs, not a provider lifetime/latency guarantee.
   The validity check, budget reservation, operation deduplication and
   revocation/cancellation check must share an authoritative dispatch order.
   A fence ordered first prevents dispatch, including concurrently queued work;
   a request already dispatched before the fence remains an in-flight effect.
   Any policy change invalidates this fixture handle pending fresh authorization.
3. Reserve/record the operation before dispatch; repeated operation references
   never dispatch again. Do not refund consumed or uncertain dispatches, reset
   the run allowance on retry, or assume an intent record proves an external
   call occurred. If crash recovery cannot distinguish intent from dispatch,
   count it conservatively against budget and block pending reconciliation.
   Audit/authority outage blocks dependent dispatch, but not containment,
   revocation or termination. Durability/ordering mechanism remains UD-4.
4. Cancellation, revocation, expiry or terminal state fences all new use.
   Distinguish this **local dispatch fence** from provider credential expiry,
   provider-side revocation and already-dispatched effects. Measure residual
   authority and collateral scope later; no immediate remote revocation or
   recall of data/effects is claimed.
5. A lost write acknowledgement yields `unknown`: block dependent work and
   blind replay, preserve consumed budget and safe operation/effect references,
   and require trusted reconciliation. No automatic read/list permission is
   implied for reconciliation; separately approve any necessary provider action.
   Even an already existing object is not proof this attempt created it.
6. Restart cannot reactivate checkpointed handles. Recover current policy,
   authoritative fences, budget, effect and cleanup state or remain blocked.
   A retry needs a fresh decision and distinct attempt-bound handle/resource
   identities within the remaining run allowance; a terminal attempt never
   reopens. The exact second-attempt identities require a new reviewed fixture,
   not a wildcard expansion of W. Unknown effects must first be reconciled.

No credential values belong in prompts, diagnostics, audit/checkpoint records
or exports. Provider errors, responses and presigned URLs are untrusted and may
contain authority; return only the closed classified response above. Authorized
recipients may retain/encode any bytes they legitimately receive. A broker
reduces raw credential exposure, not all exfiltration or misuse: allowed read
content, object names/bodies, outputs and timing remain potential channels.
This fixture fixes names/body references and classifies responses, while later
egress qualification must test the remaining channels. Generic masking is not
proof of safety. See the [synthetic-canary/vector plan](../qualification/credential-cases.md); no enforcement,
leakage detector or provider-permission test has run.

## Primary-source evidence and limits
AWS is illustrative, **not a selected cloud provider**. Session-policy intersection does not cover every resource-policy case: direct same-account role-session ARN grants are not constrained by implicit denies in identity/session/boundary policies; explicit denies still apply. Effective permission tests must cover the complete policy/resource topology. [AssumeRole access control](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_assumerole.html), [policy boundaries](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html).

Role-session revocation can have wider role-session impact and policy propagation is eventually consistent; this run establishes no fixed latency. [Revocation](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_revoke-sessions.html), [consistency](https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot.html#troubleshoot_general_eventual-consistency). Retrieved in bounded official-document review 2026-10-22; no credentials, account or live resource inspected.

## Closure
Require approved operation/provider surface, effective least-privilege resource policy, credential lifetime and measured residual-authority/revocation behavior, leakage tests, broker fencing/restart/effect-reconciliation evidence, owner identities and bound revision/hash. All MISSING. [`../qualification/credential-cases.md`](../qualification/credential-cases.md) is a plan only; UD-3/TASK-003 remain incomplete.
