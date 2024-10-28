use chrono::{self, NaiveTime};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub fn run(time: f32) {
    let now = Instant::now();

    let mut time_left = (time * 1000.0) as f64;
    let mut time_fmt;
    loop {
        time_left -= 1.0;

        sleep(Duration::from_millis(1));

        time_fmt = get_time(time_left);

        print!("\r{}", time_left);
        if time_left == 0.0 {
            break;
        }
    }
}

fn get_time(millisec: f64) -> String {
    let milliseconds = millisec.floor() as u32;
    let seconds = (milliseconds / 1000) % 60;
    let min = (milliseconds / 3600) % 60;
    let hours = seconds / 3600;

    let timer = NaiveTime::from_hms_milli_opt(hours, min, seconds, milliseconds);
    timer.unwrap().to_string()
}
