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
