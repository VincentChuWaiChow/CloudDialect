# Packet16 — frozen catalog static review

Native sidecar `/root/catalog_integration_review`, read-only, completed within the bounded tranche. No writes/tests/source execution/network/probes/commits or recursive/follow-up dispatches. Inherited model/effort and separately enforced isolation are unverified.

Required finding: P2, Boon built-ins bypass admitted support bytes. `catalog.rs` accepted a mapping for a built-in meta-schema URI, `CatalogUrlLoader` would return its supplied `false`, but pinned `boon/src/loader.rs:121–133,232–241` loads the engine-owned built-in first. P0-D exact authority is not met by ignoring the supplied mapping. Recommendation: real-engine RED plus rejection or actual precedence enforcement; cover exact built-in names and HTTP/HTTPS aliases, not a broad host-prefix rule.

Otherwise static PASS: digest encoding, immutable owned boundary, recursive embedding, static-helper consistency, deny-unknown adapter, explicit format controls and safety limitations.

Parent repair: one consolidated pass under [repair eval](../catalog-repair-eval.md). Runtime RED `551e8cd` demonstrates the substitution, GREEN `cc65d4a` rejects identifiers that the actual pinned engine will not dispatch as that exact external resource. All 40 built-in aliases, normalized/fragment variants and ordinary same-host/custom support controls pass. Required source-local unit tests added. Clippy initially flagged test `.err().expect()`; replaced with `.map(|_| ()).expect_err()` without dumping schema bytes or suppressing the lint. Initial failure and final PASS evidence are retained. No independent post-repair reviewer signoff is claimed.
