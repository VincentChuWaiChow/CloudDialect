"""Local document/graph/scope checks, not a semantic or runtime proof."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[3]
RUN = Path(__file__).resolve().parent
SPEC = ROOT / ".kiro/specs/python-oss-to-s3-pack"


def check(doc):
    errors = []

    def require(condition, label):
        if not condition:
            errors.append(label)

    req, design, tasks = (doc[n] for n in ("requirements", "design", "tasks"))
    require(all(line == line.rstrip() for value in doc.values() for line in value.splitlines()), "no trailing whitespace")
    require(all(value.count("```") % 2 == 0 for value in doc.values()), "balanced code fences")
    require(all(h in req for h in ("# Requirements Document", "## Introduction", "## Glossary", "## Requirements")), "canonical requirements headings")
    require(all(h in design for h in ("## Overview", "## Components and Interfaces", "## Correctness Properties", "## Testing Strategy")), "canonical design headings")
    require(tasks.startswith("# Implementation Plan:") and "## Task Dependency Graph" in tasks, "canonical task headings")
    ids = {f"REQ-{i:03}" for i in range(1, 11)}
    des = {f"DES-{i:03}" for i in range(1, 11)}
    require(set(re.findall(r"\*\*ID:\*\* (REQ-\d+)", req)) == ids, "ten requirement groups")
    criteria = re.findall(r"^\d+\. (.+)\n   Verification: (.+)$", req, re.M)
    require(len(criteria) == 67, "67 criteria with verification")
    require(all(c.count("SHALL") == 1 and "Evidence: `MISSING`" in v for c, v in criteria), "atomic criteria and MISSING evidence")
    for i in range(1, 6):
        require(bool(re.search(rf"\| UD-{i} \| UNRESOLVED", req)), f"UD-{i} unresolved")
    require(all(f"| AC-{i:02}" in req for i in (1, 2, 3, 4, 5, 10)), "six acceptance gates")
    require(set(re.findall(r"^\*\*(DES-\d+)\*\*", design, re.M)) == des, "ten design definitions")
    require(set(re.findall(r"REQ-\d+", design)) == ids, "design requirement coverage")
    blocks = re.findall(r"^  - \[ \] (\d+\.\d+) (TASK-\d+):.*?\n(.*?)(?=^  - \[|^- \[|^## |\Z)", tasks, re.M | re.S)
    require(len(blocks) == 17, "17 unchecked task entries")
    graph = {n: set() for n, _, _ in blocks}
    require(len(graph) == 17, "unique task numbers")
    require({tid for _, tid, _ in blocks} == {f"TASK-{i:03}" for i in range(1, 18)}, "unique task IDs")
    for number, _, body in blocks:
        match = re.search(r"Depends on: (.*?)\. Owner:", body)
        require(match is not None, f"prerequisites {number}")
        graph[number] = set(re.findall(r"\d+\.\d+", match[1])) if match else set()
        require(bool(re.search(r"Trace: .*DES-\d+.*REQ-\d+", body)), f"trace {number}")
        require("done criteria:" in body and "Files:" in body, f"deliverable/check {number}")
    require(set(re.findall(r"DES-\d+", tasks)) == des, "task design coverage")
    expected = {(p, n) for n, ps in graph.items() for p in ps}
    edges = set(re.findall(r"(\d+\.\d+) → (\d+\.\d+)", tasks))
    require(edges == expected, "arrows match every prerequisite")
    require(all(p in graph for p, _ in expected), "known prerequisite targets")
    depth, visiting = {}, set()

    def level(node):
        if node in visiting:
            raise ValueError("dependency cycle")
        if node not in depth:
            visiting.add(node)
            depth[node] = 1 + max((level(p) for p in graph[node]), default=0)
            visiting.remove(node)
        return depth[node]

    try:
        require(max(map(level, graph)) == 6, "approved six-level graph")
    except (ValueError, KeyError) as exc:
        errors.append(str(exc))
    waves = re.search(r"```json\n(.*?)\n```", tasks, re.S)
    require(waves is not None, "wave JSON present")
    if waves:
        require(json.loads(waves[1]) == {"waves": [{"id": i, "tasks": [str(i + 1)]} for i in range(3)]}, "three parent waves")
    # Each advertised parallel group must have no transitive dependency within it.
    def ancestors(node, seen):
        if node in seen:
            return seen
        seen.add(node)
        for parent in graph.get(node, set()):
            ancestors(parent, seen)
        return seen
    for line in tasks.splitlines():
        if re.fullmatch(r"\d+\.\d+(?: \|\| \d+\.\d+)+", line):
            group = set(re.findall(r"\d+\.\d+", line))
            require(all(not ((ancestors(n, set()) - {n}) & group) for n in group), "parallel prerequisite independence")
    return errors


doc = {name: (SPEC / f"{name}.md").read_text() for name in ("requirements", "design", "tasks")}
errors = check(doc)
# Negative controls ensure the checker is not vacuously passing.
for name, old, new in (
    ("requirements", "Evidence: `MISSING`", "Evidence: `VERIFIED`"),
    ("requirements", "| UD-1 | UNRESOLVED", "| UD-1 | RESOLVED"),
    ("tasks", "1.1 → 1.3", "1.2 → 1.3"),
    ("tasks", "  - [ ] 1.1", "  - [x] 1.1"),
    ("design", "**DES-001**", "**DES-999**"),
):
    changed = dict(doc)
    changed[name] = changed[name].replace(old, new, 1)
    if not check(changed):
        errors.append(f"negative control missed: {old}")
baseline = json.loads((RUN / "baseline.json").read_text())
for path, digest in baseline["preserved"].items():
    if hashlib.sha256((ROOT / path).read_bytes()).hexdigest() != digest:
        errors.append(f"preexisting artifact changed: {path}")


def git(*args):
    return subprocess.check_output(["rtk", "proxy", "git", *args], cwd=ROOT, text=True).strip()


if git("rev-parse", "HEAD") != baseline["head"] or git("branch", "--show-current") != baseline["branch"]:
    errors.append("baseline branch/HEAD changed")
if git("diff", "HEAD", "--name-only"):
    errors.append("tracked baseline files changed")
allowed = (".kiro/specs/python-oss-to-s3-pack/", ".workflow/ultracode/python-oss-to-s3-pack/")
for path in git("ls-files", "--others", "--exclude-standard").splitlines():
    if path not in baseline["preserved"] and not path.startswith(allowed):
        errors.append(f"unowned new file: {path}")
print(json.dumps({"status": "FAIL" if errors else "PASS", "criteria": 67, "requirements": 10, "design_elements": 10, "tasks": 17, "leaf_depth": 6, "negative_controls": 5, "errors": errors}, indent=2))
raise SystemExit(bool(errors))
