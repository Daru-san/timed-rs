use crate::time;

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct Stopwatch();

impl Stopwatch {
    fn new() {
        Stopwatch;
    }
    fn run() {
        let now = Instant::now();
        loop {
            let time = time::get_time(now.elapsed().as_millis().try_into().unwrap());
            print!("\r{}", time);
            sleep(Duration::from_millis(1));
        }
    }
}
