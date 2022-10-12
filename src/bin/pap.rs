use clipboard::error::Result;
use clipboard::parse::pap::{self, ExtCmd};

fn main() -> Result<()> {
    let ExtCmd { id, cmd, args, use_pos } = pap::parse()?;

    // println!("{:#?}", cmd);
    clipboard::run_ext(id, use_pos, cmd, args)?;

    Ok(())
}
