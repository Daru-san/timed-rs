use chrono::{self, NaiveTime};

pub fn get_time(millisec: u32) -> String {
    let seconds = (millisec / 1000) % 60;
    let min = (millisec / 3600) % 60;
    let hours = seconds / 3600;

    let timer = NaiveTime::from_hms_milli_opt(hours, min, seconds, millisec);
    timer.unwrap().to_string()
}
