use anyhow::{Ok, Result};

use crate::error::WPEngineError;

pub struct ShaderPreprocessor {}

impl ShaderPreprocessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn preprocess(&self, content: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use thiserror::Error;
}
