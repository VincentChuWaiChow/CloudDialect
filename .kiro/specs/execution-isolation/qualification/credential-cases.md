# Credential qualification plan — TASK-003

Status: **NOT RUN — enforcement and provider qualification**. The 2026-10-30
work is OFFLINE review of proposed fixture descriptions only. No mock broker,
runtime enforcement, credential inspection, native sandbox or provider call is
implemented/run by this packet. Later synthetic placement/execution requires
separate disposable-environment approval; no live mutation or host-secret
probing is authorized. UD-3/TASK-003 remain OPEN/incomplete.

## Fixture and count convention

Use only the exact context, R/W, request references, fixed payload and half-open
test-clock validity interval in the [proposed operation contract](../decisions/UD-3-credentials.md#proposed-operation-contract--review-fixture-only).
Each row/subvector is an independent fresh scenario unless a prefix is stated.
`pre + new = total` counts **external operation dispatches**, not proposals or
effects. `pre` is work already dispatched before the fence/stimulus; `new` is
dispatches allowed after it. Effect count is observed separately and may remain
unknown. All actual dispatch counts in this offline packet are **zero**; table
counts are expectations for future authorized tests, not measured results.

Evidence keys: `A` independently observed authorization/context, decision order,
test clock, policy/fence revisions and budget reservations; `D` trusted dispatch
observation correlated to operation references, not worker logs; `E` independently
observed exact resource/version/body/effect state or explicit unknown status;
`L` controlled leakage-channel observations and detector positive controls;
`P` selected provider's effective policy/configuration plus authorized allow/deny
tests and measured residual authority. None of these enforcement evidence sets
is supplied by the present document review.

| Stable case / concrete vector | Expected dispatches `pre + new = total` | Required result and later evidence |
|---|---|---|
| CRED-01 positive read | `0 + 1 = 1` | Correct authenticated context, `synthetic-read-01`, `read_fixture`, R/exact version, valid clock: return only operation reference and digest-checked fixed 11 fixture bytes; no credential/URL/raw error. A,D,E,L. |
| CRED-01 positive create | `0 + 1 = 1` | `synthetic-create-01`, `create_candidate`, absent W, fixed `synthetic-payload-01`: one 13-byte creation, no other object touched. Independent preexisting W subvector is also `0 + 1 = 1`: provider atomically denies the attempted create, zero writes, not check-then-write. A,D,E,P. |
| CRED-01 finite allowance / duplicate | `3 + 0 = 3` | Prefix read-01, read-02, create-01 exhausts three-call run allowance. Duplicate reference or any fourth request makes no further call; the create count remains one. Byte counts stay within 22 read / 13 create bytes; these are fixture limits only. A,D,E. |
| CRED-02 ambient | `0 + 0 = 0` | Approved future synthetic sentinels in fake environment/file/agent/metadata channels must not be acquired by unapproved worker/descendants, including startup. Never probe real host secrets. A,D,L and independently observed boundary denial; documentation cannot prove absence. |
| CRED-03 wrong scope | `0 + 0 = 0` each | Independently substitute tenant `synthetic-tenant-b`, **same-tenant project `synthetic-project-b`**, run `synthetic-run-02`, attempt `synthetic-attempt-02`, input `synthetic-input-02` or another purpose while retaining the original handle. Deny every mismatch; caller fields cannot rebind trusted channel context. A,D. |
| CRED-03 wrong resource / operation | `0 + 0 = 0` each | R's sibling `fixture-v2`, other version, W's sibling `candidate-v2`, read W, create R, overwrite/delete/list/admin or alternate payload reference: deny; no prefix/wildcard authority. A,D,E. |
| CRED-03 malformed / extra fields | `0 + 0 = 0` each | Missing/duplicate/extra field, create body/URL/verb/header override, read with payload field, forged policy/expiry or copied handle on another channel: reject before dispatch. UD-2 representation must preserve these semantics. A,D. |
| CRED-04 confused deputy | `0 + 0 = 0` each | Redirect/alternate endpoint supplied in request, generic signing or shell execution, or an allowed operation with a forbidden target: reject without contact. If an already dispatched read returns a redirect, `1 + 0 = 1`: do not follow it or expose authority-bearing response. A,D,E,L. |
| CRED-04 offline dependency | `0 + 0 = 0` | Even an otherwise valid R/W request in run offline mode is blocked before any external call; no credential/provider fallback. A,D and run-scoped network observation. |
| CRED-05 expiry boundary | `0 + 1 = 1` just before; `0 + 0 = 0` at/after | Independent test-clock inputs `2030-01-01T00:00:59.999Z`, `00:01:00Z`, `00:01:00.001Z` on the same date. Check validity at authoritative dispatch, not merely at enqueue. Unverifiable time also denies (`0 + 0 = 0`). A,D. |
| CRED-05 revoked / policy changed | `0 + 0 = 0` each | Revoke before dispatch or replace current `synthetic-policy-01` with `synthetic-policy-02`; old handle cannot call. Provider revocation/residual scope measured separately, not inferred from local denial. A,D,P. |
| CRED-05 cancel first, concurrent queue | `0 + 0 = 0` | Enqueue read-01/read-02/create-01, then order cancellation **before any dispatch**: fence all three; queue admission is not dispatch permission. A,D. |
| CRED-05 dispatch first, cancel then queue | `1 + 0 = 1` | Dispatch create-01, then cancellation, then release queued reads: zero further calls, already-dispatched creation may complete (0/1/unknown effect). No claim of recall, immediate remote revocation or acceptance. A,D,E,P. |
| CRED-06 restart / old attempt | `0 + 0 = 0` each | Restart with old checkpointed handle, revoke then restart, or present attempt-01 handle after separately authorized attempt-02: no resurrection. A crash after one dispatch gives `1 + 0 = 1`; unknown intent-vs-dispatch blocks conservatively rather than reporting zero consumption. A,D,E. |
| CRED-06 fresh retry prerequisite | `1 + 0 = 1` until reauthorized | Prefix one reconciled prior dispatch. Retry stays blocked until current policy, fresh handle, separately reviewed exact attempt-02 resources, confirmed cleanup and remaining shared budget exist. A later authorized call would total `1 + 1 = 2`, never reset to zero. No attempt-02 grant is approved here. A,D,E. |
| CRED-07 unknown external write | `1 + 0 = 1` | Lose acknowledgement after create-01 dispatch. Effect is explicitly unknown; block replay/dependent work, preserve budget and operation reference. No automatic reconciliation read/list or new-attempt write. Reconcile with separately authorized trusted evidence before retry decision. A,D,E. |
| CRED-08 leak channels | `0 + 0 = 0` for denied exports | Place synthetic canaries only in approved fixture streams: stdout/stderr/crash/tool error, encoded variants, model context, audit/checkpoint/export. Raw secret-bearing/unknown channels withheld; safe receipts contain case IDs/booleans, not payload. Positive detector control plus negative output observation required; no universal detector claim. A,D,L. |
| CRED-08 authorized-recipient abuse | read `0 + 1 = 1`, then forbidden export `1 + 0 = 1` | An authorized read recipient tries to encode returned fixture bytes into new object name/body, diagnostic, model route or allowed-tool arguments. Fixed W/payload refs prevent broker widening; independent egress policy must deny unauthorized channels. A substituted canary response fails the fixed-body digest check, not a valid read. Legitimately released bytes cannot be recalled; report timing/covert-channel and retention residual risks, not perfect secrecy. A,D,L. |
| CRED-09 outage | `0 + 0 = 0`, or `1 + 0 = 1` with one in flight | Lose audit or authority before dispatch/cleanup: deny dependent calls/acceptance while containment, revocation and termination continue. In-flight effect and missing observation stay explicit; unavailable observer is not a pass. A,D,E. |
| CRED-10 effective policy | Allowed `0 + 1 = 1`; broker-denied `0 + 0 = 0` | Future provider-specific fixture must test full identity/resource-policy topology and direct-session grants where relevant, exact scope and atomic create-only behavior, expiry/revocation propagation and collateral scope. Separately authorized direct-provider deny probes may require a rejected request; record that dispatch separately, never mislabel it zero or grant it to the worker. P,D,E. |

## Synthetic canaries, evidence and open review

`SYNTHETIC_NOT_A_CREDENTIAL_UD3_A` and
`SYNTHETIC_NOT_A_CREDENTIAL_UD3_B` are public **noncredential test strings**, not
secrets, handles or evidence that a detector works. Plan direct, base64 and
fragmented representations with separately controlled positive observation;
do not place them in real credential stores, query host secrets or imply actual
leakage results. Future qualification exports case IDs, detector outcomes,
channel/observer identities and safe counts rather than canary-bearing raw
streams. Authorized raw-byte recipients can copy or transform data; this is an
explicit residual risk for every proposed mechanism, not solved by masking.

Record fixture/policy/profile revisions, authenticated scope, input and operation
identities, authoritative event ordering, consumed/remaining run budget,
dispatch/effect counts, clock inputs, measured expiry/revocation/cleanup and
observer coverage. Record denied provider attempts separately from successful
effects. Bind future approved fixture bytes and suite/executable identities to
the report. Missing D/E/P/L observation remains **UNVERIFIED**, not a zero/pass.

Offline review can check internal consistency, exact scoping, traceable cases
and expected counts only. Effective provider permissions, durable atomic
ordering, leakage/ambient containment, real restart/retry behavior, lifecycle
measurements, provider/mechanism selection and named security/operations owner
approval are **MISSING**. No CRED case or UD-3 gate is closed by this plan.
