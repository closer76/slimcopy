mod ignore_file;
mod app_options;
mod ruleset;

use anyhow::{Result, Context};
use app_options::AppOptions;
use ignore_file::IgnoreFile;
use std::path::Path;

fn main() -> Result<()> {
    let options = AppOptions::from_args()?;

    let ignore_file = IgnoreFile::new(options.src.as_path(), options.ignore_file.as_path())
        .context("Ignore file syntax error.")?;

    traverse_dir(options.src.as_path(), &ignore_file)
}

fn traverse_dir(path: &Path, ignore_file: &ignore_file::IgnoreFile) -> Result<()> {
    if ignore_file.is_ignored(path, path.is_dir()) {
        println!("Skip {}", path.display());
    } else if path.is_dir() {
        for entry in path.read_dir()? {
            traverse_dir(entry?.path().as_path(), ignore_file)?;
        }
    } else {
        // TODO: copy
    }

    Ok(())
}
