use std::ops::Add;
use std::iter::Sum;

#[derive(Debug, Clone, Copy)]
pub struct TypeCounter {
    copied: u64,
    skipped: u64,
    symlink: u64,
    no_update: u64,
}

impl TypeCounter {
    pub fn new() -> Self {
        TypeCounter {
            copied: 0,
            skipped: 0,
            symlink: 0,
            no_update: 0,
        }
    }

    pub fn count_copied(self) -> Self {
        Self {
            copied: self.copied + 1,
            ..self
        }
    }

    pub fn count_skipped(self) -> Self {
        Self {
            skipped: self.skipped + 1,
            ..self
        }
    }

    pub fn count_symlink(self) -> Self {
        Self {
            symlink: self.symlink + 1,
            ..self
        }
    }

    pub fn count_no_update(self) -> Self {
        Self {
            no_update: self.no_update + 1,
            ..self
        }
    }
}

impl Add for TypeCounter {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            copied: self.copied + other.copied,
            skipped: self.skipped + other.skipped,
            symlink: self.symlink + other.symlink,
            no_update: self.no_update + other.no_update,
        }
    }
}

impl<'a> Sum<&'a TypeCounter> for TypeCounter {
    fn sum<I: Iterator<Item = &'a TypeCounter>>(iter: I) -> Self {
        iter.fold(TypeCounter::new(), |accu, &item| {
            accu + item
        })
    }
}