# Isolation qualification plan — TASK-001

Status: **NOT RUN**. No disposable environment authorized. T-01/T-04; AC-05/06/08 planning only.

## Environment admission
Owner must identify disposable host/VM/runner, permitted actions, finite test ceilings and emergency cleanup, trusted control-plane/image/runtime identities, synthetic tenant/project resources, forbidden-file/credential/network sentinels, independent observers and compatibility owner. No repository session or agent sandbox is assumed approved. Never probe actual host credentials, metadata or production endpoints; emulate forbidden targets inside disposable test infrastructure.

## Cases and oracles
| Case | Probe | Required evidence |
|---|---|---|
| ISO-01 positive | Valid admitted worker reads approved input/toolchain and writes private scratch | Allowed behavior succeeds without broader grants; exact input/attempt/image recorded. |
| ISO-02 startup | Dependency/build/interpreter hooks read forbidden sentinel or open network before hello | Denied from first untrusted instruction; no pre-confinement execution. |
| ISO-03 filesystem | Traversal, link/mount races, hard links, devices, inherited descriptors, host-control sockets | Independent sentinel/access observation shows no escape or input/oracle writes. |
| ISO-04 descendants | Double-fork, setsid, concurrent fork, parent exit/crash/disconnect | Domain-wide termination and owned-child reaping confirmed; reservation not released on parent exit alone. |
| ISO-05 stop failure | Cancel/exhaust limits, kill VMM/helper, deny cleanup confirmation | Dispatch/acceptance closed; bounded cleanup or CLEANUP_UNCONFIRMED quarantine retaining reservation. |
| ISO-06 network | IPv4/IPv6, DNS, redirects/proxies, inherited sockets, synthetic local/metadata endpoints | Destination/action/purpose policy holds for full domain and run-serving core paths. |
| ISO-07 offline | Run all declared offline paths including core services | Zero attributable outbound traffic under stated observer coverage; missing offline dependency blocks before call. |
| ISO-08 observer controls | Generate permitted synthetic traffic/access before and after negatives | Observer detects controls; observer outage/blind spot invalidates relevant negative evidence. |
| ISO-09 forged verifier | Import shadowing, assertion removal, fabricated pass/files/logs, zero/partial/skipped tests | External protected oracle rejects/blocks every seeded critical defect; no self-reported acceptance. |
| ISO-10 identity | Swap candidate/oracle/configuration after check; replay old attempt | Frozen binding mismatch blocks; observations cannot be relabelled. |
| ISO-11 valid verifier | Independent known-good candidate and protected complete tests | Produces valid observation/intent; final acceptance still awaits approval/policy/budget/cleanup/durable commit. |
| ISO-12 unsupported host | Remove each required control/profile capability | Launch denied; no downgrade or bypass. |

Record test/fixture/executable/profile/observer hashes, exact scope, raw safe observation references, expected/actual disposition, CPU/memory/output/cleanup measurements, failures and blind spots. Observation of no traffic is meaningful only with working positive controls and run attribution. Do not label OS support, whole-host silence or universal sandbox security from a passing fixture subset.
