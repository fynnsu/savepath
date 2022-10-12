use std::env;
use std::error::Error;

use clipboard::parser::CMD;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dir = env::current_exe().expect("Unable to access executible path.");
    dir.pop();
    dir.push("clipboard.json");
    let filename = dir.to_str().expect("Path should be valid");

    let cmd = clipboard::parser::parse()?;

    println!("{:#?}", cmd);

    match cmd {
        CMD::List => clipboard::list(filename),
        CMD::Clear => clipboard::clear(filename),
        CMD::Add { files } => {
            // Temporarily convert PathBufs to strings
            // let files = files.iter().map(|e| e.to_string_lossy().into_owned()).collect();
            clipboard::add(filename, files)?
        }
        CMD::ExtCmd { id, cmd } => {
            clipboard::run_ext(filename, id, cmd)?
        }
    }

    Ok(())
}
