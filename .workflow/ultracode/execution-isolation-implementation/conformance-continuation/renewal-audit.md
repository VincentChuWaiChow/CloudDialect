# Renewal completion audit — bounded items only

| Criterion | Result | Authoritative evidence |
| --- | --- | --- |
| Intended compiled/executed regression RED | PASS | `03738e4`; 9 pass/1 fail, actual 2 versus expected 4 gaps |
| Same target GREEN after minimal fix | PASS | `0396883`; 10 pass, no store calls, empty verification projection |
| Exact two-file reviewed seed admitted | PASS | `408af0d`; `seed-exact-copies.json` |
| Default and positive-prefix fixture count | PASS | Each 1; `seed-checks.json` |
| Zero-selection guard | PASS | Exit 1, correct message, not ignored |
| Full implemented Cargo suite | PASS | 286 Rust tests + 1 fixture, `integrated-checks.json` |
| All-target check/fmt/Clippy | PASS | Current renewal commands all exit 0 |
| Python regression | PASS | 15 checks |
| Re-executed fail-closed CLI matrix | PASS, parent-only | All 67 existing controls matched against current repaired binary |
| Additional spec-only checker suite | FAIL, pre-existing | 8 pass/1 error; unchanged blanket checkbox predicate rejects the same 8 supporting marks at baseline and now |
| New independent repaired-tree review | SKIPPED | No new agents approved; prior static proposed-patch review only |
| Fresh coverage | SKIPPED / gate incomplete | Matching LLVM utilities absent; no new install authorized |
| Declared Rust 1.88 MSRV | UNVERIFIED | Current compiler is Rust 1.99.1; no separate MSRV check |
| Full required corpus / consumer handoff | MISS | Only one seed delivered; remaining inventory/qualification incomplete |
| Spec 2 runtime/gate completion | MISS | All 21 numbered tasks and all six UD/BG gates open; no runtime crate |
| Scope, lockfiles and unrelated preservation | PASS | `scope-audit.json`; exact approved crate-path allowlist |

Tooling exception is disclosed in the report/setup record, not relabeled a RED
receipt. Local commits prove history only, not remote publication/CI/runtime.
