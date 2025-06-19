use chrono::{Datelike, Local, Timelike};

fn main() {
    let now = Local::now();
    let is_weekday = now.weekday().number_from_monday() <= 5;
    let hour = now.hour();
    let in_worktime = hour >= 8 && hour < 18;

    if is_weekday && in_worktime {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
