use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

pub fn find_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir().context("current directory")?;
    loop {
        if dir.join("data").join("tools.yaml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("could not find data/tools.yaml from the current directory");
        }
    }
}

pub fn tools_path(root: &Path) -> PathBuf {
    root.join("data").join("tools.yaml")
}

pub fn config_path(root: &Path) -> PathBuf {
    root.join("data").join("config.yaml")
}

pub fn metadata_path(root: &Path) -> PathBuf {
    root.join("data").join("metadata.json")
}

pub fn snapshot_dir(root: &Path) -> PathBuf {
    root.join("data").join("snapshots")
}

pub fn discover_dir(root: &Path) -> PathBuf {
    root.join("discover")
}

pub fn digest_dir(root: &Path) -> PathBuf {
    root.join("digest")
}
