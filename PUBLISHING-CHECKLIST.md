# Publishing and enforcement readiness checklist

**Do not publish the proposed LICENSE.md as an operative legal instrument before the items below are completed.**

## Rights and legal clearance

- [ ] Confirm the exact copyright owner (individual versus company) and authority to license.
- [ ] Have qualified intellectual-property counsel review LICENSE.md, the commercial agreement, the CLA and jurisdiction-specific enforceability.
- [ ] Resolve all placeholders and ensure the project, license and marketing documentation agree.
- [ ] Identify pre-existing OSS commitments or versions: previously granted permissions do not automatically disappear.
- [ ] Review third-party dependencies and contribution history for copyright, patent and compatibility concerns.
- [ ] Agree on whether internal production business use, public-benefit/nonprofit funded products, sponsorship and paid consulting are commercial.
- [ ] Confirm no unintended coverage of tools merely used by a developer to produce an unrelated output.
- [ ] Ensure commercial attribution is in the SIGNED agreement, not merely in the README.
- [ ] Implement contributor agreement collection and pre-merge enforcement.
- [ ] Check export controls, consumer rules, fair-use/statutory exceptions and governing law.

## Commercial operations

- [ ] Define a standard commercial fee menu, if any, and exceptions/waiver authority.
- [ ] Select billing, contract storage, renewal, reporting and evidence controls.
- [ ] Define gross/net revenue for any revenue-sharing arrangement, audits and confidentiality.
- [ ] Plan how companies request licenses before production, without unexpected license traps.
- [ ] Validate license terms against realistic enterprise procurement and counsel questions.

## Repository operations

- [ ] Keep LICENSE.md at the repository root and make its draft label part of counsel review.
- [ ] Publish NOTICE, ATTRIBUTION.md and COMMERCIAL-LICENSING.md consistently.
- [ ] Do not attach an MIT, Apache, AGPL or OSI-open-source badge to the restricted framework.
- [ ] Document separate licenses for examples, documentation, third-party components where applicable.
- [ ] Tag licensed versions; keep a record of commit hash, license revision and prior releases.
- [ ] Audit package metadata, container labels, generated documentation and copy-pasted file headers for conflicting license declarations.
- [ ] Review with a real external contributor and a commercial purchasing customer before launch.

## Scenario tests (answer consistently from every file)

1. A student runs the project for a school assignment without revenue: community path.
2. A paid SaaS uses the framework for a new 'free' tier: commercial path.
3. A company pilots the framework in a sandbox before deciding whether to buy: community evaluation, isolated from production.
4. A developer plans to sell an app but has made $0 so far: commercial path **before** commercial launch/use.
5. A third party removes credit and redistributes a noncommercial fork: prohibited notice removal.
6. A for-profit business runs a genuinely unrelated charity/community hobby project: potentially community path depending on actual activity.
7. A contributor submits a PR with unclear employer ownership: do not merge.
8. A reseller makes an authorized payment but has no executed contract: no commercial rights yet.