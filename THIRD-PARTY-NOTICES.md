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
