# P0 report — documentation reconciliation only

## Outcome
**Draft delivered for owner review; not implementation-ready or gate-approved.** User explicitly authorized Spec 1 documentation changes only. No Rust/catalog/dependency/license changes, runtime tests, sandbox probes, deployment, commits or pushes. Original end-to-end execution-isolation goal is not complete.

## Deliverables
- `.kiro/specs/contracts-evidence-foundation/p0-contract-amendments.md`: concrete proposed P0-A–F result/outcome/projection, typed capability requirements, verification/binding/failure/approval references, immutable catalog snapshot and scoped document-store API contracts; version/adoption approval conditions.
- `requirements.md`: corresponding changed/new criteria 4.1/4.3/4.4/4.8/5.8/8.3/8.6/8.7/9.5, evidence remains MISSING.
- `design.md`: aligned model/schema/entrypoint/snapshot/context/fixture semantics; optional instance schema selector with trusted DocumentKind, full SemVer and resolver-support entries. Existing broader layout remains unchanged.
- `tasks.md`: aligned implementation contracts/fixtures, task 17 moved before task 13 integration, all task numbers/checkmarks preserved. No future implementation task is authorized by this edit.

## Exact checks and scope
| Check | Result | Interpretation |
|---|---|---|
| `rtk proxy python3 .workflow/ultracode/execution-isolation-implementation/verify-p0.py` | PASS, exit 0, 10 tests | Four embedded full schemas meta-checked with installed jsonschema 4.26.0; JSON/TOML syntax, required/binding/failure/SemVer/claim-shape boundaries, API/scope/checkmark checks. No current Rust or native evidence. |
| `rtk proxy python3 -m unittest discover -s tests -p test_contracts_scaffold.py -v` | PASS, exit 0, 7 tests | Existing scaffold/source regressions only. |
| `rtk git diff --check` | PASS, exit 0 | No new tracked-diff whitespace errors. |
| Changed numbered criteria through existing EARS validator function | PASS, 9/9 | `results/p0-ears-changed-criteria.json`; preserves real generic-rule checks without confusing metadata bullets with requirements. |
| Generic requirements validator | FAIL before and after | Baseline 213 extracted items/168 FAIL; current 228/178 FAIL. It extracts verification/evidence metadata bullets as requirements, alongside real pre-existing compound/format issues. Five added criteria account for 15 added extracted lines; all nine edited criteria individually pass. Do not report whole requirements clean. |
| Generic design validator | FAIL before and after | Same 5/11 pass, 6/11 fail: broader legacy headings/sections/resilience shape; no unrelated reformatting to game it. |
| Generic task validator | FAIL before and after | Expects N.N task numbering; Spec 1 uses stable flat 1–26 numbering. No renumbering outside P0. |
| Preservation/links/fence/JSON scope audit | PASS | `results/p0-final-audit.json`; only authorized Spec 1 documents plus run artifacts changed relative to P0 baseline. Existing Spec 2 decision drafts, research, source, locks/licenses preserved. |

## Independent review and repair
One fresh-context read-only reviewer found five contradictions (schema dispatch, phase outcomes, catalog/store/overlay, TOML entrypoint, SemVer). Parent applied one consolidated P0 repair and reran focused checks. Actual reviewer verdict remains FAIL for the earlier snapshot; no independent post-repair PASS is claimed. Full finding/disposition table in `results/04-p0-review.md`.

The focused grader initially failed because it assumed an optional date-time checker was installed and rejected pre-existing trailing whitespace. Its setup was corrected to expose the missing checker and compare only new whitespace. Original failure output remains `results/p0-grader-initial.txt`; this was not runtime RED/GREEN or business-logic evidence.

## Explicit limitations
- Installed Python jsonschema 4.26.0 lacks the optional date-time format checker. A malformed date ending in Z is structurally accepted here; semantic date validity remains UNVERIFIED and mandatory. No package was installed. Unknown external retrieval is denied by registry callback, not proved by native traffic capture.
- Short digest lengths, duplicate test IDs/obligation membership, protected expected identities, observation origin and approval authority require the specified semantic/Spec 2 checks. A structurally valid forged approver/origin is deliberately accepted by the schema-only grader to demonstrate that trust boundary, not to claim approval.
- Context7 JSON Schema and Python jsonschema docs consulted; wrong Boon project rejected. Official pinned boon 0.6.1 docs inaccessible: exact adapter API/error mapping UNVERIFIED. No placeholder exports or guessed implementation added.
- External publication history/target version and owner contract approval remain unresolved. Breaking published format changes require MAJOR/migration review. Native Kiro/SMT, Rust compile for these proposed APIs, coverage, behavioral, isolation, credential, durability and budget qualification NOT RUN.
- Four cumulative native dispatches (three initial preparation, one P0 reviewer); no follow-ups. Inherited model/effort, actual observed values unavailable. No usage/dollar savings claim. Limits/usage tracked without resetting earlier counts.

## Next gate
Owners review P0-A–F and approve or revise exact contract/version/snapshot/authority semantics before any Spec 1 implementation is scheduled. BG-01 still needs delivered real APIs/schemas, current conformance/consumer evidence and upstream/core approval. Spec 2 UD-1–UD-5 remain OPEN; no execution runtime starts from documentation approval alone.
