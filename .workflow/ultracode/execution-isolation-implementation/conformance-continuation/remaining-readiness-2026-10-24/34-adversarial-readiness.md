# Independent packet34 — actual remaining contract blockers

Read-only/source readiness, no actionable implementation repair performed.
Fact: no collision-specific rejection contract exists. Task20line498/Req10.7
cross-author rejection wording conflicts with Req10.3nonblocking reserved-prefix
warnings and convention42–48registry-governance ownership. extensions.rs23–46
scans one root; mandatory_extensions.rs33–43independently blocks each required
unknown namespace. Two A/B claims with required_extensions:[acme] prove absence
of trusted implemented semantics, NOT conflict detection or ownership auth.

Recommended owner clarification: independently validate two distinct owner claims
for the same acme prefix; preserve optional fields and verbatim reserved warnings;
required acme gives semantic blocking-gap at required_extensions[0] in each.
Explicitly label this non-recognition, not cross-document conflict detection;
actual prefix conflicts remain registry governance. If actual collision rejection
is required instead, trusted inputs/API/assignment/diagnostic contracts are missing.
Owner must choose before claiming full collision-class coverage.

Second actual gap: round_trip checks extension bytes and StructuralResult equality,
not protected core tests[*].outcome (fixtures.rs399–423, expected fields103–122).
A structurally valid core fail→pass rewrite could evade that assertion. Recommend
minimal test-harness-only before/after core verification projection comparison,
retaining exact raw extension bytes, with regression/TDD. Do not silently change
runner under data-only approval. Existing extension_validation157–167 and
extension_policy98–130tests support but don't substitute that fixture assertion.

Inventory26adversarial drafts: reserved variants6,collision3,invalid formats4,
optional/mandatory4,core override2,P0authority7.24code-approval/2owner-oracle gates.
All remain drafts awaiting exact oracles/approval. Valid approve/deploy/delete
requests, forged origin/approver strings or self-consistent replay may pass Spec1
local consistency; real rejection needs protected expected context/policy/identity
and Spec2enforcement. Direct x.capabilities shadow locally fails semantic at
x.capabilities; incomplete binding structurally fails. No forged Native proof.

Actual checks: rule/spec/source inspection, line extraction and JSON counting.
No tests/builds/edits/Git/network/provider/native/credentials or recursive agents.
Two guessed paths were corrected to actual catalog/schemas and tests/support.
