extern crate chrono;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

static CHRONO_TIME_FORMAT: &str = "%H:%M:%S";
static CHRONO_DATE_FORMAT: &str = "%Y-%m-%d";
static CHRONO_DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_unix_timestamp_ms() -> i64 {
    Utc::now().timestamp()
}

pub fn time_stamp_to_date(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    datetime.format(CHRONO_DATE_FORMAT).to_string()
}

pub fn time_stamp_to_time(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    datetime.format(CHRONO_TIME_FORMAT).to_string()
}

pub fn time_stamp_to_date_time(time_stamp: i64) -> String {
    let utc = NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let datetime = Utc.from_utc_datetime(&utc);
    datetime.format(CHRONO_DATE_TIME_FORMAT).to_string()
}

pub fn date_time_to_time_stamp(date: &str) -> i64 {
    let date_time = NaiveDateTime::parse_from_str(date, CHRONO_DATE_TIME_FORMAT);
    if date_time.is_err() {
        return 0;
    }
    date_time.ok().unwrap().timestamp()
}

pub fn time_stamp_from_year_and_day(year: i32, day_of_year: u32) -> i64 {
    let date = NaiveDate::from_yo_opt(year, day_of_year).unwrap();
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    Utc.from_utc_datetime(&datetime).timestamp()
}

/// Check if String literal is matching SQL time format: HH:MM:SS or HH:MM:SS.SSS
pub fn is_valid_time_format(time_str: &str) -> bool {
    // Check length of the string
    if !(8..=12).contains(&time_str.len()) {
        return false;
    }

    // Split the string into hours, minutes, seconds, and optional milliseconds
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() < 3 || parts.len() > 4 {
        return false;
    }

    // Extract hours, minutes, seconds, and optionally milliseconds
    let hours = parts[0].parse::<u32>().ok();
    let minutes = parts[1].parse::<u32>().ok();
    let seconds_parts: Vec<&str> = parts[2].split('.').collect();
    let seconds = seconds_parts[0].parse::<u32>().ok();
    let milliseconds = if seconds_parts.len() == 2 {
        seconds_parts[1].parse::<u32>().ok()
    } else {
        Some(0)
    };

    // Validate the parsed values
    hours.is_some()
        && minutes.is_some()
        && seconds.is_some()
        && milliseconds.is_some()
        && hours.unwrap() < 24
        && minutes.unwrap() < 60
        && seconds.unwrap() < 60
        && milliseconds.unwrap() < 1000
}
