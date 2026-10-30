"""Draft-document consistency only; never launches a worker or grants authority.

Requires the already-installed developer tool `jsonschema`; no dependency is
installed or fetched. This is not a production parser, broker or qualification.
"""

import copy
import json
from pathlib import Path
import re
import unittest

from jsonschema import Draft202012Validator


SPEC = Path(__file__).resolve().parents[3] / ".kiro/specs/execution-isolation"


def proposal(path, name):
    text = (SPEC / path).read_text()
    blocks = re.findall(
        rf"<!-- offline-proposal: {re.escape(name)} -->\s*```json\n(.*?)\n```",
        text,
        re.DOTALL,
    )
    if len(blocks) != 1:
        raise AssertionError(f"expected one {name} JSON block, got {len(blocks)}")
    return json.loads(blocks[0])


class OfflineProposalTests(unittest.TestCase):
    def test_handoff_does_not_reintroduce_raw_diagnostic_retention(self):
        handoff = (SPEC / "decisions/BG-01-upstream.md").read_text()
        self.assertNotIn("retain locally under policy", handoff)
        self.assertIn("raw descriptions are transient-only", handoff)

    def test_ipc_schema_and_examples(self):
        schema = proposal("decisions/UD-2-ipc.md", "ipc-schema")
        # The artifact grader is offline: do not allow an edited reference to
        # turn schema validation into an implicit documentation/network fetch.
        self.assertEqual(schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
        pending = [schema]
        while pending:
            node = pending.pop()
            if isinstance(node, dict):
                for keyword in ("$ref", "$dynamicRef"):
                    if keyword in node:
                        self.assertTrue(node[keyword].startswith("#/"), "external schema reference")
                pending.extend(node.values())
            elif isinstance(node, list):
                pending.extend(node)
        Draft202012Validator.check_schema(schema)
        validator = Draft202012Validator(schema)
        cases = proposal("qualification/ipc-cases.md", "ipc-vectors")
        self.assertTrue(cases, "empty schema coverage is not evidence")
        self.assertEqual(len(cases), len({case["id"] for case in cases}))
        self.assertEqual({case["valid"] for case in cases}, {True, False})
        methods = set()
        for case in cases:
            with self.subTest(case=case["id"]):
                self.assertEqual(validator.is_valid(case["message"]), case["valid"])
                if case["valid"] and "method" in case["message"]:
                    methods.add(case["message"]["method"])
                    extra = copy.deepcopy(case["message"])
                    extra["accept"] = True
                    self.assertFalse(validator.is_valid(extra), "unknown authority field")
                    extra = copy.deepcopy(case["message"])
                    extra["id"] = None
                    self.assertFalse(validator.is_valid(extra), "null request ID")
        self.assertEqual(methods, {"worker.hello", "worker.execute", "core.tool", "worker.cancel"})

    def test_numbered_tasks_and_gates_stay_open(self):
        tasks = (SPEC / "tasks.md").read_text()
        self.assertFalse(re.search(r"^\s*- \[[xX]\] \d+\.", tasks, re.MULTILINE))
        self.assertEqual(len(re.findall(r"^  - \[ \] \d+\.\d+ TASK-\d+", tasks, re.MULTILINE)), 21)
        for gate in ("UD-1-isolation", "UD-2-ipc", "UD-3-credentials", "UD-4-persistence", "UD-5-budgets", "BG-01-upstream"):
            with self.subTest(gate=gate):
                self.assertIn("OPEN", (SPEC / "decisions" / f"{gate}.md").read_text().split("\n\n", 2)[1])


if __name__ == "__main__":
    unittest.main()
