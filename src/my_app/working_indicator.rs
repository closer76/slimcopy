use std::io::Write;

pub struct WorkingIndicator {
    max_step: u64,
    step: u64,
    last_percentage: u64,
}

impl WorkingIndicator {
    pub fn new(max: u64) -> Self {
        WorkingIndicator {
            max_step: max,
            step: 0,
            last_percentage: 0,
        }
    }

    pub fn init(&mut self) {
        print!("Processing....");
        self.draw();
    }

    pub fn update(&mut self, step: u64) {
        self.step += step;
        let new_percentage = self.step * 100 / self.max_step;
        if new_percentage != self.last_percentage {
            self.last_percentage = new_percentage;
            print!("\u{0008}\u{0008}\u{0008}\u{0008}");
            self.draw();
        }
    }

    pub fn done(&self) {
        println!("\u{0008}\u{0008}\u{0008}\u{0008}Done."); // u0008 == backspace
    }

    fn draw(&self) {
        print!("{:>3}%", self.last_percentage);
        let _ = std::io::stdout().flush();
    }
}
