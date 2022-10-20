use savepath::parse::sap::{self, CMD};
use savepath::state::Config;

fn main() -> anyhow::Result<()> {
    let cmd = sap::parse();

    let mut config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            Config::empty()
        }
    };

    // println!("{:#?}", cmd);

    match cmd {
        CMD::List => {
            savepath::list(&config);
        }
        CMD::Clear => {
            savepath::clear(&mut config)?;
        }
        CMD::Add { files } => {
            savepath::add(&mut config, files)?;
        }
        CMD::Alias { alias, shell } => {
            let shell = savepath::shell_from_str(&shell);
            savepath::print_alias(shell, &alias);
        }
    }

    Ok(())
}
