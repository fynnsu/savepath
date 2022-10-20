use clipboard::parse::sap::{self, CMD};
use clipboard::state::Config;

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
            clipboard::list(&config);
        }
        CMD::Clear => {
            clipboard::clear(&mut config)?;
        }
        CMD::Add { files } => {
            clipboard::add(&mut config, files)?;
        }
        CMD::Alias { alias, shell } => {
            let shell = clipboard::shell_from_str(&shell);
            clipboard::print_alias(shell, &alias);
        }
    }

    Ok(())
}
