mod repkg;

use std::path::PathBuf;

use anyhow::Result;
use console::style;
use repkg::repkg;
use structopt::StructOpt;

#[macro_export]
macro_rules! vprintln {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            println!($($arg)*);
        }
    }
}

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt()]
    pub files: Vec<PathBuf>,

    #[structopt(
        short,
        long,
        default_value = "./output",
        help = "Output directory"
    )]
    pub output: PathBuf,

    #[structopt(short, long, help = "Verbose output")]
    pub verbose: bool,
}

fn main() -> Result<()> {
    let opt = Options::from_args();

    for (i, path) in opt.files.iter().enumerate() {
        vprint_progress(
            opt.verbose,
            i + 1,
            opt.files.len(),
            &path.to_string_lossy(),
        );
        repkg(&path, &opt)?;
    }

    Ok(())
}

fn vprint_progress(verbose: bool, i: usize, total: usize, path: &str) {
    vprintln!(
        verbose,
        "{} Unpacking {}...",
        style(format!("[{}/{}]", i, total)).bold().dim(),
        path
    );
}
