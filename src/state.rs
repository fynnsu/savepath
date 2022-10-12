use prettytable::{format, Table};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Single(SubEntry),
    Set(Vec<SubEntry>),
}

impl Entry {
    pub fn new(dir: PathBuf, files: Vec<String>) -> Self {
        let mut v: Vec<SubEntry> = files.iter().map(|x| SubEntry::new(&dir, x)).collect();

        if v.len() == 1 {
            Self::Single(v.remove(0))
        } else {
            Self::Set(v)
        }
    }

    pub fn get_path(&self) -> PathBuf {
        match self {
            Entry::Single(s) => s.path.clone(),
            Entry::Set(set) => set.iter().next().unwrap().path.clone(),
        }
    }

    // TODO: Remove
    // pub fn get_name_str(&self) -> String {
    //     match self {
    //         Entry::Single(s) => s.filename.clone(),
    //         Entry::Set(set) => set
    //             .iter()
    //             .enumerate()
    //             .map(|(i, x)| format!(".{}: {}", i, x.filename))
    //             .collect::<Vec<String>>()
    //             .join(", "),
    //     }
    // }

    pub fn get_table(&self) -> Table {
        let mut t = Table::new();
        match self {
            Entry::Single(s) => {
                let _ = t.add_row(row![&s.filename]);
            }
            Entry::Set(set) => set.iter().enumerate().for_each(|(i, x)| {
                let _ = t.add_row(row![format!(".{}", i), x.filename]);
            }),
        };
        t.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        t
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entry::Single(s) => fmt.write_str(&format!("{:#?}\t{}", s.path, s.filename))?,
            Entry::Set(set) => {
                fmt.write_str(&format!("{:#?}\t{{", set.iter().next().unwrap().path))?;
                for (i, s) in set.iter().enumerate() {
                    fmt.write_str(&format!(" .{}: {},", i, s.filename))?;
                }
                fmt.write_str("}}")?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubEntry {
    path: PathBuf,
    filename: String,
}

impl SubEntry {
    pub fn new(path: &PathBuf, filename: &str) -> Self {
        SubEntry {
            path: PathBuf::from(path),
            filename: String::from(filename),
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn full_path(&self) -> PathBuf {
        let mut p = self.path.clone(); 
        p.push(&self.filename);
        p
    }
}

// State will be a Vec<Entry>

#[cfg(test)]
mod state_tests {

    use super::*;

    #[test]
    fn test_serialize() {
        let x = vec![
            Entry::Single(SubEntry {
                path: PathBuf::new(),
                filename: String::from("test.json"),
            }),
            Entry::Set(vec![
                SubEntry {
                    path: PathBuf::new(),
                    filename: String::from("1"),
                },
                SubEntry {
                    path: PathBuf::new(),
                    filename: String::from("2"),
                },
            ]),
        ];

        let s = serde_json::to_string(&x).unwrap();
        println!("{}", s);

        let r: Vec<Entry> = serde_json::from_str(&s).unwrap();
        println!("{:#?}", r);
    }
}
