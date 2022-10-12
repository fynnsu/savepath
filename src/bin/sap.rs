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
        CMD::Alias => (),
    }

    Ok(())
}
