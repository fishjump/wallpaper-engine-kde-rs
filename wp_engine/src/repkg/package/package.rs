use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, SeekFrom};

use super::package_entry::PackageEntry;
use super::package_entry_type::PackageEntryType;
use crate::error::WPEngineError;
use crate::repkg::byteorder_ext::WPReadBytesExt;

pub struct Package {
    pub magic: String,
    pub header_size: u64,
    pub entries: Vec<PackageEntry>,
}

impl Package {
    pub fn read_from(reader: &mut BufReader<File>) -> Result<Package, WPEngineError> {
        let pkg_start = reader.wp_stream_position()?;

        let magic_size = reader.wp_read_i32()?;
        let magic = reader.wp_read_string(magic_size as usize)?;

        if magic.len() != magic_size as usize {
            return Err(WPEngineError::RepkgInvalidPakMagicError(format!(
                "[read_magic_from]magic size mismatch, expected: {}, actual: {}",
                magic_size,
                magic.len()
            )));
        }

        let entry_count = reader.wp_read_i32()?;
        let mut entries = Vec::new();
        entries.reserve_exact(entry_count as usize);

        for _ in 0..entry_count {
            entries.push(PackageEntry::read_from(reader)?);
        }

        let data_start = reader.wp_stream_position()?;
        let header_size = data_start - pkg_start;

        for ele in &mut entries {
            ele.populate_bytes(reader, data_start)?;
        }

        Ok(Package {
            magic,
            header_size,
            entries,
        })
    }
}

impl Debug for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Package")
            .field("magic", &self.magic)
            .field("header_size", &format_args!("0x{:x}", &self.header_size))
            .field("entries", &self.entries)
            .finish()
    }
}
