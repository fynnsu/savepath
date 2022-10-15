use clipboard::error::Result;
use clipboard::parse::sap::{self, CMD};

fn main() -> Result<()> {
    let cmd = sap::parse()?;

    // println!("{:#?}", cmd);

    match cmd {
        CMD::List => {
            clipboard::list()?;
        }
        CMD::Clear => {
            clipboard::clear()?;
        }
        CMD::Add { files } => {
            clipboard::add(files)?;
        }
        CMD::Alias { alias, shell } => {
            let shell = clipboard::shell_from_str(&shell);
            clipboard::print_alias(shell, &alias);
        }
    }

    Ok(())
}
