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

## First owner checkpoint — proposed 2026-10-29

The owner requested continued work **through explicit gate approvals**, then
explicitly approved **the Linux/KVM Firecracker+jailer candidate direction** on
2026-10-29. This records checkpoint A only, not an approved supported profile or
security/operations qualification. No disposable environment, general host access
or native action is approved; UD-1/TASK-001/TASK-007 remain open. The separately
approved, now-completed metadata-only inspections are recorded in the
[qualification plan](../qualification/isolation-plan.md); they do not extend
checkpoint A or authorize further access.

**Checkpoint A — direction approved, evaluation actions still pending:** one Linux/KVM
per-attempt microVM candidate, using Firecracker with its matching hardened
jailer setup. Treat Linux as the first evaluation target only; do not claim
supported macOS, generic CI or other hosts. Do not build a second adapter or
silently substitute a shared-kernel/process fallback if prerequisites fail.

| Checkpoint | Owner decision and evidence | What it does not authorize |
| --- | --- | --- |
| A — candidate direction | Owner approved the Linux/KVM Firecracker+jailer candidate direction on 2026-10-29 | No host queries, hardware/device/Docker access, installation, resource creation or native tests |
| B — environment/action admission | Name the disposable target and accountable owner; approve exact artifacts/versions/hashes, commands, writable resources, privilege/network scope, finite safety ceilings, observers, emergency cleanup and any spend | No use of the present repository host by implication; no production access, real secrets or unlisted actions |
| C — mechanism/profile acceptance | Review actual bounded results, supported-profile identity, observer limits, cleanup measurements and residual risks; record required security/operations approval | No blanket OS support, AC closure or TASK-007 authorization from candidate approval or a successful VM boot alone |

The first feasibility slice after B is proposed in the existing
[qualification plan](../qualification/isolation-plan.md#first-feasibility-slice--approval-request).
The owner has since excluded the daily-use laptop from native/adversarial work
and requested a plan because no separate target exists. The
[disposable lab proposal](../qualification/isolation-plan.md#separate-disposable-lab-proposal--planning-only-2026-10-29)
is planning only: provider, operator, exact commands/artifacts and spend remain
unapproved. The owner subsequently selected **"No paid cloud"**: the paid-VM
choices are now historical, and a
[free hosted-runner candidate](../qualification/isolation-plan.md#no-paid-cloud-revision--free-runner-candidate-planning-only)
is being considered only if repository eligibility and actual KVM can be
established under separate approval. No workflow or visibility change is
authorized. An evaluation-direction answer is not permission to run that slice. Tooling
source/license admission and the unresolved dependency/license review remain
separate; no allowance or implementation gate is waived.

## Docker image packaging — proposed, not executed

At the initial packaging question, no particular image or Docker host had been
identified or authorized. The later laptop metadata inspections grant no build
or workload authority. Yes: use a pinned Docker/OCI image as
userspace build input for a compatible guest root filesystem, while Firecracker
boots that filesystem with a separately admitted guest kernel and init setup.
The [official rootfs recipe](https://github.com/firecracker-microvm/firecracker/blob/main/docs/rootfs-and-kernel-setup.md)
demonstrates using a Docker image to populate an ext4 guest filesystem. This is
not direct OCI execution by Firecracker and does not select that example image.

Recommendation: reuse Docker for image preparation, not as a replacement for the
approved microVM direction or as an excuse to add another orchestration layer.
Building/preparing an image is itself executable activity requiring approved
inputs, source/license admission and an admitted disposable build environment.
Pin the input image digest/architecture, build recipe and resulting filesystem
and guest-kernel digests in checkpoint B. No pull, build, export or boot ran.

An ordinary container shares a host kernel; it does not supply missing KVM
capabilities. Running the VMM inside a container would still require an admitted
Linux/KVM host and separately reviewed device, cgroup, namespace and privilege
access. Do not default to `--privileged` or expose a rootful Docker daemon socket
to untrusted work. See [Docker privilege guidance](https://docs.docker.com/engine/containers/run/#runtime-privilege-and-linux-capabilities).
This packaging proposal authorizes no local Docker/host inspection or change to
checkpoint B/C. Later bounded metadata-only approvals are recorded separately;
the laptop remains protected and no native evaluation is authorized on it.

## Primary-source facts
- Firecracker uses KVM and requires hardened host/jailer-equivalent setup; guest network filtering is a host responsibility. A microVM does not by itself satisfy offline, resource or verifier guarantees. [Production guidance](https://github.com/firecracker-microvm/firecracker/blob/main/docs/prod-host-setup.md).
- cgroup v2 can kill a descendant cgroup tree; an empty `populated` observation is not proof of every child reap, storage release or external-effect disposition. [Kernel documentation](https://docs.kernel.org/admin-guide/cgroup-v2.html).
- Apple's Containerization uses Linux VMs on supported Apple platforms; this is feasibility evidence, not CloudDialect compatibility. [Official source](https://github.com/apple/containerization).
- Context7 resolve/query of `/rust-lang/rust`, corroborated with current standard-library docs: `Command` inherits environment/stdio in common spawning paths; `env_clear` and explicit handles are necessary but insufficient. Dropping `Child` does not stop/wait; single-child kill/wait is not a qualified descendant-domain cleanup mechanism. [Command](https://doc.rust-lang.org/std/process/struct.Command.html), [Child](https://doc.rust-lang.org/std/process/struct.Child.html).

Sources reviewed 2026-10-22. No actual platform/runtime version or hardware capability was qualified.

Documentation refresh on 2026-10-29 (facts, not local qualification): Firecracker
still requires KVM/hardware virtualization and host hardening; guest egress
filtering remains a host responsibility. Its jailer must match the statically
linked VMM version. Explicitly configure and verify cgroup placement rather than
trusting a flag/default: the documented cgroup-v2 path can proceed without moving
the VMM when a requested parent cgroup is absent. Unsupported/missing controls
must block the feasibility attempt. See the current
[host guidance](https://github.com/firecracker-microvm/firecracker/blob/main/docs/prod-host-setup.md)
and [jailer contract](https://github.com/firecracker-microvm/firecracker/blob/main/docs/jailer.md).
No version is selected from these moving documentation URLs; pin source/runtime
and image identities in B before any command is approved.

## Required observation boundary
Keep trusted test selection, oracle/expected outcomes, approvals and final decision outside candidate-writable/executable authority. Authenticate observation origin **and** independently measure the behavior claimed. A domain receipt, signed runner log, nonce or JSON marked trusted proves neither assertion execution nor correctness when the runner imports candidate code. Prefer bounded behavioral interfaces with an external trusted oracle; if required observations cannot be protected for an operation, block that operation rather than accept worker-reported passes.

Offline observation must cover workers, descendants and run-serving core/model/credential/resolver/telemetry/audit paths. Worker-only network isolation is insufficient. Handle/mount/environment restrictions must exist before dependency resolution, runtime startup or build hooks.

## Qualification and closure
[`../qualification/isolation-plan.md`](../qualification/isolation-plan.md) defines T-01/T-04 and observer controls. Native tests are NOT RUN: no approved disposable environment exists in this run. Residual risks include kernel/VMM vulnerabilities, host trust compromise, authorized data-flow abuse, observation blind spots and incomplete oracle coverage.

Required approval record: security/operations owner identities, selected immutable host/image/runtime/profile identities, supported environment matrix and compatibility owner, observed startup/escape/descendant/forgery reports, observer coverage, measured cleanup bounds, residual-risk acceptance, decision revision/hash. All MISSING; TASK-001 and dependent TASK-007 remain incomplete.
