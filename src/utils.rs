use std::collections::HashSet;

use super::version::semantic_version::SemanticVersion;
use anyhow::{Result, bail};

pub fn select_single_version<T, I>(versions: I) -> Result<SemanticVersion>
where
    I: IntoIterator<Item = T>,
    T: TryInto<SemanticVersion>,
    anyhow::Error: From<T::Error>,
{
    let mut set = HashSet::new();

    for v in versions {
        set.insert(v.try_into().map_err(anyhow::Error::from)?);
    }

    match set.len() {
        0 => bail!("No packages found."),
        1 => Ok(set.into_iter().next().unwrap()),
        _ => bail!("Workspace has packages with different versions."),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn sv(s: &str) -> SemanticVersion {
        SemanticVersion::try_from(cargo_metadata::semver::Version::parse(s).unwrap()).unwrap()
    }

    #[test]
    fn select_single_version_ok() {
        let mut set = std::collections::HashSet::new();
        set.insert(sv("1.2.3"));

        let v = select_single_version(set).unwrap();
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn select_single_version_empty() {
        let set: HashSet<SemanticVersion> = std::collections::HashSet::new();
        assert!(select_single_version(set).is_err());
    }

    #[test]
    fn select_single_version_conflict() {
        let mut set = std::collections::HashSet::new();
        set.insert(sv("1.2.3"));
        set.insert(sv("1.2.4"));

        assert!(select_single_version(set).is_err());
    }
}
