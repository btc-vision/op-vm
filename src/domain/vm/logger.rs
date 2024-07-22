/*use chrono::{DateTime, Local};
use wasmer::RuntimeError;

pub fn log_time_diff(_time: &DateTime<Local>, _object: &str) {
    let time2 = Local::now();

    let ns = (time2 - _time).num_nanoseconds().ok_or(RuntimeError::new("Failed to get time")).expect("Failed to get time");
    let ms = ns as f64 / 1_000_000.0;

    println!("{} took {}ms", _object, ms);
}*/
