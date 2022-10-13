use std::env;
use std::path::PathBuf;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use crate::error::Result;
use crate::parse::pap::ExtCmd;
use crate::state::Config;

pub mod error;
pub mod parse;
pub mod state;
pub mod utils;

const ALIAS_PLACEHOLDER: &str = "SAVEPATH_ALIAS_PLACEHOLDER";

pub fn create_modified_cmd(mut ext_cmd: ExtCmd) -> Result<String> {
    let config = Config::load()?;

    let id_path = config.get(ext_cmd.id)?.path().to_string_lossy().to_owned();

    if ext_cmd.use_pos {
        ext_cmd.args = ext_cmd
            .args
            .iter()
            .map(|x| {
                if x == "$" {
                    From::from(id_path.clone())
                } else {
                    From::from(x)
                }
            })
            .collect();
    } else {
        ext_cmd.args.insert(0, From::from(id_path));
    }

    let mut cmd = ext_cmd.cmd.clone();
    cmd.push(' ');
    cmd.push_str(&ext_cmd.args.join(" "));
    Ok(cmd)
}

pub fn print_modified_cmd(ext_cmd: ExtCmd) -> Result<()> {
    let cmd = create_modified_cmd(ext_cmd)?;

    println!("{}", cmd);

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
