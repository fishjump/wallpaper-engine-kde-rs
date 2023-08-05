use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, SeekFrom};
use std::path::Path;

use anyhow::Result;

use super::package_entry_type::PackageEntryType;
use crate::repkg::byteorder_ext::WPReadBytesExt;

pub struct PackageEntry {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub bytes: Vec<u8>,
    pub entry_type: PackageEntryType,
}

impl PackageEntry {
    pub fn read_from(reader: &mut BufReader<File>) -> Result<PackageEntry> {
        let path_size = reader.wp_read_i32()?;
        let path = reader.wp_read_string(path_size as usize)?;

        let offset = reader.wp_read_i32()? as u64;
        let size = reader.wp_read_i32()? as u64;

        let bytes = vec![0u8; size as usize];

        let entry_type = PackageEntryType::from_file_name(path.as_str());

        Ok(PackageEntry {
            path,
            offset,
            bytes,
            size,
            entry_type,
        })
    }

    pub fn populate_bytes(&mut self, reader: &mut BufReader<File>, start: u64) -> Result<()> {
        reader.wp_seek(SeekFrom::Start(start + self.offset))?;
        reader.wp_read_data(&mut self.bytes, self.size as usize)?;

        Ok(())
    }

    pub fn name(&self) -> &str {
        let path = Path::new(self.path.as_str());

        match path.file_stem() {
            Some(name) => name.to_str().unwrap(),
            None => "",
        }
    }

    pub fn extension(&self) -> &str {
        let path = Path::new(self.path.as_str());

        match path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "",
        }
    }

    pub fn dir(&self) -> &str {
        let path = Path::new(self.path.as_str());

        match path.parent() {
            Some(dir) => dir.to_str().unwrap(),
            None => "",
        }
    }
}

impl Debug for PackageEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // ignore bytes, it's too large and unreadable
        f.debug_struct("PackageEntry")
            .field("path", &self.path)
            .field("offset", &format_args!("0x{:x}", &self.offset))
            .field("size", &format_args!("0x{:x}", &self.size))
            .field("entry_type", &self.entry_type)
            .finish()
    }
}
