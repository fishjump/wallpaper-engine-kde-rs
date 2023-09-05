use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use anyhow::Result;
use console::style;
use wp_engine::repkg::Package;

use crate::{vprintln, Options};

pub fn repkg(path: &PathBuf, opt: &Options) -> Result<PathBuf> {
    fs::create_dir_all(&opt.output)?;

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let pkg = Package::read_from(&mut reader)?;

    for (i, entry) in pkg.entries.iter().enumerate() {
        let output = path_cat(&opt.output, &entry.path);
        vprint_progress(
            opt.verbose,
            i + 1,
            pkg.entries.len(),
            &output.to_string_lossy(),
        );

        if let Some(base) = output.parent() {
            fs::create_dir_all(base)?;
        }

        let file = File::create(output)?;
        let data = &entry.bytes;

        let mut writer = BufWriter::new(file);
        writer.write_all(data)?;
    }

    Ok(PathBuf::new())
}

fn path_cat(dir: &PathBuf, file: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(dir);
    path.push(file);
    path
}

fn vprint_progress(verbose: bool, i: usize, total: usize, path: &str) {
    vprintln!(
        verbose,
        "{} Writing {}...",
        style(format!("[{}/{}]", i, total)).bold().green().dim(),
        path
    );
}
