use std::io::{Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::WPEngineError;

pub trait WPReadBytesExt {
    fn wp_read_i32(&mut self) -> Result<i32, WPEngineError>;
    fn wp_read_u32(&mut self) -> Result<u32, WPEngineError>;
    fn wp_read_f32(&mut self) -> Result<f32, WPEngineError>;
    fn wp_read_string(&mut self, size: usize) -> Result<String, WPEngineError>;
    fn wp_read_string_dyn(&mut self) -> Result<String, WPEngineError>;
    fn wp_read_data(&mut self, data: &mut Vec<u8>, size: usize) -> Result<(), WPEngineError>;

    fn wp_stream_position(&mut self) -> Result<u64, WPEngineError>;
    fn wp_seek(&mut self, pos: SeekFrom) -> Result<u64, WPEngineError>;
}

impl<T: ReadBytesExt + Seek> WPReadBytesExt for T {
    fn wp_stream_position(&mut self) -> Result<u64, WPEngineError> {
        let pos = self.stream_position();
        if let Err(err) = pos {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_stream_position]stream_position failed, message: {}",
                err
            )));
        }

        Ok(pos.unwrap())
    }

    fn wp_seek(&mut self, pos: SeekFrom) -> Result<u64, WPEngineError> {
        let pos = self.seek(pos);
        if let Err(err) = pos {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_seek]seek failed, message: {}",
                err
            )));
        }

        Ok(pos.unwrap())
    }

    fn wp_read_i32(&mut self) -> Result<i32, WPEngineError> {
        let data = self.read_i32::<LittleEndian>();
        if let Err(err) = data {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_read_i32]read failed, message: {}",
                err
            )));
        }

        Ok(data.unwrap())
    }

    fn wp_read_u32(&mut self) -> Result<u32, WPEngineError> {
        let data = self.read_u32::<LittleEndian>();
        if let Err(err) = data {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_read_u32]read failed, message: {}",
                err
            )));
        }

        Ok(data.unwrap())
    }

    fn wp_read_f32(&mut self) -> Result<f32, WPEngineError> {
        let data = self.read_f32::<LittleEndian>();
        if let Err(err) = data {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_read_f32]read failed, message: {}",
                err
            )));
        }

        Ok(data.unwrap())
    }

    fn wp_read_string(&mut self, size: usize) -> Result<String, WPEngineError> {
        let mut buf = vec![0u8; size];
        let res = self.read(buf.as_mut_slice());
        if let Err(err) = res {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_read_string]read failed, message: {}",
                err
            )));
        }

        let len = res.unwrap();
        if len != size {
            return Err(WPEngineError::RepkgReadSizeMismatchError(format!(
                "[wp_read_string]read size mismatch, expected: {}, actual: {}",
                size, len
            )));
        }

        let str = String::from_utf8(buf);
        if let Err(err) = str {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_read_string]read failed, message: {}",
                err
            )));
        }

        Ok(str.unwrap())
    }

    fn wp_read_string_dyn(&mut self) -> Result<String, WPEngineError> {
        let mut buf = String::new();

        let res = self.read_u8();
        if let Err(err) = res {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_read_string_dyn]read failed, message: {}",
                err
            )));
        }

        let mut ch = res.unwrap() as char;
        while ch != '\0' {
            buf.push(ch);

            let res = self.read_u8();
            if let Err(err) = res {
                return Err(WPEngineError::RepkgGenericError(format!(
                    "[wp_read_string_dyn]read failed, message: {}",
                    err
                )));
            }

            ch = res.unwrap() as char;
        }

        Ok(buf)
    }

    fn wp_read_data(&mut self, data: &mut Vec<u8>, size: usize) -> Result<(), WPEngineError> {
        let res = self.read_exact(data.as_mut_slice());
        if let Err(err) = res {
            return Err(WPEngineError::RepkgGenericError(format!(
                "[wp_read_data]read failed, message: {}",
                err
            )));
        }

        Ok(())
    }
}
