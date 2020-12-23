use crate::config::Config;
use crate::downloader::Downloader;
use crate::fetcher::Releases;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Lts;

impl Lts {
    pub fn init(&self, config: Config) {
        let release = Releases::fetch().lts().unwrap();

        Downloader.download(&release, &config);
    }
}