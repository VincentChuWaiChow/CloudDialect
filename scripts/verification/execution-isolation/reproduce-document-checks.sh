#!/usr/bin/env bash
# Document-only grader adapter; no runtime or security qualification.
# Run from repository root. Checker rules and recorded run data remain unchanged.
set -eu
rtk proxy python3 - <<'PY'
from pathlib import Path
import contextlib
import importlib
import io
import json
import sys
import tempfile
import unittest

root = Path.cwd()
run = root / ".workflow/ultracode/execution-isolation-implementation"
old = root / ".workflow/ultracode/execution-isolation"
sys.path.insert(0, str(old))
checker = importlib.import_module("check_spec")
suite = importlib.import_module("test_check_spec")
baseline = json.loads((old / "baseline.json").read_text())
baseline["head"] = json.loads((run / "baseline.json").read_text())["ref"]
with tempfile.TemporaryDirectory(prefix="document-baseline-", dir=run) as tmp:
    checker.RUN = Path(tmp)
    (checker.RUN / "baseline.json").write_text(json.dumps(baseline))
    stream = io.StringIO()
    with contextlib.redirect_stdout(stream):
        checker.main()
    result = unittest.TextTestRunner(stream=stream, verbosity=2).run(
        unittest.defaultTestLoader.loadTestsFromModule(suite)
    )
    print(stream.getvalue())
    if not result.wasSuccessful():
        raise SystemExit(1)
PY
