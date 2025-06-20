use chrono::{Datelike, Local, Timelike};
use rand::prelude::IndexedRandom;
use std::io::Write;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let is_weekday = now.weekday().number_from_monday() <= 5;
    let hour = now.hour();
    let in_worktime = hour >= 8 && hour < 18;

    let items = vec![1, 3, 5, 7, 11];

    loop {
        if !(is_weekday && in_worktime) {
            if let Some(c) = std::env::args().nth(1) {
                let args = std::env::args().skip(1);

                let status = std::process::Command::new(c)
                    .args(args)
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status()
                    .expect("failed to execute command");

                std::process::exit(status.code().unwrap_or(1));
            } else {
                println!("Need a command to run bra");
                std::process::exit(2);
            }
        }

        let mut rng = rand::rng();

        print!(".");
        std::io::stdout().flush()?;

        if let Some(time) = items.choose(&mut rng) {
            thread::sleep(Duration::from_secs(*time));
        } else {
            // Eh sleep 7 seconds if that failed for whatever reason I guess
            thread::sleep(Duration::from_secs(7));
        }
    }
}
