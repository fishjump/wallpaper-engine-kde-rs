use thiserror::Error;

#[derive(Error, Debug)]
pub enum WPEngineError {
    // Vfs errors
    #[error("VfsFileNotFoundError, reason: {0}")]
    VfsFileNotFoundError(String),

    #[error("VfsMalformPathError, reason: {0}")]
    VfsMalformPathError(String),

    #[error("VfsFetchFileError, reason: {0}")]
    VfsFetchFileError(String),

    #[error("VfsUpstreamError, reason: {0}")]
    VfsUpstreamError(String),

    // Repkg errors
    #[error("RepkgGenericError, reason: {0}")]
    RepkgGenericError(String),

    // byteorder_ext errors
    #[error("RepkgReadSizeMismatchError, reason: {0}")]
    RepkgReadSizeMismatchError(String),

    // invalid magic
    #[error("RepkgInvalidPakMagic, reason: {0}")]
    RepkgInvalidPakMagicError(String),

    #[error("RepkgReadDataError, reason: {0}")]
    RepkgReadDataError(String),

    #[error("RepkgInvalidTexMagic1, reason: {0}")]
    RepkgInvalidTexMagic1(String),

    #[error("RepkgInvalidTexMagic2, reason: {0}")]
    RepkgInvalidTexMagic2(String),

    #[error("RepkgInvalidTexFormat, reason: {0}")]
    RepkgInvalidTexFormat(String),

    #[error("RepkgInvalidTexFrameMagic, reason: {0}")]
    RepkgInvalidTexFrameMagic(String),

    #[error("RepkgInvalidTexFrameInfoError, reason: {0}")]
    RepkgInvalidTexFrameInfoError(String),

    #[error("RepkgInvalidTexImageContainerVersion, reason: {0}")]
    RepkgInvalidTexImageContainerVersion(String),

    #[error("RepkgInvalidFreeImageFormat, reason: {0}")]
    RepkgInvalidFreeImageFormat(String),

    #[error("RepkgInvalidFreeImageFormatError, reason: {0}")]
    RepkgInvalidFreeImageFormatError(String),

    #[error("RepkgTooManyTexImagesError, reason: {0}")]
    RepkgTooManyTexImagesError(String),

    #[error("RepkgTooManyTexMipmapsError, reason: {0}")]
    RepkgTooManyTexMipmapsError(String),

    #[error("RepkgTooManyTexFramesError, reason: {0}")]
    RepkgTooManyTexFramesError(String),
}
