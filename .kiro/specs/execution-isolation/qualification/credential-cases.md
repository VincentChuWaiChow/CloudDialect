# Credential qualification plan — TASK-003

Status: **NOT RUN**. Synthetic canaries/resources only after disposable-environment approval. No live cloud mutation or actual host-secret probing authorized.

| Case | Stimulus | Required observation |
|---|---|---|
| CRED-01 positive | Bound valid test operation, correct resource and policy | Exactly authorized operation dispatches; worker gets only classified response, not secret bytes. |
| CRED-02 ambient | Synthetic outside-domain environment/file/agent/metadata credentials | Unapproved worker has neither values nor acquisition channels, including descendants/startup. |
| CRED-03 scope | Wrong tenant/project/attempt/action/resource/purpose; replay handle | Denied before dispatch, including same-tenant cross-project. |
| CRED-04 confused deputy | Redirect, alternate endpoint, arbitrary signing, allowed API with forbidden target | Broker cannot widen destination or effective authority. |
| CRED-05 expiry/revocation | Expire/revoke/cancel during active work, race with dispatch | New use fenced in authoritative order; measured in-flight/provider residual authority stated honestly. |
| CRED-06 retry/restart | Broker crash, old checkpoint, old attempt handle | Fresh authorization required; terminal attempt never reopens. |
| CRED-07 effect uncertainty | External write timeout/lost acknowledgement | Uncertain outcome recorded and reconciled; no blind retry. |
| CRED-08 leaks | Canary in stdout/stderr/crash/tool response, encoded variants, prompts/checkpoints/audit/exports | Secret-bearing raw channels withheld; no persistent/external leak within declared policy and observer scope. |
| CRED-09 outage | Audit/authority unavailable during cleanup | Dependent dispatch blocked; containment and revocation still proceed. |
| CRED-10 effective policy | Provider-specific direct session-resource grant, policy change, revocation propagation | Effective authority bounded, no production deploy/delete/cross-project test privilege; collateral scope measured. |

Record grant/policy/profile/fixture identities, dispatch/effect counts and authoritative ordering, measured expiry/revocation/cleanup outcomes and safe observer references. Positive controls must show detector sensitivity without exporting canary values. A synthetic suite cannot prove actual provider permissions; any later live-resource check needs separately authorized account/resources/actions/lifetime.
