use std::fs::File;
use std::io::BufReader;

use crate::error::WPEngineError;
use crate::repkg::byteorder_ext::WPReadBytesExt;

#[derive(Debug)]
pub enum TexFrameInfo {
    V1(TexFrameInfoV1),
    V2V3(TexFrameInfoV2V3),
}

#[derive(Debug)]
pub struct TexFrameInfoV1 {
    pub image_id: i32,
    pub frame_time: f32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub width_y: i32,
    pub height: i32,
    pub height_x: i32,
}

#[derive(Debug)]
pub struct TexFrameInfoV2V3 {
    pub image_id: i32,
    pub frame_time: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub width_y: f32,
    pub height: f32,
    pub height_x: f32,
}

impl TexFrameInfo {
    pub fn read_from_v1(reader: &mut BufReader<File>) -> Result<TexFrameInfo, WPEngineError> {
        Ok(TexFrameInfo::V1(TexFrameInfoV1 {
            image_id: reader.wp_read_i32()?,
            frame_time: reader.wp_read_f32()?,
            x: reader.wp_read_i32()?,
            y: reader.wp_read_i32()?,
            width: reader.wp_read_i32()?,
            width_y: reader.wp_read_i32()?,
            height: reader.wp_read_i32()?,
            height_x: reader.wp_read_i32()?,
        }))
    }

    pub fn read_from_v2v3(reader: &mut BufReader<File>) -> Result<TexFrameInfo, WPEngineError> {
        Ok(TexFrameInfo::V2V3(TexFrameInfoV2V3 {
            image_id: reader.wp_read_i32()?,
            frame_time: reader.wp_read_f32()?,
            x: reader.wp_read_f32()?,
            y: reader.wp_read_f32()?,
            width: reader.wp_read_f32()?,
            width_y: reader.wp_read_f32()?,
            height: reader.wp_read_f32()?,
            height_x: reader.wp_read_f32()?,
        }))
    }
}
