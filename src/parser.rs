use std::path::PathBuf;
use clap::{arg, command, value_parser, Arg, ArgAction, Command};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Id (pub isize,);

fn parse_id(s: &str) -> Result<Id, String> {

    if let Ok(id) = s.parse() {
        return Ok(Id(id));
    }

    Err(String::from(
        "Couldn't parse file id. Must be integer.",
    ))
}

#[derive(Debug)]
pub enum CMD {
    Add { files: Vec<PathBuf> },
    List,
    Clear,
    ExtCmd { id: Id, cmd: Vec<String> },
}

pub fn parse() -> Result<CMD> {
    let matches = command!()
        .subcommand_negates_reqs(true)
        .args_conflicts_with_subcommands(true)
        .arg(
            arg!(-i --id <ID> "The id of the file to modify")
                .value_parser(parse_id)
                .default_value("0"),
        )
        .arg(
            Arg::new("ext")
                .action(ArgAction::Set)
                .num_args(1..)
                .required(true)
                .trailing_var_arg(true),
        )
        .subcommand(
            Command::new("add").arg(
                Arg::new("files")
                    .action(ArgAction::Append)
                    .value_parser(value_parser!(PathBuf))
                    .required(true)
                    .help("Files to save to clipboard"),
            ),
        )
        .subcommand(Command::new("list"))
        .subcommand(Command::new("clear"))
        .get_matches();

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
            let ext_cmd = matches
                .get_many("ext")
                .expect("ext is a required argument")
                .cloned()
                .collect();

            let id: Id = matches
                .get_one::<Id>("id")
                .expect("id has a default value")
                .clone();

            Ok(CMD::ExtCmd { id, cmd: ext_cmd })
        }
        Some(_) => unreachable!("There are no other subcommands"),
    }
}

// pub fn main() {
//     let mut matches = Command::new("Clipboard Add")
//         .about("Explains in brief what the program does")
//         .no_binary_name(true)
//         .arg(
//             Arg::new("files")
//                 .action(ArgAction::Append)
//                 .value_parser(value_parser!(PathBuf))
//                 .help("Files to save to clipboard"),
//         )
//         .get_matches_from(vec!["--help"]);
// }
