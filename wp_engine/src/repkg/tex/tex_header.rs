use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

use super::tex_flags::TexFlags;
use super::tex_format::TexFormat;
use crate::repkg::byteorder_ext::WPReadBytesExt;

#[derive(Debug)]
pub struct TexHeader {
    pub format: TexFormat,
    pub flags: TexFlags,
    pub tex_width: i32,
    pub tex_height: i32,
    pub img_width: i32,
    pub img_height: i32,
    pub unk1: u32,
}

impl TexHeader {
    pub fn read_from(reader: &mut BufReader<File>) -> Result<TexHeader> {
        let format = reader.wp_read_u32()?;
        let flags = reader.wp_read_u32()?;
        let tex_width = reader.wp_read_i32()?;
        let tex_height = reader.wp_read_i32()?;
        let img_width = reader.wp_read_i32()?;
        let img_height = reader.wp_read_i32()?;
        let unk1 = reader.wp_read_u32()?;

        Ok(TexHeader {
            format: TexFormat::wp_try_from(format)?,
            flags: TexFlags::from_bits_retain(flags),
            tex_width,
            tex_height,
            img_width,
            img_height,
            unk1,
        })
    }
}
