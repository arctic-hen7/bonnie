// This file defines the current version of Bonnie
// This MUST be updated before all releases!

pub const BONNIE_VERSION: &str = "0.2.1";

// The different between two major/minor/patch versions
#[derive(Debug, PartialEq, Eq)]
pub enum VersionDifference {
    TooOld,
    TooNew,
}

// The compatibility of two versions with one another
#[derive(Debug, PartialEq, Eq)]
pub enum VersionCompatibility {
    Identical,
    DifferentMajor(VersionDifference), // Only this means the versions are incompatible
    DifferentMinor(VersionDifference),
    DifferentPatch(VersionDifference),
    DifferentBetaVersion(VersionDifference), // In beta, this also means the versions are incompatible
}

#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    patch: u16,
    minor: u16,
    major: u16,
}
impl Version {
    // Compares this with another version returns their compatibility
    // It will return an embedded version difference as to whether the version being compared to is too old/new or nothing if they're identical
    pub fn is_compatible_with(&self, comparison: &Version) -> VersionCompatibility {
        let compatibility = match self.major {
            _ if self.major > comparison.major => {
                VersionCompatibility::DifferentMajor(VersionDifference::TooOld)
            }
            _ if self.major < comparison.major => {
                VersionCompatibility::DifferentMajor(VersionDifference::TooNew)
            }
            _ if self.minor > comparison.minor => {
                VersionCompatibility::DifferentMinor(VersionDifference::TooOld)
            }
            _ if self.minor < comparison.minor => {
                VersionCompatibility::DifferentMinor(VersionDifference::TooNew)
            }
            _ if self.patch > comparison.patch => {
                VersionCompatibility::DifferentPatch(VersionDifference::TooOld)
            }
            _ if self.patch < comparison.patch => {
                VersionCompatibility::DifferentPatch(VersionDifference::TooNew)
            }
            _ => VersionCompatibility::Identical,
        };
        // If we're in beta (0.x.x), any difference is tantamount to treason
        if self.major == 0 && !matches!(compatibility, VersionCompatibility::Identical) {
            // Here we figure out if the comparison version is too old or too new
            VersionCompatibility::DifferentBetaVersion(match compatibility {
                VersionCompatibility::DifferentMajor(version_difference) => version_difference,
                VersionCompatibility::DifferentMinor(version_difference) => version_difference,
                VersionCompatibility::DifferentPatch(version_difference) => version_difference,
                _ => panic!("Critical logic failure. You should report this as a bug."), // This shouldn't be possible, we know more than the compiler
            })
        } else {
            compatibility
        }
    }
}

// This breaks a given version down into major/minor/patch numbers
pub fn get_version_parts(version_str: &str) -> Result<Version, String> {
    let split: Vec<&str> = version_str.split('.').collect();
    // Get each component of that
    let patch = split.get(2)
        .ok_or_else(|| String::from(
            "Couldn't extract the patch version number from the given version string. If the version string in your Bonnie configuration file is definitely of the form 'x.y.z', you should report this as a bug."
        ))?
        .parse::<u16>()
        .map_err(|_| String::from(
            "Couldn't serialize the patch version number from the given version string into an integer. If the version string in your Bonnie configuration file is definitely of the form 'x.y.z', where each of those are integers, you should report this as a bug."
        ))?;
    let minor = split.get(1)
        .ok_or_else(|| String::from(
            "Couldn't extract the minor version number from the given version string. If the version string in your Bonnie configuration file is definitely of the form 'x.y.z', you should report this as a bug."
        ))?
        .parse::<u16>()
        .map_err(|_| String::from(
            "Couldn't serialize the minor version number from the given version string into an integer. If the version string in your Bonnie configuration file is definitely of the form 'x.y.z', where each of those are integers, you should report this as a bug."
        ))?;
    let major = split.get(0)
        .ok_or_else(|| String::from(
            "Couldn't extract the major version number from the given version string. If the version string in your Bonnie configuration file is definitely of the form 'x.y.z', you should report this as a bug."
        ))?
        .parse::<u16>()
        .map_err(|_| String::from(
            "Couldn't serialize the major version number from the given version string into an integer. If the version string in your Bonnie configuration file is definitely of the form 'x.y.z', where each of those are integers, you should report this as a bug."
        ))?;
    // Construct a version
    Ok(Version {
        patch,
        minor,
        major,
    })
}

// TESTING

// Creates a version from a vector for convenience (testing utility only)
// This will panic if something goes wrong
fn build_version(parts: Vec<u16>) -> Version {
    // We specify in reverse (e.g. [1, 2, 3] -> 1.2.3)
    Version {
        patch: parts[2],
        minor: parts[1],
        major: parts[0],
    }
}

