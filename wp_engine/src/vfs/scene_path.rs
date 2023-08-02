use std::fmt::Display;

use super::scene_vfs_error::SceneVFSError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ScenePath {
    path: String,
}

impl ScenePath {
    pub fn new(path: &str) -> Result<Self, SceneVFSError> {
        Ok(ScenePath {
            path: Self::simplify(path)?,
        })
    }

    fn simplify(path: &str) -> Result<String, SceneVFSError> {
        let mut stack = Vec::new();
        for part in path.split('/') {
            match part {
                ".." => {
                    if let None = stack.pop() {
                        return Err(SceneVFSError::MalformPathError(format!(
                            "too many parent directories, path: {}",
                            path
                        )));
                    };
                }
                "." => (),
                _ => stack.push(part),
            }
        }

        Ok(stack.join("/"))
    }
}

impl Display for ScenePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}