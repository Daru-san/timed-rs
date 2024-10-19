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
    let time: String;
    let milliseconds = millisec % 100;
    let seconds = (millisec / 1000) % 60;
    let min = (millisec / 3600) % 60;
    let hours = seconds / 3600;
    time = format!("{:02}:{:02}:{:02}:{:02}", hours, min, seconds, milliseconds);

    return time.to_string();
}
