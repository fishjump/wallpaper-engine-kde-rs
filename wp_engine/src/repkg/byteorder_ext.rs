use std::io::{Seek, SeekFrom};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::wp_error;

pub trait WPReadBytesExt {
    fn wp_read_i32(&mut self) -> Result<i32>;
    fn wp_read_u32(&mut self) -> Result<u32>;
    fn wp_read_f32(&mut self) -> Result<f32>;
    fn wp_read_string(&mut self, size: usize) -> Result<String>;
    fn wp_read_string_dyn(&mut self) -> Result<String>;
    fn wp_read_data(&mut self, data: &mut Vec<u8>, size: usize) -> Result<()>;

    fn wp_stream_position(&mut self) -> Result<u64>;
    fn wp_seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

impl<T: ReadBytesExt + Seek> WPReadBytesExt for T {
    fn wp_stream_position(&mut self) -> Result<u64> {
        let pos = self.stream_position();
        if let Err(err) = pos {
            return wp_error!(
                RepkgGenericError,
                format!("stream_position failed, message: {}", err)
            );
        }

        Ok(pos.unwrap())
    }

    fn wp_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let pos = self.seek(pos);
        if let Err(err) = pos {
            return wp_error!(RepkgGenericError, format!("seek failed, message: {}", err));
        }

        Ok(pos.unwrap())
    }

    fn wp_read_i32(&mut self) -> Result<i32> {
        let data = self.read_i32::<LittleEndian>();
        if let Err(err) = data {
            return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
        }

        Ok(data.unwrap())
    }

    fn wp_read_u32(&mut self) -> Result<u32> {
        let data = self.read_u32::<LittleEndian>();
        if let Err(err) = data {
            return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
        }

        Ok(data.unwrap())
    }

    fn wp_read_f32(&mut self) -> Result<f32> {
        let data = self.read_f32::<LittleEndian>();
        if let Err(err) = data {
            return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
        }

        Ok(data.unwrap())
    }

    fn wp_read_string(&mut self, size: usize) -> Result<String> {
        let mut buf = vec![0u8; size];
        let res = self.read(buf.as_mut_slice());
        if let Err(err) = res {
            return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
        }

        let len = res.unwrap();
        if len != size {
            return wp_error!(RepkgReadSizeMismatchError, size, len);
        }

        let str = String::from_utf8(buf);
        if let Err(err) = str {
            return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
        }

        Ok(str.unwrap())
    }

    fn wp_read_string_dyn(&mut self) -> Result<String> {
        let mut buf = String::new();

        let res = self.read_u8();
        if let Err(err) = res {
            return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
        }

        let mut ch = res.unwrap() as char;
        while ch != '\0' {
            buf.push(ch);

            let res = self.read_u8();
            if let Err(err) = res {
                return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
            }

            ch = res.unwrap() as char;
        }

        Ok(buf)
    }

    fn wp_read_data(&mut self, data: &mut Vec<u8>, size: usize) -> Result<()> {
        let res = self.read_exact(data.as_mut_slice());
        if let Err(err) = res {
            return wp_error!(RepkgGenericError, format!("read failed, message: {}", err));
        }

        Ok(())
    }
}
