use std::fs::File;
use std::io::{BufReader, SeekFrom};
use std::vec;

use super::package::{Package, PackageEntry};
use crate::error::WPEngineError;
use crate::repkg::byteorder_ext::WPReadBytesExt;
use crate::repkg::package::PackageEntryType;

pub struct PackageLoader {}

impl PackageLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_from(&self, reader: &mut BufReader<File>) -> Result<Package, WPEngineError> {
        let pkg_start = reader.wp_stream_position()?;

        let magic = self.read_magic_from(reader)?;
        let mut entries = self.read_entries_from(reader)?;

        let data_start = reader.wp_stream_position()?;
        let header_size = data_start - pkg_start;

        self.read_entry_data_from(reader, data_start, &mut entries)?;

        Ok(Package {
            magic,
            header_size,
            entries,
        })
    }

    fn read_magic_from(&self, reader: &mut BufReader<File>) -> Result<String, WPEngineError> {
        let magic_size = reader.wp_read_i32()?;
        let magic = reader.wp_read_string(magic_size as usize)?;

        if magic.len() != magic_size as usize {
            return Err(WPEngineError::RepkgPackageLoaderError(format!(
                "magic size mismatch, expected: {}, actual: {}",
                magic_size,
                magic.len()
            )));
        }

        Ok(magic)
    }

    fn read_entries_from(
        &self,
        reader: &mut BufReader<File>,
    ) -> Result<Vec<PackageEntry>, WPEngineError> {
        let entry_count = reader.wp_read_i32()?;
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            let path_size = reader.wp_read_i32()?;
            let path = reader.wp_read_string(path_size as usize)?;

            let offset = reader.wp_read_i32()? as u64;
            let size = reader.wp_read_i32()? as u64;

            let bytes = vec![0u8; size as usize];

            let entry_type = PackageEntryType::from_file_name(path.as_str());

            entries.push(PackageEntry {
                path,
                offset,
                bytes,
                size,
                entry_type,
            });
        }

        Ok(entries)
    }

    fn read_entry_data_from(
        &self,
        reader: &mut BufReader<File>,
        data_start: u64,
        entries: &mut Vec<PackageEntry>,
    ) -> Result<(), WPEngineError> {
        for ele in entries {
            reader.wp_seek(SeekFrom::Start(data_start + ele.offset))?;
            reader.wp_read_data(&mut ele.bytes, ele.size as usize)?;
        }

        Ok(())
    }
}
