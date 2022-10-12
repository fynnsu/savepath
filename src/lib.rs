use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command};
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use crate::error::{Error, Result};
use crate::state::Config;

pub mod error;
pub mod parser;
pub mod state;

pub fn run_ext(
    id: parser::Id,
    use_pos: bool,
    cmd_name: OsString,
    mut args: Vec<OsString>,
) -> Result<()> {
    let config = Config::load()?;

    let id_path = get_path(&config, &id)?;

    if use_pos {
        args = args
            .iter()
            .map(|x| {
                if x == "$" {
                    From::from(id_path)
                } else {
                    From::from(x)
                }
            })
            .collect();
    } else {
        args.insert(0, From::from(id_path));
    }
    let mut cmd = Command::new(cmd_name);
    let cmd = cmd.args(args);

    println!("Running Command:\n{:?}\n", cmd);

    let output = cmd.output()?;

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

    table.set_titles(row!["Id", "Path"]);

    for (i, v) in config.state.iter().enumerate() {
        table.add_row(row![i, v.path().to_string_lossy()]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.printstd();

    Ok(())
}

fn get_path<'a>(config: &'a Config, id: &parser::Id) -> Result<&'a PathBuf> {
    let x = config.get(id.0)?;
    Ok(x.path())
}

pub fn add(files: Vec<PathBuf>) -> Result<()> {
    let cur_dir = env::current_dir()?;

    let mut config = Config::load()?;
    config.extend(cur_dir, files)?;
    config.save()?;
    Ok(())
}

pub fn clear() -> Result<()> {
    Config::empty().save()?;
    Ok(())
}
