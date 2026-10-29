"""Offline tests: load the draft collector with synthetic OS/signal/stdout ports.

The fixed workflow envelope is frozen verbatim, avoiding a YAML dependency and
rejecting extra triggers, actions or out-of-block configuration. No real host
metadata, environment values, devices, signal handlers or CI are inspected.
"""

import ast
import io
import json
from pathlib import Path
import re
import stat
import textwrap
from types import SimpleNamespace
import unittest


WORKFLOW = Path(__file__).resolve().parents[3] / ".github/workflows/isolation-preflight.yml"
ENVELOPE = """name: Isolation metadata preflight
on: workflow_dispatch
permissions: {}
concurrency:
  group: isolation-metadata-preflight
  cancel-in-progress: false
jobs:
  metadata:
    runs-on: ubuntu-24.04
    timeout-minutes: 2
    steps:
      - name: Collect allowlisted public metadata
        shell: python
        run: |
"""
SECRET = "SYNTHETIC_PRIVATE_VALUE_DO_NOT_EMIT"


def collector_source():
    text = WORKFLOW.read_text()
    if not text.startswith(ENVELOPE):
        raise AssertionError("Workflow envelope differs from the approved local-draft contract")
    body = text[len(ENVELOPE):]
    if not body or any(line.strip() and not line.startswith(" " * 10) for line in body.splitlines()):
        raise AssertionError("Unexpected content outside the inline Python block")
    return textwrap.dedent(body)


class SyntheticEnvironment:
    def __init__(self):
        self.values = {"ImageOS": "ubuntu24", "ImageVersion": "20261023.334.3"}
        self.reads = []

    def get(self, key, default=None):
        if key not in {"ImageOS", "ImageVersion"}:
            raise AssertionError("Unapproved environment access")
        self.reads.append(key)
        return self.values.get(key, default)

    def __iter__(self):
        raise AssertionError("Environment dumps are forbidden")


class SyntheticOS:
    def __init__(self):
        self.environ = SyntheticEnvironment()
        # Deliberately omit hostname and verbose kernel-build fields.
        self.identity = SimpleNamespace(system="Linux", release="6.17.0-1022-azure", machine="x86_64")
        self.uname_error = None
        self.stat_error = None
        self.mode = stat.S_IFCHR
        self.stat_calls = []

    def uname(self):
        if self.uname_error:
            raise self.uname_error
        return self.identity

    def stat(self, path, *, follow_symlinks):
        self.stat_calls.append((path, follow_symlinks))
        if path != "/dev/kvm" or follow_symlinks is not False:
            raise AssertionError("Only non-following KVM node metadata is allowed")
        if self.stat_error:
            raise self.stat_error
        return SimpleNamespace(st_mode=self.mode)


class SyntheticSignal:
    SIGALRM = 14

    def __init__(self):
        self.handler = None
        self.alarms = []
        self.expire = False

    def signal(self, number, handler):
        if number != self.SIGALRM:
            raise AssertionError("Unexpected signal")
        self.handler = handler

    def alarm(self, seconds):
        self.alarms.append(seconds)
        if seconds and self.expire:
            self.handler(self.SIGALRM, None)


def validated_definitions(source):
    """Reject definition-time work; imports are validated but never executed."""
    tree = ast.parse(source)
    modules = {"json", "os", "re", "signal", "stat", "sys"}
    calls = {
        "DeadlineExceeded", "MetadataBlocked", "blocked_record", "collect_metadata",
        "write_record", "main", "SystemExit", "re.fullmatch", "os.uname", "os.stat",
        "os.environ.get", "stat.S_ISCHR", "json.dumps", "len", "line.encode",
        "sys.stdout.write", "signal.signal", "signal.alarm",
    }
    constant = ast.dump(ast.parse("OUTPUT_LIMIT_BYTES = 65536").body[0])
    guard = ast.dump(ast.parse('if __name__ == "__main__":\n    raise SystemExit(main())').body[0])
    exceptions = {ast.dump(ast.parse(f"class {name}(Exception):\n    pass").body[0])
                  for name in ("MetadataBlocked", "DeadlineExceeded")}
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom):
            raise AssertionError("Unapproved import")
        if isinstance(node, ast.Import):
            if node not in tree.body or len(node.names) != 1 or node.names[0].name not in modules or node.names[0].asname:
                raise AssertionError("Unapproved import")
        if isinstance(node, ast.Call) and ast.unparse(node.func) not in calls:
            raise AssertionError("Unapproved collector call")
    definitions = []
    for node in tree.body:
        if isinstance(node, ast.Import):
            continue
        if isinstance(node, ast.If) and ast.dump(node) == guard:
            continue
        if isinstance(node, ast.FunctionDef):
            if (node.decorator_list or node.args.defaults or node.args.kw_defaults
                    or node.returns or getattr(node, "type_params", [])
                    or any(arg.annotation for arg in ast.walk(node.args) if isinstance(arg, ast.arg))):
                raise AssertionError("Definition-time execution is forbidden")
        elif isinstance(node, ast.ClassDef) and ast.dump(node) in exceptions:
            pass
        elif isinstance(node, ast.Assign) and ast.dump(node) == constant:
            pass
        else:
            raise AssertionError("Unexpected executable top-level statement")
        definitions.append(node)
    return ast.Module(body=definitions, type_ignores=[])


