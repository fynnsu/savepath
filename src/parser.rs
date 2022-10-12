use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;

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
    Add { files: Vec<PathBuf> },
    List,
    Clear,
    ExtCmd { id: Id, cmd: String, args: Vec<String> },
}

fn build_parser() -> Command {
    command!()
        .subcommand_negates_reqs(true)
        .args_conflicts_with_subcommands(true)
        .arg(
            arg!(-i --id <ID> "The id of the file to modify")
                .value_parser(parse_id)
                .default_value("0"),
        )
        .arg(
            Arg::new("ext_cmd")
                .action(ArgAction::Set)
                .required(true)
                .help("External command to run.")
        )
        .arg(
            Arg::new("ext_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .help("Arguments for external command.")
        )
        .subcommand(
            Command::new("add").arg(
                Arg::new("files")
                    .action(ArgAction::Append)
                    .value_parser(value_parser!(PathBuf))
                    .required(true)
                    .help("Files to save to clipboard"),
            ).about("Add new paths to clipboard.")
        )
        .subcommand(Command::new("list").about("List all paths on clipboard."))
        .subcommand(Command::new("clear").about("Clear clipboard."))
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
            let ext_cmd: String = matches
                .get_one::<String>("ext_cmd")
                .expect("ext_cmd is a required argument").clone();
            let cmd_args: Vec<String> = matches
                .get_many("ext_args")
                .unwrap_or_default()
                .cloned()
                .collect();

            let id: Id = matches
                .get_one::<Id>("id")
                .expect("id has a default value")
                .clone();

            Ok(CMD::ExtCmd { id, cmd: ext_cmd, args: cmd_args })
        }
        Some(_) => unreachable!("There are no other subcommands"),
    }
}
