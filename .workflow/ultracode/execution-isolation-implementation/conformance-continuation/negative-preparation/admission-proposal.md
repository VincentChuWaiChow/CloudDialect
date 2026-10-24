# Negative fixture admission proposal — not approved/applied

## Scope and boundaries
23 canonical task-19 cases (baseline 15 plus eight P0), in existing bounded
batches 3.2.9–3.2.21, at most two directories/four files each. 22 expected Fail,
one newer-MINOR Warn; never normalize Warn into rejection or security acceptance.
This is a proposal only, not an approval request superseding the pending positive
question. Any eventual admission requires explicit bounded owner approval.
No production/runner/schema/dependency/version changes, new agents, native/provider
probes, pushes or releases. Stop for unexpected behavior; no implicit repairs.

## Exact canonical directories (input + expected only)

### 3.2.9 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/behavioral-contract/missing-id/` — behavioral-contract, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/behavioral-contract/obligation-no-evidence-no-assumption/` — behavioral-contract, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.10 — 2 files
- `crates/clouddialect-contracts/fixtures/negative/behavioral-contract/invalid-behavioral-fidelity/` — behavioral-contract, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.11 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/evidence-record/missing-classification/` — evidence-record, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/evidence-record/inferred-empty-reasoning-chain/` — evidence-record, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.12 — 2 files
- `crates/clouddialect-contracts/fixtures/negative/evidence-record/unknown-empty-assumptions/` — evidence-record, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.13 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/verification-result/empty-tests-array/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/verification-result/nonpass-without-failure-description/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.14 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/verification-result/acceptance-missing-approver/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/verification-result/unknown-hash-algorithm/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.15 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/verification-result/missing-binding/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/verification-result/missing-configuration/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.16 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/verification-result/nonpass-string-failure-description/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/verification-result/missing-test-obligation/` — verification-result, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.17 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/pack-manifest/missing-toolchain-for-full-operation/` — manifest, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/pack-manifest/invalid-behavioral-fidelity/` — manifest, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.18 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/pack-manifest/inline-contract-body/` — manifest, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/pack-manifest/missing-capabilities/` — manifest, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.19 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/pack-manifest/unknown-capability-version/` — manifest, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/pack-manifest/duplicate-capability-id/` — manifest, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.20 — 2 files
- `crates/clouddialect-contracts/fixtures/negative/pack-manifest/capability-unknown-field/` — manifest, structural/local fail; exact diagnostic multiset in contract.json.

### 3.2.21 — 4 files
- `crates/clouddialect-contracts/fixtures/negative/version/unsupported-major/` — evidence-record, structural/local fail; exact diagnostic multiset in contract.json.
- `crates/clouddialect-contracts/fixtures/negative/version/unsupported-minor-newer/` — evidence-record, structural/local warn; exact diagnostic multiset in contract.json.

## Evidence and acceptance after future approval
Frozen bytes/hashes in contract.json/drafts; 41 actual runner invocations matched:
23-case composite, 13 isolated batches, 23 valid-baseline counterexamples, three
wrong-oracle controls and a full recheck asserting the compatibility-document
reference. Initial 22/23 result and the justified two-diagnostic oracle refinement
are preserved, not hidden. No production fix occurred. Typed manifest decoder
shape errors currently use unparseable/null path as explicitly recorded, not a
fabricated semantic field path. Byte-offset comparison is outside this runner's
metadata; no complete failure-actionability/authenticity/native claim.

After explicit admission, copy exact approved bytes per batch, verify actual
counts/default negative selection, full Cargo/check/fmt/Clippy/Python and wrong-
oracle controls. Task 19 may be checked only after all its 23 cases are delivered
and verified; tasks 20–22, full corpus/coverage/traceability/consumer/MSRV/native
and all Spec 2 tasks/gates remain open. The canonical corpus is currently one
seed; neither this draft nor temporary success constitutes delivery.
