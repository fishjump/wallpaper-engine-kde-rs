use std::fmt::{Debug, Formatter, Result};
use std::path::Path;

pub struct Package {
    pub magic: String,
    pub header_size: u64,
    pub entries: Vec<PackageEntry>,
}

impl Debug for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Package")
            .field("magic", &self.magic)
            .field("header_size", &format_args!("0x{:x}", &self.header_size))
            .field("entries", &self.entries)
            .finish()
    }
}

pub struct PackageEntry {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub bytes: Vec<u8>,
    pub entry_type: PackageEntryType,
}

impl Debug for PackageEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // ignore bytes, it's too large and unreadable
        f.debug_struct("PackageEntry")
            .field("path", &self.path)
            .field("offset", &format_args!("0x{:x}", &self.offset))
            .field("size", &format_args!("0x{:x}", &self.size))
            .field("entry_type", &self.entry_type)
            .finish()
    }
}

impl PackageEntry {
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

#[derive(Debug)]
pub enum PackageEntryType {
    Binary,
    Tex,
}

impl PackageEntryType {
    pub fn from_file_name(file_name: &str) -> Self {
        let path = Path::new(file_name.trim());

        match path.extension() {
            Some(ext) => match ext.to_str().unwrap().to_lowercase().as_str() {
                "tex" => PackageEntryType::Tex,
                _ => PackageEntryType::Binary,
            },
            None => PackageEntryType::Binary,
        }
    }
}
