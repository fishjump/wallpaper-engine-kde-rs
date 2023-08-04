use std::fs::File;
use std::io::BufReader;

use num_enum::TryFromPrimitive;

use super::constant;
use super::free_image_format::FreeImageFormat;
use super::tex_format::TexFormat;
use super::tex_image::TexImage;
use crate::error::WPEngineError;
use crate::repkg::byteorder_ext::WPReadBytesExt;

#[derive(Debug)]
pub struct TexImageContainer {
    pub magic: String,
    pub format: FreeImageFormat,
    pub images: Vec<TexImage>,
    pub version: TexImageContainerVersion,
}

impl TexImageContainer {
    pub fn read_from(
        reader: &mut BufReader<File>,
        tex_format: TexFormat,
    ) -> Result<TexImageContainer, WPEngineError> {
        let magic = reader.wp_read_string_dyn()?;

        let image_count = reader.wp_read_i32()?;
        if image_count > constant::MAXIMUM_IMAGE_COUNT {
            return Err(WPEngineError::RepkgTooManyTexImagesError(format!(
                "too many images: {}, expect less than {}",
                image_count,
                constant::MAXIMUM_IMAGE_COUNT
            )));
        }

        let format = match magic.as_str() {
            "TEXB0001" | "TEXB0002" => FreeImageFormat::FifUnknown,
            "TEXB0003" => FreeImageFormat::wp_try_from(reader.wp_read_u32()?)?,
            _ => {
                return Err(WPEngineError::RepkgInvalidTexMagic2(format!(
                        "invalid tex image container magic: {}, expect one of [TEXB0001, TEXB0002, TEXB0003]",
                        magic
                    )));
            }
        };

        let version = match magic.as_str() {
            "TEXB0001" => TexImageContainerVersion::Version1,
            "TEXB0002" => TexImageContainerVersion::Version2,
            "TEXB0003" => TexImageContainerVersion::Version3,
            _ => {
                return Err(WPEngineError::RepkgInvalidTexMagic2(format!(
                        "invalid tex image container magic: {}, expect one of [TEXB0001, TEXB0002, TEXB0003]",
                        magic
                    )));
            }
        };

        let mut images = Vec::new();
        images.reserve_exact(image_count as usize);
        for _ in 0..image_count {
            let image = TexImage::read_from(reader, tex_format, format, version)?;
            images.push(image);
        }

        Ok(TexImageContainer {
            magic,
            format,
            images,
            version,
        })
    }
}

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum TexImageContainerVersion {
    Version1 = 1,
    Version2 = 2,
    Version3 = 3,
}

impl TexImageContainerVersion {
    fn wp_try_from(value: u32) -> Result<Self, WPEngineError> {
        let value = value as u8;
        match Self::try_from(value) {
            Ok(v) => Ok(v),
            Err(_) => Err(WPEngineError::RepkgInvalidTexImageContainerVersion(
                format!(
                    "invalid version: {}, expect one of [Version1(1), Version2(2), Version3(3)]",
                    value
                ),
            )),
        }
    }
}
