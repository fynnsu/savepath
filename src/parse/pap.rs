use clap::{arg, command, value_parser, Arg, ArgAction, Command};

use crate::error::Result;

#[derive(Debug)]
pub struct ExtCmd {
    pub id: usize,
    pub cmd: String,
    pub args: Vec<String>,
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
                .help("External command to run."),
        )
        .arg(
            Arg::new("ext_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .help("Arguments for external command."),
        )
}

pub fn parse() -> Result<ExtCmd> {
    let matches = match build_parser().try_get_matches() {
        Ok(m) => m, // TODO: Remove this crap after pull-request to clap
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let cmd: String = matches
        .get_one::<String>("ext_cmd")
        .expect("ext_cmd is a required argument")
        .clone();

    let args: Vec<String> = matches
        .get_many("ext_args")
        .unwrap_or_default()
        .cloned()
        .collect();

    let id: usize = *matches
        .get_one::<usize>("id")
        .expect("id has a default value");

    let use_pos: bool = matches.get_flag("pos");

    Ok(ExtCmd {
        id,
        cmd,
        args,
        use_pos,
    })
}
