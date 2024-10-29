use chrono::{prelude::*, DateTime};
use std::{thread::sleep, time::Duration};

#[derive(Debug)]
struct Clock {
    format: String,
}

impl Clock {
    fn new(clock_format: String) {
        Clock {
            format: clock_format,
        };
    }
    fn run(&self) {
        let format = &self.format;
        loop {
            let local: DateTime<Local> = Local::now();
            print!("\r{}", local.format(format));
            sleep(Duration::from_millis(10));
        }
    }
}
