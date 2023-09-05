use std::sync::Arc;

use super::scene_path::ScenePath;

#[derive(Debug)]
pub struct SceneFile {
    #[allow(clippy::rc_buffer)]
    // to allow accessing the same object as writable
    pub content: SceneFileContent,
}

#[derive(Debug)]
pub enum SceneFileContent {
    Absent(ScenePath),
    Present(Arc<Vec<u8>>),
}

impl SceneFileContent {
    pub fn empty() -> Self {
        SceneFileContent::Present(Arc::new(Vec::new()))
    }
}

impl From<String> for SceneFileContent {
    fn from(str: String) -> Self {
        SceneFileContent::Present(Arc::new(Vec::from(str.as_bytes())))
    }
}

impl Default for SceneFileContent {
    fn default() -> Self {
        SceneFileContent::empty()
    }
}
