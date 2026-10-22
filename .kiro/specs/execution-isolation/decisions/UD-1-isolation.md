# UD-1 — isolation and verifier observation

Status: **OPEN — owner decision and native evidence MISSING**. TASK-001, 2026-10-22. Security/operations approvers not supplied. REQ-001/002/003/005/008; DES-001/002/008.

## Comparison and proposed initial scope
| Candidate | Benefit | Cost / limitation |
|---|---|---|
| Ordinary process/process group | Simple lifecycle API | Reject as a security boundary: no filesystem/network/ambient-authority confinement or dependable hostile-descendant containment. |
| Linux namespaces + seccomp + capabilities + cgroup v2 | Smaller runtime surface; native Linux controls | Shared host kernel; separately qualify mounts, handles, startup, egress and descendant lifecycle. Containers alone do not select these controls. |
| Per-attempt Linux microVM with hardened VMM/jailer | Separate guest kernel for arbitrary build/dependency code | Requires hardware virtualization, image/runtime patching, host resource controls and network filtering. Greater operational cost. |
| Apple VM/containerization profile | Potential macOS deployment path | Separate adapter, prerequisites and qualification; not Linux evidence by analogy. |
| CI runner | Reproducible disposable environment candidate | Runner is not itself a sandbox; inherited authority, virtualization and observers must be qualified. |

Recommendation (inference, not selection): evaluate a Linux/KVM per-attempt microVM first for arbitrary repository/build execution; compare against a hardened Linux shared-kernel profile if operations cost makes microVMs unsuitable. Do not implement both speculatively. macOS, generic CI and unavailable Linux profiles must deny launch until separately qualified. Required host capabilities absent means ISOLATION_UNAVAILABLE, never a process-only fallback.

## Primary-source facts
- Firecracker uses KVM and requires hardened host/jailer-equivalent setup; guest network filtering is a host responsibility. A microVM does not by itself satisfy offline, resource or verifier guarantees. [Production guidance](https://github.com/firecracker-microvm/firecracker/blob/main/docs/prod-host-setup.md).
- cgroup v2 can kill a descendant cgroup tree; an empty `populated` observation is not proof of every child reap, storage release or external-effect disposition. [Kernel documentation](https://docs.kernel.org/admin-guide/cgroup-v2.html).
- Apple's Containerization uses Linux VMs on supported Apple platforms; this is feasibility evidence, not CloudDialect compatibility. [Official source](https://github.com/apple/containerization).
- Context7 resolve/query of `/rust-lang/rust`, corroborated with current standard-library docs: `Command` inherits environment/stdio in common spawning paths; `env_clear` and explicit handles are necessary but insufficient. Dropping `Child` does not stop/wait; single-child kill/wait is not a qualified descendant-domain cleanup mechanism. [Command](https://doc.rust-lang.org/std/process/struct.Command.html), [Child](https://doc.rust-lang.org/std/process/struct.Child.html).

Sources reviewed 2026-10-22. No actual platform/runtime version or hardware capability was qualified.

## Required observation boundary
Keep trusted test selection, oracle/expected outcomes, approvals and final decision outside candidate-writable/executable authority. Authenticate observation origin **and** independently measure the behavior claimed. A domain receipt, signed runner log, nonce or JSON marked trusted proves neither assertion execution nor correctness when the runner imports candidate code. Prefer bounded behavioral interfaces with an external trusted oracle; if required observations cannot be protected for an operation, block that operation rather than accept worker-reported passes.

Offline observation must cover workers, descendants and run-serving core/model/credential/resolver/telemetry/audit paths. Worker-only network isolation is insufficient. Handle/mount/environment restrictions must exist before dependency resolution, runtime startup or build hooks.

## Qualification and closure
[`../qualification/isolation-plan.md`](../qualification/isolation-plan.md) defines T-01/T-04 and observer controls. Native tests are NOT RUN: no approved disposable environment exists in this run. Residual risks include kernel/VMM vulnerabilities, host trust compromise, authorized data-flow abuse, observation blind spots and incomplete oracle coverage.

Required approval record: security/operations owner identities, selected immutable host/image/runtime/profile identities, supported environment matrix and compatibility owner, observed startup/escape/descendant/forgery reports, observer coverage, measured cleanup bounds, residual-risk acceptance, decision revision/hash. All MISSING; TASK-001 and dependent TASK-007 remain incomplete.
