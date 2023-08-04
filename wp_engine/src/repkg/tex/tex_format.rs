use num_enum::TryFromPrimitive;

use crate::error::WPEngineError;

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
    pub fn wp_try_from(value: u32) -> Result<Self, WPEngineError> {
        let value = value as u8;
        let format = Self::try_from(value);
        if let Err(_) = format {
            return Err(WPEngineError::RepkgInvalidTexFormat(format!(
                "invalid tex format: {}, expect one of [RGBA8888(0), DXT5(4), DXT3(6), DXT1(7), RG88(8), R8(9))]",
                value
            )));
        }

        Ok(format.unwrap())
    }
}
