use clipboard::error::Result;
use clipboard::parse::pap::{self};
use clipboard::utils;

fn main() -> Result<()> {
    let ext_cmd = pap::parse()?;

    let cmd = clipboard::create_modified_cmd(ext_cmd)?;

    utils::write_command(&cmd, false);

    Ok(())
}
