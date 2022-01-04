mod app_options;
mod ignore_file;
mod logger;
mod type_counter;

use anyhow::{Context, Result};
use fs_extra::dir::get_size;
use std::fs;
use std::path::Path;
use app_options::AppOptions;
use ignore_file::IgnoreFile;
use logger::Logger;
use type_counter::TypeCounter;

pub struct MyApp {
    options: AppOptions,
    ignore_file: IgnoreFile,
    log: Logger,
}

impl MyApp {
    pub fn new() -> Result<Self> {
        let options = AppOptions::from_args()?;

        let ignore_file = IgnoreFile::new(options.src.as_path(), options.ignore_file.as_path())
            .context("Ignore file syntax error.")?;
    
        let log = match &options.log_file {
            Some(path) => Logger::to_file(path),
            _ => Logger::new(true),
        };

        Ok(MyApp {options, ignore_file, log})
    }

    pub fn run(&self) -> Result<TypeCounter> {
        self.traverse_tree(&self.options.src)
    }

    fn traverse_tree(&self, path: &Path) -> Result<TypeCounter> {
        if self.ignore_file.is_ignored(path, path.is_dir()) {
            self.log.add(format!("Skip {}", path.display()).as_str());
            let counter = TypeCounter::new();
            Ok(counter.count_skipped(get_size(path).unwrap_or(0)))
        } else if path.is_dir() {
            path.read_dir()?
                .map(|entry| self.traverse_tree(entry?.path().as_path()))
                .collect::<Result<Vec<TypeCounter>>>()
                .map(|v| v.iter().sum())
        } else {
            self.copy_file(path)
        }    
    }

    fn copy_file(&self, src_path: &Path) -> Result<TypeCounter> {
        let src_meta = src_path.symlink_metadata()?;
        let counter = TypeCounter::new();
        if src_meta.file_type().is_symlink() {
            self.log.add(&format!("Skip symbolic link \"{}\"", src_path.display()));
            Ok(counter.count_symlink())
            // TODO: There should be better ways to handle symbolic links...
            // let link_target = src_path.read_link()?;
            // std::os::windows::fs::symlink_dir(link_target, dest_path)?;
        } else {
            let dest_path = self.options.dest.join(src_path.strip_prefix(&self.options.src)?);
            let dest_dir = dest_path.parent().unwrap();
            if dest_path.exists() {
                let dest_meta = fs::metadata(&dest_path)?;
    
                // If force-copy is not set, copy only newer files
                if !self.options.force_copy {
                    match (src_meta.modified(), dest_meta.modified()) {
                        (Ok(src_time), Ok(dest_time)) if src_time > dest_time => (),
                        _ => {
                            self.log.add(&format!("Old {}", src_path.display()));
                            return Ok(counter.count_no_update(get_size(src_path).unwrap_or(0)));
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
    
            self.log.add(&format!("Copy {}", src_path.display()));
            fs::copy(&src_path, &dest_path)
                .with_context(|| format!("Failed to copy file to \"{}\"", dest_path.display()))
                .map(|size| counter.count_copied(size))
        }
    }    
}