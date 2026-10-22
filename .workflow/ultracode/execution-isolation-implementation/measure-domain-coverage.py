"""Reproduce emitted-production coverage; not branch or native qualification."""

import json
import os
from pathlib import Path
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[3]
RESULTS = Path(__file__).resolve().parent / "results"
CRATE = ROOT / "crates/clouddialect-contracts"
PROFILES = Path(tempfile.mkdtemp(prefix="clouddialect-domain-profiles-", dir="/tmp"))
LLVM = Path("/tmp/clouddialect-phase-llvm-tools")
ENV = dict(os.environ, CARGO_HOME="/tmp/clouddialect-phase-cargo",
           CARGO_TARGET_DIR="/tmp/clouddialect-phase-coverage-target",
           RUSTFLAGS="-Cinstrument-coverage",
           LLVM_PROFILE_FILE=str(PROFILES / "%p-%m.profraw"),
           LD_LIBRARY_PATH=str(LLVM))


def run(args):
    result = subprocess.run(["rtk", "proxy", *map(str, args)], cwd=ROOT, env=ENV,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode:
        raise RuntimeError(f"check failed ({result.returncode}): {result.stderr}")
    return result


command = ["cargo", "test", "--manifest-path", CRATE / "Cargo.toml", "--frozen", "--tests"]
tests = run(command)
(RESULTS / "domain-coverage-tests.txt").write_text(tests.stdout + tests.stderr)
metadata = run(command + ["--no-run", "--message-format=json"])
artifacts = [json.loads(line) for line in metadata.stdout.splitlines() if line.startswith("{")]
artifacts = [a for a in artifacts if a.get("reason") == "compiler-artifact"
             and a.get("executable") and a.get("profile", {}).get("test")]
expected = {p.stem for p in (CRATE / "tests").glob("*.rs")} | {"clouddialect_contracts"}
assert {a["target"]["name"] for a in artifacts} == expected
objects = sorted({a["executable"] for a in artifacts})
raw = sorted(PROFILES.glob("*.profraw"))
assert raw
profile = PROFILES / "merged.profdata"
run([LLVM / "llvm-profdata", "merge", "-sparse", *raw, "-o", profile])
command = [LLVM / "llvm-cov", "export", objects[0]]
for obj in objects[1:]:
    command += ["-object", obj]
export = run(command + ["-instr-profile=" + str(profile), "-summary-only"])
(RESULTS / "domain-coverage.json").write_text(export.stdout)
files = [f for data in json.loads(export.stdout)["data"] for f in data["files"]
         if f["filename"].startswith(str(CRATE / "src") + "/")]
totals = {}
for metric in ("lines", "functions", "regions"):
    count = sum(f["summary"][metric]["count"] for f in files)
    covered = sum(f["summary"][metric]["covered"] for f in files)
    totals[metric] = {"count": count, "covered": covered,
                      "percent": 100 * covered / count if count else None}
summary = {"production_files": files, "production_totals": totals, "objects": objects,
           "profiles_directory": str(PROFILES), "threshold": 80,
           "scope": "Emitted production mappings only; tests/dependencies/unmapped DTOs excluded. "
                    "No branch, whole-spec or native runtime coverage claimed."}
(RESULTS / "domain-coverage-summary.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(totals, indent=2))
assert totals["lines"]["percent"] is not None and totals["lines"]["percent"] >= 80
