//! Internal document-format compatibility, not pack ranges or schema dialects.

use std::sync::OnceLock;

use regex::Regex;
use semver::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VersionCheckResult {
    Supported(usize),
    MinorNewer(usize),
    UnsupportedMajor,
    UnsupportedVersion,
    InvalidFormat,
}

/// Indices refer to the trusted caller's admitted document-version list.
pub(super) fn check_version_compatibility(declared: &str, admitted: &[&str]) -> VersionCheckResult {
    use VersionCheckResult::*;

    let Ok(version) = Version::parse(declared) else {
        // SemVer core integers are unbounded in the grammar, but the approved
        // implementation profile supports only u64. Do not mislabel overflow
        // as malformed, or loosen suffix grammar merely to classify it.
        static GRAMMAR: OnceLock<Regex> = OnceLock::new();
        let grammar = GRAMMAR.get_or_init(|| {
            Regex::new(r"\A(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*)?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?\z")
                .expect("fixed SemVer grammar")
        });
        return if grammar.is_match(declared) {
            UnsupportedVersion
        } else {
            InvalidFormat
        };
    };
    if !version.pre.is_empty() {
        return UnsupportedVersion;
    }
    let stable: Vec<_> = admitted
        .iter()
        .enumerate()
        .filter_map(|(index, text)| Version::parse(text).ok().map(|v| (index, v)))
        .filter(|(_, v)| v.pre.is_empty())
        .collect();
    if let Some((index, _)) = stable
        .iter()
        .find(|(_, v)| version.cmp_precedence(v).is_eq())
    {
        return Supported(*index);
    }
    let Some(highest_minor) = stable
        .iter()
        .filter(|(_, v)| v.major == version.major)
        .map(|(_, v)| v.minor)
        .max()
    else {
        return UnsupportedMajor;
    };
    if version.minor > highest_minor
        && let Some((index, _)) = stable
            .iter()
            .find(|(_, v)| v.major == version.major && v.minor == 0 && v.patch == 0)
    {
        return MinorNewer(*index);
    }
    UnsupportedVersion
}
