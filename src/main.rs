use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, Duration as ChronoDuration};

const CHECK_INTERVAL_MINUTES: i64 = 5;
fn main() {
    loop {
        let _ = Command::new("gcalcli")
            .arg("refresh")
            .output()
            .expect("Failed to refresh gcalcli data");

        let now = Local::now();
        let end_time = now + ChronoDuration::minutes(CHECK_INTERVAL_MINUTES);

        let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let end_time_str = end_time.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{}", now_str);
        println!("{}", end_time_str);
        let output = Command::new("gcalcli")
            .arg("agenda")
            .arg(&now_str)
            .arg(&end_time_str)
            .output()
            .expect("Failed to execute gcalcli");

        let result = String::from_utf8_lossy(&output.stdout);

        if has_valid_event(&result, &now) {
            println!("You have an event!");
        } else {
            println!("No valid events found. Beeping...");
            beep_progressively();
        }

        sleep(Duration::from_secs((CHECK_INTERVAL_MINUTES * 60) as u64));
    }
}

fn has_valid_event(output: &str, current_time: &chrono::DateTime<chrono::Local>) -> bool {
    let current_date = current_time.format("%a %b %d").to_string();
    let lines: Vec<&str> = output.lines().collect();
    let mut is_today = false;

    for line in &lines {
        if line.contains(&current_date) {
            is_today = true;
            continue; // Move to the next line
        }
        
        let is_timed_event = line.contains("am") || line.contains("pm");
        if is_today &&  is_timed_event{
            return true;
        }
    }
    false
}

fn beep_progressively() {
    let beep_duration_secs = 0.5;
    let total_duration_secs = (CHECK_INTERVAL_MINUTES * 60) as f64;
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