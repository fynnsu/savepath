use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use crate::error::{Result, Error};
use crate::state::Config;
use crate::parse::parser;

pub mod error;
pub mod parse;
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
    let mut cmd = cmd_name.clone();
    cmd.push(" ");
    cmd.push(args.join(&OsString::from(" ")));

    println!("{}", cmd.to_str().ok_or(Error::BadString)?);

    Ok(())
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
