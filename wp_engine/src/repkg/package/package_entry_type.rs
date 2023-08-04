use std::path::Path;

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
