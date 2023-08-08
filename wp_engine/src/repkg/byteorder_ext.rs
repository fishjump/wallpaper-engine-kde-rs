use std::io::{Seek, SeekFrom};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::{wp_error, wp_result};

pub trait WPReadBytesExt {
    fn wp_read_i32(&mut self) -> Result<i32>;
    fn wp_read_u32(&mut self) -> Result<u32>;
    fn wp_read_f32(&mut self) -> Result<f32>;
    fn wp_read_string(&mut self, size: usize) -> Result<String>;
    fn wp_read_string_dyn(&mut self) -> Result<String>;
    fn wp_read_data(&mut self, data: &mut Vec<u8>) -> Result<()>;

    fn wp_stream_position(&mut self) -> Result<u64>;
    fn wp_seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

impl<T: ReadBytesExt + Seek> WPReadBytesExt for T {
    fn wp_stream_position(&mut self) -> Result<u64> {
        self.stream_position().map_err(|err| {
            wp_error!(
                RepkgGenericError,
                format!("stream_position failed, message: {}", err)
            )
        })
    }

    fn wp_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.seek(pos).map_err(|err| {
            wp_error!(
                RepkgGenericError,
                format!("seek failed, message: {}", err)
            )
        })
    }

    fn wp_read_i32(&mut self) -> Result<i32> {
        self.read_i32::<LittleEndian>().map_err(|err| {
            wp_error!(
                RepkgGenericError,
                format!("read failed, message: {}", err)
            )
        })
    }

    fn wp_read_u32(&mut self) -> Result<u32> {
        self.read_u32::<LittleEndian>().map_err(|err| {
            wp_error!(
                RepkgGenericError,
                format!("read failed, message: {}", err)
            )
        })
    }

    fn wp_read_f32(&mut self) -> Result<f32> {
        self.read_f32::<LittleEndian>().map_err(|err| {
            wp_error!(
                RepkgGenericError,
                format!("read failed, message: {}", err)
            )
        })
    }

    fn wp_read_string(&mut self, size: usize) -> Result<String> {
        let mut buf = vec![0u8; size];
        let res = self.read(buf.as_mut_slice());
        if let Err(err) = res {
            return wp_result!(
                RepkgGenericError,
                format!("read failed, message: {}", err)
            );
        }

        let len = res.unwrap();
        if len != size {
            return wp_result!(RepkgReadSizeMismatchError, size, len);
        }

        String::from_utf8(buf).map_err(|err| {
            wp_error!(
                RepkgGenericError,
                format!("read failed, message: {}", err)
            )
        })
    }

    fn wp_read_string_dyn(&mut self) -> Result<String> {
        let mut buf = String::new();

        let res = self.read_u8();
        if let Err(err) = res {
            return wp_result!(
                RepkgGenericError,
                format!("read failed, message: {}", err)
            );
        }

        let mut ch = res.unwrap() as char;
        while ch != '\0' {
            buf.push(ch);

            let res = self.read_u8();
            match res {
                Ok(data) => ch = data as char,
                Err(err) => {
                    return wp_result!(
                        RepkgGenericError,
                        format!("read failed, message: {}", err)
                    )
                }
            }
        }

        Ok(buf)
    }

    fn wp_read_data(&mut self, data: &mut Vec<u8>) -> Result<()> {
        self.read_exact(data.as_mut_slice()).map_err(|err| {
            wp_error!(
                RepkgGenericError,
                format!("read failed, message: {}", err)
            )
        })
    }
}
