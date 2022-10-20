use std::env;
use std::path::PathBuf;
#[macro_use]
extern crate prettytable;
use anyhow::Context;
use prettytable::{format, Table};

use crate::parse::pap::ExtCmd;
use crate::state::Config;

pub mod parse;
mod shell;
pub mod state;
pub mod utils;

pub use shell::{print_alias, shell_from_str, Shell};

/// Creates a command string using the ext_cmd object and current config state
///
/// # Arguments
/// config: &Config - the current config state
/// ext_cmd: &ExtCmd - the external command object
///
/// # Returns
/// Result<String> - the modified command string
pub fn create_modified_cmd(config: &Config, ext_cmd: &ExtCmd) -> anyhow::Result<String> {
    let path_entry = match config.get(ext_cmd.id) {
        Some(p) => p,
        None => return Err(anyhow::anyhow!("Invalid entry index {}", ext_cmd.id)),
    };
    let id_path = path_entry.path().to_string_lossy();

    let id_path: String = format!("\"{}\"", id_path);

    let mut cmd_args = ext_cmd.cmd_args.clone();
    cmd_args.insert(ext_cmd.cur_pos, id_path);

    Ok(cmd_args.join(" "))
}

/// Creates a table of the current config state and prints it to stdout
///
/// # Arguments
/// config: &Config - the current config state
pub fn list(config: &Config) {
    let mut table = Table::new();

    table.set_titles(row!["Id", "Path"]);

    for (i, v) in config.iter().enumerate() {
        table.add_row(row![i, v.path().to_string_lossy()]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.printstd();
}

/// Adds the given files to the current config state
///
/// # Arguments
/// config: &mut Config - the current config state
/// files: Vec<PathBuf> - the files to add to the config state
///
/// # Errors
/// IoError - if the current working directory cannot be determined or accessed
pub fn add(config: &mut Config, files: Vec<PathBuf>) -> anyhow::Result<()> {
    let cur_dir = env::current_dir().context("Could not determine current working directory")?;
    config.extend(cur_dir, files)
}

pub fn clear(config: &mut Config) -> anyhow::Result<()> {
    config.clear()
}
