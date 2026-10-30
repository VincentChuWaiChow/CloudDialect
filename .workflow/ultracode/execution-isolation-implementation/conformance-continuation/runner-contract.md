# Runner contract frozen before implementation

Scope: test-only harness, no executable fixture code, dependencies or runtime.
Expected metadata requires phase and document_kind; only structural(pass/warn/
fail), semantic(pass/fail), behavioral(deferred/evidence-complete/blocked) and
artifact-hash(expected_match boolean,no outcome) accepted. JSON kind explicit;
manifest selects TOML, behavioral requires behavioral-contract, artifact requires
verification-result. candidate_id required for behavioral; candidate_bytes byte
array for hash fixtures. All diagnostics/warnings/blockers matched as exact-size
one-to-one multisets; category/verbatim keys exact, optional field_path exact
including explicit null, descriptions/assumptions substring. Unknown metadata
fields, types, unsupported phase/outcome combinations fail, never ignored.

Synthetic store metadata: store array of entries with lookup_id, lookup_version,
reference, content_hash, bytes byte-array; optional store_error boolean. Exact
version retrieval or versionless unique evidence lookup; duplicates/failed store
return safe failures. No ambient files/network/project backend or actual native
scope claim. Existing production resolver validates returned bytes/claims.

Optional round_trip flag applies only to structurally valid JSON typed document
families; verify exact encoded qualified extension values after typed roundtrip,
not Value equality. No normalization of accepting outcomes or verification fields.

Discover directory pairs(input.json or input.toml,expected.json); reject missing
pair, dual input, symlinks and unsafe/absolute/traversal selectors. Component-wise
prefix matching, deterministic order, no discovered/selected fixture is failure.
Default root=crate/fixtures. Test-only --fixture-root accepts explicit temporary
control roots; it is not a public production CLI or native security boundary.

Each fixture runs in a fresh copy of the trusted test binary, not a shell or
fixture-selected command. Parent enforces fixed5second timeout, kills and reaps
child on expiry. Trusted worker stderr is bounded; no wait-on-full-pipe hang.
Controlled sleeping test child validates timeout/reaping, not untrusted execution
isolation. Whole default Cargo suite needs at least one genuine fixture; explicit
seed batch3.2.1 requested, remaining corpus stays gated and unclaimed.
