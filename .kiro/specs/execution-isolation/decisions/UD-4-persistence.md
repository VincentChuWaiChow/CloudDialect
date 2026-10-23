# UD-4 — persistence, ordering and retention proposal

Status: **OPEN — backend, retention values and restore evidence MISSING**. TASK-004, 2026-10-22. Operations/security approvers not supplied. REQ-007/009/010; DES-007/009/010.

## Alternatives and smallest proposal
| Option | Benefit | Limitation |
|---|---|---|
| Local SQLite + immutable scoped artifact storage | Offline, small operational surface, one metadata transaction | Storage/VFS durability must be qualified; no intrinsic project authorization or authenticated history. |
| Remote PostgreSQL | Central transaction ordering for multiple coordinators | Network/credential/outage/operations surface; cannot serve a strict offline request remotely. |
| Object storage alone | Immutable copies and backups | Strong consistency does not supply a multi-key state/audit/budget transaction. |
| Custom append log | Offline | Bespoke torn-write recovery, conditional ordering, indexing and restore add correctness burden. |

Recommendation (inference, not selection): evaluate one trusted coordinator and one local SQLite metadata database with separately scoped immutable artifacts. Start by comparing rollback journal with `synchronous=EXTRA`; do not add distributed coordination or WAL until a demonstrated requirement warrants it. Exact SQLite version, Rust binding/dependency/license, filesystem/VFS and journal configuration remain unselected.

Context7 resolve/query of SQLite plus official docs reviewed 2026-10-22: synchronization guarantees depend on journal mode and storage behavior; WAL adds same-host/network-filesystem and backup/checkpoint considerations. Current WAL docs flag a version-specific reset corruption fix: any selected engine must be verified against current advisories, not inferred from a wrapper name. [Synchronization](https://www.sqlite.org/pragma.html#pragma_synchronous), [WAL](https://www.sqlite.org/wal.html). PostgreSQL asynchronous commit may lose acknowledged transactions on crash. [PostgreSQL](https://www.postgresql.org/docs/current/wal-async-commit.html). S3 lacks atomic cross-key updates. [S3 consistency](https://docs.aws.amazon.com/AmazonS3/latest/userguide/Welcome.html#ConsistencyModel).

## Proposed authoritative ordering
- Serialize cancellation/revocation/exhaustion/acceptance through conditional state/version checks and a monotonically ordered transition identity, not wall timestamps.
- Commit state, linked audit event, budget disposition and checkpoint together. Persist/verify immutable artifacts before committing their references; orphan artifacts never imply acceptance.
- Lost acknowledgement/timeout means unknown outcome: reconcile by unique transition identity before dispatch, retry or acceptance. Repeated committed operations are read back, not appended blindly.
- Required persistence failure closes dispatch/acceptance but cannot block emergency containment, termination or revocation.
- Every read/list/cache/export/delete/restore uses trusted tenant/project authorization; database file permissions alone do not provide that.
- Hashes/chains detect specified alterations, not an older valid prefix by themselves. If rollback protection is promised, define an independently trusted authenticated head/checkpoint compatible with offline mode and qualify it; otherwise disclose the limit.

## Retention decision table — values deliberately unset
For **each** class below, owner must approve permitted content, tenant/project scope, retention trigger/duration, expiry action, deletion authority, immediate live-access invalidation, retained-copy restrictions, backup expiry/physical-erasure limits and restore filtering. Missing policy denies dependent persistence/admission, not unlimited retention.

| Data class | Duration / policy approval |
|---|---|
| Inputs and private workspaces | MISSING |
| Candidates, patches, produced artifacts | MISSING |
| Private dependency/build caches | MISSING |
| Retrieval examples and indexes | MISSING |
| Prompts/responses and safe diagnostics | MISSING |
| Verification observations/reports | MISSING |
| Audit/approvals/checkpoints/budget metadata | MISSING |
| Exports and recipient limitations | MISSING |
| Backups/replicas/journals/temporary/quarantine copies | MISSING |

Credential values and secret-bearing raw diagnostics never become a normal retained class. Keep current deletion restrictions independently of a restored historical snapshot, or deny restored access when those restrictions cannot be established. Preserve honest tombstones/unavailable-content references without pretending expired evidence remains retrievable.

Use a consistent approved backup mechanism, not an unqualified copy of live database files. SQLite `secure_delete` is not whole-system/backup erasure evidence. [Backup API](https://www.sqlite.org/backup.html), [secure deletion limits](https://www.sqlite.org/pragma.html#pragma_secure_delete).

## Closure
[`../qualification/restore-cases.md`](../qualification/restore-cases.md) is NOT RUN. Need backend/profile/version approval, durability/tamper scope, per-class retention policies, tested transaction/restore/ordering reports, current deletion-state authority, compatibility owner, approver identities and bound decision revision/hash. All remain MISSING; TASK-004 is incomplete.
