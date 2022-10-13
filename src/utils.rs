use std::{path::PathBuf, fs};

use directories::ProjectDirs;

use crate::error::{Error, Result};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "fynnsu";
const APPLICATION: &str = "clipboard";
const SHELL_FOLDER: &str = "shell";
const CONFIG_FILE: &str = "config.ron";

fn get_config_dir() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let path = PathBuf::from(proj_dirs.cache_dir());
        Ok(path)
    } else {
        Err(Error::ApplicationDirNotAccessible)
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join(PathBuf::from(CONFIG_FILE)))
}

pub fn get_shell_template(shell_name: &str) -> Result<String> {
    let path = get_config_dir()?.join(SHELL_FOLDER).join(format!("{}.txt", shell_name));
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}