"""Task 1 scaffold checks; run with Python 3.11+ and no third-party packages."""

from pathlib import Path
import re
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "crates" / "clouddialect-contracts"
DEPENDENCIES = {
    "boon", "toml", "include_dir", "uuid", "serde", "serde_json", "regex",
    "sha2", "sha3", "semver",
}


class ContractsScaffoldTests(unittest.TestCase):
    def manifest(self):
        path = CRATE / "Cargo.toml"
        self.assertTrue(path.is_file(), "task 1 library manifest is missing")
        return tomllib.loads(path.read_text())

    def test_package_is_a_standalone_library(self):
        manifest = self.manifest()
        self.assertEqual(manifest["package"]["name"], "clouddialect-contracts")
        self.assertEqual(manifest["package"]["edition"], "2024")
        self.assertFalse(manifest["package"]["publish"])
        self.assertNotIn("bin", manifest)
        self.assertNotIn("workspace", manifest)
        self.assertFalse((CRATE / "src" / "main.rs").exists())
        self.assertFalse((CRATE / "src" / "bin").exists())
        self.assertTrue((CRATE / "src" / "lib.rs").is_file())

    def test_required_dependencies_are_exactly_pinned(self):
        dependencies = self.manifest()["dependencies"]
        self.assertEqual(set(dependencies), DEPENDENCIES)
        for name, dependency in dependencies.items():
            with self.subTest(dependency=name):
                version = dependency if isinstance(dependency, str) else dependency["version"]
                self.assertRegex(version, r"^=\d+\.\d+\.\d+(?:\+[0-9A-Za-z.-]+)?$")

    def test_required_dependency_features_are_enabled(self):
        dependencies = self.manifest()["dependencies"]
        self.assertIn("derive", dependencies["serde"]["features"])
        self.assertIn("v4", dependencies["uuid"]["features"])
        self.assertIn("serde", dependencies["toml"]["features"])
        self.assertIn("raw_value", dependencies["serde_json"]["features"])
        self.assertEqual(dependencies["sha2"], {"version": "=0.11.1", "default-features": False})
        self.assertEqual(dependencies["sha3"], {"version": "=0.12.0", "default-features": False})
        self.assertEqual(dependencies["semver"], "=1.0.28")
        self.assertEqual(self.manifest()["package"]["rust-version"], "1.88")

    def test_public_module_tree_exists(self):
        lib = CRATE / "src" / "lib.rs"
        self.assertTrue(lib.is_file(), "task 1 library root is missing")
        source = lib.read_text()
        modules = set(re.findall(r"^pub mod (\w+);$", source, re.MULTILINE))
        self.assertEqual(modules, {"schema", "validator", "manifest", "errors"})
        for module in modules:
            with self.subTest(module=module):
                self.assertTrue((CRATE / "src" / module / "mod.rs").is_file())

    def test_conformance_integration_target_exists(self):
        self.assertTrue((CRATE / "tests" / "conformance.rs").is_file())

    def test_lockfile_resolves_direct_dependencies_with_checksums(self):
        dependencies = self.manifest()["dependencies"]
        lock = CRATE / "Cargo.lock"
        self.assertTrue(lock.is_file(), "task 1 committed lockfile is missing")
        packages = tomllib.loads(lock.read_text())["package"]
        for name, dependency in dependencies.items():
            version = dependency if isinstance(dependency, str) else dependency["version"]
            with self.subTest(dependency=name):
                matches = [p for p in packages if p["name"] == name and p["version"].split("+")[0] == version[1:]]
                self.assertEqual(len(matches), 1)
                self.assertEqual(matches[0]["source"], "registry+https://github.com/rust-lang/crates.io-index")
                self.assertRegex(matches[0]["checksum"], r"^[0-9a-f]{64}$")

    def test_package_retains_repository_license(self):
        package = self.manifest()["package"]
        self.assertNotIn("license", package, "do not mislabel the crate as MIT/Apache")
        self.assertEqual((CRATE / package["license-file"]).resolve(), ROOT / "LICENSE.md")


if __name__ == "__main__":
    unittest.main()
