use chrono::{prelude::*, DateTime};
use std::{thread::sleep, time::Duration};

pub fn run(format: &str) {
    loop {
        let local: DateTime<Local> = Local::now();
        print!("\r{}", local.format(format));
        sleep(Duration::from_millis(10));
    }
}
