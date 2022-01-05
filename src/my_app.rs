mod app_options;
mod ignore_file;
mod logger;
mod type_counter;
mod working_indicator;

use anyhow::{Context, Result};
use app_options::AppOptions;
use fs_extra::dir::get_size;
use ignore_file::IgnoreFile;
use logger::Logger;
use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use type_counter::TypeCounter;
use working_indicator::WorkingIndicator;

pub struct MyApp {
    options: AppOptions,
    ignore_file: IgnoreFile,
    log: Logger,
    db: HashMap<PathBuf, (u64, u64)>,
    progress: RefCell<WorkingIndicator>,
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

        print!("Collecting info of the source directory...");
        let _ = stdout().flush();
        let (db, count, _) = Self::collect_dir_info(&options.src)?;
        println!("Done.");

        let progress = RefCell::new(WorkingIndicator::new(count));

        Ok(MyApp {
            options,
            ignore_file,
            log,
            db,
            progress,
        })
    }

    pub fn run(&self) -> Result<TypeCounter> {
        self.progress.borrow_mut().init();
        let result = self.traverse_tree(&self.options.src);
        self.progress.borrow().done();
        result
    }

    fn collect_dir_info(item: &Path) -> Result<(HashMap<PathBuf, (u64, u64)>, u64, u64)> {
        if item.symlink_metadata()?.file_type().is_symlink() {
            Ok((HashMap::new(), 0, 0))
        } else if item.is_dir() {
            item.read_dir()?
                .par_bridge()
                .map(|entry| {
                    let entry = entry.context("I/O error")?;
                    Self::collect_dir_info(entry.path().as_path())
                })
                .collect::<Result<Vec<_>>>()
                .map(|v| {
                    let (mut map, count, size) = v.into_par_iter().reduce(
                        || (HashMap::new(), 0, 0),
                        |(map1, count1, size1), (map2, count2, size2)| {
                            (
                                map1.into_iter().chain(map2).collect(),
                                count1 + count2,
                                size1 + size2,
                            )
                        },
                    );
                    map.insert(item.to_path_buf(), (count, size));
                    (map, count, size)
                })
        } else {
            // Size calculation is time-consuming, so it's skipped here.
            // If you really want to pre-calculates the sizes of directories,
            // use this line to replace the next:
            // ```
            // Ok(HashMap::new(), 1, Info::File(get_size(item).unwrap_or(0)))
            // ```
            Ok((HashMap::new(), 1, 0))
        }
    }

    fn traverse_tree(&self, path: &Path) -> Result<TypeCounter> {
        if self.ignore_file.is_ignored(path, path.is_dir()) {
            let file_count = self.db.get(&path.to_path_buf()).unwrap_or(&(0, 0)).0;
            self.log
                .add(format!("Skip {} files in {}", file_count, path.display()).as_str());
            let size = get_size(path).unwrap_or(0);
            self.progress.borrow_mut().update(file_count);
            let counter = TypeCounter::new();
            Ok(counter.count_skipped(file_count, size))
        } else if path.is_dir() {
            path.read_dir()?
                .map(|entry| self.traverse_tree(entry?.path().as_path()))
                .collect::<Result<Vec<TypeCounter>>>()
                .map(|v| v.iter().sum())
        } else {
            self.progress.borrow_mut().update(1);
            self.copy_file(path)
        }
    }

    fn copy_file(&self, src_path: &Path) -> Result<TypeCounter> {
        let src_meta = src_path.symlink_metadata()?;
        let counter = TypeCounter::new();
        if src_meta.file_type().is_symlink() {
            self.log
                .add(&format!("Skip symbolic link \"{}\"", src_path.display()));
            Ok(counter.count_symlink())
            // TODO: There should be better ways to handle symbolic links...
            // let link_target = src_path.read_link()?;
            // std::os::windows::fs::symlink_dir(link_target, dest_path)?;
        } else {
            let dest_path = self
                .options
                .dest
                .join(src_path.strip_prefix(&self.options.src)?);
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
                fs::create_dir_all(&dest_dir).with_context(|| {
                    format!("Cannot create directory \"{}\"", dest_dir.display())
                })?;
            }

            self.log.add(&format!("Copy {}", src_path.display()));
            fs::copy(&src_path, &dest_path)
                .with_context(|| format!("Failed to copy file to \"{}\"", dest_path.display()))
                .map(|size| counter.count_copied(size))
        }
    }
}
