"""Grade documentation only. No Rust/runtime, native isolation or approval proof."""

from copy import deepcopy
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tomllib
import unittest

from jsonschema import Draft202012Validator, FormatChecker
from referencing import Registry
from referencing.exceptions import NoSuchResource


ROOT = Path(__file__).resolve().parents[3]
RUN = Path(__file__).resolve().parent
SPEC = ROOT / ".kiro/specs/contracts-evidence-foundation"
TEXTS = {name: (SPEC / name).read_text() for name in (
    "requirements.md", "design.md", "tasks.md", "p0-contract-amendments.md"
)}
SCHEMAS = [json.loads(body) for body in re.findall(
    r"```json\n(.*?)\n```", TEXTS["design.md"], re.S
) if body.lstrip().startswith('{\n  "$schema"')]
SCHEMA = next(s for s in SCHEMAS if "/verification-result/" in s["$id"])


def deny_retrieval(uri):
    raise NoSuchResource(ref=uri)


VALIDATOR = Draft202012Validator(
    SCHEMA, registry=Registry(retrieve=deny_retrieval), format_checker=FormatChecker()
)
ARTIFACT = {"identity": "fixture", "content_hash": {"algorithm": "sha-256", "value": "a" * 64}}
VALID = {
    "id": "acee3ad0-709c-466b-b163-a947a58d711a", "format_version": "1.0.0",
    "candidate": deepcopy(ARTIFACT), "configuration": deepcopy(ARTIFACT),
    "pack": {"id": "io.fixture.pack", "version": "1.2.3-rc.1", "content_hash": ARTIFACT["content_hash"]},
    "toolchain": {"id": "fixture-toolchain", "version": "1.2.3+build.1", "content_hash": ARTIFACT["content_hash"]},
    "binding": {**{key: "fixture" for key in ("tenant_id", "project_id", "run_id", "attempt_id")},
                **{key: deepcopy(ARTIFACT) for key in ("input", "dependencies", "suite", "oracle", "policy")},
                "obligation_ids": ["obligation-1"]},
    "tests": [{"test_id": "test-1", "obligation_ids": ["obligation-1"], "outcome": "pass", "origin_identity": "untrusted-claim"}],
}


