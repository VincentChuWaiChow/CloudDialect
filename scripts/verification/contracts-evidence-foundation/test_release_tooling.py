"""Task 26 file-contract checks, not license-engine or signing qualification."""

from pathlib import Path
import re
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[3]
ALLOW = {
    "MIT", "Apache-2.0", "Apache-2.0 WITH LLVM-exception", "BSD-2-Clause",
    "BSD-3-Clause", "ISC", "Unicode-DFS-2016",
}
DENY = {
    "GPL-2.0", "GPL-3.0", "AGPL-1.0", "AGPL-3.0", "SSPL-1.0",
    "LGPL-2.0", "LGPL-2.1", "LGPL-3.0",
}


class ReleaseToolingTests(unittest.TestCase):
    def readme(self):
        return (ROOT / "release/README.md").read_text()

    def test_supported_allow_only_license_policy(self):
        policy = tomllib.loads((ROOT / "deny.toml").read_text())
        self.assertEqual(set(policy), {"licenses"})
        licenses = policy["licenses"]
        self.assertEqual(set(licenses), {"allow"})
        self.assertEqual(set(licenses["allow"]), ALLOW)
        self.assertEqual(len(licenses["allow"]), len(ALLOW))
        self.assertTrue(DENY.isdisjoint(licenses["allow"]))

    def test_design_uses_the_same_supported_policy(self):
        design = (ROOT / ".kiro/specs/contracts-evidence-foundation/design.md").read_text()
        section = design.split("### License Screening", 1)[1].split("### Traceability Artifacts", 1)[0]
        excerpt = re.search(r"```toml\n(.*?)\n```", section, re.DOTALL)
        self.assertIsNotNone(excerpt)
        documented = tomllib.loads(excerpt.group(1))
        actual = tomllib.loads((ROOT / "deny.toml").read_text())
        self.assertEqual(documented, actual)
        task = (ROOT / ".kiro/specs/contracts-evidence-foundation/tasks.md").read_text()
        section = task.split("26. Add release tooling configuration and documentation", 1)[1]
        self.assertIn("allow-only", section)
        self.assertIn("all other licenses are denied by default", section)

    def test_sbom_procedure_and_cli_mismatch_are_explicit(self):
        text = self.readme()
        for required in (
            "cargo cyclonedx --format json --output-file release/sbom.cdx.json",
            "does not support `--output-file`", "--override-filename sbom.cdx",
            "CycloneDX JSON", "direct and transitive", "versions", "licenses",
            "cp crates/clouddialect-contracts/sbom.cdx.json release/sbom.cdx.json",
        ):
            self.assertIn(required, text)

    def test_nested_manifest_and_current_license_policy(self):
        text = self.readme()
        for required in (
            "cargo deny check licenses", "0.16.0", "allow-only",
            "all other licenses are denied by default",
            "cargo deny --manifest-path crates/clouddialect-contracts/Cargo.toml",
            "--config deny.toml check licenses", "named incompatibility report",
            "dependency name, version, and license", "non-zero", "Do not release",
        ):
            self.assertIn(required, text)
        self.assertNotIn("Compatibility blocker", text)
        for license_id in DENY:
            self.assertIn(f"`{license_id}`", text)

    def test_signing_and_offline_verification(self):
        text = self.readme()
        for required in (
            "minisign -S -s /path/to/private.key -m", "minisign -V -p release/minisign.pub -m",
            ".tar.gz.minisig", "no network access", "before extraction",
            "private key must never be committed", "placeholder is not a usable public key",
        ):
            self.assertIn(required, text)

    def test_placeholder_is_comment_only_not_a_key(self):
        lines = (ROOT / "release/minisign.pub").read_text().splitlines()
        self.assertTrue(lines)
        self.assertTrue(all(line.startswith("#") for line in lines))
        self.assertIn("actual public key", " ".join(lines))
        self.assertIn("first release is cut", " ".join(lines))

    def test_ordered_pre_release_gate(self):
        text = self.readme().split("## Pre-release gate sequence", 1)[1]
        steps = re.findall(r"^([1-5])\. (.+)$", text, re.MULTILINE)
        self.assertEqual([number for number, _ in steps], list("12345"))
        for (_, step), required in zip(steps, (
            "cargo deny check licenses", "cargo test --test conformance",
            "SBOM", "archive", "Sign",
        )):
            self.assertIn(required, step)


if __name__ == "__main__":
    unittest.main()
