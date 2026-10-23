# Packet12: final bounded domain/rules/inventory review
## Objective
Read-only verify behavior-preserving domain move, rule symlinks and newcataloginventory safety.
## Context
User explicitly requested domain organization/sharedrules/gitcommit+push; no nativeprobes/releases. Read canonical AGENTS.md. Parent owns integration, code freeze for review; docs/evidence may update.
## Sources
results/pre-domain-source-snapshot.json, currentcrate source, tests/public_api_compatibility.rs, test_ai_rules.py/test_domain_layout.py, CatalogIndex tests, canonicalrule/cliententrypoints and relevant requirements.
## Ownership
No writes.
## Do
Compare moved publictype bodies/serde/macro behavior to snapshot; ensure alloldpublicfacades same definitions, noauth/acceptance added. Review catalog bootstrap doesn't trustinputURI, exactallfive/uniqueURIpath/missingmaterial/escapechecks. Verify relative clientsymlink availability and nofalseactivationclaim. Flag required defects only withpathlines/evidence.
## Do not
No edits/network/secrets/subagents/followups/commits/push/probes or source execution. Shellcommands prefixrtk. No demand fullfuturevalidators to approve this bounded scope.
## Expected output
PASS/FAIL, requiredfindings, limitations <=10minutes.
## Verification
Read-only inspection; parent runs fullchecks. Nativeclientactivation unverified.
## Handoff format
Summary; findings; paths; verificationlimits.
