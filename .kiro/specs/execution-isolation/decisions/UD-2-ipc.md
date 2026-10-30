# UD-2 — bounded worker IPC proposal

Status: **OPEN — OFFLINE DRAFT, not an approved wire contract**. Prepared 2026-10-22; detailed 2026-10-30 for TASK-002. Owners: core/worker owners, identities and approvals not supplied. REQ-001/003/006/010; DES-001/006/010.

## Facts and alternatives
Context7 resolved then queried `/websites/jsonrpc` (primary specification coverage, rather than a language-specific implementation). JSON-RPC 2.0 defines messages, correlation and standard errors, but is transport agnostic. It does not provide this application's framing, worker authentication, cancellation, backpressure or acceptance authority. [Primary specification](https://www.jsonrpc.org/specification), fetched 2026-10-22; response/error semantics refreshed through Context7 resolve/query on 2026-10-30.

Alternatives: custom length-prefixed messages reduce JSON-RPC flexibility but create a new protocol; HTTP adds a listener/authentication surface; JSON-RPC on private stdio avoids an additional listener but still requires an application profile. Recommendation (inference): evaluate a deliberately restricted JSON-RPC/stdin-stdout profile first. No library or transport is selected by this record.

## Proposed transport and compatibility profile

Everything below is a proposal for owner review, not an activated profile.
Core creates private pipes through the approved isolation launcher: stdin/stdout
carry protocol only; stderr is untrusted diagnostics. Neither stream is persisted
raw. No inherited host IPC endpoints, listener or descriptor passing is added.

Frames are UTF-8 without BOM: one compact JSON object followed by LF, without
leading/trailing whitespace or literal CR/LF inside the JSON text (escaped string
newlines are allowed). Fragmentation across reads and several frames in one read
are valid. Blank frames, EOF before LF, invalid UTF-8 and duplicate object keys at
any depth are rejected before schema validation. This is application framing,
not framing defined by JSON-RPC. Batches, notifications and null/numeric IDs are
deliberately unsupported; this is **not full JSON-RPC interoperability**.

`jsonrpc` is exactly `2.0`; the separate application version is exactly
`clouddialect-worker/1`. A single core-initiated hello must complete before execute.
Its response echoes the exact offered version and nonce. No version list,
fallback, downgrade or reconnect-based privilege resumption exists. An unsupported
application version returns proposed code 1000 if safely correlatable, then stops;
wrong JSON-RPC version/nonce, wrong hello response or hello timeout stops without
execute. A future version requires a separately reviewed schema and compatibility
fixtures; `/1` never silently accepts additional fields.

## Exact proposed message shapes

The following Draft 2020-12 schema describes complete decoded messages, not a
parser or authorization engine. Every object is closed. `c:` IDs originate at the
core, `w:` IDs at the worker; their suffixes are proposed nonempty ASCII lexical
constraints, **not measured budgets**. `ref:` values are opaque references, never
URLs, paths, credential bytes or signed grants. No numerical frame/string/array
budgets are invented: finite whole-frame, collection and parsing bounds must come
from approved UD-5 values before use.

`core.tool` has one initial operation variant: an existing `operation_ref` with
`args.kind = references`, an admitted `input_ref` and `parameters_ref`. This is a
closed reference-only union, not a provider/action union: no provider, model,
command, arbitrary inline JSON or deploy/delete operation is selected. The trusted
operation registry must resolve the reference in the admitted context, reject
unknown operations (1003), and validate the referenced parameters against that
operation's exact typed schema **before** policy, quota, credential and concrete
use checks. A reference match is not permission. The referenced payload and its
schema identity/size are independently bounded and admitted; this schema alone
cannot validate their contents. An enabled operation family needs its own approved
contract; none is enabled by this draft.

