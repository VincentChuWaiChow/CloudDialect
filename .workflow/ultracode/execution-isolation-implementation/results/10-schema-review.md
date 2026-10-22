# Result10: schema review

## Summary
FAIL: required wrapper_chain minItems1 absent (schema47-53 vs requirement2.1 lines132-134). Evidence unresolved_assumptions only defined for unknown; root unevaluatedProperties false rejects it on observed/inferred although requirement3.1 permits any unresolved assumptions. Inherited design conflict, not new divergent interpretation.
## Evidence
Reviewer /root/schema_review inspected five schemas, four focused test files and normative spec. No tests independently executed. FullSemVer/whitespace/lowercaseUUIDv4, obligation exclusive support, verification failure-object/Unicode constraints, IDs/kinds appear correct.
## Handoff
Add wrapper negative and observed/inferred assumptions positives plus invalid assumption negative. Move common optional assumptions definition to root properties while preserving unknown nonempty requirement. Parent requested explicit one-additional-repair approval, existing repair allowance consumed.
## Files changed
None by reviewer.
## Decisions
No gate closure or broad conformance approval.
## Risks
Resource behavior requirement2.2 completeness is not established by skeleton: dependencies may omit both known behavior and assumptions. No precise dimension binding in assumption model; contract reconciliation required before claiming completeness.
## Verification run
Read-only source inspection, no command-based tests.
## Open questions
Bounded repair approval pending. Resource behavior model contract remains unresolved.
