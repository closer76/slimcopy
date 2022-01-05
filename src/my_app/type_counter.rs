use size_format::SizeFormatterBinary;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug)]
pub struct TypeCounter {
    copied: u64,
    skipped: u64,
    symlink: u64,
    no_update: u64,
    copied_size: u64,
    skipped_size: u64,
    no_update_size: u64,
}

impl TypeCounter {
    pub fn new() -> Self {
        TypeCounter {
            copied: 0,
            skipped: 0,
            symlink: 0,
            no_update: 0,
            copied_size: 0,
            skipped_size: 0,
            no_update_size: 0,
        }
    }

    pub fn count_copied(self, size: u64) -> Self {
        Self {
            copied: self.copied + 1,
            copied_size: self.copied_size + size,
            ..self
        }
    }

    pub fn count_skipped(self, count: u64, size: u64) -> Self {
        Self {
            skipped: self.skipped + count,
            skipped_size: self.skipped_size + size,
            ..self
        }
    }

    pub fn count_symlink(self) -> Self {
        Self {
            symlink: self.symlink + 1,
            ..self
        }
    }

    pub fn count_no_update(self, size: u64) -> Self {
        Self {
            no_update: self.no_update + 1,
            no_update_size: self.no_update_size + size,
            ..self
        }
    }
}

impl<'a> Add<&'a TypeCounter> for TypeCounter {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self {
        Self {
            copied: self.copied + other.copied,
            skipped: self.skipped + other.skipped,
            symlink: self.symlink + other.symlink,
            no_update: self.no_update + other.no_update,
            copied_size: self.copied_size + other.copied_size,
            skipped_size: self.skipped_size + other.skipped_size,
            no_update_size: self.no_update_size + other.no_update_size,
        }
    }
}

impl<'a> Sum<&'a TypeCounter> for TypeCounter {
    fn sum<I: Iterator<Item = &'a TypeCounter>>(iter: I) -> Self {
        iter.fold(TypeCounter::new(), |accu, item| accu + item)
    }
}

impl Display for TypeCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>6} file(s) copied,      size = {:>8}B
{:>6} file(s) not updated, size = {:>8}B
{:>6} item(s) skipped,     size = {:>8}B
{:>6} symbolic link(s)",
            self.copied,
            SizeFormatterBinary::new(self.copied_size),
            self.no_update,
            SizeFormatterBinary::new(self.no_update_size),
            self.skipped,
            SizeFormatterBinary::new(self.skipped_size),
            self.symlink
        )
    }
}