class P0DocumentationTests(unittest.TestCase):
    def test_draft_status_and_no_gate_closure(self):
        for name, text in TEXTS.items():
            with self.subTest(file=name):
                self.assertIn("DRAFT — PENDING OWNER APPROVAL", text)
                self.assertIn("BG-01", text)

    def test_full_embedded_schemas_and_toml_parse(self):
        self.assertEqual(len(SCHEMAS), 4)
        for schema in SCHEMAS:
            Draft202012Validator.check_schema(schema)
            self.assertIn("$schema", schema["properties"])
        for text in (TEXTS["design.md"], TEXTS["p0-contract-amendments.md"]):
            for body in re.findall(r"```toml\n(.*?)\n```", text, re.S):
                tomllib.loads(body)

    def test_valid_verification_and_required_fields(self):
        self.assertEqual(set(SCHEMA["required"]),
                         {"id", "format_version", "candidate", "pack", "toolchain", "configuration", "binding", "tests"})
        self.assertEqual(list(VALIDATOR.iter_errors(VALID)), [])
        for key in SCHEMA["required"]:
            value = deepcopy(VALID)
            del value[key]
            with self.subTest(missing=key):
                self.assertTrue(list(VALIDATOR.iter_errors(value)))

    def test_nonpass_failure_object_and_unicode_boundaries(self):
        for outcome in ("fail", "skip", "error"):
            value = deepcopy(VALID)
            test = value["tests"][0]
            test["outcome"] = outcome
            self.assertTrue(list(VALIDATOR.iter_errors(value)))
            for bad in ("legacy string", {"detail": ""}, {"detail": "a" * 2001}, {"detail": "safe", "raw": "no"}):
                test["failure_description"] = bad
                self.assertTrue(list(VALIDATOR.iter_errors(value)))
            test["failure_description"] = {"detail": "é" * 2000}
            self.assertEqual(list(VALIDATOR.iter_errors(value)), [])

    def test_binding_test_and_digest_shape_negatives(self):
        self.assertEqual(set(SCHEMA["$defs"]["verification_binding"]["required"]),
                         {"tenant_id", "project_id", "run_id", "attempt_id", "input", "dependencies", "suite", "oracle", "policy", "obligation_ids"})
        for key in SCHEMA["$defs"]["verification_binding"]["required"]:
            value = deepcopy(VALID)
            del value["binding"][key]
            self.assertTrue(list(VALIDATOR.iter_errors(value)))
        for key in ("pack", "toolchain"):
            value = deepcopy(VALID)
            del value[key]["content_hash"]
            self.assertTrue(list(VALIDATOR.iter_errors(value)))
        for bad in ([], ["obligation-1", "obligation-1"]):
            value = deepcopy(VALID)
            value["tests"][0]["obligation_ids"] = bad
            self.assertTrue(list(VALIDATOR.iter_errors(value)))

    def test_full_semver_boundaries(self):
        for version in ("1.2.3", "1.2.3-rc.1", "1.2.3+build.1", "1.2.3-0", "1.2.3-x-y-z.--"):
            value = deepcopy(VALID)
            value["pack"]["version"] = version
            self.assertEqual(list(VALIDATOR.iter_errors(value)), [])
        for version in ("01.2.3", "1.2.3-01", "1.2.3-", "1.2.3+", "1.2.3\n"):
            value = deepcopy(VALID)
            value["pack"]["version"] = version
            self.assertTrue(list(VALIDATOR.iter_errors(value)))

    def test_approval_shape_is_not_authentication(self):
        value = deepcopy(VALID)
        value["acceptance"] = {
            "approver_identities": ["forged-claim"], "timestamp": "2026-10-22T00:00:00Z",
            "verification_result": {"id": value["id"], "format_version": "1.0.0"},
            "approval_evidence": deepcopy(ARTIFACT),
        }
        self.assertEqual(list(VALIDATOR.iter_errors(value)), [])
        for timestamp in ("2026-10-22T00:00:00+01:00",):
            value["acceptance"]["timestamp"] = timestamp
            self.assertTrue(list(VALIDATOR.iter_errors(value)))
        value["acceptance"]["timestamp"] = "not-a-dateZ"
        if "date-time" in FormatChecker().checkers:
            self.assertTrue(list(VALIDATOR.iter_errors(value)))
        else:
            # Optional format implementation is absent: semantic date checks remain unverified.
            self.assertEqual(list(VALIDATOR.iter_errors(value)), [])
        # This deliberate structural acceptance demonstrates a separate semantic gate.
        value = deepcopy(VALID)
        value["candidate"]["content_hash"]["value"] = "a"
        self.assertEqual(list(VALIDATOR.iter_errors(value)), [])
        self.assertIn("exact length", TEXTS["p0-contract-amendments.md"])

    def test_schema_kind_and_api_consistency(self):
        for file in ("design.md", "tasks.md", "p0-contract-amendments.md"):
            text = TEXTS[file]
            self.assertIn("kind: DocumentKind, document: &[u8]", text)
            self.assertIn("validate_manifest_semantic", text)
        self.assertNotIn("ContractStore", "\n".join(TEXTS.values()))
        self.assertNotIn("BehavioralResult::Deferred", "\n".join(TEXTS.values()))
        self.assertIn("expected_match", TEXTS["tasks.md"])
        self.assertIn("never normalize EvidenceComplete", TEXTS["tasks.md"])

    def test_capability_and_catalog_contracts(self):
        for name in ("design.md", "tasks.md", "p0-contract-amendments.md"):
            self.assertIn("capabilities", TEXTS[name])
            self.assertIn("DocumentStore", TEXTS[name])
        catalog = next(s for s in SCHEMAS if "/catalog-index/" in s["$id"])
        self.assertIn("resolver_resources", catalog["required"])
        self.assertEqual(set(catalog["properties"]["schemas"]["items"]["properties"]["document_type"]["enum"]),
                         {"behavioral-contract", "evidence-record", "verification-result", "catalog-index", "extension-registry"})

    def test_scope_and_existing_task_checkmarks(self):
        baseline = json.loads((RUN / "p0-baseline.json").read_text())
        allowed = {str((SPEC / name).relative_to(ROOT)) for name in TEXTS}
        for file, digest in baseline["files"].items():
            if file not in allowed:
                self.assertEqual(hashlib.sha256((ROOT / file).read_bytes()).hexdigest(), digest, file)
        tasks = re.findall(r"^- \[([ x])\] (\d+)\.", TEXTS["tasks.md"], re.M)
        self.assertEqual(tasks, [("x" if n == 1 else " ", str(n)) for n in range(1, 27)])
        for name, text in TEXTS.items():
            original = ""
            if name != "p0-contract-amendments.md":
                original = subprocess.check_output([
                    "rtk", "proxy", "git", "show", f"{baseline['head']}:{(SPEC / name).relative_to(ROOT)}"
                ], text=True)
            old_spaces = {line for line in original.splitlines() if line.rstrip() != line}
            new_spaces = {line for line in text.splitlines() if line.rstrip() != line}
            self.assertFalse(new_spaces - old_spaces, name)


if __name__ == "__main__":
    unittest.main(verbosity=2)
