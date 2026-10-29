# Release integrity

Procedures for Req 12.1 (SBOM), Req 12.2 (offline artifact verification), and
Req 12.3 (license screening). See the specification's
[Release Integrity section and DDR-07](../.kiro/specs/contracts-evidence-foundation/design.md#release-integrity).
These files configure and document release preparation; they are not evidence
of a qualifying release or completion of the requirements' artifact checks.

Run the repository-specific commands below from the repository root. There is
no root Cargo workspace: the manifest is
`crates/clouddialect-contracts/Cargo.toml`. Commands use the repository's `rtk`
wrapper. Maintainers must provision compatible `cargo-deny`, `cargo-cyclonedx`
and `minisign` tools separately; this procedure does not install tools or keys.

## License screening (Req 12.3)

The pre-release check is `cargo deny check licenses`. For this layout, explicitly
select both the standalone manifest and the root policy:

```sh
rtk proxy cargo deny --manifest-path crates/clouddialect-contracts/Cargo.toml \
  --config deny.toml check licenses
```

[`deny.toml`](../deny.toml) uses cargo-deny's supported **allow-only** policy:
the seven allowed expressions are unchanged and all other licenses are denied by default.
This includes `GPL-2.0`, `GPL-3.0`, `AGPL-1.0`, `AGPL-3.0`, `SSPL-1.0`,
`LGPL-2.0`, `LGPL-2.1` and `LGPL-3.0`, as well as unlisted copyleft licenses.
A denied license
that cannot be satisfied through an allowed alternative must cause a non-zero
exit with a **named incompatibility report** identifying the dependency name, version, and license.
Keep those diagnostics with the release review. Do not release until this check
exits zero; a configuration error, missing license or missing tool is not a pass.
An allowed SPDX alternative does not grant rights under a denied alternative.
Do not bypass diagnostics by changing dependencies, adding policy exceptions or
mislabeling this repository's custom license as MIT/Apache.

Use cargo-deny **0.16.0** or later: that release removed `deny` and `copyleft` in
favor of the allow-only policy. The owner approved this policy/specification
migration on 2026-10-28. Do not restore those removed fields or treat TOML
parsing as a successful license scan. Crates without confidently determined
licenses also fail screening; resolve genuine findings through owner review,
not implicit exceptions. See the upstream
[removal notice](https://github.com/EmbarkStudios/cargo-deny/blob/main/CHANGELOG.md#0160---2024-08-02).

The policy was exercised with cargo-deny 0.20.2. The current locked graph fails
screening on `MIT-0`, `Unicode-3.0` and the scanner's inability to identify this
crate's custom Community License. The license text exists; that last diagnostic
is not a finding that the repository has no license. See the
[named findings](../.workflow/ultracode/release-tooling/results/license-incompatibilities.json).
Any allow-list addition or root-license clarification requires separate owner
approval; syntax migration alone does not approve additional licenses.
On 2026-10-29 the owner chose to retain the seven-license policy and leave
release blocked pending license review. No additional allowances, exceptions
or custom-license clarification are authorized by that decision.

## SBOM generation (Req 12.1)

The design specifies the following command (reference only, not a runnable
command for the documented upstream CLI or this repository layout):

```text
cargo cyclonedx --format json --output-file release/sbom.cdx.json
```

The documented cargo-cyclonedx CLI does not support `--output-file`. Its
`--override-filename` takes a filename prefix, not a directory path. For the
standalone crate, use this alternative and copy the generated file into release/:

```sh
rtk proxy cargo cyclonedx --manifest-path crates/clouddialect-contracts/Cargo.toml \
  --format json --all --target all --override-filename sbom.cdx
rtk proxy cp crates/clouddialect-contracts/sbom.cdx.json release/sbom.cdx.json
```

See upstream [CLI source (cargo-cyclonedx 0.5.7)](https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/cargo-cyclonedx-0.5.7/cargo-cyclonedx/src/cli.rs)
and [usage documentation](https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/main/cargo-cyclonedx/README.md).
Check the provisioned tool's help and actual output before adopting the command;
SBOM generation has not been executed as part of this configuration task.

The intended output is a machine-readable **CycloneDX JSON** inventory derived
from Cargo metadata and the committed `Cargo.lock`, listing direct and transitive
dependencies, their versions and declared licenses. `--all` includes transitive
dependencies; `--target all` includes target-specific dependencies. Development
dependencies are not part of cargo-cyclonedx's shipped-dependency inventory.
Review the inventory against the actual release feature/target profile, confirm
licenses are populated, and validate it with a conforming CycloneDX tool.
Include `release/sbom.cdx.json` in the archive, not only as a separate download.
Missing or unparseable inventory blocks release; do not invent license metadata.

## Artifact signing and offline verification (Req 12.2 / DDR-07)

Replace `<version>` with the approved release version. After building the entire
archive, the maintainer signs it using an externally held private key:

```sh
rtk proxy minisign -S -s /path/to/private.key -m release/clouddialect-contracts-<version>.tar.gz
```

This creates `release/clouddialect-contracts-<version>.tar.gz.minisig`. Publish
that detached signature alongside the unchanged archive. The private key must never be committed
or included in an archive, SBOM, log or command output.

A consumer with the archive, its adjacent `.minisig` file, a trusted copy of the
published public key, and minisign verifies it with:

```sh
rtk proxy minisign -V -p release/minisign.pub -m release/clouddialect-contracts-<version>.tar.gz
```

Verification requires **no network access** and no third-party signing service.
Establish trust in the public key separately; a key downloaded with an untrusted
archive is not itself a trust anchor. Verify **before extraction** and stop on
any non-zero exit. Archive signing does not protect files modified after extraction.
See [minisign usage](https://jedisct1.github.io/minisign/).

**First-release blocker:** [`minisign.pub`](minisign.pub) is comment-only. This
placeholder is not a usable public key and cannot verify signatures. The
maintainer commits the actual public key when the first release is cut, keeps
the matching private key outside the repository, and must demonstrate successful
offline verification before qualifying a release. Do not generate or substitute
a test key and claim release provenance.

## Pre-release gate sequence

Run these gates in order; stop at the first failure or unresolved prerequisite:

1. Run `cargo deny check licenses` using the manifest/config command above with supported tooling; require exit zero with reviewed diagnostics.
2. Run `cargo test --test conformance`: `rtk proxy cargo test --manifest-path crates/clouddialect-contracts/Cargo.toml --locked --test conformance`; require all selected canonical fixtures to pass.
3. Generate the SBOM as described above, inspect dependency/license coverage and validate its CycloneDX JSON format.
4. Build the versioned release archive containing `catalog/`, the SBOM, the approved validator binary and required license/notices files. This crate currently provides a library, not a validator executable; binary build/packaging is outside task 26 and must be supplied and reviewed before this gate can pass.
5. Sign the final archive, then verify it offline with the committed real public key before publishing the archive and adjacent signature under separate release authorization.

Do not change the archive after signing. Retain gate results and named
incompatibility reports with the release review. No archive, SBOM, signing key,
signature, CI automation or publication is created by this documentation task.
