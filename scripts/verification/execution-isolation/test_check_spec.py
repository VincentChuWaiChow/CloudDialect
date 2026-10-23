"""Positive and seeded-negative controls for the document grader only."""

import contextlib
import io
from pathlib import Path
import tempfile
import unittest

import check_spec


class SpecGraderTests(unittest.TestCase):
    def setUp(self):
        self.original = check_spec.SPEC
        self.temp = tempfile.TemporaryDirectory(dir=check_spec.RUN)
        check_spec.SPEC = Path(self.temp.name)
        for name in ("requirements", "design", "tasks", "analysis"):
            (check_spec.SPEC / f"{name}.md").write_text(
                (self.original / f"{name}.md").read_text()
            )

    def tearDown(self):
        check_spec.SPEC = self.original
        self.temp.cleanup()

    def replace(self, name, old, new):
        path = check_spec.SPEC / f"{name}.md"
        text = path.read_text()
        self.assertIn(old, text)
        path.write_text(text.replace(old, new, 1))

    def reject(self):
        with contextlib.redirect_stdout(io.StringIO()):
            with self.assertRaisesRegex(SystemExit, "FAIL:"):
                check_spec.main()

    def test_valid_documents_pass(self):
        with contextlib.redirect_stdout(io.StringIO()):
            check_spec.main()

    def test_missing_document_rejected(self):
        (check_spec.SPEC / "design.md").unlink()
        self.reject()

    def test_missing_shall_rejected(self):
        self.replace("requirements", "THE Core SHALL bind", "THE Core may bind")
        self.reject()

    def test_false_evidence_status_rejected(self):
        self.replace("requirements", "**Evidence status:** `MISSING`", "**Evidence status:** `VERIFIED`")
        self.reject()

    def test_orphan_design_rejected(self):
        self.replace("design", "**DES-010**", "**DES-999**")
        self.reject()

    def test_dropped_dependency_edge_rejected(self):
        self.replace("tasks", "1.1 → 1.7\n", "")
        self.reject()

    def test_consistent_but_cyclic_graph_rejected(self):
        self.replace("tasks", "Dependencies: none; no prerequisite", "Dependencies: 12.1; no prerequisite")
        self.replace("tasks", "1.1 → 1.7\n", "12.1 → 1.1\n1.1 → 1.7\n")
        self.reject()

    def test_parallel_file_collision_rejected(self):
        self.replace("tasks", "`EXEC/src/budget.rs`", "`EXEC/src/policy.rs`")
        self.reject()

    def test_false_implementation_completion_rejected(self):
        self.replace("tasks", "  - [ ] 2.1", "  - [x] 2.1")
        self.reject()


if __name__ == "__main__":
    unittest.main()
