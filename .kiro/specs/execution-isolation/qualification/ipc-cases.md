# IPC qualification plan — TASK-002

Status: **OFFLINE DRAFT; actual interoperability / native qualification NOT RUN**.
UD-2 is not activated; UD-5 values remain unset. The synthetic vectors below may
be checked locally as document consistency only. Execute worker/transport cases
only after the wire contract, finite profile and environment are approved.

## Decoded-message shape vectors (synthetic, offline only)

`valid` means only “matches the proposed Draft 2020-12 message schema.” It does
not mean admitted, authorized, authenticated, executed or accepted. Positive
responses use the same IDs as their example requests; the shape checker does not
keep a pending-request table. The examples contain synthetic references only;
they do not identify actual artifacts, secrets, providers or operation approvals.

<!-- offline-proposal: ipc-vectors -->
```json
[
  {"id":"shape-hello-request","valid":true,"message":{"jsonrpc":"2.0","id":"c:h1","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1"}}},
  {"id":"shape-hello-response","valid":true,"message":{"jsonrpc":"2.0","id":"c:h1","result":{"kind":"hello","protocol":"clouddialect-worker/1","nonce":"ref:nonce1"}}},
  {"id":"shape-execute-request","valid":true,"message":{"jsonrpc":"2.0","id":"c:e1","method":"worker.execute","params":{"attempt_token":"ref:attempt1","input_ref":"ref:input1","operation_ref":"ref:op1","output_slot_ref":"ref:slot1"}}},
  {"id":"shape-execute-response","valid":true,"message":{"jsonrpc":"2.0","id":"c:e1","result":{"kind":"candidate","output_ref":"ref:output1"}}},
  {"id":"shape-tool-request","valid":true,"message":{"jsonrpc":"2.0","id":"w:t1","method":"core.tool","params":{"attempt_token":"ref:attempt1","operation_ref":"ref:op1","args":{"kind":"references","input_ref":"ref:input1","parameters_ref":"ref:params1"}}}},
  {"id":"shape-tool-response","valid":true,"message":{"jsonrpc":"2.0","id":"w:t1","result":{"kind":"tool","result_ref":"ref:toolresult1"}}},
  {"id":"shape-cancel-request","valid":true,"message":{"jsonrpc":"2.0","id":"c:x1","method":"worker.cancel","params":{"attempt_token":"ref:attempt1","target_id":"c:e1","reason":"USER_CANCELLED"}}},
  {"id":"shape-cancel-response","valid":true,"message":{"jsonrpc":"2.0","id":"c:x1","result":{"kind":"cancel","target_id":"c:e1","acknowledged":true}}},
  {"id":"shape-hello-error","valid":true,"message":{"jsonrpc":"2.0","id":"c:h1","error":{"code":1000,"message":"VERSION_MISMATCH"}}},
  {"id":"shape-execute-error","valid":true,"message":{"jsonrpc":"2.0","id":"c:e1","error":{"code":1009,"message":"OPERATION_FAILED"}}},
  {"id":"shape-tool-error","valid":true,"message":{"jsonrpc":"2.0","id":"w:t1","error":{"code":1002,"message":"PERMISSION_DENIED"}}},
  {"id":"shape-cancel-error","valid":true,"message":{"jsonrpc":"2.0","id":"c:x1","error":{"code":-32603,"message":"Internal error"}}},
  {"id":"shape-unknown-envelope-field","valid":false,"message":{"jsonrpc":"2.0","id":"c:h1","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1"},"accept":true}},
  {"id":"shape-unknown-param-field","valid":false,"message":{"jsonrpc":"2.0","id":"c:h1","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1","project":"other"}}},
  {"id":"shape-null-request-id","valid":false,"message":{"jsonrpc":"2.0","id":null,"method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1"}}},
  {"id":"shape-numeric-id","valid":false,"message":{"jsonrpc":"2.0","id":1,"method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1"}}},
  {"id":"shape-id-trailing-newline","valid":false,"message":{"jsonrpc":"2.0","id":"c:h1\n","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1"}}},
  {"id":"shape-ref-trailing-newline","valid":false,"message":{"jsonrpc":"2.0","id":"c:h1","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1\n"}}},
  {"id":"shape-notification","valid":false,"message":{"jsonrpc":"2.0","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1"}}},
  {"id":"shape-batch","valid":false,"message":[]},
  {"id":"shape-scalar","valid":false,"message":"worker ready"},
  {"id":"shape-result-plus-error","valid":false,"message":{"jsonrpc":"2.0","id":"c:e1","result":{"kind":"candidate","output_ref":"ref:output1"},"error":{"code":1008,"message":"CANCELLED"}}},
  {"id":"shape-array-params","valid":false,"message":{"jsonrpc":"2.0","id":"c:h1","method":"worker.hello","params":["clouddialect-worker/1","ref:nonce1"]}},
  {"id":"shape-missing-param","valid":false,"message":{"jsonrpc":"2.0","id":"c:e1","method":"worker.execute","params":{"attempt_token":"ref:attempt1","input_ref":"ref:input1","operation_ref":"ref:op1"}}},
  {"id":"shape-jsonrpc-version","valid":false,"message":{"jsonrpc":"1.0","id":"c:h1","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:nonce1"}}},
  {"id":"shape-application-version","valid":false,"message":{"jsonrpc":"2.0","id":"c:h1","method":"worker.hello","params":{"protocol":"clouddialect-worker/2","nonce":"ref:nonce1"}}},
  {"id":"shape-unknown-method","valid":false,"message":{"jsonrpc":"2.0","id":"w:t1","method":"core.accept","params":{}}},
  {"id":"shape-wrong-id-issuer","valid":false,"message":{"jsonrpc":"2.0","id":"c:t1","method":"core.tool","params":{"attempt_token":"ref:attempt1","operation_ref":"ref:op1","args":{"kind":"references","input_ref":"ref:input1","parameters_ref":"ref:params1"}}}},
  {"id":"shape-inline-command","valid":false,"message":{"jsonrpc":"2.0","id":"w:t1","method":"core.tool","params":{"attempt_token":"ref:attempt1","operation_ref":"ref:op1","args":{"kind":"command","command":"synthetic-do-not-run"}}}},
  {"id":"shape-provider-field","valid":false,"message":{"jsonrpc":"2.0","id":"w:t1","method":"core.tool","params":{"attempt_token":"ref:attempt1","operation_ref":"ref:op1","args":{"kind":"references","input_ref":"ref:input1","parameters_ref":"ref:params1","provider":"unselected"}}}},
  {"id":"shape-cancel-arbitrary-reason","valid":false,"message":{"jsonrpc":"2.0","id":"c:x1","method":"worker.cancel","params":{"attempt_token":"ref:attempt1","target_id":"c:e1","reason":"untrusted free text"}}},
  {"id":"shape-accepting-result","valid":false,"message":{"jsonrpc":"2.0","id":"c:e1","result":{"kind":"candidate","output_ref":"ref:output1","accepted":true}}},
  {"id":"shape-error-data","valid":false,"message":{"jsonrpc":"2.0","id":"w:t1","error":{"code":1002,"message":"PERMISSION_DENIED","data":"synthetic-sensitive-marker"}}},
  {"id":"shape-error-free-text","valid":false,"message":{"jsonrpc":"2.0","id":"w:t1","error":{"code":1002,"message":"synthetic-sensitive-marker"}}},
  {"id":"shape-error-wrong-code-message","valid":false,"message":{"jsonrpc":"2.0","id":"w:t1","error":{"code":1002,"message":"OFFLINE_DEPENDENCY"}}},
  {"id":"shape-null-response-id","valid":false,"message":{"jsonrpc":"2.0","id":null,"error":{"code":-32600,"message":"Invalid Request"}}}
]
```

