use chrono::{prelude::*, DateTime};
use std::{thread::sleep, time::Duration};

#[derive(Debug)]
pub struct Clock {
    pub format: String,
}

impl Clock {
    pub fn new(clock_format: String) -> Self {
        Clock {
            format: clock_format,
        }
    }

    pub fn run(&self) {
        let format = &self.format;
        loop {
            let local: DateTime<Local> = Local::now();
            print!("\r{}", local.format(format));
            sleep(Duration::from_millis(10));
        }
    }
}
