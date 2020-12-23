use clap::Clap;
use dirs_next::home_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Clap, Debug)]
pub struct Config {
    #[clap(long, env = "SNM_DIR")]
    pub snm_dir: Option<PathBuf>,

    #[clap(long, env = "SNM_LOGLEVEL", default_value = "info")]
    pub log_level: String,

    /// Only downloads the binary
    #[clap(short, long)]
    pub download_only: bool,
}

impl Config {
    // pub fn new() -> Self {
    //     Config { base_dir: None }
    // }

    pub fn ensure_create<P: AsRef<Path>>(&self, path: P) -> P {
        create_dir_all(&path).ok();
        path
    }

    pub fn snm_home(&self) -> PathBuf {
        self.ensure_create(
            (self.snm_dir.clone())
                .unwrap_or_else(|| home_dir().expect("Can't get home directory.").join(".snm")),
        )
    }

    pub fn release_dir(&self) -> PathBuf {
        self.ensure_create(self.snm_home().join("releases"))
    }

    pub fn alias_dir(&self) -> PathBuf {
        self.ensure_create(self.snm_home().join("aliases"))
    }

    pub fn alias_default(&self) -> PathBuf {
        self.alias_dir().join("default")
    }
}