// Test the exact private source before its structural consumer is registered.
// Structural integration must also exercise the actual production module.
#[path = "../src/validator/version.rs"]
mod version;

use version::{VersionCheckResult as V, check_version_compatibility as check};

#[test]
fn only_exact_admitted_stable_precedence_is_supported() {
    let admitted = ["1.0.0", "1.2.3+trusted", "2.0.0"];
    for (declared, index) in [
        ("1.0.0", 0),
        ("1.0.0+001", 0),
        ("1.2.3+other", 1),
        ("2.0.0", 2),
    ] {
        assert_eq!(check(declared, &admitted), V::Supported(index));
    }
    for declared in ["1.0.1", "1.1.0", "1.2.0", "1.2.4", "0.9.9"] {
        let expected = if declared.starts_with('0') {
            V::UnsupportedMajor
        } else {
            V::UnsupportedVersion
        };
        assert_eq!(check(declared, &admitted), expected, "{declared}");
    }
}

#[test]
fn newer_stable_minor_uses_only_an_admitted_major_zero_zero_base() {
    let admitted = ["1.2.3", "2.0.0", "1.0.0+base"];
    for declared in ["1.3.0", "1.3.9+build.001", "1.18446744073709551615.0"] {
        assert_eq!(check(declared, &admitted), V::MinorNewer(2), "{declared}");
    }
    assert_eq!(check("2.1.0", &admitted), V::MinorNewer(1));
    assert_eq!(check("1.4.0", &["1.2.0"]), V::UnsupportedVersion);
    assert_eq!(check("1.1.0", &["1.0.0", "1.2.0"]), V::UnsupportedVersion);
    assert_eq!(check("1.1.0", &["1.0.1"]), V::UnsupportedVersion);
}

#[test]
fn prereleases_remain_representable_but_are_not_supported() {
    for declared in [
        "1.0.0-alpha",
        "1.0.0-0",
        "1.9.0-rc.1+build",
        "2.0.0-preview",
        "1.0.0-9999999999999999999999999999999999999999999",
        "1.0.0-01a",
    ] {
        assert_eq!(
            check(declared, &["1.0.0", declared]),
            V::UnsupportedVersion,
            "{declared}"
        );
    }
    let long_build = format!("1.0.0+{}", "9".repeat(1000));
    assert_eq!(check(&long_build, &["1.0.0"]), V::Supported(0));
}

#[test]
fn full_semver_syntax_errors_are_not_overflow_or_compatibility() {
    for declared in [
        "",
        "1",
        "1.0",
        "v1.0.0",
        "01.0.0",
        "1.01.0",
        "1.0.01",
        "1.0.0-01",
        "1.0.0-",
        "1.0.0-a..b",
        "1.0.0+",
        "1.0.0+a..b",
        "1.0.0+a_b",
        "1.0.0-α",
        "1.0.0\n",
        " 1.0.0",
        "1.0.0 ",
        "1.0.0+ok\n",
        "018446744073709551616.0.0",
    ] {
        assert_eq!(
            check(declared, &["1.0.0"]),
            V::InvalidFormat,
            "{declared:?}"
        );
    }
}

#[test]
fn valid_oversized_core_numbers_are_explicitly_unsupported() {
    for declared in [
        "18446744073709551616.0.0",
        "1.18446744073709551616.0",
        "1.0.18446744073709551616",
        "1.0.18446744073709551616-alpha+build",
    ] {
        assert_eq!(
            check(declared, &["1.0.0"]),
            V::UnsupportedVersion,
            "{declared}"
        );
    }
    assert_eq!(
        check("1.0.18446744073709551615", &["1.0.18446744073709551615"]),
        V::Supported(0)
    );
}

#[test]
fn unsupported_major_and_missing_profiles_never_get_a_fallback() {
    assert_eq!(check("2.0.0", &["1.0.0"]), V::UnsupportedMajor);
    assert_eq!(check("0.1.0", &["1.0.0"]), V::UnsupportedMajor);
    assert_eq!(check("1.0.0", &[]), V::UnsupportedMajor);
    assert_eq!(
        check("1.0.0", &["garbage", "1.0.0-alpha"]),
        V::UnsupportedMajor
    );
}

#[test]
fn the_helper_does_not_parse_dialects_or_pack_ranges_as_document_versions() {
    for other_space in [
        "https://json-schema.org/draft/2020-12/schema",
        ">=1.0.0, <2.0.0",
        "^1.0",
        "2020-12",
    ] {
        assert_eq!(check(other_space, &["1.0.0"]), V::InvalidFormat);
    }
    // Pack versions are not inputs to this helper; version spaces stay separate.
}
