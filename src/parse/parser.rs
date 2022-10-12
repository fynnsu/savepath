use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::{ffi::OsString, path::PathBuf};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Id(pub isize);

fn parse_id(s: &str) -> Result<Id, String> {
    if let Ok(id) = s.parse() {
        return Ok(Id(id));
    }

    Err(String::from("Couldn't parse file id. Must be integer."))
}

#[derive(Debug)]
pub enum CMD {
    Add {
        files: Vec<PathBuf>,
    },
    List,
    Clear,
    ExtCmd {
        id: Id,
        cmd: OsString,
        args: Vec<OsString>,
        use_pos: bool,
    },
}

fn build_parser() -> Command {
    command!()
        .subcommand_negates_reqs(true)
        .args_conflicts_with_subcommands(true)
        .disable_help_subcommand(true)
        .arg(
            arg!(-i --id <ID> "The id of the file to modify")
                .value_parser(parse_id)
                .default_value("0"),
        )
        .arg(
            arg!(-p --pos "Insert path where specified by '$' in ext_args.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ext_cmd")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(value_parser!(OsString))
                .help("External command to run."),
        )
        .arg(
            Arg::new("ext_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .value_parser(value_parser!(OsString))
                .trailing_var_arg(true)
                .help("Arguments for external command."),
        )
        .subcommand(
            Command::new("add")
                .arg(
                    Arg::new("files")
                        .action(ArgAction::Append)
                        .value_parser(value_parser!(PathBuf))
                        .required(true)
                        .help("Files to save to clipboard"),
                )
                .about("Add new paths to clipboard.")
                .visible_alias("a"),
        )
        .subcommand(
            Command::new("list")
                .about("List all paths on clipboard.")
                .visible_alias("l"),
        )
        .subcommand(
            Command::new("clear")
                .about("Clear clipboard.")
                .visible_alias("c"),
        )
}

pub fn parse() -> Result<CMD> {
    let matches = build_parser().get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let files = sub_m
                .get_many("files")
                .expect("files is a required argument")
                .cloned()
                .collect();

            Ok(CMD::Add { files })
        }
        Some(("list", _)) => Ok(CMD::List),
        Some(("clear", _)) => Ok(CMD::Clear),
        None => {
            let cmd: OsString = matches
                .get_one::<OsString>("ext_cmd")
                .expect("ext_cmd is a required argument")
                .clone();
            let args: Vec<OsString> = matches
                .get_many("ext_args")
                .unwrap_or_default()
                .cloned()
                .collect();

            let id: Id = matches
                .get_one::<Id>("id")
                .expect("id has a default value")
                .clone();

            let use_pos: bool = matches.get_flag("pos");

            Ok(CMD::ExtCmd {
                id,
                cmd,
                args,
                use_pos,
            })
        }
        Some(_) => unreachable!("There are no other subcommands"),
    }
}
