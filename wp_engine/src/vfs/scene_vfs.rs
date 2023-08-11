use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};

use anyhow::Result;
use walkdir::WalkDir;

use super::scene_file::{SceneFile, SceneFileContent};
use super::scene_path::ScenePath;
use crate::error::WPEngineError;
use crate::wp_result;

#[derive(Debug)]
pub struct SceneVFS {
    files: Arc<RwLock<HashMap<ScenePath, SceneFile>>>,
}

impl SceneVFS {
    pub fn new() -> Self {
        Self {
            files: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn mount(&self, to: &str, from: &str) {
        let from = ScenePath::new(from);
        let to = ScenePath::new(to);

        self.files.write().unwrap().insert(
            to,
            SceneFile {
                content: SceneFileContent::Absent(from),
            },
        );
    }

    pub fn mount_dir(&self, to: &str, from: &str) -> Result<()> {
        let dir = WalkDir::new(from);

        for entry in dir {
            if let Err(err) = entry {
                return wp_result!(VfsDirEntryError, err.to_string());
            }

            let entry = entry.unwrap();
            if entry.file_type().is_dir() {
                continue;
            }

            let relative = entry.path().strip_prefix(from);
            if let Err(err) = relative {
                return wp_result!(VfsStripPrefixError, err.to_string());
            }

            let relative = relative.unwrap();
            let to = format!("{}/{}", to, relative.to_str().unwrap());
            let from = entry.path().to_str();
            if let None = from {
                return wp_result!(
                    VfsPathToStrError,
                    entry.path().to_path_buf()
                );
            }

            self.mount(to.as_str(), from.unwrap());
        }

        Ok(())
    }

    fn fetch(&self, path: &str) -> Result<Arc<Vec<u8>>> {
        let path = ScenePath::new(path);

        let mut map = self.files.write().unwrap();
        let file = map.get_mut(&path);
        if file.is_none() {
            return wp_result!(VfsFileNotFoundError, path.to_string());
        }

        let file = file.unwrap();

        file.content = SceneFileContent::from({
            let from = match &file.content {
                SceneFileContent::Absent(from) => from,
                SceneFileContent::Present(c) => {
                    return Ok(c.clone());
                }
            };

            let content = fs::read_to_string(from.to_string());
            match content {
                Ok(c) => c,
                Err(err) => {
                    return wp_result!(
                        VfsReadToStringError,
                        from.to_string(),
                        err.to_string()
                    );
                }
            }
        });

        match &file.content {
            SceneFileContent::Present(c) => Ok(c.clone()),
            SceneFileContent::Absent(_) => {
                unreachable!("data should be present now")
            }
        }
    }

    pub fn exists(&self, path: &str) -> bool {
        let path = ScenePath::new(path);

        self.files.read().unwrap().contains_key(&path)
    }

    pub fn read(&self, path: &str) -> Result<Arc<Vec<u8>>> {
        // read lock, read if file exists in memory
        // otherwise, fetch it
        {
            let path = ScenePath::new(path);
            let map = self.files.read().unwrap();
            let file = map.get(&path);
            if file.is_none() {
                return wp_result!(VfsFileNotFoundError, path.to_string());
            }

            let file = file.unwrap();
            match &file.content {
                SceneFileContent::Present(c) => {
                    return Ok(c.clone());
                }
                SceneFileContent::Absent(_) => (),
            };
        }

        // fetch will acquire a write lock by itself
        self.fetch(path)
    }

    pub fn write(&self, path: &str, content: &str) {
        let path = ScenePath::new(path);

        self.files.write().unwrap().insert(
            path,
            SceneFile {
                content: SceneFileContent::from(String::from(content)),
            },
        );
    }
}
