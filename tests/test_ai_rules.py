"""Repository AI-rule discovery links, not proof of native client activation."""

from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]
CANONICAL = ROOT / ".agents/rules/engineering.md"
ENTRYPOINTS = (
    "AGENTS.md",
    "CLAUDE.md",
    "GEMINI.md",
    ".github/copilot-instructions.md",
    ".kiro/steering/engineering.md",
    ".cursor/rules/engineering.mdc",
)


class AiRulesTests(unittest.TestCase):
    def test_previous_rule_copy_is_absent(self):
        previous = ROOT / ".ai/rules/engineering.md"
        self.assertFalse(previous.exists() or previous.is_symlink())

    def test_one_canonical_rule_defines_domain_ownership_and_safety(self):
        self.assertTrue(CANONICAL.is_file(), "canonical repository AI rule missing")
        text = CANONICAL.read_text()
        for requirement in (
            "Domain-first organization", "public API", "shared kernel",
            "rtk", "Context7", "resolve-library-id", "query-docs",
            "TDD", "Do not commit", "delegate", "inclusion: always",
            "alwaysApply: true",
        ):
            with self.subTest(requirement=requirement):
                self.assertIn(requirement, text)

    def test_every_entrypoint_is_a_relative_symlink_to_the_same_rule(self):
        for entrypoint in ENTRYPOINTS:
            with self.subTest(entrypoint=entrypoint):
                path = ROOT / entrypoint
                self.assertTrue(path.is_symlink(), f"{entrypoint} must not duplicate the rule")
                self.assertFalse(path.readlink().is_absolute())
                self.assertEqual(path.resolve(strict=True), CANONICAL.resolve(strict=True))


if __name__ == "__main__":
    unittest.main()
