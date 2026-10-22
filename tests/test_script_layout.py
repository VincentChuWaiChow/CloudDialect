"""Canonical verification-script ownership and legacy command compatibility."""

import ast
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]
RUNS = ".workflow/ultracode"
SCRIPTS = "scripts/verification"
MOVES = {
    f"{RUNS}/execution-isolation-implementation/measure-domain-coverage.py":
        f"{SCRIPTS}/contracts-evidence-foundation/measure-domain-coverage.py",
    f"{RUNS}/execution-isolation-implementation/verify-p0.py":
        f"{SCRIPTS}/contracts-evidence-foundation/verify-p0.py",
    f"{RUNS}/execution-isolation-implementation/verify-planning.py":
        f"{SCRIPTS}/contracts-evidence-foundation/verify-planning.py",
    f"{RUNS}/execution-isolation-implementation/reproduce-document-checks.sh":
        f"{SCRIPTS}/execution-isolation/reproduce-document-checks.sh",
    f"{RUNS}/execution-isolation/check_spec.py":
        f"{SCRIPTS}/execution-isolation/check_spec.py",
    f"{RUNS}/execution-isolation/test_check_spec.py":
        f"{SCRIPTS}/execution-isolation/test_check_spec.py",
    f"{RUNS}/python-oss-to-s3-pack/check_docs.py":
        f"{SCRIPTS}/python-oss-to-s3-pack/check_docs.py",
}


class ScriptLayoutTests(unittest.TestCase):
    def test_canonical_scripts_are_regular_files_grouped_by_domain(self):
        self.assertTrue((ROOT / "scripts/README.md").is_file())
        for canonical in MOVES.values():
            with self.subTest(canonical=canonical):
                path = ROOT / canonical
                self.assertTrue(path.is_file())
                self.assertFalse(path.is_symlink())

    def test_previous_command_paths_are_relative_compatibility_links(self):
        for legacy, canonical in MOVES.items():
            with self.subTest(legacy=legacy):
                path = ROOT / legacy
                self.assertTrue(path.is_symlink())
                self.assertFalse(path.readlink().is_absolute())
                self.assertEqual(path.resolve(strict=True), (ROOT / canonical).resolve(strict=True))

    def test_python_tools_keep_repo_root_and_recorded_data_locations(self):
        for canonical in MOVES.values():
            if not canonical.endswith(".py"):
                continue
            with self.subTest(canonical=canonical):
                path = ROOT / canonical
                self.assertEqual(path.resolve().parents[3], ROOT)
                source = path.read_text()
                ast.parse(source)
                if path.name == "test_check_spec.py":
                    continue
                self.assertIn('ROOT = Path(__file__).resolve().parents[3]', source)
                self.assertIn('ROOT / ".workflow/ultracode/', source)


if __name__ == "__main__":
    unittest.main()
