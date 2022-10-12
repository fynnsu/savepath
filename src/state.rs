use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{fmt, fs};

use crate::error::{Error, Result};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "fynnsu";
const APPLICATION: &str = "clipboard";
const CONFIG_FILE: &str = "config.ron";

fn get_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let mut path = PathBuf::from(proj_dirs.cache_dir());
        path.push(CONFIG_FILE);
        // println!("Path: {:#?}", path);
        Ok(path)
    } else {
        Err(Error::ApplicationDirNotAccessible)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub state: Vec<Entry>,
}

impl Config {
    pub fn new(cur_dir: PathBuf, files: Vec<PathBuf>) -> Self {
        let v: Vec<Entry> = files
            .iter()
            .map(|x| Entry::new(&cur_dir, x.to_path_buf()))
            .collect();

        Config { state: v }
    }

    pub fn get(&self, index: isize) -> Result<&Entry> {
        if index >= 0 {
            return self.state.get(index as usize).ok_or(Error::IndexError);
        }

        let index = self.state.len() as isize + index;

        if index < 0 {
            return Err(Error::IndexError);
        }

        return self.state.get(index as usize).ok_or(Error::IndexError);
    }

    pub fn extend(&mut self, cur_dir: PathBuf, files: Vec<PathBuf>) {
        let mut v: Vec<Entry> = files
            .iter()
            .map(|x| Entry::new(&cur_dir, x.to_path_buf()))
            .collect();
        v.append(&mut self.state); // new entries first
        self.state = v
    }

    pub fn empty() -> Self {
        Config { state: vec![] }
    }

    pub fn save(&self) -> Result<()> {
        let path = get_path()?;
        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?
        };
        let mut file = File::create(path)?;
        let data = ron::to_string(self)?;
        let data = data.as_bytes();
        file.write(data)?;

        Ok(())
    }

    pub fn load() -> Result<Self> {
        let path = get_path()?;
        if !path.exists() {
            Self::empty().save()?;
        }

        let data = fs::read_to_string(path)?;
        let config: Self = ron::from_str(&data)?;
        Ok(config)
    }

    // pub fn get_table(&self) -> Table {
    //     let mut t = Table::new();
    //     match self {
    //         Entry::Single(s) => {
    //             let _ = t.add_row(row![&s.filename]);
    //         }
    //         Entry::Set(set) => set.iter().enumerate().for_each(|(i, x)| {
    //             let _ = t.add_row(row![format!(".{}", i), x.filename]);
    //         }),
    //     };
    //     t.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    //     t
    // }
}

impl fmt::Display for Entry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!(
            "{:#?}\t{:#?}",
            self.path,
            self.filename.clone().unwrap_or_default()
        ))?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub filename: Option<OsString>,
}

impl Entry {
    pub fn new(path: &PathBuf, filename: PathBuf) -> Self {
        let name = filename.file_name();
        let mut fname = filename.clone();
        let path = if fname.pop() {
            path.clone().join(fname.as_path())
        } else {
            path.clone()
        };

        Entry {
            path,
            filename: name.map(|x| OsString::from(x)),
        }
    }

    pub fn filename(&self) -> Option<&OsStr> {
        match &self.filename {
            Some(x) => Some(&x[..]),
            None => None,
        }
    }

    pub fn full_path(&self) -> PathBuf {
        let mut p = self.path.clone();
        if self.filename.is_some() {
            p.push(self.filename().unwrap())
        }
        p
    }
}
