mod utils;

use std::thread::{self, sleep};
use std::time::Duration;
use std::process::Command;
use std::sync::{Arc, Mutex};

const REFRESH_INTERVAL_SECONDS: u64 = 10;

fn main() {
    // Parse command line arguments to set CHECK_INTERVAL_MINUTES if provided
    utils::parse_args();

    let elapsed_seconds = Arc::new(Mutex::new(0u64));


    // Start the clock thread
    let elapsed_clone = elapsed_seconds.clone();
    thread::spawn(move || clock_thread(elapsed_clone));

    // Start the event checker thread
    event_checker_thread(elapsed_seconds);
}

fn clock_thread(elapsed_seconds: Arc<Mutex<u64>>) {
    loop {
        {
            let mut elapsed = elapsed_seconds.lock().unwrap();
            *elapsed += REFRESH_INTERVAL_SECONDS;
        }
        sleep(Duration::from_secs(REFRESH_INTERVAL_SECONDS));
    }
}

fn event_checker_thread(elapsed_seconds: Arc<Mutex<u64>>) {
    loop {
        if utils::has_event_now() {
            println!("You have an event!");
        } else {
            let mut elapsed = elapsed_seconds.lock().unwrap();
            if *elapsed >= (unsafe { utils::CHECK_INTERVAL_MINUTES } * 60) {
                println!("No valid events found. Beeping...");
                beep_progressively();
                *elapsed = 0; // Reset the counter
            }
        }

        sleep(Duration::from_secs(REFRESH_INTERVAL_SECONDS));
    }
}


fn beep_progressively() {
    let beep_duration_secs = 0.5;
    let total_duration_secs = REFRESH_INTERVAL_SECONDS as f64;
    let num_beeps = (total_duration_secs / beep_duration_secs) as u32;

    for _ in 0..num_beeps {
        let _ = Command::new("play")
            .arg("-n")
            .arg("synth")
            .arg(beep_duration_secs.to_string())
            .arg("sin")
            .arg("440")
            .output();
        sleep(Duration::from_secs(beep_duration_secs as u64));
    }
}
