use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process,
};

use chrono::{DateTime, Local};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    path: PathBuf,
}

fn main() {
    let args = Opt::parse();
    if let Err(ref e) = run(&args.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let filename = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid Entry: {:?}", f)))?;
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            println!(
                "{:>5}\t{}\t{}",
                size,
                modified.format("%_d %b %H:%M").to_string(),
                filename
            );
        }
    }
    Ok(())
}
