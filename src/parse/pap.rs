use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::ffi::OsString;

use crate::error::Result;

#[derive(Debug)]
pub struct ExtCmd {
    pub id: usize,
    pub cmd: OsString,
    pub args: Vec<OsString>,
    pub use_pos: bool,
}

fn build_parser() -> Command {
    command!()
        .arg(
            arg!(-i --id <ID> "The id of the file to modify")
                .value_parser(value_parser!(usize))
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
}

pub fn parse() -> Result<ExtCmd> {
    let matches = build_parser().get_matches();

    let cmd: OsString = matches
        .get_one::<OsString>("ext_cmd")
        .expect("ext_cmd is a required argument")
        .clone();

    let args: Vec<OsString> = matches
        .get_many("ext_args")
        .unwrap_or_default()
        .cloned()
        .collect();

    let id: usize = matches
        .get_one::<usize>("id")
        .expect("id has a default value")
        .clone();

    let use_pos: bool = matches.get_flag("pos");

    Ok(ExtCmd {
        id,
        cmd,
        args,
        use_pos,
    })
}
