use std::io::{self, stderr, Read, Write};
use std::path::PathBuf;

use crossterm::{queue, terminal, QueueableCommand};

use crossterm::cursor::MoveToColumn;
use crossterm::style::{Attribute, Color, PrintStyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use directories::ProjectDirs;

use crate::error::{Error, Result};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "fynnsu";
const APPLICATION: &str = "clipboard";
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

pub fn write_command(s: &str, clear: bool) -> Result<()> {
    let mut stderr = stderr();
    if clear {
        stderr.queue(Clear(ClearType::CurrentLine))?;
        stderr.queue(MoveToColumn(0))?;
    }

    queue!(
        stderr,
        PrintStyledContent(format!("{} ", s).attribute(Attribute::Bold)),
        PrintStyledContent("[".with(Color::Grey)),
        PrintStyledContent("enter".with(Color::Green)),
        PrintStyledContent("/".with(Color::Grey)),
        PrintStyledContent("ctrl+c".with(Color::Red)),
        PrintStyledContent("]".with(Color::Grey))
    )?;

    stderr.flush()?;

    Ok(())
}

pub fn newline() {
    eprintln!();
}

pub fn getch() -> Result<u8> {
    terminal::enable_raw_mode()?;

    let mut buf = [0; 1];
    let _ = io::stdin().read(&mut buf)?;

    terminal::disable_raw_mode()?;

    Ok(buf[0])
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test_clear_line_and_write() {
        write_command("test", false).unwrap();
        thread::sleep(Duration::from_secs(2));
        write_command("new command", true).unwrap();
        thread::sleep(Duration::from_secs(2));
        write_command("another command", true).unwrap();
        let mut c = getch().unwrap();
        while c != 13u8 {
            eprintln!("Received: {}", c);
            c = getch().unwrap();
        }
    }

    #[test]
    fn test_cache_dir() {
        println!("{:?}", get_config_dir().unwrap());
    }
}

// Enter key 13u8
// ctrl+c 3u8
