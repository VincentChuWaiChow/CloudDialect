# Catalog integration repair — before changes

Packet16 found that pinned Boon resolves built-in meta-schema URIs before invoking UrlLoader. An indexed support mapping for one of those names can therefore return different bytes through the adapter than compilation uses. Required P0-D exact mapping is not satisfied by merely documenting that mismatch.

One consolidated repair within the existing 60-minute catalog tranche. Parent owns `src/schema/catalog.rs`, `src/schema/loader.rs` and `tests/catalog_loading.rs`; no other production files, dependencies or format changes. Add a real-engine regression showing indexed `false` versus compiled built-in behavior, checkpoint RED, then reject support identifiers that the pinned compiler cannot resolve as that exact external resource through its installed callback. This includes its built-in names and HTTP/HTTPS aliases, normalized aliases and fragment-bearing resource identifiers. Ordinary admitted support URIs continue to work; unknown same-host URIs are not blanket rejected.

Use an in-memory deny callback to characterize the actual pinned compiler's resource dispatch, not a second manually maintained URI table or hand-rolled URL normalization. The callback never acquires external material; no supplied schema is compiled by the inventory check. Failure to reach the callback with the exact requested resource URI rejects admission. This is engine-profile inventory enforcement, not provenance or native network qualification.

Cover all 19 pinned meta-resource paths plus the `schema` alias under HTTP and HTTPS; normalization aliases; ordinary same-host support; original custom URNs. Source-local loader tests also meet canonical task8's requested unit-test placement, without replacing integration tests. Tests-only additions require no fabricated RED.

Same focused target GREEN, full frozen Rust/check/fmt/Clippy/Python, actual doctest error classes, fresh emitted-production coverage >=80%, source scope/preservation audit. No second review dispatch/followup, recursive work or second repair. Static review findings are parent-repaired, not independently post-repair signed off. Runtime/BG-01 remain open.
