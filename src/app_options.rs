use anyhow::{bail, Result, Context};
use clap::{clap_app, crate_version};
use std::path::PathBuf;
use std::str::FromStr;

pub struct AppOptions {
    pub src: PathBuf,
    pub dest: PathBuf,
    pub ignore_file: PathBuf,
}

impl AppOptions {
    pub fn from_args() -> Result<Self> {
        let matches = clap_app!(my_app =>
            (version: crate_version!())
            (author: "Kenneth Lo <closer.tw@gmail.com>")
            (@arg SRC: +required +takes_value "Source directory")
            (@arg DEST: +required +takes_value "Destination directory")
            (@arg IGNORE_FILE: -i --ignore_file +takes_value "Reference ignored file")
        )
        .get_matches();

        let src = PathBuf::from_str(matches.value_of("SRC")
            .unwrap())?
            .canonicalize().context("Source does not exist.")?;
        if !src.is_dir() {
            bail!("Source must be a directory.");
        }

        let dest = PathBuf::from_str(matches.value_of("DEST")
            .unwrap())?;
        if !dest.exists() {
            if let Some(true) = dest.parent().map(|parent| parent.exists()) {
                // create directory
                std::fs::create_dir(&dest)
                    .with_context(|| format!("Failed to create destination directory \"{}\"", dest.display()))?;
            } else {
                bail!("Destination does not exist.");
            }
        } else if !dest.is_dir() {
            bail!("Destination must be a directory.");
        }
        let dest = dest.canonicalize()?;

        let ignore_file = match matches.value_of("IGNORE_FILE") {
            Some(value) => PathBuf::from_str(value)?,
            _ => {
                let mut path = src.clone();
                path.push(".slimcopy_rules");
                path
            }
        };

        if !ignore_file.exists() || !ignore_file.is_file() {
            bail!("Ignore file does not exist!")
        }
        println!("Ignore file = {}", ignore_file.display());

        Ok(AppOptions {src, dest, ignore_file})
    }
}