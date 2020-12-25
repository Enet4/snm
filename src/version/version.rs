use super::node_version::NodeVersion;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    Major(u64),
    MajorMinor(u64, u64),
    Full(NodeVersion),
}

impl Version {
    /// Get a appropriate version from Vec<NodeVersion>
    #[allow(dead_code)]
    pub fn to_version<'a, T>(&self, versions: T) -> Option<&'a NodeVersion>
    where
        T: IntoIterator<Item = &'a NodeVersion>,
    {
        versions
            .into_iter()
            .filter(|&v| self.match_version(v))
            .max()
    }

    pub fn match_version(&self, version: &NodeVersion) -> bool {
        match (self, version) {
            (Self::Full(a), b) if a == b => true,
            (Self::Major(major), NodeVersion::Semver(other)) => major == &other.major,
            (Self::MajorMinor(major, minor), NodeVersion::Semver(other)) => {
                *major == other.major && *minor == other.minor
            }
            (_, NodeVersion::Lts(_)) => false,
            (_, NodeVersion::Alias(_)) => false,
            _ => false,
        }
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match NodeVersion::parse(s) {
            Ok(v) => Ok(Self::Full(v)),
            Err(e) => {
                let mut parts = s.trim_start_matches("v").split(".");
                match (parts.next(), parts.next()) {
                    (Some(major), None) => Ok(Self::Major(major.parse::<u64>()?)),
                    (Some(major), Some(minor)) => Ok(Self::MajorMinor(
                        major.parse::<u64>()?,
                        minor.parse::<u64>()?,
                    )),
                    _ => Err(e),
                }
            }
        }
    }
}

// impl<'de> Deserialize<'de> for Version {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let version_str = String::deserialize(deserializer)?;
//
//         Version::from_str(&version_str).map_err(serde::de::Error::custom)
//     }
// }

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full(x) => x.fmt(f),
            Self::Major(major) => write!(f, "v{}.x.x", major),
            Self::MajorMinor(major, minor) => write!(f, "v{}.{}.x", major, minor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_only_major() {
        let ver = Version::from_str("10").unwrap();
        assert_eq!(ver, Version::Major(10))
    }

    #[test]
    fn test_not_major() {
        let ver = Version::from_str("10.15").unwrap();
        assert_ne!(ver, Version::Major(10))
    }

    #[test]
    fn test_major_minor() {
        let ver = Version::from_str("10.15").unwrap();
        assert_eq!(ver, Version::MajorMinor(10, 15))
    }

    #[test]
    fn test_not_major_minor() {
        let ver = Version::from_str("10").unwrap();
        assert_ne!(ver, Version::MajorMinor(10, 10))
    }

    #[test]
    fn test_match_full_version() {
        let ver = NodeVersion::parse("10.15.0").unwrap();
        assert!(Version::Full(ver.clone()).match_version(&ver))
    }

    #[test]
    fn test_match_major_version() {
        let ver = NodeVersion::parse("10.15.0").unwrap();
        assert!(Version::Major(10).match_version(&ver))
    }

    #[test]
    fn test_not_match_major_version() {
        let ver = NodeVersion::parse("10.15.0").unwrap();
        assert!(!Version::Major(19).match_version(&ver))
    }

    #[test]
    fn test_match_major_minor_version() {
        let ver = NodeVersion::parse("10.15.0").unwrap();
        assert!(Version::MajorMinor(10, 15).match_version(&ver))
    }

    #[test]
    fn test_not_match_major_minor_version() {
        let ver = NodeVersion::parse("10.15.0").unwrap();
        assert!(!Version::MajorMinor(10, 19).match_version(&ver))
    }

    #[test]
    fn test_major_to_version() {
        let expected = NodeVersion::parse("6.1.0").unwrap();
        let versions = vec![
            NodeVersion::parse("6.0.0").unwrap(),
            NodeVersion::parse("6.0.1").unwrap(),
            expected.clone(),
            NodeVersion::parse("7.0.1").unwrap(),
        ];
        let result = Version::Major(6).to_version(&versions);

        assert_eq!(result, Some(&expected));
    }

    #[test]
    fn test_not_major_to_version() {
        let expected = NodeVersion::parse("6.1.0").unwrap();
        let versions = vec![
            NodeVersion::parse("6.0.0").unwrap(),
            NodeVersion::parse("6.0.1").unwrap(),
            expected.clone(),
            NodeVersion::parse("6.2.0").unwrap(),
            NodeVersion::parse("7.0.1").unwrap(),
        ];
        let result = Version::Major(6).to_version(&versions);

        assert_ne!(result, Some(&expected));
    }

    #[test]
    fn test_major_minor_to_version() {
        let expected = NodeVersion::parse("6.0.1").unwrap();
        let versions = vec![
            NodeVersion::parse("6.0.0").unwrap(),
            NodeVersion::parse("6.1.0").unwrap(),
            expected.clone(),
            NodeVersion::parse("7.0.1").unwrap(),
        ];
        let result = Version::MajorMinor(6, 0).to_version(&versions);

        assert_eq!(result, Some(&expected));
    }

    #[test]
    fn test_no_major_minor_to_version() {
        let expected = NodeVersion::parse("6.0.1").unwrap();
        let versions = vec![
            NodeVersion::parse("6.0.0").unwrap(),
            NodeVersion::parse("6.1.0").unwrap(),
            expected.clone(),
            NodeVersion::parse("6.0.5").unwrap(),
            NodeVersion::parse("7.0.1").unwrap(),
        ];
        let result = Version::MajorMinor(6, 0).to_version(&versions);

        assert_ne!(result, Some(&expected));
    }
}
