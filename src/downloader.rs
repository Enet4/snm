use crate::archive::Archive;
use crate::config::Config;
use crate::fetcher::Release;
use crate::symlink::symlink_to;
use crate::url;
use std::path::PathBuf;
use ureq;

pub struct Downloader;

impl Downloader {
    pub fn download(&self, r: &Release, config: &Config) -> anyhow::Result<PathBuf> {
        let bin = url::release(&r.version);

        let release_dir = &config.release_dir();
        let dest = release_dir.join(&r.version.to_string());

        if dest.exists() {
            return Err(anyhow::Error::msg(format!(
                "Binary with version ({}) is already exists.",
                &r.version
            )));
        }

        let res = ureq::get(&bin.url).call()?;
        let len = res
            .header("Content-Length")
            .and_then(|x| x.parse::<usize>().ok())
            .ok_or(anyhow::Error::msg("Unable to get content length."))?;

        println!("Installing : {}", &r.version);
        println!("Dowloading : {}", &bin.url);
        println!("Size       : {}", &len);

        Archive::new(res).extract_into(&release_dir)?;

        std::fs::rename(&release_dir.join(bin.name), &dest)?;

        // If we are only downloading then don't need to create a symlink to default
        if !config.download_only {
            symlink_to(&dest, &config.alias_default())?;
        }

        Ok(dest)
    }
}

// installing : node-v14.15.3
// mkdir : /home/hello/n/n/versions/node/14.15.3
// fetch : https://nodejs.org/dist/v14.15.3/node-v14.15.3-linux-x64.tar.xz
// fetch : https://nodejs.org/download/release/v14.15.3/node-v14.15.3-linux-x64.tar.xz
// installed : v14.15.3 (with npm 6.14.9)
