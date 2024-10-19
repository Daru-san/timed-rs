use chrono::{prelude::*, DateTime};
use std::{thread::sleep, time::Duration};

pub fn run() {
    loop {
        let local: DateTime<Local> = Local::now().try_into().unwrap();
        print!("\r{}", local.format("%H:%M:%S").to_string());
        sleep(Duration::from_millis(10));
    }
}
