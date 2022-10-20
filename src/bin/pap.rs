use savepath::parse::pap::{self};
use savepath::state::Config;
use savepath::utils;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use event::Event::Key;

fn main() -> anyhow::Result<()> {
    let mut ext_cmd = pap::parse();
    let config = Config::load()?;

    let nfiles = config.len();

    let mut cmd_str = savepath::create_modified_cmd(&config, &ext_cmd)?;

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
                if ext_cmd.cur_pos == 0 {
                    ext_cmd.cur_pos = ext_cmd.nargs;
                } else {
                    ext_cmd.cur_pos -= 1;
                }
                cmd_str = savepath::create_modified_cmd(&config, &ext_cmd)?;
                utils::write_command(&cmd_str, true)?;
                choice = get_selection();
            }
            Selection::Right => {
                ext_cmd.cur_pos = (ext_cmd.cur_pos + 1) % (ext_cmd.nargs + 1);
                cmd_str = savepath::create_modified_cmd(&config, &ext_cmd)?;
                utils::write_command(&cmd_str, true)?;
                choice = get_selection();
            }
            Selection::Up => {
                ext_cmd.id = (ext_cmd.id + 1) % nfiles;
                cmd_str = savepath::create_modified_cmd(&config, &ext_cmd)?;
                utils::write_command(&cmd_str, true)?;
                choice = get_selection();
            }
            Selection::Down => {
                if ext_cmd.id == 0 {
                    ext_cmd.id = nfiles - 1;
                } else {
                    ext_cmd.id -= 1;
                }
                cmd_str = savepath::create_modified_cmd(&config, &ext_cmd)?;
                utils::write_command(&cmd_str, true)?;
                choice = get_selection();
            }
        }
    }
}

fn get_selection() -> Selection {
    loop {
        if let Ok(event) = event::read() {
            // eprintln!("Event: {:?}\r", event);
            match event {
                Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => return Selection::Accept,
                Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => return Selection::Cancel,
                Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => return Selection::Left,
                Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => return Selection::Right,
                Key(KeyEvent {
                    code: KeyCode::Up, ..
                }) => return Selection::Up,
                Key(KeyEvent {
                    code: KeyCode::Down,
                    ..
                }) => return Selection::Down,
                _ => continue,
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
