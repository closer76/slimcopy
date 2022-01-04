use std::cell::RefCell;
use std::{fs::File, io::Write, path::Path};

// Logger is not thread-safe since RefCell is used.

pub struct Logger {
    mute: bool,
    file: Option<RefCell<File>>,
}

impl Logger {
    pub fn new(mute: bool) -> Self {
        Logger { mute, file: None }
    }

    pub fn to_file<P: AsRef<Path>>(path: P) -> Self {
        let file = match File::create(path) {
            Ok(f) => Some(RefCell::new(f)),
            _ => None,
        };
        Logger { mute: false, file }
    }

    pub fn add(&self, msg: &str) {
        if !self.mute {
            match &self.file {
                None => println!("{}", msg),
                Some(f) => {
                    let mut file = f.borrow_mut();
                    let _ = file.write(msg.as_bytes());
                    let _ = file.write("\n".as_bytes());
                }
            }
        }
    }
}
