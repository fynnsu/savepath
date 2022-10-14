use clipboard::error::Result;
use clipboard::parse::pap::{self};
use clipboard::utils;

fn main() -> Result<()> {
    let ext_cmd = pap::parse()?;

    let cmd = clipboard::create_modified_cmd(&ext_cmd)?;

    utils::write_command(&cmd, false)?;

    let choice = accept_suggestion()?;
    utils::newline();

    if choice {
        println!("{}", cmd);
        Ok(())
    } else {
        std::process::exit(1)
    }
}

fn accept_suggestion() -> Result<bool> {
    loop {
        match utils::getch()? {
            3u8 => return Ok(false), // ctrl+c pressed
            13u8 => return Ok(true), // enter pressed
            _ => (),
        }
    }
}
