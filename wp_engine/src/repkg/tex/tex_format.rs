use anyhow::Result;
use num_enum::TryFromPrimitive;

use crate::wp_error;

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum TexFormat {
    RGBA8888 = 0,
    DXT5 = 4,
    DXT3 = 6,
    DXT1 = 7,
    RG88 = 8,
    R8 = 9,
}

impl TexFormat {
    pub fn wp_try_from(value: u32) -> Result<Self> {
        let value = value as u8;
        let format = Self::try_from(value);

        match format {
            Ok(x) => Ok(x),
            Err(_) => wp_error!(
                RepkgInvalidTexFormatError,
                "one of [RGBA8888(0), DXT5(4), DXT3(6), DXT1(7), RG88(8), R8(9))]",
                value
            ),
        }
    }
}
