use std::fs::File;
use std::io::BufReader;

use super::constant;
use super::free_image_format::FreeImageFormat;
use super::mipmap_format::MipmapFormat;
use super::tex_format::TexFormat;
use super::tex_image_container::TexImageContainerVersion;
use super::tex_mipmap::TexMipmap;
use crate::error::WPEngineError;
use crate::repkg::byteorder_ext::WPReadBytesExt;

#[derive(Debug)]
pub struct TexImage {
    pub mipmaps: Vec<TexMipmap>,
}

impl TexImage {
    pub fn read_from(
        reader: &mut BufReader<File>,
        tex_format: TexFormat,
        image_format: FreeImageFormat,
        version: TexImageContainerVersion,
    ) -> Result<TexImage, WPEngineError> {
        let mipmap_count = reader.wp_read_i32()?;
        if mipmap_count > constant::MAXIMUM_MIPMAP_COUNT {
            return Err(WPEngineError::RepkgTooManyTexMipmapsError(format!(
                "too many mipmaps: {}, expect less than {}",
                mipmap_count,
                constant::MAXIMUM_MIPMAP_COUNT
            )));
        }

        let format = MipmapFormat::from(tex_format, image_format)?;

        let mut mipmaps = Vec::new();
        mipmaps.reserve_exact(mipmap_count as usize);
        for _ in 0..mipmap_count {
            let mipmap = match version {
                TexImageContainerVersion::Version1 => TexMipmap::read_from_v1(reader, format)?,
                TexImageContainerVersion::Version2 | TexImageContainerVersion::Version3 => {
                    TexMipmap::read_from_v2v3(reader, format)?
                }
            };
            mipmaps.push(mipmap);
        }

        Ok(TexImage { mipmaps })
    }
}
