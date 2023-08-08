use std::path::PathBuf;

use thiserror::Error;

#[macro_export]
macro_rules! wp_result {
    ($e:ident) => {
        Err(crate::error::WPEngineError::$e(stdext::function_name!(), file!(), line!()).into())
    };
    ($e:ident, $($arg:expr),*) => {
        Err(crate::error::WPEngineError::$e(stdext::function_name!(), file!(), line!(),$($arg),*).into())
    };
}

#[macro_export]
macro_rules! wp_error {
    ($e:ident) => {
       crate::error::WPEngineError::$e(stdext::function_name!(), file!(), line!()).into()
    };
    ($e:ident, $($arg:expr),*) => {
        crate::error::WPEngineError::$e(stdext::function_name!(), file!(), line!(),$($arg),*).into()
    };
}

#[derive(Error, Debug)]
pub enum WPEngineError {
    // Vfs errors
    #[error("[{0}]VfsFileNotFoundError, vfs path: {3}, where: {1}:{2}")]
    VfsFileNotFoundError(&'static str, &'static str, u32, String),

    #[error("[{0}]VfsFileAlreadyExistsError, reason: {3}, where: {1}:{2}")]
    VfsReadToStringError(&'static str, &'static str, u32, String),

    #[error("[{0}]VfsReadToStringError, read from vfs path: {3}, however, the content should be loaded from {4}, but it absences, where: {1}:{2}")]
    VfsSceneFileDataAbsentError(&'static str, &'static str, u32, String, String),

    #[error("[{0}]VfsDirEntryError, reason: {3}, where: {1}:{2}")]
    VfsDirEntryError(&'static str, &'static str, u32, String),

    #[error("[{0}]VfsStripPrefixError, reason: {3}, where: {1}:{2}")]
    VfsStripPrefixError(&'static str, &'static str, u32, String),

    #[error("[{0}]VfsPathToStrError, path: {3:#?}, where: {1}:{2}")]
    VfsPathToStrError(&'static str, &'static str, u32, PathBuf),

    // Repkg errors
    #[error("[{0}]RepkgGenericError, reason: {3}, where: {1}:{2}")]
    RepkgGenericError(&'static str, &'static str, u32, String),

    // byteorder_ext errors
    #[error("[{0}]RepkgReadSizeMismatchError, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgReadSizeMismatchError(&'static str, &'static str, u32, usize, usize),

    // invalid magics
    #[error("[{0}]RepkgInvalidTexMagic1Error, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgInvalidTexMagic1Error(&'static str, &'static str, u32, &'static str, String),

    #[error("[{0}]RepkgInvalidTexMagic2Error, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgInvalidTexMagic2Error(&'static str, &'static str, u32, &'static str, String),

    #[error("[{0}]RepkgInvalidTexImageMagicError, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgInvalidTexImageMagicError(&'static str, &'static str, u32, &'static str, String),

    #[error("[{0}]RepkgInvalidTexFrameMagicError, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgInvalidTexFrameMagicError(&'static str, &'static str, u32, &'static str, String),

    // data inconsistency errors
    #[error("[{0}]RepkgInvalidTexFrameInfoError, expect at least one frame, where: {1}:{2}")]
    RepkgInvalidTexFrameInfoError(&'static str, &'static str, u32),

    // invalid enums
    #[error("[{0}]RepkgInvalidTexFormatError, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgInvalidTexFormatError(&'static str, &'static str, u32, &'static str, u8),

    #[error("[{0}]RepkgInvalidTexImageContainerVersion, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgInvalidTexImageContainerVersion(&'static str, &'static str, u32, &'static str, u8),

    #[error("[{0}]RepkgInvalidFreeImageFormat, expect: {3}, actual: {4}, where: {1}:{2}")]
    RepkgInvalidFreeImageFormat(&'static str, &'static str, u32, &'static str, u8),

    #[error("[{0}]RepkgUnknownMipmapFormatError, where: {1}:{2}")]
    RepkgUnknownMipmapFormatError(&'static str, &'static str, u32),

    // Too many series errors
    #[error("[{0}]RepkgTooManyTexImagesError, expect {3} ({4}) < {5} ({6}), where: {1}:{2}")]
    RepkgTooManyTexImagesError(
        &'static str,
        &'static str,
        u32,
        &'static str,
        i32,
        &'static str,
        i32,
    ),

    #[error("[{0}]RepkgTooManyTexMipmapsError, expect {3} ({4}) < {5} ({6}), where: {1}:{2}")]
    RepkgTooManyTexMipmapsError(
        &'static str,
        &'static str,
        u32,
        &'static str,
        i32,
        &'static str,
        i32,
    ),

    #[error("[{0}]RepkgTooManyTexFramesError, expect {3} ({4}) < {5} ({6}), where: {1}:{2}")]
    RepkgTooManyTexFramesError(
        &'static str,
        &'static str,
        u32,
        &'static str,
        i32,
        &'static str,
        i32,
    ),
}
