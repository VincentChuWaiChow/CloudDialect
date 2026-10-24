# Current inventory — 2026-10-24

42 cases/26 batches are delivered; remaining62 cases/34 batches remain incomplete.
All41prepared drafts were explicitly approved/admitted; no pending admission
question remains for them. See `admission-2026-10-24/final-report.md`. Prior draft
readiness notes below are historical; they do not waive remaining gates.

# Inventory status — 2026-10-24

Exactly seed batch 3.2.1 is now approved and delivered (`408af0d`), with unchanged
reviewed bytes and actual default/positive count 1. The remaining 103 cases in
59 batches are still draft/unapproved with the oracle/environment gates below.
No single-seed or green default-suite claim completes the full required corpus.

## Historical inventory/readiness draft follows

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
