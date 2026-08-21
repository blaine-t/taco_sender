use chrono::{Datelike, Local, TimeZone};
use rand::Rng;

pub fn get_scheduled_times() -> Vec<String> {
    // Generate scheduled times from 8PM to 11:59PM in local time everyday for 120 days
    let mut scheduled_times = Vec::new();
    let start_date = Local::now();
    let end_date = start_date + chrono::Duration::days(120);
    let mut rng = rand::rng();

    let mut current_date = start_date;
    while current_date < end_date {
        // 8PM to 11:59PM local to running device
        let scheduled_dt = Local
            .with_ymd_and_hms(
                current_date.year(),
                current_date.month(),
                current_date.day(),
                rng.random_range(20..=23),
                rng.random_range(0..60), // Random minute
                0,
            )
            .unwrap();

        if scheduled_dt > start_date {
            scheduled_times.push(scheduled_dt.timestamp().to_string());
        }
        current_date = current_date + chrono::Duration::days(1);
    }

    scheduled_times
}
