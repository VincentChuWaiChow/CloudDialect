# Phase/error result implementation eval (unit 1.2)

## Authority and scope
User requested implementation of the execution-isolation tasks, then said continue with Ultracode/agent teams. Parent interprets this, against the immediately preceding first-slice proposal, as authorization to start the Spec 1 prerequisite unit 1.2. This is an explicit interpretation, not owner attestation or Spec 2 gate closure. No commits/pushes: current instruction does not explicitly request those actions. The earlier planning-only status is historical, not blanket implementation authority. Real sandbox probes remain prohibited without approved disposable environment.

Owned crate files: src/errors/diagnostics.rs, src/errors/phase.rs, src/errors/mod.rs, src/lib.rs, tests/phase_results.rs. Parent owns all edits. No placeholders, validation algorithms, schemas/catalog, manifest/lock/dependency/version changes, BehavioralResult or accepting conversions. Preserve all other existing source/doc/research hashes from phase-results-baseline.json.

## Define success before code
1. ErrorCategory has exactly nine canonical variants and kebab-case category labels from the design taxonomy; unknown category labels reject.
2. ValidatorError/ValidatorWarning have exactly the canonical task-9 public fields. Optional unknown context stays None, parse offset zero survives, original warning field text is retained.
3. StructuralOutcome is Pass/Warn/Fail; SemanticOutcome is Pass/Fail. Serialized phase outcomes use lowercase fixture labels, reject accepting labels and semantic warn.
4. StructuralResult and SemanticResult carry the exact task-9 fields/vectors and root + errors-module exports; no field implies acceptance or another phase's correctness.
5. Types serialize/deserialize with existing Serde pins; round-trip errors/warnings/Unicode and retain exact public shapes. They are diagnostic data carriers, not validated evidence or authorization.
6. No From/Into conversion between phases. Compile-fail documentation in each direction, accompanied by actual integration imports/constructors (so missing symbols cannot masquerade as negative-test success).
7. Same real integration target must fail for missing public APIs only AFTER setup/dependency resolution succeeds, then pass after minimal implementation. Setup failure is not RED.
8. Full frozen crate tests/check, rustfmt check, Clippy -D warnings and seven scaffold regressions pass. Measure coverage >=80% of implemented behavior if matching LLVM tooling can be provisioned into /tmp; otherwise mark coverage UNVERIFIED, never estimate it from source/derive counts.

## Commands and constraints
Target: rtk proxy env CARGO_HOME=/tmp/clouddialect-phase-cargo CARGO_TARGET_DIR=/tmp/clouddialect-phase-target cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml --frozen --test phase_results. Fetch exact locked dependencies into isolated /tmp cache only if needed, with explicit tool-policy escalation for restricted public network; no global Cargo/config/credential changes. Use Context7 Serde/Cargo docs and committed lock as primary API/version inputs. No external service resources or untrusted candidate execution.

## Orchestration and bounds
Workflow-only parent execution: five cumulative sidecars exhausted; no sixth agent unless user explicitly authorizes the pending read-only-review request. No recursive fan-out or silent follow-up. One implementation slice, one bounded repair phase, <=45 minutes; cache setup <=3 minutes before reassessment, coverage tooling <=10 minutes. Shared manifest/lock untouched. Prior planning/document graders remain historical and are not broadened to conceal intentional newly authorized source changes.

## Completion
Unit 1.2 is delivered only with real tests/source/evidence; partial task-9 delivery is not a whole canonical checkbox or BG-01 completion. Overall objective retains all unfinished upstream/runtime/native/approval requirements. No synthetic RED/GREEN checkpoint commits are claimed.
