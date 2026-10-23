"""Document-only checks; not a sandbox, implementation, or SMT verifier."""

from pathlib import Path
import hashlib
import json
import re
import subprocess


ROOT = Path(__file__).resolve().parents[3]
RUN = ROOT / ".workflow/ultracode/execution-isolation"
SPEC = ROOT / ".kiro/specs/execution-isolation"


def require(condition, message):
    if not condition:
        raise SystemExit(f"FAIL: {message}")


def main():
    texts = {}
    for name in ("requirements", "design", "tasks", "analysis"):
        path = SPEC / f"{name}.md"
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
        texts[name] = path.read_text()
    req, design, tasks = (texts[n] for n in ("requirements", "design", "tasks"))
    for name, title, headings in (
        ("requirements", "# Requirements Document", ["Introduction", "Glossary", "Requirements", "Unresolved Decisions", "Threat Model", "Acceptance Gate Traceability"]),
        ("design", "# Design: execution-isolation", ["Overview", "Components and Interfaces", "Correctness Properties", "Testing Strategy"]),
        ("tasks", "# Implementation Plan: execution-isolation", ["Overview", "Tasks", "Notes", "Task Dependency Graph"]),
    ):
        require(texts[name].splitlines()[0] == title, f"{name}: title")
        for heading in headings:
            require(f"\n## {heading}\n" in texts[name], f"{name}: {heading}")
    blocks = re.findall(r"^### Requirement (\d+):[^\n]+\n(.*?)(?=^### Requirement |^## |\Z)", req, re.M | re.S)
    require([int(n) for n, _ in blocks] == list(range(1, 11)), "ten sequential requirement groups")
    criteria = set()
    for n, block in blocks:
        for field in (f"**ID:** REQ-{int(n):03}", "**User Story:**", "**Verification method:**", "**Evidence status:** `MISSING`", "#### Acceptance Criteria"):
            require(field in block, f"requirement {n}: {field}")
        items = re.findall(r"^(\d+)\. (.+)$", block, re.M)
        require([int(i) for i, _ in items] == list(range(1, len(items) + 1)), f"requirement {n}: numbering")
        require(bool(items), f"requirement {n}: nonempty")
        for i, criterion in items:
            require(len(re.findall(r"\bSHALL\b", criterion)) == 1, f"{n}.{i}: atomic SHALL")
            require(re.match(r"(THE|WHEN|IF|WHILE|WHERE)\b", criterion), f"{n}.{i}: EARS")
            criteria.add(f"{n}.{i}")
    require(len(criteria) >= 100, "unexpected loss of criterion coverage")
    require(set(re.findall(r"^\| (UD-\d) \|", req, re.M)) == {f"UD-{i}" for i in range(1, 6)}, "five unresolved decisions")
    threats = [line for line in req.splitlines() if re.search(r"\| T-0[1-6]:", line)]
    require(len(threats) == 6 and all(len(line.split("|")) == 9 for line in threats), "six complete threat rows")
    for gate in ("AC-05", "AC-06", "AC-08"):
        require(re.search(rf"^\| {gate} .*MISSING \|$", req, re.M), f"{gate}: missing evidence status")
    require("UNVERIFIED" in texts["analysis"] and "conditional" in texts["analysis"].lower(), "honest analysis status")
    req_ids = {f"REQ-{n:03}" for n in range(1, 11)}
    des_ids = set(re.findall(r"\*\*(DES-\d{3})\*\*", design))
    require(des_ids, "design IDs")
    for text, label in ((design, "design"), (tasks, "tasks")):
        require(set(re.findall(r"REQ-\d{3}", text)) == req_ids, f"{label}: exact requirement coverage")
    for criterion in criteria:
        require(re.search(rf"(?<![\d.]){re.escape(criterion)}(?![\d.])", design), f"design criterion coverage {criterion}")
    parents = re.findall(r"^- \[ \] (\d+)\. ", tasks, re.M)
    leaves = re.findall(r"^  - \[ \] (\d+\.\d+) .*?(TASK-\d{3})[^\n]*\n(.*?)(?=^  - \[ |^- \[ |^## |\Z)", tasks, re.M | re.S)
    require(parents and leaves, "task syntax")
    require(len({id_ for _, id_, _ in leaves}) == len(leaves), "unique task IDs")
    require(not re.search(r"\[[xX~\-]\]", tasks), "no implementation tasks marked complete")
    covered = set()
    for number, _, block in leaves:
        for field in ("_Requirements:", "Design:", "Files:", "Done criteria:", "Estimate:", "Dependencies:"):
            require(field in block, f"task {number}: {field}")
        covered.update(re.findall(r"DES-\d{3}", block))
    require(covered == des_ids, "task coverage of exact design elements")
    graph_text = tasks.split("## Task Dependency Graph\n", 1)[1]
    graph = json.loads(re.search(r"```json\n(.*?)\n```", graph_text, re.S)[1])
    waves = graph["waves"]
    require([w["id"] for w in waves] == list(range(len(waves))), "sequential waves")
    require(len(waves) <= 3, "maximum three scheduling waves")
    scheduled = [p for w in waves for p in w["tasks"]]
    require(len(scheduled) == len(set(scheduled)) and set(scheduled) == set(parents), "every parent scheduled once")
    wave_of = {p: w["id"] for w in waves for p in w["tasks"]}
    leaf_ids = {n for n, _, _ in leaves}
    dependencies = {}
    owned = {}
    for number, _, block in leaves:
        declaration = re.search(r"Dependencies: ([^\n;]+)", block)[1]
        dependencies[number] = set(re.findall(r"\b\d+\.\d+\b", declaration))
        require(dependencies[number] <= leaf_ids, f"task {number}: unknown dependency")
        files = re.search(r"Files: ([^\n]+)", block)[1]
        owned[number] = set(re.findall(r"`([^`]+)`", files))
        require(owned[number], f"task {number}: empty ownership")
    graph_edges = set(re.findall(r"^(\d+\.\d+) → (\d+\.\d+)$", graph_text, re.M))
    declared_edges = {(dep, n) for n, deps in dependencies.items() for dep in deps}
    require(graph_edges == declared_edges, "graph edges match leaf prerequisites exactly")
    depths = {}

    def depth(number, active):
        require(number not in active, f"dependency cycle through {number}")
        if number not in depths:
            depths[number] = 1 + max((depth(dep, active | {number}) for dep in dependencies[number]), default=0)
        return depths[number]

    for n in leaf_ids:
        depth(n, set())
        for dep in dependencies[n]:
            require(wave_of[dep.split('.')[0]] <= wave_of[n.split('.')[0]], f"backward wave dependency {dep} -> {n}")
    for i, (left, _, _) in enumerate(leaves):
        for right, _, _ in leaves[i + 1:]:
            lp, rp = left.split('.')[0], right.split('.')[0]
            if lp != rp and wave_of[lp] == wave_of[rp]:
                require(not owned[left] & owned[right], f"parallel path collision: {left}, {right}")
    baseline = json.loads((RUN / "baseline.json").read_text())
    for path, digest in baseline["inputs"].items():
        require(hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, f"preserve {path}")
    changed = subprocess.check_output(["git", "diff", "--name-only", baseline["head"]], cwd=ROOT, text=True).splitlines()
    require(all(p.startswith((".kiro/specs/execution-isolation/", ".workflow/ultracode/execution-isolation/")) for p in changed), "no unrelated tracked changes")
    for path in [*SPEC.glob("*.md"), *RUN.rglob("*.md"), *RUN.glob("*.py")]:
        require(not any(line.rstrip() != line for line in path.read_text().splitlines()), f"trailing whitespace in {path.name}")
    print(f"PASS: 10 requirement groups, {len(criteria)} atomic criteria, 6 threats, 5 open decisions, {len(des_ids)} design elements, {len(leaves)} tasks, {len(waves)} parent waves; actual leaf depth {max(depths.values())} disclosed; baseline preserved")


if __name__ == "__main__":
    main()
