# Proposed positive fixture admission — not approved or applied

## Objective and limits
Admit the remaining 12 canonical task-18 positive fixtures as seven already
bounded batches, then verify all 13 positives including the existing seed.
Proposed new 45-minute tranche begins only on explicit owner approval. Parent
alone integrates, tests and makes local milestone commits. No new agents
(30 cumulative dispatches consumed), production-code repair, dependency/feature/
version changes, native/provider probes, push, release or runtime gate closure.
A failed fixture/changed frozen source requires stop/replan, not a silent repair.

## Exact owned canonical directories (input + expected only)

### 3.2.2 — 4 files
- `crates/clouddialect-contracts/fixtures/positive/behavioral-contract/minimal-valid/` — behavioral-contract; structural/local pass, empty errors/warnings/blockers.
- `crates/clouddialect-contracts/fixtures/positive/behavioral-contract/with-known-behavior/` — behavioral-contract; structural/local pass, empty errors/warnings/blockers.

### 3.2.3 — 4 files
- `crates/clouddialect-contracts/fixtures/positive/behavioral-contract/with-unresolved-assumption-on-dependency/` — behavioral-contract; structural/local pass, empty errors/warnings/blockers.
- `crates/clouddialect-contracts/fixtures/positive/behavioral-contract/with-behavioral-notes/` — behavioral-contract; structural/local pass, empty errors/warnings/blockers.

### 3.2.4 — 2 files
- `crates/clouddialect-contracts/fixtures/positive/behavioral-contract/with-extension-field/` — behavioral-contract; structural/local pass, empty errors/warnings/blockers.

### 3.2.5 — 4 files
- `crates/clouddialect-contracts/fixtures/positive/evidence-record/observed/` — evidence-record; structural/local pass, empty errors/warnings/blockers.
- `crates/clouddialect-contracts/fixtures/positive/evidence-record/unknown/` — evidence-record; structural/local pass, empty errors/warnings/blockers.

### 3.2.6 — 4 files
- `crates/clouddialect-contracts/fixtures/positive/verification-result/minimal/` — verification-result; structural/local pass, empty errors/warnings/blockers.
- `crates/clouddialect-contracts/fixtures/positive/verification-result/with-acceptance/` — verification-result; structural/local pass, empty errors/warnings/blockers.

### 3.2.7 — 4 files
- `crates/clouddialect-contracts/fixtures/positive/pack-manifest/full-coverage/` — manifest; structural/local pass, empty errors/warnings/blockers.
- `crates/clouddialect-contracts/fixtures/positive/pack-manifest/with-unsupported-operation/` — manifest; structural/local pass, empty errors/warnings/blockers.

### 3.2.8 — 2 files
- `crates/clouddialect-contracts/fixtures/positive/pack-manifest/with-extension-section/` — manifest; structural/local pass, empty errors/warnings/blockers.

All 24 proposed file bytes and hashes are frozen in `contract.json` and `drafts/`.
Each batch owns at most two directories/four files; no hidden directory expansion.

## Verified readiness, not delivery
Actual trusted runner: 12 draft cases pass; seven isolated batch counts match;
13-case temporary composite with admitted seed passes. Nine deliberately broken
derivatives fail their retained pass oracle. All 19 invocations match expectations.
Three fidelity labels span the specifically required minimal/notes pair. P0-C
verification fields, required resource dimensions, two required provider-note
topics and empty/nonempty P0-B declarations are present. Provider notes remain
explicitly unresolved; observed/origin/approver/capability fields are synthetic
claims. No trust or provider guarantee is asserted by a fixture pass.

## Admission checks after approval
For each batch copy exactly the frozen pair bytes; run the actual custom runner
against that batch's owned paths and record counts. Then default/positive runs
must report 13, empty selection must remain nonzero, full frozen Cargo/check/
fmt/Clippy and repository Python checks must pass. No fabricated RED for data-only
additions; preserve wrong-expectation/invalid-derivative controls. Only canonical
task 18 may be checked after all its clauses are verified; tasks 19–22, remaining
full corpus, fresh coverage/MSRV/legacy-checker/traceability/consumer work and all
21 Spec 2 tasks/UD/BG gates remain open. Current canonical corpus still has only
one fixture; the temporary 13-case result is not a delivered-corpus claim.
