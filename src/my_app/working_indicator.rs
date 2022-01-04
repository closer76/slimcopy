use std::{time::Instant, io::Write};

pub struct WorkingIndicator {
    index: usize,
    last_update: Instant,
}

impl WorkingIndicator {
    pub fn new() -> Self {
        WorkingIndicator {
            index: 0,
            last_update: Instant::now(),
        }
    }

    pub fn init(&mut self) {
        self.last_update = Instant::now();
        print!("Processing....");
        self.draw();
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_millis() > 250 {
            self.index = (self.index + 1) % 4;
            self.last_update = now;
            print!("{}", '\u{0008}');   // u0008 == backspace
            self.draw();
        }
    }

    pub fn done(&self) {
        println!("{}Done.", '\u{0008}');   // u0008 == backspace        
    }

    fn draw(&self) {
        let c = match self.index {
            1 => '\\',
            2 => '|',
            3 => '/',
            _ => '-',
        };
        print!("{}", c);
        let _ = std::io::stdout().flush();
    }
}