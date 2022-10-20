use clap::{arg, command, value_parser, Arg, ArgAction, Command};

#[derive(Debug)]
pub struct ExtCmd {
    pub id: usize,
    pub cmd_args: Vec<String>,
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
            Arg::new("cmd_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .required(true)
                .help("External command with args to run"),
        )
}

pub fn parse() -> ExtCmd {
    let matches = match build_parser().try_get_matches() {
        Ok(m) => m, // TODO: Remove this after pull-request to clap
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

    let nargs = cmd_args.len();

    ExtCmd {
        id,
        cmd_args,
        cur_pos: 1,
        nargs,
    }
}
