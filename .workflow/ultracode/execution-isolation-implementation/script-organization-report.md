# Rust and custom-script organization — final scope

User clarified ordering is limited to Rust code and custom scripts. No repository-wide specification/research relocation occurred. Rust domain organization and six shared AI-rule links are described in domain-publication-report.md; existing public API paths remain intact.

## Custom scripts
Seven canonical tools now live under scripts/verification/{contracts-evidence-foundation,execution-isolation,python-oss-to-s3-pack}. scripts/README.md documents ownership and commands. Seven relative symlinks preserve old .workflow command paths, while baselines/results stay in their recorded runs. Source comparison proves only recorded RUN/RESULTS path adaptation and one shell comment changed; no checker assertions or semantic logic were relaxed. See results/script-source-parity.json and script-moves.json.

## Verification
14 current Python checks pass, including three new layout/compatibility checks. Both canonical and legacy P0 entrypoints pass the eight selected content cases. Four full historical graders retain their pre-move failure exit codes at both entrypoints: execution checker (old tracked-scope rule), its nine-case sensitivity suite (one pre-existing positive-control error), planning grader (four failures/one error), and pack grader (old branch/scope rules). These failures are disclosed, not treated as green or bypassed. See results/script-command-parity.json.

94 Rust integration tests plus6 compile-fail doctests pass. Frozen check/fmt/Clippy and shell syntax pass. Coverage tool executed successfully from its new canonical path; emitted production Rust lines remain198/204 (97.06%), excluding tests/dependencies/unmapped DTOs; no branch/native/full-spec percentage claimed. Results: script-final-check-results.json and domain-coverage-summary.json.

Packet13 independently compared source snapshots and links: PASS, no required findings; it did not execute commands.13 cumulative sidecars, zero recursion/followups; inherited model/effort and separate sandbox isolation unverified. Existing DDD elapsed limit retained; no hidden repair loop.

## Publication boundary
Earlier three signed commits remain unchanged. This scoped script-ordering follow-up is a fourth commit; normal push targets only origin/feat/execution-isolation. Independent remote SHA comparison follows the commit and is recorded in the conversation plus /tmp/clouddialect-publication-receipt.json. Research files remain unchanged/untracked and excluded. Configured SSH signing remains enabled; allowed-signer identity verification remains unavailable. No force, PR, deployment, release or native probes requested/performed. Original execution-isolation goal and BG-01/UD1–UD5 remain incomplete/open.
