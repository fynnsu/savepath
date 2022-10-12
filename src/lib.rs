use std::ffi::OsStr;
use std::{env, fmt};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;
#[macro_use]
extern crate prettytable;
use parser::Id;
use prettytable::{format, Table};
use state::Entry;

pub mod parser;
mod state;

#[derive(Debug)]
pub enum GetStateError {
    IOError(io::Error),
    JSONError(serde_json::Error),
    InvalidState,
    InvalidInput,
}

impl Error for GetStateError {}
impl fmt::Display for GetStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: Improve error messages
       match self {
        GetStateError::IOError(x) => write!(f, "IOError({})", x),
        GetStateError::JSONError(x) => write!(f, "JSONError({})", x),
        GetStateError::InvalidState => write!(f, "InvalidState"),
        GetStateError::InvalidInput => write!(f, "InvalidInput"),
       }
    }
}

impl From<io::Error> for GetStateError {
    fn from(e: io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<serde_json::Error> for GetStateError {
    fn from(e: serde_json::Error) -> Self {
        Self::JSONError(e)
    }
}

fn read_state(state_file: &str) -> Result<Vec<state::Entry>, GetStateError> {
    let mut file = match File::open(state_file) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                let mut f = File::create(state_file)?;
                f.write("[]".as_bytes())?;
                drop(f);
                File::open(state_file)?
            } else {
                return Err(GetStateError::from(e));
            }
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let state: Vec<state::Entry> = serde_json::from_str(&content)?;

    // println!("state: {:#?}", state);

    Ok(state)
}

pub fn perform_cp() {
    let output = Command::new("cp")
        .arg("test")
        .arg("test2")
        .output()
        .unwrap();

    println!("Status: {:#?}", output.status);
}


pub fn run_ext(state_file: &str, id: parser::Id, mut cmd: Vec<String>) -> Result<(), Box<dyn Error>>{

    let id_path = get_path(state_file, &id)?;

    let output = Command::new(cmd.get(0).ok_or(GetStateError::InvalidInput)?)
        .arg(id_path)
        .args(&mut cmd[1..])
        .output()?;

    println!("{:#?}", output);

    Ok(())
}

pub fn write_state(state_file: &str, state: Vec<state::Entry>) -> Result<(), io::Error> {
    let mut file = File::create(state_file)?;

    let data = serde_json::to_string(&state)?;

    file.write(data.as_bytes())?;

    Ok(())
}

pub fn read_state_f(state_file: &str) -> Vec<state::Entry> {
    read_state(state_file).expect("State file should be readible and contain a valid state")
}

pub fn list(state_file: &str) {
    let state = read_state_f(state_file);

    println!("Clipboard:\n");

    let mut table = Table::new();

    table.set_titles(row!["Id", "Path", "Name"]);

    for (i, v) in state.iter().enumerate() {
        match v {
            state::Entry::Single(s) => {
                table.add_row(row![
                    i,
                    v.get_path().into_os_string().into_string().unwrap(),
                    s.filename(),
                ]);
                ()
            }
            state::Entry::Set(s) => {
                table.add_row(row![
                    i,
                    v.get_path().into_os_string().into_string().unwrap(),
                    s.iter()
                        .map(|x| x.filename())
                        .collect::<Vec<&str>>()
                        .join(", ")
                ]);
                for (j, subentry) in s.iter().enumerate() {
                    table.add_row(row![format!(".{}", j), "", subentry.filename()]);
                }
                ()
            }
        }
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.printstd();
}

fn get_path(state_file: &str, id: &parser::Id) -> Result<PathBuf, Box<dyn Error>>{
    let state = read_state(state_file)?;

    match id {
        Id::Simple { id } => {
            if let Entry::Single(subentry) = state.get(*id).ok_or(GetStateError::InvalidInput)? {
                Ok(subentry.full_path())
            } else {
                // Simple on multi-file entry
                // Do nothing for now
                // TODO: implement multi-file command
                Err(Box::new(GetStateError::InvalidInput))
            }

        }
        Id::Specific { id, sid } => {
            if let Entry::Set(entries) = state.get(*id).ok_or(GetStateError::InvalidInput)? {
                let subentry = entries.get(*sid).ok_or(GetStateError::InvalidInput)?;
                Ok(subentry.full_path())
            } else {
                Err(Box::new(GetStateError::InvalidInput))
            }
        }
        
    }


    // Ok(PathBuf::new())
}

pub fn add(state_file: &str, files: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut cur_dir = env::current_dir()?;
    let mut first = files.first().ok_or(GetStateError::InvalidInput)?.clone();

    if first.pop() {
        // Not local file, but path to file. Add path to cur_dir
        cur_dir = cur_dir.join(first.as_path());
    }

    let fnames: Vec<&OsStr> = files
        .iter()
        .map(|x| x.file_name())
        .collect::<Option<Vec<&OsStr>>>()
        .ok_or(GetStateError::InvalidInput)?;

    let fnames: Vec<String> = fnames
        .into_iter()
        .map(|i| i.to_string_lossy()
        .as_ref()
        .to_owned())
        .collect();

    let mut state = read_state_f(state_file);

    let mut files = vec![state::Entry::new(cur_dir, fnames)];

    files.append(&mut state);

    write_state(state_file, files).expect("Writing updated state failed.");

    Ok(())
}

pub fn clear(state_file: &str) {
    write_state(state_file, Vec::new()).expect("Clearing state failed.");
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
