use chrono::{Local, Duration as ChronoDuration};
use std::process::Command;
use std::convert::TryInto;
use std::env;

pub static mut CHECK_INTERVAL_MINUTES: u64 = 5;


pub fn parse_args() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 && args[1] == "-c" {
        if let Ok(interval) = args[2].parse::<u64>() {
            unsafe {
                CHECK_INTERVAL_MINUTES = interval;
            }
        }
    }
}


pub fn get_time_strings() -> (String, String) {
    let now = Local::now();
    let end_time = now + ChronoDuration::minutes(unsafe { CHECK_INTERVAL_MINUTES.try_into().unwrap() });
    let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let end_time_str = end_time.format("%Y-%m-%d %H:%M:%S").to_string();
    (now_str, end_time_str)
}
pub fn gcalcli_refresh() {
    let _ = Command::new("gcalcli")
        .arg("refresh")
        .output()
        .expect("Failed to refresh gcalcli data");
}

pub fn get_gcalcli_agenda(start_time: &str, end_time: &str) -> String {
    let output = Command::new("gcalcli")
        .arg("agenda")
        .arg(start_time)
        .arg(end_time)
        .output()
        .expect("Failed to execute gcalcli");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn has_event_now() -> bool {
    gcalcli_refresh(); 
    let (now_str, end_time_str) = get_time_strings();
    let result = get_gcalcli_agenda(&now_str, &end_time_str);
    let now = Local::now();
    has_valid_event(&result, &now)
}

pub fn has_valid_event(output: &str, current_time: &chrono::DateTime<chrono::Local>) -> bool {
    let current_date = current_time.format("%a %b %d").to_string();
    let lines: Vec<&str> = output.lines().collect();
    let mut is_today = false;

    for line in &lines {
        if line.contains(&current_date) {
            is_today = true;
            continue; // Move to the next line
        }
        
        let is_timed_event = line.contains("am") || line.contains("pm");
        if is_today && is_timed_event {
            return true;
        }
    }
    false
}

