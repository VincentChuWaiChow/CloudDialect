# UD-3 — scoped credential-use proposal

Status: **OPEN — no delivery mechanism/provider approved or tested**. TASK-003, 2026-10-22. Security/operations approvers not supplied. REQ-004/005/006/010; DES-005/006/010.

## Comparison
| Option | Benefit | Exfiltration/revocation limitation |
|---|---|---|
| Environment | Compatible with many SDKs | Recipient obtains reusable bytes and can copy/encode them; clearing environment later does not recall copies. |
| Private mounted file / descriptor | Less broad inheritance than ambient environment | Authorized reader can retain bytes; unlink/close is not provider revocation. |
| Narrow operation broker with opaque attempt-bound handle | Worker need not receive provider secrets; policy checked at each operation | Additional trusted service/API; must not become arbitrary HTTP/signing proxy; cannot retract in-flight effects. |

Recommendation (inference): evaluate broker-mediated synthetic test operations first, with no raw provider credentials or ambient fallback in workers. Compatibility limitation: a candidate requiring arbitrary direct SDK calls is not automatically supported; approve a bounded operation interface or block, never secretly inject broader credentials. Broker responses, provider errors and presigned URLs can themselves carry sensitive data/authority and need egress classification.

## Lifecycle proposal
Bind handle to authenticated tenant/project/run/attempt, concrete operation/test resource/purpose, policy revision, expiry and remaining budget. Resolve authorization outside model/candidate context on every dispatch. Cancellation/revocation fences new operations before retry; broker restart cannot resurrect grants from a checkpoint. Retry requires fresh policy/credential decision.

Distinguish local dispatch fencing, provider-side credential expiry/revocation, and already-dispatched external effects. Unknown write outcome blocks replay until reconciled; no claimed immediate provider revocation or automatic safe retry. No credential values in prompts, diagnostic output, audit/checkpoint records or exported evidence. Do not rely on generic masking to make arbitrary raw streams safe.

## Primary-source evidence and limits
AWS is illustrative, **not a selected cloud provider**. Session-policy intersection does not cover every resource-policy case: direct same-account role-session ARN grants are not constrained by implicit denies in identity/session/boundary policies; explicit denies still apply. Effective permission tests must cover the complete policy/resource topology. [AssumeRole access control](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_assumerole.html), [policy boundaries](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html).

Role-session revocation can have wider role-session impact and policy propagation is eventually consistent; this run establishes no fixed latency. [Revocation](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_revoke-sessions.html), [consistency](https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot.html#troubleshoot_general_eventual-consistency). Retrieved in bounded official-document review 2026-10-22; no credentials, account or live resource inspected.

## Closure
Require approved operation/provider surface, effective least-privilege resource policy, credential lifetime and measured residual-authority/revocation behavior, leakage tests, broker fencing/restart/effect-reconciliation evidence, owner identities and bound revision/hash. All MISSING. [`../qualification/credential-cases.md`](../qualification/credential-cases.md) is a plan only; UD-3/TASK-003 remain incomplete.
