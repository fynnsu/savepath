use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::utils;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    state: Vec<Entry>,
}

impl Config {
    /// Returns a reference to the entry at the given index
    ///
    /// # Arguments
    /// index: usize - the index of the entry to return
    ///
    /// # Returns
    /// Option<&Entry> - the entry at the given index
    pub fn get(&self, index: usize) -> Option<&Entry> {
        self.state.get(index)
    }

    /// Create a Vec<Entry> from the given files
    ///
    /// # Arguments
    /// cur_dir: PathBuf - the current directory
    /// files: Vec<PathBuf> - the files to use
    ///
    /// # Returns
    /// anyhow::Result<Vec<Entry>> - the created Vec<Entry> object
    fn create_vec(cur_dir: PathBuf, files: Vec<PathBuf>) -> anyhow::Result<Vec<Entry>> {
        files
            .into_iter()
            .map(|x| Entry::build(&cur_dir, x))
            .collect::<anyhow::Result<Vec<Entry>>>()
    }

    /// Returns the number of entries in the config state
    pub fn len(&self) -> usize {
        self.state.len()
    }

    /// Returns true if the config state is empty
    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    /// Returns an iterator over the entries in the config state
    pub fn iter(&self) -> std::slice::Iter<Entry> {
        self.state.iter()
    }

    /// Adds the given files to the current config state and save to disk
    ///
    /// # Arguments
    /// self: &mut Config - the current config state
    /// cur_dir: PathBuf - the current working directory
    /// files: Vec<PathBuf> - the files to add to the config state
    ///
    /// # Returns
    /// Result<()> - the result of the operation
    pub fn extend(&mut self, cur_dir: PathBuf, files: Vec<PathBuf>) -> anyhow::Result<()> {
        let mut v = Config::create_vec(cur_dir, files)?;
        v.append(&mut self.state); // new entries first
        self.state = v;
        self.save()
    }

    pub fn empty() -> Self {
        Config { state: vec![] }
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.state.clear();
        self.save()
    }

    fn save(&self) -> anyhow::Result<()> {
        let path = utils::get_config_path()?;
        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?
        };
        let mut file = File::create(path)?;
        let data = ron::to_string(self)?;
        let data = data.as_bytes();
        let _ = file.write(data)?;

        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = utils::get_config_path()?;
        if !path.exists() {
            Self::empty().save()?;
        }

        let data = fs::read_to_string(path)?;
        let config: Self = ron::from_str(&data)?;
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq)]
pub struct Entry {
    path: PathBuf,
}

impl Entry {
    /// Create a new Entry from the given path and filename
    ///
    /// # Arguments
    /// path: PathBuf - the path to the file
    /// filename: PathBuf - the filename of the file
    ///
    /// # Returns
    /// Result<Entry> - the created Entry object
    pub fn build(path: &Path, filename: PathBuf) -> anyhow::Result<Self> {
        let mut path: PathBuf = path.to_path_buf();
        path = path.join(filename);
        path = path.canonicalize().context(format!(
            "Path could not be canonicalized into a valid path: {}",
            path.display() // Pre-canonicalized path
        ))?;

        Ok(Entry { path })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
