# Authorized reference workload — eval contract, 2026-10-30

## Goal and authority

User approved creating the purpose-built Python document-storage application
inside this repository: "yea build it so we can see if this works". This closes
the creation-authorization question only, not UD-1/2/3/4/5 or any BG. No cloud
calls, credentials, provider execution, native probes, commits or publishing.
No recurring automation or user-memory writes. Existing admission edits remain.

## Scope and integration surfaces

Parent owns `examples/python-oss-document-store/`: one application module,
local-only demo, colocated unittest suite, optional exact OSS SDK requirement
and README/inventory. Consumer is the local demo and future independently
reviewed migration input, not a new pack/runtime or schema implementation.
Parent may update this existing workflow root and Spec 3 supporting status.
No changes to upstream code, APIs, licenses, dependency locks or qualifications.
Graphify AST update is the only generated cross-repository output.

## Success checks defined before implementation

| Criterion | Expected signal / owner |
|---|---|
| Upload, retrieve, delete | Unit tests observe exactly `put_object`, `get_object`, `delete_object` against bounded IDs/keys; parent |
| Real document behavior | Filename/content-type round trip, replacement, empty/binary bodies, namespaces, missing/deleted objects; parent |
| Validation before SDK calls | Invalid IDs/metadata/body/size rejected with no storage calls; parent |
| Integrity and resource handling | Independent SHA-256 detects corruption (never ETag-as-MD5); bounded reads, bad/truncated metadata/body, stream failure all fail and close; parent |
| Failure semantics | Provider exceptions propagate, no application retries or partial-success claim; parent |
| TDD | Same unittest target first fails for absent implementation, then passes; captured RED/GREEN evidence; parent |
| Local demo | Deterministic JSON says local-only and demonstrates save/load/corruption rejection/delete; no SDK client or network; parent |
| Coverage | Stdlib trace executable-line coverage >=80% for application module; branch/real-service coverage not claimed; parent |
| Scope/qualification guards | Prior 67 criteria and all 17 leaves remain open; no qualifying oracle fabricated; parent |
| Review | Bounded independent Python correctness review, verified findings repaired with regression tests; parent integration |

Runner: `rtk proxy python3 -B -m unittest discover -s examples/python-oss-document-store -p 'test_*.py' -v`.
Demo: `rtk proxy python3 -B examples/python-oss-document-store/local_demo.py`.
No JS package manager is relevant. Coverage/ruff modules are unavailable;
stdlib `trace`, AST compilation and unittest need no installation.

## Team and budgets

One parent writer; `oss_sdk_contract` / docs_lookup (configured gpt-6-luna,
medium, read-only) verifies public legacy oss2 APIs through Context7 and primary
tagged source. Reuse `pack_admission` / python_reviewer (configured gpt-6.1-sol,
high, read-only) for a new bounded review after implementation. Two worker turns
maximum this continuation, no recursive fan-out, eight minutes each, one review
wave and one consolidated parent repair. Effective sandbox and actual engine
telemetry unverified. Stop SDK-specific implementation on unresolved API shape;
do not substitute V2. Default network denied; any public-source fetch escalation
is explicit and does not authorize services or executing downloaded code.

## Evidence boundary

Application and tests are generated development material, not independent
qualification answers or model holdout. A recorded source-tree digest is not a
repository commit, license approval, SDK install test or cloud receipt. Final
handoff lists concrete operations, limitations and remaining gate owners.
