# Third-party notices and license inventory

## Contracts crate dependencies

The `clouddialect-contracts` scaffold declares the following dependencies from
[crates.io](https://crates.io/). Versions and license expressions were checked
against registry metadata on October 19, 2026. No third-party source is vendored.

| Dependency and source | Pinned version | Declared license |
| --- | --- | --- |
| [boon](https://crates.io/crates/boon/0.6.1) | 0.6.1 | MIT OR Apache-2.0 |
| [include_dir](https://crates.io/crates/include_dir/0.7.4) | 0.7.4 | MIT |
| [regex](https://crates.io/crates/regex/1.13.1) | 1.13.1 | MIT OR Apache-2.0 |
| [serde](https://crates.io/crates/serde/1.0.229) | 1.0.229 | MIT OR Apache-2.0 |
| [serde_json](https://crates.io/crates/serde_json/1.0.154) | 1.0.154 | MIT OR Apache-2.0 |
| [toml](https://crates.io/crates/toml/0.9.9) | 0.9.9+spec-1.0.0 | MIT OR Apache-2.0 |
| [uuid](https://crates.io/crates/uuid/1.28.0) | 1.28.0 | Apache-2.0 OR MIT |

### Approved prerequisite additions (October 22, 2026)

The owner approved these exact direct pins and a Rust 1.88 minimum. SHA-2/SHA-3
use no default features; SemVer uses its default `std` feature. The existing
`serde_json =1.0.154` pin now enables `raw_value` for exact encoded extension
value preservation; its version and license are unchanged.

| Direct dependency and source | Pinned version | Declared license |
| --- | --- | --- |
| [sha2](https://crates.io/crates/sha2/0.11.1) | 0.11.1 | MIT OR Apache-2.0 |
| [sha3](https://crates.io/crates/sha3/0.12.0) | 0.12.0 | MIT OR Apache-2.0 |
| [semver](https://crates.io/crates/semver/1.0.28) | 1.0.28 | MIT OR Apache-2.0 |

New locked transitive packages: `block-buffer 0.12.1`, `cpufeatures 0.3.1`,
`crypto-common 0.2.2`, `digest 0.11.3`, `hybrid-array 0.4.15`, `keccak 0.2.2`,
`sponge-cursor 0.1.0` and `typenum 1.20.1`. Their downloaded manifests declare
MIT OR Apache-2.0 (`keccak` lists Apache-2.0 OR MIT); the MIT option is selected
for these additions. License files were present and hashed. All 100 registry
archives in the resulting lockfile matched their recorded SHA-256 checksums;
all 89 pre-existing registry identities/checksums were preserved. Detailed
metadata/license-file evidence is in
[approved-dependency-admission.json](.workflow/ultracode/execution-isolation-implementation/results/approved-dependency-admission.json).
This is source/license inventory, not legal clearance, vulnerability screening,
cryptographic certification or release-integrity proof.

`toml` is pinned to the last release explicitly tagged for TOML 1.0 rather than
the latest TOML 1.1 release. Cargo ignores build metadata in version requirements;
the committed `crates/clouddialect-contracts/Cargo.lock` records the full version
and checksums of the resolved graph, including transitive dependencies.

The task's shorthand MIT/Apache label is not accurate for `include_dir`, which
declares MIT only. These dependency licenses do not replace CloudDialect's
Community License. This inventory is **not** legal clearance or a release license
audit. Distribution must retain applicable third-party license/notice texts;
complete release screening, SBOM generation and redistribution notices remain
part of the later release-integrity task.

Before adding a dependency, example, generated asset or vendored file, maintainers must verify its origin, applicable license, compatibility with the CloudDialect dual-licensing model, and any notices, source obligations or attribution duties. Record those details here.

Third-party material remains governed by its own license. CloudDialect's Community License does not relicense or extinguish rights held by others.
