use crate::time;
use std::{thread::sleep, time::Duration};

#[derive(Debug)]
struct Timer {
    time_ms: u32,
}

impl Timer {
    fn new(time_in_ms: u32) {
        Timer {
            time_ms: time_in_ms,
        };
    }
    fn run(&self) {
        let time_ms = self.time_ms;
        let mut time_left = time_ms;
        let mut time_fmt;
        loop {
            time_left -= 1;

            sleep(Duration::from_millis(1));

            time_fmt = time::get_time(time_left);

            print!("\r{}", time_fmt);
            if time_left == 0 {
                break;
            }
        }
    }
}
