# Persistence and restore qualification — TASK-004

Status: **NOT RUN**; no backend or disposable environment approved. These are tests to perform, not a restore report.

| Case | Fault / stimulus | Required result |
|---|---|---|
| STORE-01 positive | Valid admission and complete acceptance transaction | Attributable durable records match exact artifact/budget/policy identities; valid readback. |
| STORE-02 uncertainty | Append/commit timeout, lost acknowledgement, duplicate transition | One reconciled transition or blocker; no blind append/replay. |
| STORE-03 crash | Before/after artifact publication and each acceptance commit boundary | Uncommitted intent never accepted; committed state read back; orphan/missing blob handled explicitly. |
| STORE-04 ordering | Cancel/revoke/exhaust races acceptance | Exactly one authoritative order and honest already-committed result. |
| STORE-05 integrity | Missing link, modified record, duplicate identity, truncated/older valid history | Detect within declared authenticated-head coverage; otherwise block/disclose unsupported guarantee. |
| STORE-06 outage | Full disk, permissions, unavailable store | No dispatch/acceptance; emergency containment remains possible. |
| STORE-07 scope | Forged IDs and same-tenant/different-project or different-tenant read/list/cache/export/delete/restore | Denied through trusted context, independent of equal artifact hashes. |
| STORE-08 retention | Missing policy, expiry, deletion with backups retained | Dependent persistence/admission denied; live access invalidated; actual retained-copy restrictions reported. |
| STORE-09 stale restore | Restore snapshot predating deletion or policy narrowing | Current deletion/policy restrictions applied before any restored data exposure; unknown restriction state denies access. |
| STORE-10 recovery | Corrupt checkpoint, missing blob/digest mismatch, unknown budget/effect/cleanup | RECOVERY_BLOCKED/ACCOUNTING_UNKNOWN; no reset or fabricated evidence. |
| STORE-11 backup | Consistent backup/restore on chosen VFS/storage, including journals | Integrity and authoritative state verified; no raw-live-copy assumption. |
| STORE-12 leakage | Synthetic secrets in worker audit/diagnostic payload | Worker cannot author trusted records; secret-bearing payload not persisted/exported. |

Record actual engine/build/config/VFS/filesystem identities, fault timing, transition IDs and readback, current deletion-policy source, safe integrity/audit references and measured durability limits. Process-crash testing alone does not prove power-loss durability. Include permitted same-project positive controls and known corrupt/stale negative fixtures. Physical erase claims may not exceed tested medium, retained-copy and recipient scope.
