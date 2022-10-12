use std::env;
use std::path::PathBuf;
use std::process::Command;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use crate::error::{Error, Result};
use crate::state::{Config, Entry};

pub mod error;
pub mod parser;
pub mod state;

// #[derive(Debug)]
// pub enum GetStateError {
//     IOError(io::Error),
//     JSONError(serde_json::Error),
//     InvalidState,
//     InvalidInput,
// }

// impl Error for GetStateError {}
// impl fmt::Display for GetStateError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         //TODO: Improve error messages
//        match self {
//         GetStateError::IOError(x) => write!(f, "IOError({})", x),
//         GetStateError::JSONError(x) => write!(f, "JSONError({})", x),
//         GetStateError::InvalidState => write!(f, "InvalidState"),
//         GetStateError::InvalidInput => write!(f, "InvalidInput"),
//        }
//     }
// }

// impl From<io::Error> for GetStateError {
//     fn from(e: io::Error) -> Self {
//         Self::IOError(e)
//     }
// }

// impl From<serde_json::Error> for GetStateError {
//     fn from(e: serde_json::Error) -> Self {
//         Self::JSONError(e)
//     }
// }

// fn read_state(state_file: &str) -> Result<Vec<state::Entry>, GetStateError> {
//     let mut file = match File::open(state_file) {
//         Ok(f) => f,
//         Err(e) => {
//             if e.kind() == io::ErrorKind::NotFound {
//                 let mut f = File::create(state_file)?;
//                 f.write("[]".as_bytes())?;
//                 drop(f);
//                 File::open(state_file)?
//             } else {
//                 return Err(GetStateError::from(e));
//             }
//         }
//     };

//     let mut content = String::new();
//     file.read_to_string(&mut content)?;

//     let state: Vec<state::Entry> = serde_json::from_str(&content)?;

//     // println!("state: {:#?}", state);

//     Ok(state)
// }

// pub fn perform_cp() {
//     let output = Command::new("cp")
//         .arg("test")
//         .arg("test2")
//         .output()
//         .unwrap();

//     println!("Status: {:#?}", output.status);
// }

pub fn run_ext(id: parser::Id, mut cmd: Vec<String>) -> Result<()> {
    let config = Config::load()?;

    let id_path = get_path(config, &id)?;

    let output = Command::new(cmd.get(0).ok_or(Error::IndexError)?)
        .arg(id_path)
        .args(&mut cmd[1..])
        .output()?;

    println!("{:?}", output.stdout);
    eprintln!("{:?}", output.stderr);

    if !output.status.success() {
        return Err(Error::ExtCmdFailed(output.status));
    }

    Ok(())
}

// pub fn write_state(state_file: &str, state: Vec<state::Entry>) -> Result<(), io::Error> {
//     let mut file = File::create(state_file)?;

//     let data = serde_json::to_string(&state)?;

//     file.write(data.as_bytes())?;

//     Ok(())
// }

// pub fn read_state_f(state_file: &str) -> Vec<state::Entry> {
//     read_state(state_file).expect("State file should be readible and contain a valid state")
// }

pub fn list() -> Result<()> {
    let config = Config::load()?;

    println!("Clipboard:\n");

    let mut table = Table::new();

    table.set_titles(row!["Id", "Path", "Name"]);

    for (i, v) in config.state.iter().enumerate() {
        let Entry { path, filename: _ } = v;
        let fname = v
            .filename()
            .ok_or(Error::BadString)?
            .to_str()
            .ok_or(Error::BadString)?;
        table.add_row(row![i, path.to_string_lossy(), fname]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.printstd();

    Ok(())
}

fn get_path(config: Config, id: &parser::Id) -> Result<PathBuf> {
    let x = config.get(id.0)?;
    Ok(x.full_path())
}

pub fn add(files: Vec<PathBuf>) -> Result<()> {
    let cur_dir = env::current_dir()?;

    let mut config = Config::load()?;
    config.extend(cur_dir, files);
    config.save()?;
    Ok(())
}

pub fn clear() -> Result<()> {
    Config::empty().save()?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use std::{
//         fs::{self, File},
//         io::Write,
//     };

//     use super::*;

//     #[test]
//     fn run_cp() {
//         perform_cp();
//     }

//     #[test]
//     fn read_state() {
//         let filename = "/tmp/test_read_state.json";
//         let content = "[\"test\", \"two\"]";

//         let mut file = File::create(filename).unwrap();

//         file.write(content.as_bytes()).unwrap();

//         let state = crate::read_state(filename).unwrap();

//         let correct = vec!["test", "two"];

//         // assert_eq!(state, correct);
//     }

//     #[test]
//     fn read_non_existent_file() {
//         let filename = "/tmp/test_read_non_existent_file.json";

//         let _ = fs::remove_file(filename);

//         let state = crate::read_state(filename).unwrap();

//         let s1: Vec<String> = vec![];

//         // assert_eq!(s1, state);
//     }

//     #[test]
//     fn write_and_read_state() {
//         let filename = "/tmp/test_write_and_read_state.json";
//         let s1: Vec<String> = vec!["one", "two", "three"]
//             .iter()
//             .map(|x| String::from(*x))
//             .collect();

//         // crate::write_state(filename, s1.clone()).unwrap();

//         let s2 = crate::read_state(filename).unwrap();

//         // assert_eq!(s1, s2);
//     }
// }
