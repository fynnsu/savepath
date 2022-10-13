use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use crate::error::{Error, Result};
use crate::state::Config;

pub mod error;
pub mod parse;
pub mod state;
pub mod utils;

const ALIAS_PLACEHOLDER: &str = "SAVEPATH_ALIAS_PLACEHOLDER";

pub fn print_modified_cmd(
    id: usize,
    use_pos: bool,
    cmd_name: OsString,
    mut args: Vec<OsString>,
) -> Result<()> {
    let config = Config::load()?;

    let id_path = config.get(id)?.path();

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

pub fn print_alias(alias_name: &str, shell_name: &str) -> Result<()> {
    // Read script template for shell_name
    let script = utils::get_shell_template(shell_name)?;

    // replace alias_placeholdr with alias_name
    let script = script.replace(ALIAS_PLACEHOLDER, alias_name);

    // print resulting string
    println!("{}", script);

    Ok(())
}

pub fn list() -> Result<()> {
    let config = Config::load()?;

    println!("Clipboard:\n");

    let mut table = Table::new();

    table.set_titles(row!["Id", "Path"]);

    for (i, v) in config.iter().enumerate() {
        table.add_row(row![i, v.path().to_string_lossy()]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.printstd();

    Ok(())
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
