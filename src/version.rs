// This file defines the current version of Bonnie
// This MUST be updated before all releases!

pub const BONNIE_VERSION: &str = "0.2.0";

// The different between two major/minor/patch versions
pub enum VersionDifference {
    TooOld,
    TooNew,
}

// The compatibility of two versions with one another
pub enum VersionCompatibility {
    Identical,
    DifferentMajor(VersionDifference), // Only this means the versions are incompatible
    DifferentMinor(VersionDifference),
    DifferentPatch(VersionDifference),
    DifferentBetaVersion(VersionDifference) // In beta, this also means the versions are incompatible
}

pub struct Version {
    patch: u16,
    minor: u16,
    major: u16
}
impl Version {
    // Compares this with another version returns their compatibility
    pub fn is_compatible_with(&self, comparison: &Version) -> VersionCompatibility {
        let compatibility = match self.major {
            _ if self.major > comparison.major => VersionCompatibility::DifferentMajor(VersionDifference::TooOld),
            _ if self.major < comparison.major => VersionCompatibility::DifferentMajor(VersionDifference::TooNew),
            _ if self.minor > comparison.minor => VersionCompatibility::DifferentMinor(VersionDifference::TooOld),
            _ if self.minor < comparison.minor => VersionCompatibility::DifferentMinor(VersionDifference::TooNew),
            _ if self.patch > comparison.patch => VersionCompatibility::DifferentPatch(VersionDifference::TooOld),
            _ if self.patch < comparison.patch => VersionCompatibility::DifferentPatch(VersionDifference::TooNew),
            _ => VersionCompatibility::Identical
        };
        // If we're in beta (0.x.x), any difference is tantamount to treason
        if self.major == 0 && !matches!(compatibility, VersionCompatibility::Identical) {
            // Here we figure out if the comparison version is too old or too new
            VersionCompatibility::DifferentBetaVersion(
                match compatibility {
                    VersionCompatibility::DifferentMajor(version_difference) => version_difference,
                    VersionCompatibility::DifferentMinor(version_difference) => version_difference,
                    VersionCompatibility::DifferentPatch(version_difference) => version_difference,
                    _ => panic!("Critical logic failure. You should report this as a bug.") // This shouldn't be possible, we know more than the compiler
                }
            )
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
        major
    })
}