## Framing and state scenarios (separate; NOT RUN)

These are concrete proposed fixtures/procedures, **not executed by the JSON
Schema checker**. A JSON decoder may erase duplicate keys; decoded vectors above
cannot test that boundary. Byte notation below uses `\n = 0a`, `\r = 0d`, and
`\xff = ff`; text between them is literal ASCII/UTF-8.

| Scenario | Concrete stimulus / context | Expected observation |
|---|---|---|
| FRAME-01 valid fragmentation | Serialize `shape-hello-request` compactly + `\n`; split after byte 1, then before LF; alternatively send two independently valid frames in one write | Only complete LF-terminated messages parsed; no partial dispatch. |
| FRAME-02 duplicate key | `{"jsonrpc":"2.0","id":"c:h1","id":"c:h2","method":"worker.hello","params":{"protocol":"clouddialect-worker/1","nonce":"ref:n"}}\n` | Duplicate-aware parser rejects before schema; no reply using either ambiguous ID; stop/contain. Also repeat a key in nested `params`. |
| FRAME-03 invalid encoding | Insert byte `ff` into nonce string before LF; separately prepend UTF-8 BOM `ef bb bf` | Reject before decoded-message handling, no operation or null-ID reply. |
| FRAME-04 incomplete/malformed | Send `{"jsonrpc":"2.0"}` then EOF without LF; separately send `not-json\n`, blank `\n`, CRLF or a literal LF within a string | Reject framing/JSON safely; no operation. Escaped `\\n` is not a delimiter inside the decoded string. |
| FRAME-05 approved caps | Parameterize a valid frame/queue/collection at `B-1`, `B`, `B+1` for each positive finite approved UD-5 byte/item bound; analogous nesting boundary and overflow counter input | Exact-bound behavior recorded; `B+1` rejected without unbounded allocation. Currently UNSET, no run or numerical budget claim. |
| STATE-01 supported path | hello request/result; execute; tool request/result; execute candidate result | One admitted tool operation at most; candidate remains non-accepting pending core identity freeze, cleanup and independent verification. |
| STATE-02 nonce mismatch | `shape-hello-response`, but `nonce = ref:other`, matching pending `c:h1` | Shape valid, semantic failure: stop, execute dispatch count zero. |
| STATE-03 request before hello | Send shape-valid `shape-tool-request` before hello completion | No registry use/external dispatch; stop. |
| STATE-04 wrong correlation | While `c:e1` pending, send candidate result with `id = c:other`; separately send hello-shaped result with `id = c:e1` | Both can match schema; pending ID/method/kind check rejects; no acceptance. |
| STATE-05 duplicate/replay | Send `shape-tool-request` twice while pending and again after completion; replay with `attempt_token = ref:old` on another channel | No duplicate operation or rebinding, stop; old/terminal responses dropped. |
| STATE-06 unresolved operation | Executing context, valid tool request but `operation_ref = ref:unknown`; separately wrong `parameters_ref` payload type/identity | Schema-valid; lookup returns 1003 for unknown op; typed payload failure denied before use; no guessed provider/action. |
| STATE-07 references/authority | Valid shape but foreign-project input/output/result reference, forbidden operation or widened parameters in referenced artifact | Scope/identity/type/policy checks deny before use; output outside admitted slot never freezes as candidate. |
| STATE-08 cancel versus pipe | Core orders cancellation of pending `c:e1` before acceptance; stdin/stdout/stderr full or worker silent; try `worker.cancel`/ACK if writable | Dispatch/acceptance fenced and full-domain termination initiated without any write/ACK/audit wait; deadline and independent cleanup checked. No rollback assertion for dispatched effects. |
| STATE-09 late result / ACK | After STATE-08, send `shape-execute-response`, tool result and `shape-cancel-response`; separately ACK with wrong `target_id` | Candidate/tool output dropped; matching ACK advisory only; mismatched ACK rejected. No state restoration or cleanup receipt. |
| STATE-10 acceptance ordering | Order cancellation before durable acceptance, then reverse that ordering in a separate fixture | Before: no accepted terminal state. After: committed outcome read back, no reopening/replay. Schema cannot grade ordering. |
| STATE-11 blocked resource bounds | Flood ignored/invalid stdout and stderr while queues/full pending table are at approved bounds | All bytes charged; bounded storage/ID tracking, independent stop still works. No raw diagnostics persisted. |
| STATE-12 unknown effect | Simulate operation dispatch, then lost result; schema-valid 1010 error or process disconnect | Record unresolved disposition, stop/reconcile, no blind retry; a worker's claim is not an authenticated effect receipt. |

## Future interoperability and containment qualification

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
