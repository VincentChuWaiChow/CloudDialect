# Custom verification scripts

Group executable source by responsibility and business domain. Keep generated
reports, baselines and receipts in their recorded `.workflow/` runs, not here.

```text
scripts/
  verification/
    contracts-evidence-foundation/
      measure-domain-coverage.py
      verify-p0.py
      verify-planning.py
    execution-isolation/
      check_spec.py
      test_check_spec.py
      reproduce-document-checks.sh
    python-oss-to-s3-pack/
      check_docs.py
```

Each script has one canonical implementation. Previous `.workflow/` command
paths are relative compatibility symlinks. Do not edit or duplicate a link's
implementation; change its canonical file. Tests remain under `tests/`, except
the historical checker sensitivity suite kept beside its checker.

## Run from the repository root

```sh
rtk proxy python3 -m unittest discover -s tests -v
rtk proxy python3 scripts/verification/contracts-evidence-foundation/verify-p0.py P0DocumentationTests.test_full_embedded_schemas_and_toml_parse
rtk proxy python3 scripts/verification/contracts-evidence-foundation/measure-domain-coverage.py
```

The coverage script requires the documented compiler-matched LLVM tools and
isolated Cargo cache under `/tmp`; it does not install dependencies or tools.
The shell reproduction helper also expects the repository root as its working
directory. Python tools derive the repository root from their canonical path.

**Historical graders are not current implementation acceptance checks.** Their
full default runs retain original draft/branch/hash/scope assertions and may
fail after later authorized implementation. The sensitivity suite and shell
adapter inherit those checks. Preserve the failures and explain their scope;
do not weaken assertions or refresh baselines merely to make a moved tool green.
Focused P0 content cases, current Rust/Python tests and explicit current scope
audits are separate evidence. No script here qualifies native isolation or
authorizes acceptance, deployment, release or security-gate closure.
