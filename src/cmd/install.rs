use crate::config::Config;
use crate::downloader::Downloader;
use crate::fetcher::Fetcher;
use crate::version::Version;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Install {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    version: Version,
}

impl super::Command for Install {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let release = Fetcher::fetch(&config.dist_mirror)?.find_release(&self.version);

        match release {
            Some(r) => {
                Downloader.download(&r, &config)?;
            }
            _ => println!("No release found with the version {}", &self.version),
        }

        Ok(())
    }
}
