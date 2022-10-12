use clipboard::error::Result;
use clipboard::parser::CMD;

fn main() -> Result<()> {
    let cmd = clipboard::parser::parse()?;

    // println!("{:#?}", cmd);

    match cmd {
        CMD::List => {
            clipboard::list()?;
        }
        CMD::Clear => {
            clipboard::clear()?;
        }
        CMD::Add { files } => {
            // Temporarily convert PathBufs to strings
            // let files = files.iter().map(|e| e.to_string_lossy().into_owned()).collect();
            clipboard::add(files)?;
        }
        CMD::ExtCmd { id, cmd , args } => {
            clipboard::run_ext(id, cmd, args)?;
        }
    }

    Ok(())
}
