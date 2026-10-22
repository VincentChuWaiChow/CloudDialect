# UD-5 — finite resource measurement proposal

Status: **OPEN — no measurements or approved values**. TASK-005, 2026-10-22. Core/operations approvers not supplied. REQ-001/003/010; DES-003/001/010.

## Proposed accounting semantics — not an activated profile
| Resource | Unit / accounting proposal |
|---|---|
| CPU | Integer microseconds, non-overlapping descendant/verifier domain deltas summed across all attempts. |
| Wall | Integer nanoseconds from original run clock basis; no retry reset; unresolved clock mapping after restart blocks recovery. |
| Memory | Bytes, aggregate live usage/reservation and peak across active domains; not sum of historical attempt peaks. Specify swap separately. |
| Processes | Concurrent kernel tasks including threads across the full active run domain. |
| Output | Cumulative raw bytes observed before truncation/discard, including stdout/stderr/IPC; separately define artifact treatment to avoid double counting. |
| Workspace/storage | Live allocated bytes and peak across scoped workspaces/artifacts; define sparse/link/snapshot/retained-copy accounting. |
| Attempts | Integer admitted attempts charged before dispatch, never replenished by retry/restart. |
| Cleanup | Nanoseconds from stop condition to independently confirmed domain termination and required reaping. |
| External operations | Attempted calls per authorized operation class, including failed and retried calls. |
| Model usage | Separate typed provider input/output/cache units; unknown usage reconciled conservatively, never reset. |

Reserve before dispatch. Release concurrent capacity only after confirmed cleanup; never refund historical CPU/output/attempt/external consumption when a capacity reservation is released. Aggregate parallel work without double counting nested counters. Counter corruption/reset/overflow/unrecoverable accounting blocks further dispatch/resumption.

Primary-source fact: Linux cgroup v2 `cpu.max` is bandwidth control, not a cumulative CPU-time limit; descendant CPU counters, byte-based memory and thread-inclusive PID accounting have different semantics. Enforcement overshoot and sampling blind spots must be measured. [Kernel cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html), reviewed 2026-10-22. These facts do not select cgroups or prove enforcement for the UD-1 profile.

## Reference workload and measurement plan
Propose a versioned synthetic admitted-input → analysis/build → produced/frozen candidate → independent verification → cleanup workload, plus controlled retry/restart and parallel-capacity variants. Include the sandbox/VM overhead and run-serving core operations; declare which host/image resources are reserved versus charged. Separate warm/cold-cache trials. No actual fixture/toolchain/collector version or repetitions are selected yet, so this is not an executable benchmark or a completed reproducibility claim.

Before execution owners must approve immutable workload/fixture/toolchain commands, disposable environment, finite safety ceilings, collector/clock/counter scope, sample resolution, trial/concurrency/cache plan, observer positive controls and overshoot/cleanup tolerances. Do not run saturation/fork/storage/network probes in this repository session.

For each approved finite bound test below/equal/above the smallest supported increment, including simultaneous reservations, descendants/verifier, cancellation, cleanup timeout, retries, restart/counter reset, output/storage flooding and unknown usage. Exact-bound admission semantics must be approved: reservations exceeding remaining discrete allowance deny before dispatch; runtime threshold and overshoot handling must match measured enforcement granularity. UD-2 parser/queue/time bounds reconcile here before TASK-007.

## Measurement artifact and closure
[`../qualification/budget-measurements.json`](../qualification/budget-measurements.json) has empty observations and null limits **by design**. Null values are missing evidence, not unlimited/default values. Future trials need metric/unit/aggregation, configured bound, raw safe samples, crossing/detection/stop/cleanup timestamps, overshoot, uncertainty, fixture/profile/collector identities and verdict.

Closure requires actual reproducible reference and saturation reports, approved positive finite values for every mandatory and enabled external allowance, defined restart/cumulative/capacity semantics, measured enforcement lag and owner identities bound to profile/decision hashes. All MISSING; TASK-005 remains incomplete and execution must return BUDGET_UNCONFIGURED until resolved.
