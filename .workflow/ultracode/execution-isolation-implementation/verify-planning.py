"""Static planning contract checks, not implementation or execution permission."""

import hashlib
import json
from pathlib import Path
import re
import unittest


ROOT = Path(__file__).resolve().parents[3]
RUN = Path(__file__).resolve().parent
SPEC = ROOT / ".kiro/specs/contracts-evidence-foundation"
TEXT = (SPEC / "implementation-plan.md").read_text()
GRAPH = json.loads(re.search(r"```json\n(.*?)\n```", TEXT, re.S)[1])
BLOCKS = dict(re.findall(
    r"^  - \[ \] (\d+\.\d+) (.*?)(?=^  - \[ \]|^- \[ \]|^## |\Z)", TEXT, re.M | re.S
))


def depths(graph):
    result = {}

    def visit(node, path=()):
        assert node not in path, ("cycle", path, node)
        assert all(dependency in graph for dependency in graph[node]), node
        if node not in result:
            result[node] = 1 + max((visit(d, (*path, node)) for d in graph[node]), default=0)
        return result[node]

    for node in graph:
        visit(node)
    return result


class ImplementationPlanningTests(unittest.TestCase):
    def test_all_units_have_acyclic_matching_prerequisites(self):
        self.assertEqual(set(BLOCKS), set(GRAPH))
        self.assertEqual(len(GRAPH), 28)
        for node, block in BLOCKS.items():
            declaration = re.search(r"Dependencies:([^\n]+)", block)[1]
            self.assertEqual(set(re.findall(r"\d+\.\d+", declaration)), set(GRAPH[node]), node)
        self.assertGreater(max(depths(GRAPH).values()), 3)
        self.assertIn("G-SCHEDULE", TEXT)
        self.assertIn("not a claim of leaf depth three", TEXT)

    def test_canonical_task_coverage(self):
        covered = set()
        for block in BLOCKS.values():
            line = next(line for line in block.splitlines() if "Spec1:" in line).split("Requirements:")[0]
            for first, last in re.findall(r"(\d+)[–-](\d+)", line):
                covered.update(range(int(first), int(last) + 1))
            covered.update(map(int, re.findall(r"\b\d+\b", line)))
        self.assertTrue(set(range(2, 27)) <= covered)

    def test_concrete_units_have_bounded_scope_and_checks(self):
        for node, block in BLOCKS.items():
            if node in {"1.1", "3.2", "3.7"}:
                continue
            for field in ("Owned:", "Cases:", "Target:", "Done:", "Estimate:"):
                self.assertIn(field, block, (node, field))
            owned = re.search(r"Owned:([^\n]+)", block)[1]
            self.assertLessEqual(len(re.findall(r"`([^`]+)`", owned)), 5, node)
        self.assertIn("NOT a dispatch unit", BLOCKS["3.2"])
        self.assertIn("not a bounded leaf", BLOCKS["3.7"])

    def test_first_slice_stays_narrow(self):
        self.assertEqual(set(re.findall(r"`([^`]+)`", re.search(r"Owned:([^\n]+)", BLOCKS["1.2"])[1])), {
            "src/errors/diagnostics.rs", "src/errors/phase.rs", "src/errors/mod.rs", "src/lib.rs", "tests/phase_results.rs"
        })
        self.assertIn("no BehavioralResult placeholder", BLOCKS["1.2"])
        self.assertIn("G-CODE", BLOCKS["1.2"])
        self.assertIn("ManifestValidationResult", BLOCKS["1.11"])
        self.assertIn("ManifestOutcome", BLOCKS["1.11"])
        self.assertIn("1.11", GRAPH["2.7"])
        self.assertIn("repository-root `THIRD-PARTY-NOTICES.md`", BLOCKS["3.6"])

    def test_fixture_filter_is_not_libtest_name_filter(self):
        for term in ("harness = false", "Cargo.toml", "zero", "nonzero", "fixture counts"):
            self.assertIn(term, BLOCKS["3.1"])
        self.assertIn("default libtest", BLOCKS["3.1"].lower())

    def test_approval_and_pinned_source_are_not_runtime_proof(self):
        self.assertIn("PLANNING ONLY — NO IMPLEMENTATION AUTHORIZED", TEXT)
        self.assertIn("Source verification now confirms", TEXT)
        for name in ("requirements.md", "design.md", "tasks.md", "p0-contract-amendments.md"):
            self.assertIn("Planning", (SPEC / name).read_text())
        source = json.loads((RUN / "results/planning-boon-api.json").read_text())
        self.assertTrue(source["matches_committed_lock"])
        self.assertFalse(source["installed"])
        self.assertFalse(source["executed"])
        self.assertFalse((ROOT / "crates/clouddialect-execution").exists())

    def test_preserves_unowned_files(self):
        baseline = json.loads((RUN / "planning-baseline.json").read_text())
        allowed = {str((SPEC / name).relative_to(ROOT)) for name in (
            "requirements.md", "design.md", "tasks.md", "p0-contract-amendments.md", "implementation-plan.md"
        )}
        for file, digest in baseline["files"].items():
            if file not in allowed:
                self.assertEqual(hashlib.sha256((ROOT / file).read_bytes()).hexdigest(), digest, file)


if __name__ == "__main__":
    unittest.main(verbosity=2)
