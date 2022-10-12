use clipboard::error::Result;
use clipboard::parse::parser::CMD;

fn main() -> Result<()> {
    let cmd = clipboard::parse::parser::parse()?;

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
        CMD::ExtCmd {
            id,
            cmd,
            args,
            use_pos,
        } => {
            clipboard::run_ext(id, use_pos, cmd, args)?;
        }
    }

    Ok(())
}
