use clap::{
    arg, command, parser::ValueSource, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command,
};
use std::path::PathBuf;

use crate::error::{Error, Result};

#[derive(Debug)]
pub enum CMD {
    Add { files: Vec<PathBuf> },
    List,
    Clear,
    Alias { alias: String, shell: String },
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
                .default_value("pap")
                .default_missing_value("pap")
                .requires("shell")
                .help("Generate alias with optional name. Requires '--shell'"),
        )
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .value_name("SHELL")
                .help("Specify which shell to generate alias for. Must be one of [zsh|bash|fish].") // |cmd|powershell
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

fn check_alias_flag(m: &ArgMatches) -> bool {
    matches!(m.value_source("alias"), Some(ValueSource::CommandLine))
}

pub fn parse() -> Result<CMD> {
    let matches = build_parser().get_matches();

    if matches.get_flag("list") {
        Ok(CMD::List)
    } else if matches.get_flag("clear") {
        Ok(CMD::Clear)
    } else if check_alias_flag(&matches) {
        let alias: String = matches
            .get_one::<String>("alias")
            .ok_or(Error::ParseError)?
            .clone();
        let shell: String = matches
            .get_one::<String>("shell")
            .ok_or(Error::ParseError)?
            .clone();
        Ok(CMD::Alias { alias, shell })
    } else {
        let files = matches
            .get_many("files")
            .expect("files is a required argument")
            .cloned()
            .collect();

        Ok(CMD::Add { files })
    }
}
