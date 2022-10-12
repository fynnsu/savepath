use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
    pub fn build(cur_dir: PathBuf, files: Vec<PathBuf>) -> Result<Self> {
        let v = Config::create_vec(cur_dir, files)?;

        Ok(Config { state: v })
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

    fn create_vec(cur_dir: PathBuf, files: Vec<PathBuf>) -> Result<Vec<Entry>> {
        let v: Vec<Entry> = files
            .into_iter()
            .map(|x| Entry::build(&cur_dir, x.to_path_buf()))
            .collect::<Result<Vec<Entry>>>()?;

        Ok(v)
    }

    pub fn extend(&mut self, cur_dir: PathBuf, files: Vec<PathBuf>) -> Result<()> {
        let mut v = Config::create_vec(cur_dir, files)?;
        v.append(&mut self.state); // new entries first
        self.state = v;
        Ok(())
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
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entry {
    path: PathBuf,
}

impl Entry {
    pub fn build(path: &PathBuf, filename: PathBuf) -> Result<Self> {
        let mut path = path.clone();
        path = path.join(filename);
        path = path.canonicalize()?;

        Ok(Entry { path })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
