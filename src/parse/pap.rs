use clap::{arg, command, value_parser, Arg, ArgAction, Command};

use crate::error::Result;

#[derive(Debug)]
pub struct ExtCmd {
    pub id: usize,
    pub cmd_args: Vec<String>,
    pub use_pos: bool,
    pub cur_pos: usize,
    pub nargs: usize,
}

fn build_parser() -> Command {
    command!()
        .arg(
            arg!(-i --id <ID> "The id of the path to use")
                .value_parser(value_parser!(usize))
                .default_value("0"),
        )
        .arg(
            arg!(-p --pos "Insert path where specified by '$' in ext_args.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("cmd_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .required(true)
                .help("External command with args to run"),
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

    let cmd_args: Vec<String> = matches
        .get_many("cmd_args")
        .unwrap_or_default()
        .cloned()
        .collect();

    let id: usize = *matches
        .get_one::<usize>("id")
        .expect("id has a default value");

    let use_pos: bool = matches.get_flag("pos");

    let nargs = cmd_args.len();

    Ok(ExtCmd {
        id,
        cmd_args,
        use_pos,
        cur_pos: 1,
        nargs,
    })
}
