use thiserror::Error;

#[derive(Error, Debug)]
pub enum WPEngineError {
    #[error("VfsFileNotFoundError, reason: {0}")]
    VfsFileNotFoundError(String),

    #[error("VfsMalformPathError, reason: {0}")]
    VfsMalformPathError(String),

    #[error("VfsFetchFileError, reason: {0}")]
    VfsFetchFileError(String),

    #[error("VfsUpstreamError, reason: {0}")]
    VfsUpstreamError(String),

    #[error("RepkgUpstreamError, reason: {0}")]
    RepkgPackageLoaderError(String),
}