def synthetic_collector(source=None):
    tree = validated_definitions(collector_source() if source is None else source)
    fake_os, fake_signal, output = SyntheticOS(), SyntheticSignal(), io.StringIO()
    namespace = {"__name__": "synthetic_preflight", "json": json, "re": re, "stat": stat,
                 "os": fake_os, "signal": fake_signal, "sys": SimpleNamespace(stdout=output)}
    exec(compile(tree, "<draft-collector>", "exec"), namespace)
    return namespace, fake_os, fake_signal, output


class WorkflowContractTests(unittest.TestCase):
    def test_manual_only_workflow_envelope(self):
        self.assertTrue(WORKFLOW.is_file(), "Manual-only metadata preflight workflow is missing")
        self.assertTrue(WORKFLOW.read_text().startswith(ENVELOPE))
        ast.parse(collector_source())

    def test_only_standard_library_and_approved_os_calls(self):
        tree = ast.parse(collector_source())
        imports = {alias.name for node in ast.walk(tree) if isinstance(node, ast.Import) for alias in node.names}
        self.assertEqual(imports, {"json", "os", "re", "signal", "stat", "sys"})
        self.assertFalse(any(isinstance(node, ast.ImportFrom) for node in ast.walk(tree)))
        self.assertNotIn("${{", WORKFLOW.read_text())
        os_calls, environment_keys = [], []
        for node in ast.walk(tree):
            if not isinstance(node, ast.Call):
                continue
            name = ast.unparse(node.func)
            self.assertNotIn(name, {"open", "exec", "eval", "__import__", "print"})
            if name.startswith("os."):
                os_calls.append(name)
                self.assertIn(name, {"os.uname", "os.environ.get", "os.stat"})
            if name == "os.environ.get":
                environment_keys.append(ast.literal_eval(node.args[0]))
            if name == "os.stat":
                self.assertEqual(ast.literal_eval(node.args[0]), "/dev/kvm")
                self.assertEqual([(k.arg, ast.literal_eval(k.value)) for k in node.keywords], [("follow_symlinks", False)])
        self.assertEqual(sorted(environment_keys), ["ImageOS", "ImageVersion"])
        self.assertEqual(os_calls.count("os.stat"), 1)

    def test_synthetic_loader_rejects_top_level_host_probe(self):
        with self.assertRaisesRegex(AssertionError, "top-level"):
            synthetic_collector(collector_source() + "\nunused = os.uname()\n")

    def test_synthetic_loader_rejects_definition_time_host_probe(self):
        source = collector_source().replace("def collect_metadata():", "def collect_metadata(unused=os.uname()):")
        with self.assertRaisesRegex(AssertionError, "Definition-time"):
            synthetic_collector(source)

    def test_synthetic_loader_rejects_unapproved_import(self):
        with self.assertRaisesRegex(AssertionError, "Unapproved import"):
            synthetic_collector(collector_source() + "\nimport subprocess\n")


