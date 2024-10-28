use chrono::{self, TimeDelta};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub fn run() {
    let now = Instant::now();
    loop {
        let time = get_time(now.elapsed().as_millis().try_into().unwrap());
        print!("\r{}", time);
        sleep(Duration::from_millis(1));
    }
}

fn get_time(millisec: i32) -> String {
    let timer = TimeDelta::milliseconds(millisec.into());
    let seconds = timer.num_seconds() % 60;
    let min = timer.num_minutes() % 60;
    let hours = timer.num_hours();
    let mil = timer.num_milliseconds() % 100;

    format!("{:02}:{:02}:{:02}:{:02}", hours, min, seconds, mil)
}
