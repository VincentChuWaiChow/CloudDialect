# Isolation qualification plan — TASK-001

Status: **NOT RUN** for native qualification. No disposable environment authorized. T-01/T-04; AC-05/06/08 planning only. The limited metadata and Docker preflights below are not ISO-case passes.

## Environment admission
Owner must identify disposable host/VM/runner, permitted actions, finite test ceilings and emergency cleanup, trusted control-plane/image/runtime identities, synthetic tenant/project resources, forbidden-file/credential/network sentinels, independent observers and compatibility owner. No repository session or agent sandbox is assumed approved. Never probe actual host credentials, metadata or production endpoints; emulate forbidden targets inside disposable test infrastructure.

## First feasibility slice — approval request

Status: **PROPOSED, NOT AUTHORIZED, NOT RUN**. This is checkpoint B in the
[UD-1 owner-review sequence](../decisions/UD-1-isolation.md#first-owner-checkpoint--proposed-2026-10-29).
Candidate-direction approval at A does not approve these actions or this host.
Checkpoint A was explicitly approved on 2026-10-29; B remains pending. The owner
asked about Docker images, not permission to use a Docker daemon or a named host.
The subsequent proposed target is **"local laptop"**, owner-reported **Linux**.
The owner approved only kernel/architecture, Docker CLI location and KVM device
visibility checks. Its relationship to the available terminal environment remains
unconfirmed. The owner subsequently confirmed **"Daily-use laptop—protect it"**:
the laptop is **not disposable and is excluded from native/adversarial testing**.
Do not treat the protected laptop or a
container on it as an admitted disposable adversarial lab; no general Docker
control, VM/native test or image operation is authorized by the target name or
initial preflight. Bounded read-only Docker inspection was approved separately.

### Limited read-only preflight — 2026-10-29

Actual terminal result: `Linux 7.0.0-38-generic x86_64`; Docker CLI located at
`/usr/bin/docker`, without invoking it. `/dev/kvm` was not visible in the restricted
view; a separately permission-reviewed metadata-only repeat outside that sandbox
returned a visible character device. This is not proof of KVM opening permission,
working ioctls, virtualization capability, Docker daemon availability, physical
laptop identity, a disposable lab or a qualified isolation boundary.

The initial metadata preflight opened no device, contacted no Docker daemon,
changed no privileged configuration, pulled/built no image and launched no
container/VM. Exact scoped observations are in the
[preflight record](../../../../.workflow/ultracode/execution-isolation-status-audit-2026-10-29/results/laptop-readonly-preflight-2026-10-29.json).
UD-1 and all numbered runtime tasks remain open.

### Bounded read-only Docker inspection — access denied

The owner subsequently approved current-context inspection and a version/OS/
architecture query only for a local Unix-socket endpoint, with a 10-second
timeout per command and a stop on remote endpoints or denied access. Context
inspection succeeded and confirmed a local Unix endpoint. The version query
exited 1 with access denied; the inspection stopped without retry, sudo, startup
or privilege fallback. No successful server response or version/platform result
was obtained. This does not determine whether the sandbox or host permissions
caused the denial.

The [bounded Docker record](../../../../.workflow/ultracode/execution-isolation-status-audit-2026-10-29/results/docker-readonly-preflight-2026-10-29.json)
contains safe observations and no raw account/context diagnostics. No image,
container, VM, KVM-device open or network change occurred. Any outside-sandbox
retry or further Docker/native action requires a new explicit scope approval;
access failure is not permission to change Docker groups, socket permissions or
daemon configuration. Checkpoint B and UD-1 remain open.

### Separately approved outside-sandbox retry — 2026-10-29

The owner explicitly approved one read-only repeat outside the restricted
sandbox, with the same local-Unix-only, 10-second-per-command and stop-on-denial
limits. Context show, context inspection and the endpoint-bound server version
query all exited 0. The server reported **Docker 29.8.1, linux, amd64**. The
[retry record](../../../../.workflow/ultracode/execution-isolation-status-audit-2026-10-29/results/docker-outside-sandbox-readonly-retry-2026-10-29.json)
preserves this result without private context/socket identifiers. The earlier
denial remains historical evidence; the successful retry does not prove its
precise cause.

That one retry is consumed. No sudo, daemon startup, permission change, image
operation, container/VM launch or KVM opening occurred. Docker API access is not
KVM capability, physical-laptop equivalence, a disposable environment or native
isolation qualification. No gate or numbered task closes from this metadata.
The owner has now resolved the disposability question: the named laptop must be
protected as a daily-use machine. The next missing admission fact is a separate
disposable Linux test target and its accountable operator. An ordinary Docker
container does not change the laptop's protected status. No such target is
admitted, and no local VM/container workaround, provisioning, spend, further
host/device inspection or workload execution is authorized by this answer.

### Proposed native feasibility — still not authorized

Propose only a microVM feasibility subset inspired by ISO-01's allowed input,
private scratch and normal teardown, plus bounded ISO-12-inspired missing-
prerequisite checks. Keep all fixture data synthetic and disposable. No actual
CloudDialect admitted worker/run binding exists yet, so this cannot mark either
full ISO-01 or ISO-12 PASS. It is not ISO-04 hostile-descendant qualification or
AC-05/AC-06/AC-08 closure.
Do not add fork/storage/network saturation, escape or forged-verifier probes to
this first slice. Any negative access/network claim requires approved working
ISO-08 positive observer controls; a missing guest NIC is not full-run offline
proof and says nothing about run-serving core traffic.

Before execution, the owner must supply/review and explicitly approve:

- Disposable target identifier and accountable environment/security/operations
  owners, permitted operator access and any approved provisioning cost. A new
  cloud machine requires separate creation/spend authorization; none exists here.
- Exact host/kernel, Firecracker+jailer, guest kernel/rootfs, fixture/launcher and
  observer versions/hashes plus source/license admission. Do not install or use
  guessed latest binaries, grant host devices, or infer nested KVM availability.
- If preparing the guest filesystem from Docker/OCI material, specify the image
  digest and architecture, bounded build/preparation recipe, trusted init setup,
  resulting rootfs and separate guest-kernel digests. Admit base-image/tooling
  licenses and build authority separately; an image is not a disposable host or
  proof of a qualified boundary. No image pull/build or Docker access is implied.
- Exact command list and all host writes/privileges, process/cgroup scope,
  network destinations, private scratch paths and synthetic resources. Do not
  inspect this repository host's devices, Docker socket, metadata or credentials.
- Positive finite **experimental safety ceilings** for CPU, memory, wall time,
  process/thread count, output/storage, attempts and cleanup; any enabled
  network/external calls and spend must also be bounded. Values are MISSING,
  not unlimited or proposed measured runtime budgets. UD-5 still requires actual
  measurements and its own approved runtime accounting/limits.
- Teardown owner, emergency stop/quarantine actions and exact evidence retention
  policy. Failure to confirm cleanup stops further attempts, not an automatic
  retry or permission to modify unrelated host resources.

If the target, ceilings, exact actions or required observer cannot be approved,
record the missing prerequisite and stop. A missing microVM capability requires
owner re-planning, never a silent fallback. No work in this repository has yet
executed this slice or established the profile required to close UD-1.

## No-paid-cloud revision — free runner candidate, planning only

On 2026-10-29 the owner selected **"No paid cloud"**. The paid AWS/GCP proposal
and USD 5 ceiling below are **superseded and excluded**, not an approved fallback.
The daily-use laptop stays protected. No workflow, account query, publication,
repository-visibility change, install, device opening or workload is authorized.

**Candidate, not a selection:** an existing owner-approved **public GitHub
repository** using a standard x64 `ubuntu-24.04` hosted runner. GitHub documents
public standard-runner compute as free and each such job as a fresh VM. Do not
use `ubuntu-slim`, which is container-backed, or a larger/paid runner. The current
public standard Linux allocation is documented as 4 CPU / 16 GB RAM / 14 GB SSD,
not the superseded cloud-VM shape. GitHub's Linux Android-acceleration statement
is only a feasibility lead: it does **not** qualify Firecracker, KVM capabilities,
our observer boundary or cleanup. See the [hosted-runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

The owner subsequently answered **"Yes—already public; consider it"**, reporting
that CloudDialect is public on GitHub under their control and allowing candidate
consideration. This is owner-reported information, not an account/visibility
query or workflow authorization. Free eligibility and repository settings have
**not** been independently checked. Private
runner allowances are not a zero-charge guarantee; larger runners are billed,
and artifact/cache storage has separate allowances. The candidate therefore
excludes private paid/minute-dependent runs, larger runners, caches, uploaded
artifacts, custom images and every other billable service. No billing setting
will be changed. See [GitHub Actions billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions).
Do not make CloudDialect public or copy private source merely to obtain free CI.
An existing public synthetic-only lab repository could be proposed by its owner;
creating or publishing one also requires separate authorization.

The first possible action would be **a separately approved metadata-only job**,
not Firecracker execution: manual trigger only, one job, no automatic retries,
proposed job timeout two minutes, each command ten seconds, safe output <=64 KiB.
That output ceiling applies only to the proposed allowlisted command collector,
not GitHub's automatic runner/job/platform logs; its exact bounded behavior still
requires approval. Treat the public repository's workflow, emitted metadata and
run logs as public disclosures, not confidential evidence storage. Admit only
synthetic/non-sensitive content and review every output field before publication.
Its exact command manifest would cover only kernel/architecture, specifically
named runner-image identity fields and `/dev/kvm` stat metadata. No environment
dump, source checkout/execution, secrets, OIDC, privileged setup, device opening,
KVM ioctl, image download, container/VM boot, cache or artifact upload. Specify
empty workflow token permissions and no PR/push/schedule trigger in the proposal.
The owner subsequently approved the two-file local draft and synthetic tests.
The workflow now exists locally only; no publication or dispatch occurred.
Publishing and running still need explicit authorization for the named
repository/ref and exact reviewed revision.

The OS label is not an immutable image pin: hosted images rotate. Record actual
image/kernel identity before accepting any observation. A subsequent native
proposal must bind and check the admitted image/kernel profile and separate
Firecracker/jailer/guest/fixture hashes; any mismatch stops before boot. This
metadata proposal cannot replace the complete B manifest or approve the later
benign feasibility slice. Job timeout and runner disposal are not measured
descendant cleanup, CPU accounting or full-run offline guarantees.

**Local-draft approval — granted, not publication or execution:** the owner
answered **"Approve local draft and offline tests only"** for
`.github/workflows/isolation-preflight.yml` and focused offline contract tests at
`scripts/verification/execution-isolation/test_hosted_preflight.py` for review.
The files were created only after this separate approval. The
planned workflow has only a manual `workflow_dispatch` trigger, standard x64
`ubuntu-24.04`, empty token permissions and a two-minute job timeout. Its single
bounded standard-library Python collector would allow only kernel/architecture,
the specifically named `ImageOS`/`ImageVersion` fields and `/dev/kvm` stat metadata;
no external subprocess, device open/ioctl or environment dump. Propose a
cooperative ten-second collection-only alarm and <=64 KiB allowlisted output, with missing or
malformed required identity data blocking the preflight. Tests would be synthetic
and offline, not host probes. No checkout, external action, secret/OIDC use,
download, install, cache, artifact upload, privilege change or workload belongs
in this draft. The collector is inline to avoid checkout or external actions.
Its [24 synthetic tests](../../../../scripts/verification/execution-isolation/test_hosted_preflight.py)
pass, including fail-closed identity/device metadata, output boundaries and
definition-time-probe rejection before fake-port-bound execution. One static
review's two corrections were integrated by the parent: safe synthetic loading
and explicit collection-only deadline wording. There is no independent
post-correction reread or live timing/KVM qualification. The
[local workflow](../../../../.github/workflows/isolation-preflight.yml) was
delivered uncommitted/unpublished. The owner subsequently authorized **"git commit
and git push everything"** for the current worktree/branch. That publication
checkpoint permits no default-branch merge, workflow dispatch, paid usage or
native test; those remain unauthorized. Publication results require Git readback,
not this pre-publication document snapshot.

Publication is an independent blocker: GitHub documents that a manual workflow
must exist on the default branch to receive `workflow_dispatch`. A feature-branch
draft/push alone is not a dispatchability guarantee, and no default-branch merge
or automatic-trigger workaround is permitted by this plan. See
[manual workflow requirements](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/manually-run-a-workflow).

If the free candidate cannot be admitted, obtain an owner-supplied spare or
borrowed disposable KVM-capable Linux target, or keep native work blocked. Do not
silently fall back to the protected laptop, a paid trial or an assumed free tier.
UD-1–UD-5/BG-01 and all numbered runtime tasks remain open. These public-document
facts were checked through Context7 and GitHub's official pages on 2026-10-29;
no actual runner, KVM or repository eligibility has been verified.

## Separate disposable lab proposal — planning only, 2026-10-29

**Historical paid-VM proposal — superseded by "No paid cloud" above.** Its USD 5
ceiling, AWS/GCP choices and provisioning sequence are not the current route and
must not be executed or treated as authority. Retained for decision history only.

**Owner requested a lab plan, not provisioning. No existing target is available.**
This proposal supplies choices for checkpoint B; it is not environment admission,
an executable runbook, a budget approval or a replacement for UD-1–UD-5/BG-01.
The daily-use laptop remains excluded. No local VM/container workaround, install,
cloud account query, credential access or spend is authorized. No provider was
selected, and no resource was created.

### Candidate shape and documented feasibility

Propose one separate, short-lived **Linux x86_64 cloud VM with explicitly enabled
nested virtualization**, used only for synthetic fixtures. The cloud VM is the
disposable lab host; Firecracker runs the inner guest with a separate kernel.
Do not choose bare metal or additional adapters merely to bypass a missing
prerequisite. Provider documentation establishes a candidate, not actual KVM or
Firecracker qualification on a particular instance.

| Candidate, not selected | Primary-source basis | Still must be established |
| --- | --- | --- |
| AWS EC2 `m8i.large`, desired 2 vCPU / 8 GiB | [Instance specifications](https://docs.aws.amazon.com/ec2/latest/instancetypes/gp.html); [nested KVM support and explicit CPU option](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/amazon-ec2-nested-virtualization.html) | Named sandbox account/region, exact type's feature readback, image, permissions, capacity, price and usable KVM. No instance role or production connectivity. |
| Google Compute Engine `n2-standard-2`, desired 2 vCPU / 8 GB | [N2 specifications](https://docs.cloud.google.com/compute/docs/general-purpose-machines); [nested virtualization restrictions](https://docs.cloud.google.com/compute/docs/instances/nested-virtualization/overview?hl=en); [explicit enablement](https://docs.cloud.google.com/compute/docs/instances/nested-virtualization/enabling) | Named isolated project/zone, organization-policy permission, exact image/type, price and usable KVM. No workload service-account credentials or production connectivity. |

The AWS user guide lists more families than its current
[launch-template API reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_LaunchTemplateCpuOptionsRequest.html),
which restricts the feature to specified eighth-generation Intel families. M8i is
in their overlap; do not extrapolate support to an arbitrary cheaper type. GCP
also excludes E2 and other listed configurations. No free-tier eligibility or
price is asserted. Prefer an already controlled isolated sandbox over creating a
new cloud account, but do not infer one from local credentials/configuration.
If neither cloud option is acceptable, stop for an owner-supplied alternative;
do not use the laptop by default.

### Proposed experimental ceilings — NOT APPROVED, NOT MEASURED

These are planning safety ceilings for a benign feasibility slice, not the
measured runtime budgets required by UD-5. Do not write them into
`budget-measurements.json` as approved values. The provider-specific command
manifest must identify and verify every enforcement mechanism before execution.

| Scope | Proposed ceiling | Required enforcement / stop condition |
| --- | --- | --- |
| Resources | One on-demand lab VM; desired 2 vCPU / 8 GB class; one boot disk <=20 GiB; no extra disks/snapshots or paid premium OS | Exact provider resource inventory and immutable image selection; replan if image/shape cannot fit. No Spot interruption or commitment purchase. |
| Lifetime / cost | <=2 hours from creation through teardown; proposed total ceiling USD 5 | Obtain a current all-in regional quote first, including disk, IP, network and teardown tail. No approved spend exists. Alerts are not hard caps; an external accountable operator and independently scheduled teardown must be specified. If the quote/control cannot fit, do not provision. |
| Attempts | One benign boot/fixture attempt, concurrency one, no automatic retry | Operator/launcher counter; failed prerequisites stop before boot. A follow-up attempt needs new approval. |
| Attempt / cleanup time | <=30 seconds running; <=15 seconds domain cleanup | External watchdog outside the inner guest; cleanup uncertainty quarantines the lab and prevents further attempts. Cloud teardown still follows its separate deadline. |
| Guest / VMM | One guest vCPU, 256 MiB guest RAM; host attempt cgroup <=1 GiB, CPU bandwidth/quota <=one CPU and <=64 host processes/threads | Validate selected kernel/controller semantics, placement and readback. Finite cumulative CPU-time allowance, accounting/stop mechanism and enforcement overshoot are MISSING B fields; the wall watchdog is not a measured CPU-accounting guarantee. Specify guest process accounting separately. Missing control means no launch. Host task limits do not count or qualify guest descendants. |
| Data / output | Read-only rootfs <=256 MiB; private scratch <=64 MiB; combined captured output <=1 MiB; artifact downloads <=1 GiB | Fixed-size storage and bounded collectors; stop at the bound. No arbitrary dependency builds, saturation, fork bombs or hostile scripts. |
| Network | Artifact acquisition only from an exact approved destination list; zero guest NICs in the first slice; no production/metadata access from test domain | Separate acquisition and test phases, reviewed network namespace/filter rules and operator-only management path. Management traffic is not zero whole-run traffic; no ISO-06/07 or offline guarantee follows. |

The management/control-plane credential stays outside the disposable VM and all
guest-visible paths. Do not copy `.aws`, `.config/gcloud`, SSH agents, repository
secrets, laptop mounts or rootful Docker sockets. Privileged jailer/device/cgroup
setup, if later approved, occurs only inside the disposable lab, with an exact
owned-path/command manifest and no blanket `--privileged` grant.

### Planned sequence and observable stopping points

1. **Choose the provider and accountable operator, information only.** Supply a
   non-sensitive sandbox account/project label and region/zone; no credentials.
   This does not approve account access, setup or billing.
2. **Prepare and approve the complete B manifest before any action.** Fill exact
   resource/identity/network scopes, current quote and cost ceiling, immutable
   Linux image/kernel identity, matching static Firecracker+jailer release and
   checksums, separate guest kernel/rootfs/init/fixture hashes, command list and
   privileges, observation/retention policy and provider teardown inventory.
   Include permission to remove only the newly owned lab resources. Missing
   fields remain blockers; no copy/paste provisioning command is supplied here.
3. **After that explicit approval only**, create the exact disposable resource
   set, validate its identity, and perform the admitted host/KVM preflight. A
   visible device is insufficient: actual KVM API/capability checks require
   device opening and must be listed explicitly. Failure stops without changing
   instance family, permissions or isolation mechanism on the owner's behalf.
4. Admit pinned artifacts and bounded control setup, then run only the approved
   ISO-01/ISO-12-inspired benign subset. Docker/OCI userspace preparation, if
   selected, happens in the lab under a pinned recipe—not on the laptop. Do not
   copy moving `latest`/CI download examples. Verify guest kernel, rootfs and init
   separately; a Docker image is not bootable microVM admission by itself.
5. Capture bounded safe logs, hashes, timestamps, actual resource/cleanup values
   and every missing control. Reject negative network/access claims without
   working independent positive observer controls. A guest boot is feasibility
   only, not a full ISO case, adversarial qualification or approval evidence.
6. Stop the inner domain, confirm cleanup, export only reviewed bounded evidence,
   then delete only the manifest's newly owned resources. Read back VM, disks,
   addresses, temporary rules and any scheduler/logging resources; a stopped VM
   alone is not deletion or proof that billing ended. Missing confirmation means
   UNKNOWN and operator escalation, not broad deletion or another attempt.

Propose retaining sanitized evidence for seven days after operator review, with
explicit retention/deletion approval in B; no automatic evidence deletion or
snapshot/backup creation is authorized now. Future schedules/teardown automation
also require explicit scope approval; none was enabled by preparing this plan.

### Remaining admission and evidence gaps

Provider/account/project, region/zone, operator/security owners, exact image and
runtime/kernel/rootfs/fixture pins, approved commands/privileges, usable KVM,
network/observer controls, current quote and approved spend, cumulative CPU-time
allowance/accounting/stop mechanism/overshoot, enforcement readback,
cleanup/retention approval and actual native results are all **MISSING**.
UD-1–UD-5/BG-01, TASK-001/TASK-007 and the other numbered tasks stay open.
The existing seven-license cargo gate still fails; this plan grants no license
exception, release clearance or permission to distribute lab artifacts.

Primary documentation was checked via Context7 and official provider/Firecracker
pages on 2026-10-29. The moving links identify feasibility sources, not admitted
version pins. In addition to the provider sources above, use the
[Firecracker host guidance](https://github.com/firecracker-microvm/firecracker/blob/main/docs/prod-host-setup.md),
[matching jailer contract](https://github.com/firecracker-microvm/firecracker/blob/main/docs/jailer.md),
[KVM API/capability initialization](https://github.com/firecracker-microvm/firecracker/blob/main/src/vmm/src/vstate/kvm.rs)
and [kernel/rootfs preparation](https://github.com/firecracker-microvm/firecracker/blob/main/docs/rootfs-and-kernel-setup.md).
Select the versioned source, verify artifact provenance/licenses, and review
security updates before B. Documentation review is not a native probe.

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