<!-- offline-proposal: ipc-schema -->
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "urn:clouddialect:offline-proposal:worker-ipc:1",
  "oneOf": [
    {"$ref": "#/$defs/helloRequest"}, {"$ref": "#/$defs/executeRequest"},
    {"$ref": "#/$defs/toolRequest"}, {"$ref": "#/$defs/cancelRequest"},
    {"$ref": "#/$defs/helloResponse"}, {"$ref": "#/$defs/executeResponse"},
    {"$ref": "#/$defs/toolResponse"}, {"$ref": "#/$defs/cancelResponse"},
    {"$ref": "#/$defs/errorResponse"}
  ],
  "$defs": {
    "coreId": {"type": "string", "pattern": "^c:[A-Za-z0-9._-]+$", "not": {"pattern": "[\\r\\n]"}},
    "workerId": {"type": "string", "pattern": "^w:[A-Za-z0-9._-]+$", "not": {"pattern": "[\\r\\n]"}},
    "id": {"oneOf": [{"$ref": "#/$defs/coreId"}, {"$ref": "#/$defs/workerId"}]},
    "ref": {"type": "string", "pattern": "^ref:[A-Za-z0-9._-]+$", "not": {"pattern": "[\\r\\n]"}},
    "helloParams": {
      "type": "object", "additionalProperties": false, "required": ["protocol", "nonce"],
      "properties": {"protocol": {"const": "clouddialect-worker/1"}, "nonce": {"$ref": "#/$defs/ref"}}
    },
    "helloRequest": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "method", "params"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/coreId"}, "method": {"const": "worker.hello"}, "params": {"$ref": "#/$defs/helloParams"}}
    },
    "executeRequest": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "method", "params"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/coreId"}, "method": {"const": "worker.execute"}, "params": {
        "type": "object", "additionalProperties": false, "required": ["attempt_token", "input_ref", "operation_ref", "output_slot_ref"],
        "properties": {"attempt_token": {"$ref": "#/$defs/ref"}, "input_ref": {"$ref": "#/$defs/ref"}, "operation_ref": {"$ref": "#/$defs/ref"}, "output_slot_ref": {"$ref": "#/$defs/ref"}}
      }}
    },
    "toolRequest": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "method", "params"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/workerId"}, "method": {"const": "core.tool"}, "params": {
        "type": "object", "additionalProperties": false, "required": ["attempt_token", "operation_ref", "args"],
        "properties": {"attempt_token": {"$ref": "#/$defs/ref"}, "operation_ref": {"$ref": "#/$defs/ref"}, "args": {
          "oneOf": [{"type": "object", "additionalProperties": false, "required": ["kind", "input_ref", "parameters_ref"],
            "properties": {"kind": {"const": "references"}, "input_ref": {"$ref": "#/$defs/ref"}, "parameters_ref": {"$ref": "#/$defs/ref"}}}]
        }}
      }}
    },
    "cancelRequest": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "method", "params"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/coreId"}, "method": {"const": "worker.cancel"}, "params": {
        "type": "object", "additionalProperties": false, "required": ["attempt_token", "target_id", "reason"],
        "properties": {"attempt_token": {"$ref": "#/$defs/ref"}, "target_id": {"$ref": "#/$defs/coreId"}, "reason": {"enum": ["USER_CANCELLED", "BUDGET_EXHAUSTED", "CONTROL_FAILED"]}}
      }}
    },
    "helloResponse": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "result"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/coreId"}, "result": {
        "type": "object", "additionalProperties": false, "required": ["kind", "protocol", "nonce"],
        "properties": {"kind": {"const": "hello"}, "protocol": {"const": "clouddialect-worker/1"}, "nonce": {"$ref": "#/$defs/ref"}}
      }}
    },
    "executeResponse": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "result"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/coreId"}, "result": {
        "type": "object", "additionalProperties": false, "required": ["kind", "output_ref"],
        "properties": {"kind": {"const": "candidate"}, "output_ref": {"$ref": "#/$defs/ref"}}
      }}
    },
    "toolResponse": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "result"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/workerId"}, "result": {
        "type": "object", "additionalProperties": false, "required": ["kind", "result_ref"],
        "properties": {"kind": {"const": "tool"}, "result_ref": {"$ref": "#/$defs/ref"}}
      }}
    },
    "cancelResponse": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "result"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/coreId"}, "result": {
        "type": "object", "additionalProperties": false, "required": ["kind", "target_id", "acknowledged"],
        "properties": {"kind": {"const": "cancel"}, "target_id": {"$ref": "#/$defs/coreId"}, "acknowledged": {"const": true}}
      }}
    },
    "errorResponse": {
      "type": "object", "additionalProperties": false, "required": ["jsonrpc", "id", "error"],
      "properties": {"jsonrpc": {"const": "2.0"}, "id": {"$ref": "#/$defs/id"}, "error": {
        "enum": [
          {"code": -32600, "message": "Invalid Request"},
          {"code": -32601, "message": "Method not found"},
          {"code": -32602, "message": "Invalid params"},
          {"code": -32603, "message": "Internal error"},
          {"code": 1000, "message": "VERSION_MISMATCH"},
          {"code": 1001, "message": "PROTOCOL_VIOLATION"},
          {"code": 1002, "message": "PERMISSION_DENIED"},
          {"code": 1003, "message": "UNKNOWN_OPERATION"},
          {"code": 1004, "message": "OFFLINE_DEPENDENCY"},
          {"code": 1005, "message": "BUDGET_UNCONFIGURED"},
          {"code": 1006, "message": "BUDGET_EXHAUSTED"},
          {"code": 1007, "message": "ACCOUNTING_UNKNOWN"},
          {"code": 1008, "message": "CANCELLED"},
          {"code": 1009, "message": "OPERATION_FAILED"},
          {"code": 1010, "message": "EFFECT_UNKNOWN"}
        ]
      }}
    }
  }
}
```

## Proposed errors and disposition

All numeric application assignments are **proposed**, outside JSON-RPC's reserved
`-32768..-32000` range. Standard codes retain their standard meanings. Error
objects above are exact code/message pairs: no `data`, raw exception, telemetry,
provider response, secret, resource name or arbitrary text is allowed. A safe
core-owned audit event may link the request ID to an allowlisted category; worker
claims never become trusted audit facts.

| Code | Fixed message / use | Disposition |
|---|---|---|
| -32600 | Invalid Request: decoded but invalid envelope with safely usable ID | Close dispatch; contain domain. |
| -32601 | Method not found: unsupported method with safely usable ID | No operation; close dispatch. |
| -32602 | Invalid params: known method, invalid method parameters | No operation; close dispatch. |
| -32603 | Internal error: local receiver failure | Stop dependent attempt; never disclose internal detail. |
| 1000 | VERSION_MISMATCH | Reject hello; stop, no downgrade. |
| 1001 | PROTOCOL_VIOLATION: correlation/state/context violation | Stop; no execution or acceptance. |
| 1002 | PERMISSION_DENIED: current scope/policy/egress/credential-use denial | Deny operation; no automatic retry or wider fallback. |
| 1003 | UNKNOWN_OPERATION: registry lookup miss | Deny before use; no arbitrary operation discovery/fallback. |
| 1004 | OFFLINE_DEPENDENCY: operation requires an external service | Deny before any external call. |
| 1005 | BUDGET_UNCONFIGURED | Block launch/use; missing bound is not unlimited. |
| 1006 | BUDGET_EXHAUSTED | Close dispatch/acceptance; terminate domain. |
| 1007 | ACCOUNTING_UNKNOWN | Block dispatch/recovery; never reset accounting. |
| 1008 | CANCELLED | Pending work abandoned; no output promotion. |
| 1009 | OPERATION_FAILED: reconciled failure, not unknown external effect | No acceptance; no automatic retry authorization. |
| 1010 | EFFECT_UNKNOWN: operation may have taken effect | Block/reconcile; never blindly replay. |

An error is emitted at most once only for an ID unambiguously decoded on this
channel, within the active context and safe write deadline; it is not an excuse to
wait on a blocked pipe. Uncorrelatable malformed frames (including duplicate keys,
invalid encoding/JSON, missing/null/invalid ID or oversized frames) stop/contain
without an error reply. Thus this profile deliberately does **not** emit the base
JSON-RPC `id:null` parse/invalid-request replies (including code -32700). A
well-formed request normally receives one matching success or error; shutdown or
crash can leave it unanswered. Unanswered is not success or retry permission.
Errors involving terminal/old attempts are dropped, not written into a new attempt.

## Direction, state and cancellation (not schema guarantees)

IPC substates refine, but do not replace, DES-010 attempt states. `awaiting-hello`
and `ready` are within prepared; `executing` is within running. One execute request
per worker channel is proposed; any retry uses a new attempt/channel.

| Event / direction | Allowed IPC state | Required check / next state |
|---|---|---|
| `worker.hello`, core → worker | awaiting-hello; no pending hello | One core-owned nonce/version; remain awaiting-hello until exact correlated reply. |
| hello result, worker → core | awaiting-hello with pending same `c:` ID | Exact nonce/version match → ready; otherwise stopping. Nonce is association, not authentication. |
| `worker.execute`, core → worker | ready | Bound attempt/input/operation/output slot, admitted controls and reserved budget → executing. |
| `core.tool`, worker → core | executing; execute outstanding | Current attempt token, resolved typed references, policy/budget/egress checks; no state advance. |
| tool result/error, core → worker | executing with pending same `w:` ID | Authorized result reference is scoped/immutable and data-policy permitted, not raw telemetry or a grant. |
| execute candidate result, worker → core | executing with pending same `c:` ID | Output must resolve only inside admitted output slot; freeze/verify identity and independently clean descendants. Then leave worker channel; candidate is never acceptance. |
| `worker.cancel`, core → worker | stopping, after core ordered stop of outstanding execute | Separate fresh `c:` ID; target is exact outstanding execute ID. Advisory only; stop already effective. |
| cancel result, worker → core | stopping with pending matching cancel/target IDs | Advisory ACK only; no state restoration, cleanup receipt or acceptance. |
| any error response, reverse of original request | Matching outstanding ID/method and permitted phase only | Apply table above; a worker-reported category does not attest core state or effect disposition. |
| any other request/result; replay; cross-attempt token; response of wrong kind | Any | Reject without operation; close dispatch and contain; drop terminal/late output. |

IDs never repeat within an attempt, even after a response. The pending table binds
ID, issuer, method, context and expected result kind; schemas cannot establish
this correlation. IDs are retired atomically with their outcome; duplicate,
unsolicited, wrong-direction, wrong-kind or stale responses cannot reopen work.
Exactly matching synthetic IDs in the examples prove shape only, not runtime
correlation. Opaque attempt tokens match core-held authenticated tenant/project/
run/attempt/policy bindings; worker strings cannot change those bindings.

The authoritative core ordering of cancel/revocation/budget exhaustion versus
acceptance is independent of pipe processing. If stop wins before durable
acceptance, fence **all new run operations and acceptance immediately**, invalidate
pending worker output and start whole-domain termination/revocation and cleanup
deadline without waiting for a cancel write, ACK, stderr drain or audit storage.
Already-dispatched external effects require reconciliation, not a claim of rollback.
If acceptance already durably committed, report that outcome without reopening or
retroactively cancelling it. Unconfirmed cleanup retains reservations/quarantine.

## Bounds and security obligations outside the schema

Approved UD-5 profile symbols, all currently **UNSET**, are required for
`MAX_FRAME_BYTES`, `MAX_NESTING`, `MAX_COLLECTION_ITEMS`, `MAX_INPUT_QUEUE_BYTES`,
`MAX_OUTPUT_QUEUE_BYTES`, `MAX_PENDING_REQUESTS`, `MAX_REQUEST_IDS`,
`MAX_REFERENCED_PAYLOAD_BYTES`, `MAX_TOTAL_OUTPUT_BYTES`,
`HANDSHAKE_DEADLINE`, `READ_DEADLINE`, `WRITE_DEADLINE` and `CLEANUP_DEADLINE`.
These are proposed names for approved bounds, not new measured values or defaults.
Reject frames above the bound before growing a buffer/parsing; use checked
accounting, independently capped queues and total stdout/stderr/IPC charging.
Apply limits also to ignored/invalid traffic and referenced payloads; cap ID replay
tracking across the attempt. Full queues never disable the independent stop path;
no unbounded `read_to_end`, diagnostics retention or pending-ID collection.

Schema validation cannot prove framing, duplicate-key rejection, allocation caps,
state/correlation, immutable reference resolution, authorization, independent
observer authenticity, cancellation ordering, descendant termination, durability,
credential revocation or offline zero-egress. These require separate implementation
and qualification. No language or nonce exchange authenticates worker observations.

## Missing closure evidence
Exact proposed method/request/result/error schemas, version policy, numeric error mapping and phase/state table are supplied above for review only. All finite bounds remain unapproved and require reconciliation with BG-01 and UD-5. UD-1/UD-2 must separately demonstrate protected verifier observation. Synthetic schema/example consistency is not runtime implementation, parser qualification or cross-language interoperability: actual interoperability and native qualification are **NOT RUN**.

Review cases: [`../qualification/ipc-cases.md`](../qualification/ipc-cases.md). Required owner record: approver identities, revision/content hash, schema artifact identities, selected bounded profile, test report and compatibility scope. Until supplied, UD-2 stays OPEN and TASK-007 cannot start.
