use clap::{arg, command, value_parser, Arg, ArgAction, ArgGroup, Command};
use std::path::PathBuf;

use crate::error::Result;

#[derive(Debug)]
pub enum CMD {
    Add { files: Vec<PathBuf> },
    List,
    Clear,
    Alias,
}

fn build_parser() -> Command {
    command!()
        .arg(arg!(-l --list "List saved paths").action(ArgAction::SetTrue))
        .arg(arg!(-c --clear "Clear saved paths").action(ArgAction::SetTrue))
        .arg(
            Arg::new("alias")
                .long("alias")
                .short('a')
                .num_args(0..2)
                .value_name("ALIAS")
                .default_value("up")
                .default_missing_value("up"),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .num_args(0..2)
                .value_name("WHEN")
                .default_value("auto")
                .overrides_with("color")
                .require_equals(true)
                .default_missing_value("always"),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Set)
                .value_parser(value_parser!(PathBuf))
                .num_args(1..)
                .help("Paths to save"),
        )
        .group(
            ArgGroup::new("mode")
                .args(&["list", "clear", "alias", "files"])
                .required(true),
        )
}

pub fn parse() -> Result<CMD> {
    let matches = build_parser().get_matches();

    if matches.get_flag("list") {
        Ok(CMD::List)
    } else if matches.get_flag("clear") {
        Ok(CMD::Clear)
    } else if matches.contains_id("alias") {
        Ok(CMD::Alias)
    } else {
        let files = matches
            .get_many("files")
            .expect("files is a required argument")
            .cloned()
            .collect();

        Ok(CMD::Add { files })
    }
}
