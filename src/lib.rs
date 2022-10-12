use std::env;
use std::path::PathBuf;
use std::process::Command;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use crate::error::{Error, Result};
use crate::state::{Config, Entry};

pub mod error;
pub mod parser;
pub mod state;

pub fn run_ext(id: parser::Id, mut cmd: Vec<String>) -> Result<()> {
    let config = Config::load()?;

    let id_path = get_path(config, &id)?;

    let output = Command::new(cmd.get(0).ok_or(Error::IndexError)?)
        .arg(id_path)
        .args(&mut cmd[1..])
        // .spawn()?;
        .output()?;

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        Err(Error::ExtCmdFailed(output.status))
    } else {
        Ok(())
    }
}

pub fn list() -> Result<()> {
    let config = Config::load()?;

    println!("Clipboard:\n");

    let mut table = Table::new();

    table.set_titles(row!["Id", "Path", "Name"]);

    for (i, v) in config.state.iter().enumerate() {
        let Entry { path, filename: _ } = v;
        let fname = v
            .filename()
            .ok_or(Error::BadString)?
            .to_str()
            .ok_or(Error::BadString)?;
        table.add_row(row![i, path.to_string_lossy(), fname]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.printstd();

    Ok(())
}

fn get_path(config: Config, id: &parser::Id) -> Result<PathBuf> {
    let x = config.get(id.0)?;
    Ok(x.full_path())
}

pub fn add(files: Vec<PathBuf>) -> Result<()> {
    let cur_dir = env::current_dir()?;

    let mut config = Config::load()?;
    config.extend(cur_dir, files);
    config.save()?;
    Ok(())
}

pub fn clear() -> Result<()> {
    Config::empty().save()?;
    Ok(())
}