// Tests for comparing versions
// None of these actually test how we interpret what's valid/invalid for compatibility (that logic is in `read_cfg.rs`)
#[test]
fn identifies_identical_versions() {
    let version = build_version(vec![2, 3, 4]);
    let comparison = build_version(vec![2, 3, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(compat, VersionCompatibility::Identical);
}
#[test]
fn identifies_major_too_new() {
    let version = build_version(vec![2, 3, 4]);
    let comparison = build_version(vec![3, 3, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentMajor(VersionDifference::TooNew)
    );
}
#[test]
fn identifies_major_too_old() {
    let version = build_version(vec![2, 3, 4]);
    let comparison = build_version(vec![1, 3, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentMajor(VersionDifference::TooOld)
    );
}
#[test]
fn identifies_minor_too_new() {
    let version = build_version(vec![2, 3, 4]);
    let comparison = build_version(vec![2, 4, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentMinor(VersionDifference::TooNew)
    );
}
#[test]
fn identifies_minor_too_old() {
    let version = build_version(vec![2, 3, 4]);
    let comparison = build_version(vec![2, 2, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentMinor(VersionDifference::TooOld)
    );
}
#[test]
fn identifies_patch_too_new() {
    let version = build_version(vec![2, 3, 4]);
    let comparison = build_version(vec![2, 3, 5]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentPatch(VersionDifference::TooNew)
    );
}
#[test]
fn identifies_patch_too_old() {
    let version = build_version(vec![2, 3, 4]);
    let comparison = build_version(vec![2, 3, 3]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentPatch(VersionDifference::TooOld)
    );
}
// All those tests for beta
#[test]
fn identifies_identical_versions_in_beta() {
    let version = build_version(vec![0, 3, 4]);
    let comparison = build_version(vec![0, 3, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(compat, VersionCompatibility::Identical);
}
#[test]
fn identifies_major_too_new_in_beta() {
    let version = build_version(vec![0, 3, 4]);
    let comparison = build_version(vec![1, 3, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentBetaVersion(VersionDifference::TooNew)
    );
}
#[test]
fn identifies_minor_too_new_in_beta() {
    let version = build_version(vec![0, 3, 4]);
    let comparison = build_version(vec![0, 4, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentBetaVersion(VersionDifference::TooNew)
    );
}
#[test]
fn identifies_minor_too_old_in_beta() {
    let version = build_version(vec![0, 3, 4]);
    let comparison = build_version(vec![0, 2, 4]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentBetaVersion(VersionDifference::TooOld)
    );
}
#[test]
fn identifies_patch_too_new_in_beta() {
    let version = build_version(vec![0, 3, 4]);
    let comparison = build_version(vec![0, 3, 5]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentBetaVersion(VersionDifference::TooNew)
    );
}
#[test]
fn identifies_patch_too_old_in_beta() {
    let version = build_version(vec![0, 3, 4]);
    let comparison = build_version(vec![0, 3, 3]);
    let compat = version.is_compatible_with(&comparison);

    assert_eq!(
        compat,
        VersionCompatibility::DifferentBetaVersion(VersionDifference::TooOld)
    );
}

// Tests for splitting the version into its parts
#[test]
fn returns_correct_part_division() {
    let version = "1.2.3";
    let parts = get_version_parts(version);

    assert_eq!(parts, Ok(build_version(vec![1, 2, 3])))
}
#[test]
fn returns_error_on_missing_patch_number() {
    let version = "1.2";
    let parts = get_version_parts(version);

    if parts.is_ok() {
        panic!("Didn't return an error on missing patch number.")
    }
}
#[test]
fn returns_error_on_missing_minor_number() {
    let version = "1";
    let parts = get_version_parts(version);

    if parts.is_ok() {
        panic!("Didn't return an error on missing minor number.")
    }
}
#[test]
fn returns_error_on_invalid_patch_number() {
    let version = "1.2.x";
    let parts = get_version_parts(version);

    if parts.is_ok() {
        panic!("Didn't return an error on invalid patch number.")
    }
}
#[test]
fn returns_error_on_invalid_minor_number() {
    let version = "1.x.3";
    let parts = get_version_parts(version);

    if parts.is_ok() {
        panic!("Didn't return an error on invalid minor number.")
    }
}
#[test]
fn returns_error_on_invalid_major_number() {
    let version = "x.2.3";
    let parts = get_version_parts(version);

    if parts.is_ok() {
        panic!("Didn't return an error on invalid major number.")
    }
}
