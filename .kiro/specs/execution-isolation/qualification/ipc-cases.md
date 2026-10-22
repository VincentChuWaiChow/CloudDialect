# IPC qualification plan — TASK-002

Status: **NOT RUN**. UD-2 proposal is not activated; UD-5 values remain unset. Execute only after the wire schema, phase table and approved environment exist.

| Case | Stimulus | Expected observation |
|---|---|---|
| IPC-01 positive | Supported hello then valid execute/result | Only admitted operation dispatched; output frozen by core; no automatic acceptance. |
| IPC-02 startup | Wrong JSON-RPC/application version, nonce, message before hello, handshake stalls | No worker operation; bounded stop/cleanup, no downgrade. |
| IPC-03 framing | Split headerless LF frame across writes, multi-frame write, EOF mid-frame, invalid UTF-8 | Valid fragmentation preserved; malformed/incomplete input rejected without dispatch. |
| IPC-04 shape | Array/batch, scalar, duplicate keys, unknown field, result+error, invalid params/id | Profile violation closes dispatch; safe bounded diagnostic only. |
| IPC-05 limits | Frame/queue/nesting/output at below/equal/above approved bounds | Defined exact-bound behavior; pre-allocation cap; excess cannot consume unbounded memory. |
| IPC-06 replay | Duplicate ID, repeated response, reordered result, wrong attempt token, prior-attempt replay | No duplicate operation or identity rebinding. |
| IPC-07 authority | Valid tool request with forbidden resource, widened project/policy or deploy/delete | External policy denies; zero external dispatch. |
| IPC-08 cancellation | Cancel while stdin/stdout/stderr pipe is full or worker is silent | No new operations; termination starts independently; cleanup deadline enforced. |
| IPC-09 late result | Success text or output after cancel/exhaustion/revocation/terminal state | No accepted transition, stale evidence discarded. |
| IPC-10 diagnostics | Protocol text on stderr, log text on stdout, synthetic secret in either | No acceptance from text; raw diagnostics not persisted/exported. |
| IPC-11 channel | Worker forges nonce, claims trusted observer, replaces result file | No observation-authenticity receipt; independent observer required. |
| IPC-12 failures | Worker crash/disconnect/normal exit with surviving descendants | Whole-domain cleanup; reservation retained/quarantine if unconfirmed. |

Harness must exercise Rust core and at least one independent worker implementation against the same approved schema; unit parser passes alone are insufficient. Record exact byte fixtures, protocol/schema/profile hashes, observed dispatch count, peak queue/allocation, deadlines and cleanup receipts. Seed known-good and known-bad controls; prove every negative actually reaches the intended boundary. No raw credentials or untrusted secret-bearing stream bytes in reports.
