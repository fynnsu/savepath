use std::io::{stderr, Write};
use std::path::PathBuf;

use crossterm::{queue, QueueableCommand};

use crossterm::cursor::MoveToColumn;
use crossterm::style::{Attribute, Color, PrintStyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use directories::ProjectDirs;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "fynnsu";
const APPLICATION: &str = "savepath";
const CONFIG_FILE: &str = "config.ron";

/// Get the path to the application data directory
fn get_config_dir() -> anyhow::Result<PathBuf> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or_else(|| anyhow::anyhow!("Application directory not accessible"))
        .map(|x| x.cache_dir().to_path_buf())
}

/// Get the path to the config file
pub fn get_config_path() -> anyhow::Result<PathBuf> {
    Ok(get_config_dir()?.join(PathBuf::from(CONFIG_FILE)))
}

/// Print the given string to stderr with confirmation prompt
pub fn write_command(s: &str, clear: bool) -> anyhow::Result<()> {
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
        PrintStyledContent("←".with(Color::Blue)),
        PrintStyledContent("/".with(Color::Grey)),
        PrintStyledContent("↑".with(Color::Blue)),
        PrintStyledContent("/".with(Color::Grey)),
        PrintStyledContent("↓".with(Color::Blue)),
        PrintStyledContent("/".with(Color::Grey)),
        PrintStyledContent("→".with(Color::Blue)),
        PrintStyledContent("/".with(Color::Grey)),
        PrintStyledContent("ctrl+c".with(Color::Red)),
        PrintStyledContent("]".with(Color::Grey))
    )?;

    stderr.flush()?;

    Ok(())
}

/// Print newline to stderr
pub fn newline() {
    eprintln!();
}
