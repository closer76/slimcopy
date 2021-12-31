mod app_options;
mod ignore_file;
mod ruleset;

use anyhow::{Context, Result};
use app_options::AppOptions;
use ignore_file::IgnoreFile;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let options = AppOptions::from_args()?;

    let ignore_file = IgnoreFile::new(options.src.as_path(), options.ignore_file.as_path())
        .context("Ignore file syntax error.")?;

    traverse_dir(options.src.as_path(), &ignore_file, &options)
}

fn traverse_dir(path: &Path, ignore_file: &IgnoreFile, options: &AppOptions) -> Result<()> {
    if ignore_file.is_ignored(path, path.is_dir()) {
        println!("Skip {}", path.display());
        Ok(())
    } else if path.is_dir() {
        path.read_dir()?
            .map(|entry| traverse_dir(entry?.path().as_path(), ignore_file, options))
            .collect::<Result<()>>()
    } else {
        copy_file(path, &options)
    }
}

fn copy_file(src_path: &Path, options: &AppOptions) -> Result<()> {
    let src_meta = src_path.symlink_metadata()?;

    if src_meta.file_type().is_symlink() {
        // TODO: There should be better ways to handle symbolic links...
        println!("Skip symbolic link \"{}\"", src_path.display());
        // let link_target = src_path.read_link()?;
        // std::os::windows::fs::symlink_dir(link_target, dest_path)?;
        Ok(())
    } else {
        let dest_path = options.dest.join(src_path.strip_prefix(&options.src)?);
        let dest_dir = dest_path.parent().unwrap();
        if dest_path.exists() {
            let dest_meta = fs::metadata(&dest_path)?;

            // If force-copy is not set, copy only newer files
            if !options.force_copy {
                match (src_meta.modified(), dest_meta.modified()) {
                    (Ok(src_time), Ok(dest_time)) if src_time > dest_time => (),
                    _ => {
                        println!("No update: {}", src_path.display());
                        return Ok(());
                    }
                };
            }

            // Remove read-only attribute before overwriting existing file
            let mut permission = dest_meta.permissions();
            if permission.readonly() {
                permission.set_readonly(false);
                fs::set_permissions(&dest_path, permission)?;
            }
        } else if !dest_dir.exists() {
            fs::create_dir_all(&dest_dir)
                .with_context(|| format!("Cannot create directory \"{}\"", dest_dir.display()))?;
        }

        fs::copy(&src_path, &dest_path)
            .with_context(|| format!("Failed to copy file to \"{}\"", dest_path.display()))
            // We don't need u64 returned from fs::copy(), so dispose it.
            .map(|_| ())
    }
}
