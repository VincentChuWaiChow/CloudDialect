# UD-2 — bounded worker IPC proposal

Status: **OPEN — proposal only, not an approved wire contract**. Prepared 2026-10-22 for TASK-002. Owners: core/worker owners, identities and approvals not supplied. REQ-001/003/006/010; DES-001/006/010.

## Facts and alternatives
Context7 resolved then queried `/websites/jsonrpc` (primary specification coverage, rather than a language-specific implementation). JSON-RPC 2.0 defines messages, correlation and standard errors, but is transport agnostic. It does not provide this application's framing, worker authentication, cancellation, backpressure or acceptance authority. [Primary specification](https://www.jsonrpc.org/specification), fetched 2026-10-22.

Alternatives: custom length-prefixed messages reduce JSON-RPC flexibility but create a new protocol; HTTP adds a listener/authentication surface; JSON-RPC on private stdio avoids an additional listener but still requires an application profile. Recommendation (inference): evaluate a deliberately restricted JSON-RPC/stdin-stdout profile first. No library or transport is selected by this record.

## Proposed profile for owner review
- Core creates private pipes through the approved isolation launcher; stdout is protocol-only, stderr is untrusted diagnostics. Neither stream is persisted raw. No inherited host IPC endpoints or descriptor passing.
- UTF-8, one compact JSON object per LF-terminated frame, no literal newlines inside strings. Bound accumulated bytes **before** allocation/parsing; reject EOF with an incomplete frame. This is an application framing proposal, not JSON-RPC framing.
- Require `jsonrpc: "2.0"`, string request IDs unique in the attempt, object parameters and exact method-specific schemas. Reject duplicate object keys, unknown fields, invalid encodings, excess nesting, batches and null/numeric IDs in this restricted profile. Batch rejection is a deliberate interoperability restriction, not base-protocol behavior.
- Initial core request `worker.hello` proposes application version `clouddialect-worker/1`. Response must echo that exact version and core nonce before an operation is sent. No downgrade. The nonce/correlation proves association with a pipe exchange, **not trusted worker identity or observation authenticity**.
- Core request `worker.execute`: opaque core-issued attempt token, admitted-input reference and operation reference; worker echoes correlation only. Core-held authenticated tenant/project/run/attempt/policy bindings remain authoritative; worker-supplied strings cannot change them.
- Worker request `core.tool`: same attempt token, operation reference and typed bounded arguments. It proposes an operation; external policy/egress/budget/credential checks authorize or deny it. No arbitrary command execution or acceptance method.
- `worker.execute` response may describe output artifact references; the core verifies/freezes content identity and treats logs/result fields/exit status as untrusted, never as acceptance.
- Core request `worker.cancel`: outstanding operation ID and safe reason category. Acceptance of cancellation immediately closes the core dispatch/acceptance gates; no worker acknowledgement is needed to initiate whole-domain termination. A response is advisory, not a cleanup receipt.
- Every request has exactly one matching response, with exactly one of `result`/`error`. No unsolicited results, repeated IDs, old-attempt messages or late results after stop. No notifications in the proposed initial profile: security-relevant requests require correlation.
- Standard parse/invalid-request/method/parameter/internal errors retain JSON-RPC meanings. Execution semantic blockers remain separate; numeric application-code assignments and full method/error schemas are **not approved or finalized**. On a malicious/uncorrelatable frame, close dispatch and contain the domain; do not echo raw payloads in diagnostics.
- Independently bound input/output queues, outstanding requests, frame size, nesting, read/write/handshake time and total output using approved UD-5 values. Full pipes must not deadlock cancellation; supervisor can terminate without writing to the worker. No unbounded `read_to_end` or output collection.

## Missing closure evidence
Exact method/request/result/error schemas, version-compatibility policy, numeric error mapping, phase/state table and all finite bounds require owner review and reconciliation with BG-01 and UD-5. `worker.hello` authenticates no observation channel. UD-1/UD-2 must separately demonstrate protected verifier observation. No runtime implementation, parser tests or cross-language interoperability tests have run.

Review cases: [`../qualification/ipc-cases.md`](../qualification/ipc-cases.md). Required owner record: approver identities, revision/content hash, schema artifact identities, selected bounded profile, test report and compatibility scope. Until supplied, UD-2 stays OPEN and TASK-007 cannot start.
