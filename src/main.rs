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
            let filetype = if metadata.file_type().is_dir() {
                "dir".to_string()
            } else if metadata.file_type().is_file() {
                "file".to_string()
            } else {
                "sym".to_string()
            };
            println!(
                "{:>5}\t{}\t{}\t{}",
                size,
                modified.format("%_d %b %H:%M").to_string(),
                filetype,
                filename
            );
        }
    }
    Ok(())
}
