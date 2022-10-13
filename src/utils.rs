use std::{fs, path::PathBuf};

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
    let path = get_config_dir()?
        .join(SHELL_FOLDER)
        .join(format!("{}.txt", shell_name));
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn write_command(s: &str, clear: bool) {
    if clear {
        const CLEAR_TERM_LINE: &str = "\033[1K\r"; //TODO: move this somewhere better
        eprint!("{}", CLEAR_TERM_LINE)
    }

    eprint!("{} (enter/ctr+c)", s);
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test_clear_line_and_write() {
        write_command("test", false);
        thread::sleep(Duration::from_secs(2));
        write_command("new command", true);
        thread::sleep(Duration::from_secs(2));
        write_command("another command", true);
    }
}
