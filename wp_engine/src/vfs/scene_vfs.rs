use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};

use super::scene_file::{SceneFile, SceneFileContent};
use super::scene_path::ScenePath;
use super::scene_vfs_error::SceneVFSError;

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

    pub fn link(&self, from: &str, to: &str) -> Result<(), SceneVFSError> {
        let from = ScenePath::new(from)?;
        let to = ScenePath::new(to)?;
        let res = self.files.write().unwrap().insert(
            to,
            SceneFile {
                content: SceneFileContent::Absent(from),
            },
        );

        Ok(())
    }

    fn fetch(&self, path: &str) -> Result<Arc<Vec<u8>>, SceneVFSError> {
        let path = ScenePath::new(path)?;

        let mut map = self.files.write().unwrap();
        let file = map.get_mut(&path);
        if file.is_none() {
            return Err(SceneVFSError::FileNotFoundError(format!(
                "fetching a non-linked on-disk file, i don't know where it is on the disk, path(vfs): {}",
                path
            )));
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
                    return Err(SceneVFSError::UpstreamError(format!(
                        "error from fs::read_to_string, from(phy): {}, to(vfs): {}, message: {}",
                        from, path, err
                    )))
                }
            }
        });

        let from = match &file.content {
            SceneFileContent::Absent(from) => from,
            SceneFileContent::Present(c) => {
                return Ok(c.clone());
            }
        };

        match &file.content {
            SceneFileContent::Present(c) => Ok(c.clone()),
            SceneFileContent::Absent(_) => Err(SceneVFSError::FetchFileError(format!(
                "fetch file from disk failed, from(phy): {}, to(vfs): {}",
                from, path
            ))),
        }
    }

    pub fn exists(&self, path: &str) -> Result<bool, SceneVFSError> {
        let path = ScenePath::new(path)?;

        Ok(self.files.read().unwrap().contains_key(&path))
    }

    pub fn read(&self, path: &str) -> Result<Arc<Vec<u8>>, SceneVFSError> {
        let path = ScenePath::new(path)?;

        // read lock
        {
            let map = self.files.read().unwrap();
            let file = map.get(&path);
            if file.is_none() {
                return Err(SceneVFSError::FileNotFoundError(format!(
                    "file doesn't exist, path: {}",
                    path
                )));
            }

            let file = file.unwrap();
            match &file.content {
                SceneFileContent::Present(c) => {
                    return Ok(c.clone());
                }
                SceneFileContent::Absent(_) => (),
            }
        }

        // fetch will acquire a write lock by itself
        self.fetch(path.to_string().as_str())
    }

    pub fn write(&self, path: &str, content: &str) -> Result<(), SceneVFSError> {
        let path = ScenePath::new(path)?;

        self.files.write().unwrap().insert(
            path,
            SceneFile {
                content: SceneFileContent::from(String::from(content)),
            },
        );

        Ok(())
    }
}
