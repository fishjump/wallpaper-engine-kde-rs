use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;

use super::mipmap_format::MipmapFormat;
use crate::error::WPEngineError;
use crate::repkg::byteorder_ext::WPReadBytesExt;

pub struct TexMipmap {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub decompressed_bytes: i32,
    pub is_lz4_compressed: bool,
    pub format: MipmapFormat,
}

impl TexMipmap {
    pub fn read_from_v1(
        reader: &mut BufReader<File>,
        format: MipmapFormat,
    ) -> Result<TexMipmap, WPEngineError> {
        let width = reader.wp_read_i32()?;
        let height = reader.wp_read_i32()?;

        let data_count = reader.wp_read_i32()?;
        let mut data = vec![0u8; data_count as usize];
        reader.wp_read_data(&mut data, data_count as usize)?;

        Ok(TexMipmap {
            data,
            width,
            height,
            decompressed_bytes: 0,    // unused in v1
            is_lz4_compressed: false, // unused in v1
            format,
        })
    }

    pub fn read_from_v2v3(
        reader: &mut BufReader<File>,
        format: MipmapFormat,
    ) -> Result<TexMipmap, WPEngineError> {
        let width = reader.wp_read_i32()?;
        let height = reader.wp_read_i32()?;
        let is_lz4_compressed = reader.wp_read_i32()? == 1;
        let decompressed_bytes = reader.wp_read_i32()?;

        let data_count = reader.wp_read_i32()?;
        let mut data = vec![0u8; data_count as usize];
        reader.wp_read_data(&mut data, data_count as usize)?;

        Ok(TexMipmap {
            data,
            width,
            height,
            decompressed_bytes,
            is_lz4_compressed,
            format,
        })
    }
}

impl Debug for TexMipmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TexMipmap")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("decompressed_bytes", &self.decompressed_bytes)
            .field("is_lz4_compressed", &self.is_lz4_compressed)
            .field("format", &self.format)
            .finish()
    }
}
