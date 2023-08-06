use anyhow::Result;

use super::shader_type::ShaderType;
use crate::vfs::scene_vfs::SceneVFS;

#[derive(Debug)]
pub struct Shader {
    pub typ: ShaderType,
    pub content: String,
}

impl Shader {
    pub fn new(typ: ShaderType, content: String) -> Self {
        Self { typ, content }
    }

    pub fn preprocess(&mut self, vfs: &SceneVFS) -> Result<String> {
        let mut lines = self.content.lines().collect::<Vec<&str>>();
        let mut new_lines = Vec::new();

        for line in lines {
            if line.starts_with("#include") {
            } else {
                new_lines.push(String::from(line));
            }
        }

        Ok(new_lines.join("\n"))
    }
}
