use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

use super::constant;
use super::tex_frame_info::TexFrameInfo;
use crate::repkg::byteorder_ext::WPReadBytesExt;
use crate::wp_error;

#[derive(Debug, Default)]
pub struct TexFrameInfoContainer {
    pub magic: String,
    pub frames: Vec<TexFrameInfo>,
    pub gif_width: i32,
    pub gif_height: i32,
}

impl TexFrameInfoContainer {
    pub fn read_from(reader: &mut BufReader<File>) -> Result<TexFrameInfoContainer> {
        let magic = reader.wp_read_string_dyn()?;

        let frame_count = reader.wp_read_i32()?;
        if frame_count > constant::MAXIMUM_FRAME_COUNT {
            return wp_error!(
                RepkgTooManyTexFramesError,
                stringify!(frame_count),
                frame_count,
                stringify!(constant::MAXIMUM_FRAME_COUNT),
                constant::MAXIMUM_FRAME_COUNT
            );
        }

        let (mut gif_width, mut gif_height) = match magic.as_str() {
            "TEXS0001" | "TEXS0002" => (0, 0),
            "TEXS0003" => {
                let width = reader.wp_read_i32()?;
                let height = reader.wp_read_i32()?;
                (width, height)
            }
            _ => {
                return wp_error!(
                    RepkgInvalidTexFrameMagicError,
                    "one of [TEXS0001, TEXS0002, TEXS0003]",
                    magic
                );
            }
        };

        let mut frames = Vec::new();
        frames.reserve_exact(frame_count as usize);
        for _ in 0..frame_count {
            let frame = match magic.as_str() {
                "TEXS0001" => TexFrameInfo::read_from_v1(reader)?,
                "TEXS0002" | "TEXS0003" => TexFrameInfo::read_from_v2v3(reader)?,
                _ => {
                    return wp_error!(
                        RepkgInvalidTexFrameMagicError,
                        "one of [TEXS0001, TEXS0002, TEXS0003]",
                        magic
                    );
                }
            };

            frames.push(frame);
        }

        if gif_width == 0 || gif_height == 0 {
            if let Some(frame) = frames.first() {
                (gif_width, gif_height) = match frame {
                    TexFrameInfo::V1(x) => (x.width, x.height),
                    TexFrameInfo::V2V3(x) => (x.width as i32, x.height as i32),
                };
            } else {
                return wp_error!(RepkgInvalidTexFrameInfoError);
            }
        }

        Ok(TexFrameInfoContainer {
            magic,
            frames,
            gif_width,
            gif_height,
        })
    }
}
