use crate::time;

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Stopwatch;

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch
    }
    pub fn run(&self) {
        let now = Instant::now();
        loop {
            let time = time::get_time(now.elapsed().as_millis().try_into().unwrap());
            print!("\r{}", time);
            sleep(Duration::from_millis(1));
        }
    }
}
