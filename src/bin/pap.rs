use clipboard::error::Result;
use clipboard::parse::pap::{self};
use clipboard::state::Config;
use clipboard::utils;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use event::Event::Key;

fn main() -> Result<()> {
    //TODO: manage errors better in this function
    let mut ext_cmd = pap::parse()?;

    if ext_cmd.use_pos {
        // Positional usage (can't move left and right)
    }

    let nfiles = Config::load()?.len();

    let mut cmd_str = clipboard::create_modified_cmd(&ext_cmd)?;

    utils::write_command(&cmd_str, false)?;

    enable_raw_mode()?;

    let mut choice = get_selection();

    loop {
        match choice {
            Selection::Accept => {
                disable_raw_mode()?;
                utils::newline();
                println!("{}", cmd_str);
                std::process::exit(0);
            }
            Selection::Cancel => {
                disable_raw_mode()?;
                std::process::exit(1);
            }
            Selection::Left => {
                choice = get_selection();
            }
            Selection::Right => {
                choice = get_selection();
            }
            Selection::Up => {
                ext_cmd.id = (ext_cmd.id + 1) % nfiles;
                cmd_str = clipboard::create_modified_cmd(&ext_cmd)?;
                utils::write_command(&cmd_str, true)?;
                choice = get_selection();
            }
            Selection::Down => {
                if ext_cmd.id == 0 {
                    ext_cmd.id = nfiles - 1;
                } else {
                    ext_cmd.id -= 1;
                }
                cmd_str = clipboard::create_modified_cmd(&ext_cmd)?;
                utils::write_command(&cmd_str, true)?;
                choice = get_selection();
            }
        }
    }
}

fn get_selection() -> Selection {
    loop {
        if let Ok(event )= event::read() {
            // eprintln!("Event: {:?}\r", event);
            match event {
                Key(KeyEvent{code: KeyCode::Enter, ..}) => return Selection::Accept,
                Key(KeyEvent{code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, ..}) => return Selection::Cancel,
                Key(KeyEvent{code: KeyCode::Left, ..}) => return Selection::Left,
                Key(KeyEvent{code: KeyCode::Right, ..}) => return Selection::Right,
                Key(KeyEvent{code: KeyCode::Up, ..}) => return Selection::Up,
                Key(KeyEvent{code: KeyCode::Down, ..}) => return Selection::Down,
                _ => continue
            }
        }
    }
}

enum Selection {
    Accept,
    Cancel,
    Left,
    Right,
    Up,
    Down,
}
