# EVAL REPORT: execution-isolation implementation readiness

The full eval contract was defined in `eval-contract.md` before decision-artifact writes. This report grades preparation only, never runtime security.

## Capability evals
- PASS: requested local branch `feat/execution-isolation` created at baseline `3b9cb6bd0ee82dbef3b731a35ce68fc778c4906a`.
- PASS: six explicitly OPEN decisions and six planned qualification/measurement artifacts drafted.
- PASS: actual source/export probe confirms missing required APIs (library compile exit 0; consumer E0432 exit 1). This is an observed blocker, not a passing compatibility test.
- PASS: zero fabricated measurements/approvals; runtime crate absent and dependent implementation not started.
- PASS (parent verified repair): independent reviewer found one P2 evidence-packaging gap and no gate-safety defect. Runnable adapter/exit record now retained; no independent post-repair sign-off. Added prerequisite plan received parent review only.
- PASS: original 96 files preserved; branch/HEAD, new-file scope, no invented limits, new-file whitespace/local links/JSON checks in `results/final-structure-checks.json`.

## Regression evals
- PASS: `rtk proxy python3 -m unittest discover -s tests -p test_contracts_scaffold.py -v` — seven tests, exit 0.
- FAIL (pre-existing historical baseline): `rtk proxy python3 .workflow/ultracode/execution-isolation/check_spec.py` — exit 1, scope compared with old `ad2b4bd...` ref.
- FAIL (same historical baseline): `rtk proxy python3 .workflow/ultracode/execution-isolation/test_check_spec.py -v` — eight pass, positive case errors; exit 1.
- PASS with explicit baseline adaptation: import unchanged checker/tests, set `RUN` to temporary directory **inside this workspace**, copy historical baseline and change only `head` to this run's `3b9cb6bd...`; `check_spec.main()` and all nine sensitivity tests pass. Evidence: `results/fresh-baseline-document-checks.txt`. This does not repair the historical command or grade new drafts semantically.
- PASS: `rtk git diff --check` — tracked diff only; new-file whitespace checked separately at final audit.
- PASS: `rtk proxy bash .workflow/ultracode/execution-isolation-implementation/reproduce-document-checks.sh` — exit 0; exact baseline adapter preserved, checker plus nine sensitivity tests. `check-results.json` retains command and original raw failures.

## Metrics
Statistical pass@k/pass^k NOT MEASURED. Test counts are deterministic grader results, not agent success probabilities. Runtime coverage NOT MEASURED because implementation is gated.

## Verification
Exact command outputs retained in `results/`, upstream compiler invocation/exit data in `upstream-probe.json`, collected regression commands in `check-results.json`. Fresh-baseline adapter's first error is preserved in `results/fresh-baseline-adapter-error.txt`; subsequent pass is separate, not a replacement of raw failure evidence.

## Status
NEEDS OWNER INPUT / DEPENDENCY DELIVERY. Full requested goal is not complete. Runtime TDD checkpoints have not started; no production code has been edited. No native isolation/credential/restore/budget/adversarial qualification, Cargo runtime build/lint or >=80% coverage is claimed.
