# Result 01: isolation and credentials

## Summary
Native sidecar `/root/isolation_credentials` completed one read-only research pass. UD-1/UD-3 remain OPEN; no writes or probes.
## Evidence
Relevant spec clauses plus official Firecracker production guidance, kernel cgroup v2, Apple Containerization source, AWS effective-policy/revocation documentation. Parent corroborated central Firecracker/kernel/AWS pages. Parent performed Context7 Rust resolve/query. URLs retained in decision records.
## Handoff
- Summary: evaluate Linux/KVM microVM and narrow operation broker; proposals only.
- Changed surfaces: none by agent; parent authored UD-1/3 and qualification plans.
- Contracts satisfied: alternatives, portability, startup/descendant and observer trust, credential exposure/lifecycle risks.
- Assumptions: no approved native environment or credential provider; all outcomes MISSING.
- Local checks: read-only source/docs review.
- Integration evidence: external oracle required; authenticated runner origin is not behavioral correctness; session policy alone is not complete effective-authority proof.
- Risks: image/VMM/host maintenance, authorized-operation abuse, in-flight effects, provider propagation, unsupported direct SDK workflows.
## Files changed
None by agent.
## Decisions
Parent accepts recommendations as review candidates, not architecture approval. No worker-visible raw secrets is a proposal, not a demonstrated guarantee.
## Risks
No native isolation, cleanup, leakage or provider qualification.
## Verification run
Official-document and spec inspection only; actual agent model/effort UNVERIFIED, inherited settings requested.
## Open questions
Owner/profile/observer/environment selection and residual-risk acceptance.
