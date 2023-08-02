use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneVFSError {
    #[error("FileNotFoundError, reason: {0}")]
    FileNotFoundError(String),

    #[error("MalformPathError, reason: {0}")]
    MalformPathError(String),

    #[error("FetchFileError, reason: {0}")]
    FetchFileError(String),

    #[error("UpstreamError, reason: {0}")]
    UpstreamError(String),
}
