use clipboard::parse::sp::{self, CMD};
use clipboard::error::Result;

fn main() -> Result<()>{
    let cmd = sp::parse()?;

    // println!("{:#?}", cmd);

    match cmd {
        CMD::List => {
            clipboard::list()?;
        },
        CMD::Clear => {
            clipboard::clear()?;
        },
        CMD::Add { files } => {
            clipboard::add(files)?;
        },
        CMD::Alias => {
            ()
        }
    }

    Ok(())
}