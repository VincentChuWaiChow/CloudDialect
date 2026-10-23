# Packet13: custom-script ordering review
## Objective
Read-only final review of seven script relocations and compatibility paths.
## Context
User clarified Rust and custom scripts only; parent created scripts/verification/<domain>, retained relative oldpath symlinks, and kept all recorded run data in .workflow. New scope plus delegate-default admits one additional bounded read-only sidecar (13 cumulative). Existing DDD60minute deadline retained, no followups/recursive agents; no remaining automatic repair budget.
## Sources
results/pre-script-source-snapshot.json, results/script-moves.json, seven canonical scripts, scripts/README.md, tests/test_script_layout.py, .ai/rules/engineering.md.
## Ownership
None, behaviorally read-only.
## Do
Compare source to snapshot, ensure only RUN/RESULTS data path adaptation and shellcomment changed; no checker assertion relaxation. ROOT.parents3 remains correct, imports/shellCWD coherent, legacy links relative and correct. Test meaningfulness/README caveats.
## Do not
No edits/sourceexecution/network/secrets/probes/commits/push/subagents/followups. All shellcommands prefixrtk. Do not claim historical fullgraders pass; preexisting scope/draft failures preserved.
## Expected output
PASS/FAIL, required findings withpaths/lines, verificationlimits <=8minutes.
## Verification
Static source/symlink comparison only; parent runs commands and remoteproof.
## Handoff format
Summary; findings; limitations.
