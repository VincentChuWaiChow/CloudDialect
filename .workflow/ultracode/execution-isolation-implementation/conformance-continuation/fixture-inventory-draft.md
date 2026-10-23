# Required fixture inventory — draft, not implementation approval

Source: canonical Spec1tasks18–21 and their named P0/VC1 additions. This inventory
expands every baseline fixture family without replacing any with smoke checks.
The JSON companion records exact paths, kind/phase/disposition, per-case oracle
constraints, gate and unique bounded batch ownership. Seed3.2.1 is the only
implementation approval currently requested. No remaining fixture file created.

Counts: 104 case directories in 60 batches, each<=2directories/4files.
13positive,15baseline+8P0negative, all8adversarial classes with both handled/rejected
cases (including3invalid-prefix cases and two collision owner claims),3roundtrip,
5baseline integration, named store/snapshot/digest/verification/version follow-ons.

## Not ready / cannot be guessed
- Exact complete diagnostic multisets must be frozen before dispatch; category
  constraints alone are not a completed oracle. No probe/test result invented.
- Reserved prefixes alone cannot be a structural rejection. Proposed rejection
  uses separately unknown mandatory semantics; the namespace-collision wording
  still needs confirmation rather than fake publisher authentication.
- Network-disabled and same-tenant cross-project cases require actual approved
  environment/backend qualification; empty fake stores or notes do not prove it.
- Snapshot environment immunity needs trusted named-variable setup, not a field
  claiming that an ambient overlay was ignored.
- Prohibited action/forged approver/origin/replayed-binding declarations may be
  locally valid. Spec1does not authenticate them; their Spec2rejection remains a
  separate required gate, never normalized to document rejection or acceptance.

This is a scheduling/readiness draft, not completion of3.2or tasks18–22. No new
technology choice, version/dependency, code fix, native probe or publication.
