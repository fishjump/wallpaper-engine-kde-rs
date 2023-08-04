use std::fs::File;
use std::io::BufReader;

use super::tex_header::TexHeader;
use super::tex_image_container::TexImageContainer;
use crate::error::WPEngineError;
use crate::repkg::byteorder_ext::WPReadBytesExt;

#[derive(Debug)]
pub struct Tex {
    pub magic1: String, // always "TEXV0005"
    pub magic2: String, // always "TEXI0001"
    pub header: TexHeader,
    pub image_container: TexImageContainer,
    // pub frame_info_container: TexFrameInfoContainer,
}

impl Tex {
    pub fn read_from(reader: &mut BufReader<File>) -> Result<Tex, WPEngineError> {
        let magic1 = reader.wp_read_string_dyn()?;
        if magic1 != "TEXV0005" {
            return Err(WPEngineError::RepkgInvalidTexMagic1(format!(
                "invalid magic1: {}, expect {}",
                magic1, "TEXV0005"
            )));
        }

        let magic2 = reader.wp_read_string_dyn()?;
        if magic2 != "TEXI0001" {
            return Err(WPEngineError::RepkgInvalidTexMagic2(format!(
                "invalid magic2: {}, expect {}",
                magic2, "TEXI0001"
            )));
        }

        let header = TexHeader::read_from(reader)?;
        let image_container = TexImageContainer::read_from(reader, header.format)?;
        // let frame_info_container = if header.flags.contains(TexFlags::IS_GIF) {
        //     TexFrameInfoContainer::read_from(reader)?
        // } else {
        //     TexFrameInfoContainer::default()
        // };

        Ok(Tex {
            magic1,
            magic2,
            header,
            image_container,
            // frame_info_container,
        })
    }
}
