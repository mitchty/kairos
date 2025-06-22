use chrono::{Datelike, Local, Timelike};
use rand::prelude::IndexedRandom;
use std::io::Write;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = vec![1, 3, 5, 7, 11];

    loop {
        let now = Local::now();
        let is_weekday = now.weekday().number_from_monday() <= 5;
        let hour = now.hour();
        let is_afterhours = !(hour >= 8 && hour < 18);

        // Only succeed when we're not on a weekay bt 8am-6pm locally
        if is_weekday && is_afterhours {
            if let Some(c) = std::env::args().nth(1) {
                let args = std::env::args().skip(1);

                let status = std::process::Command::new(c)
                    .args(args)
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status()?;

                std::process::exit(status.code().unwrap_or(1));
            } else {
                println!("Need a command to run bra");
                std::process::exit(2);
            }
        } else {
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
}