class CollectorTests(unittest.TestCase):
    def setUp(self):
        self.ns, self.os, self.signal, self.output = synthetic_collector()

    def run_collector(self):
        code = self.ns["main"]()
        emitted = self.output.getvalue()
        self.assertLessEqual(len(emitted.encode("utf-8")), 65536)
        self.assertNotIn(SECRET, emitted)
        record = json.loads(emitted)
        self.assertEqual(record["scope"], "host-metadata-only")
        self.assertEqual(record["qualification"], "NOT_EVALUATED")
        self.assertIs(record["kvm_opened"], False)
        self.assertEqual(self.signal.alarms, [10, 0])
        return code, record

    def assert_blocked(self, reason):
        code, record = self.run_collector()
        self.assertEqual(code, 1)
        self.assertEqual(record["status"], "BLOCKED")
        self.assertEqual(record["reason"], reason)
        self.assertNotIn("metadata", record)

    def test_valid_metadata_is_observation_not_qualification(self):
        code, record = self.run_collector()
        self.assertEqual(code, 0)
        self.assertEqual(record["status"], "OBSERVED")
        self.assertEqual(set(record), {"scope", "qualification", "kvm_opened", "status", "metadata"})
        self.assertEqual(record["metadata"], {
            "kernel": {"system": "Linux", "release": "6.17.0-1022-azure", "architecture": "x86_64"},
            "runner_image": {"os": "ubuntu24", "version": "20261023.334.3"},
            "kvm_character_device_visible": True,
        })
        self.assertEqual(self.os.stat_calls, [("/dev/kvm", False)])
        self.assertEqual(self.os.environ.reads, ["ImageOS", "ImageVersion"])

    def test_missing_image_os_blocks_before_device_metadata(self):
        self.os.environ.values.pop("ImageOS")
        self.assert_blocked("runner_identity_invalid")
        self.assertEqual(self.os.stat_calls, [])

    def test_missing_image_version_blocks(self):
        self.os.environ.values.pop("ImageVersion")
        self.assert_blocked("runner_identity_invalid")

    def test_wrong_image_os_blocks_without_echo(self):
        self.os.environ.values["ImageOS"] = SECRET
        self.assert_blocked("runner_identity_invalid")

    def test_malformed_image_version_blocks_without_echo(self):
        self.os.environ.values["ImageVersion"] = SECRET + "\n"
        self.assert_blocked("runner_identity_invalid")

    def test_unsupported_os_blocks(self):
        self.os.identity.system = "Darwin"
        self.assert_blocked("unsupported_host")

    def test_unsupported_architecture_blocks(self):
        self.os.identity.machine = "aarch64"
        self.assert_blocked("unsupported_host")

    def test_kernel_control_characters_block_without_echo(self):
        self.os.identity.release = SECRET + "\n"
        self.assert_blocked("kernel_identity_invalid")

    def test_kernel_length_boundary(self):
        self.os.identity.release = "k" * 128
        code, record = self.run_collector()
        self.assertEqual(code, 0)
        self.assertEqual(len(record["metadata"]["kernel"]["release"]), 128)

    def test_oversized_kernel_identity_blocks(self):
        self.os.identity.release = "k" * 129
        self.assert_blocked("kernel_identity_invalid")

    def test_unavailable_uname_sanitizes_error(self):
        self.os.uname_error = OSError(SECRET)
        self.assert_blocked("metadata_unavailable")

    def test_missing_kvm_node_blocks(self):
        self.os.stat_error = FileNotFoundError(SECRET)
        self.assert_blocked("kvm_node_missing")

    def test_denied_kvm_metadata_blocks(self):
        self.os.stat_error = PermissionError(SECRET)
        self.assert_blocked("kvm_metadata_denied")

    def test_regular_file_is_not_kvm_character_device(self):
        self.os.mode = stat.S_IFREG
        self.assert_blocked("kvm_not_character_device")

    def test_symlink_is_not_followed_or_accepted(self):
        self.os.mode = stat.S_IFLNK
        self.assert_blocked("kvm_not_character_device")

    def test_unexpected_error_has_no_traceback_or_diagnostics(self):
        self.os.uname_error = RuntimeError(SECRET)
        self.assert_blocked("metadata_unavailable")

    def test_collection_deadline_blocks_and_disarms_alarm(self):
        self.signal.expire = True
        self.assert_blocked("collection_timeout")
        self.assertEqual(self.os.stat_calls, [])

    def test_oversized_output_is_replaced_not_truncated_into_success(self):
        record = {"status": "OBSERVED", "metadata": SECRET + "x" * 65536}
        code = self.ns["write_record"](record)
        emitted = self.output.getvalue()
        self.assertEqual(code, 1)
        self.assertLessEqual(len(emitted.encode("utf-8")), 65536)
        self.assertNotIn(SECRET, emitted)
        self.assertEqual(json.loads(emitted)["reason"], "output_limit")

    def test_output_limit_includes_newline_and_accepts_exact_boundary(self):
        record = {"status": "OBSERVED", "metadata": ""}
        overhead = len(json.dumps(record, sort_keys=True, separators=(",", ":")).encode("utf-8")) + 1
        for size in (65535, 65536, 65537):
            with self.subTest(encoded_bytes=size):
                self.output.seek(0)
                self.output.truncate(0)
                record["metadata"] = "x" * (size - overhead)
                code = self.ns["write_record"](record)
                emitted = self.output.getvalue()
                if size <= 65536:
                    self.assertEqual(code, 0)
                    self.assertEqual(len(emitted.encode("utf-8")), size)
                else:
                    self.assertEqual(code, 1)
                    self.assertLessEqual(len(emitted.encode("utf-8")), 65536)
                    self.assertEqual(json.loads(emitted)["reason"], "output_limit")


if __name__ == "__main__":
    unittest.main()
