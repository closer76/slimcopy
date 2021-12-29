mod ruleset;
mod ignore_file;

use std::{fs, path::Path};
use clap::clap_app;
use anyhow::{Result, bail};

fn main() -> Result<()> {
    let matches = clap_app!(my_app =>
        (version: "0.8")
        (author: "Kenneth Lo <closer.tw@gmail.com>")
        (@arg SRC: -s --src +required +takes_value "Source directory")
        (@arg DEST: -d --dest +takes_value "Destination directory")
        (@arg IGNORE_FILE: -i --ignore_file +takes_value "Reference ignored file")
    ).get_matches();

    let src_path = Path::new(matches.value_of("SRC").unwrap());
    let ignore_file = match matches.value_of("IGNORE_FILE") {
        Some(value) => {
            ignore_file::IgnoreFile::new(src_path, Path::new(value))?
        }
        _ => bail!("Ignore file does not exist!"),
    };
    traverse_dir(src_path, &ignore_file)
}

fn traverse_dir(root: &Path, ignore_file: &ignore_file::IgnoreFile) -> Result<()> {
    if root.is_dir() {
        for entry in root.read_dir()? {
            let entry_name = entry?.path();
            if ignore_file.is_ignored(entry_name.as_path(), entry_name.is_dir()) {
                println!("{}", entry_name.display());
            } else if entry_name.is_dir() {
                traverse_dir(entry_name.as_path(), ignore_file)?;
            }
        }
    }

    Ok(())
}