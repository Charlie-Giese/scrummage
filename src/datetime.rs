// Module for dealing with times/dates of fixtures, taking care of timezones etc
// Author: Charlie Giese
//

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use chrono_tz::Europe::London;

// Function for converting UK time to local timezone
fn uk_naive_to_local(naive_datetime: NaiveDateTime) -> DateTime<Local> {
    // Step 1: Convert NaiveDateTime to DateTime<London> (timezone-aware)
    let london_datetime = London
        .from_local_datetime(&naive_datetime)
        .single()
        .expect("Invalid or ambiguous date (due to DST)");

    // Step 2: Convert London DateTime to UTC
    let utc_datetime = london_datetime.with_timezone(&Local);

    utc_datetime
}

// Function for making DateTimes from time/date strings
pub fn format_datetimes(times: Vec<String>, dates: Vec<String>, year: i32) -> Vec<DateTime<Local>> {
    let mut dtvec = Vec::<DateTime<Local>>::new();

    for (time, date) in times.iter().zip(dates) {
        let date_without_weekday = date.splitn(2, ' ').nth(1).unwrap();
        let cleaned_date = date_without_weekday
            .replace("st", "")
            .replace("nd", "")
            .replace("rd", "")
            .replace("th", "");
        let full_date_str = format!("{} {}", cleaned_date, year);
        let parsed_date = NaiveDate::parse_from_str(&full_date_str, "%d %B %Y").unwrap();
        let parsed_time = NaiveTime::parse_from_str(time, "%H:%M").unwrap();

        let datetime_uk = NaiveDateTime::new(parsed_date, parsed_time);
        let datetime_local = uk_naive_to_local(datetime_uk);

        dtvec.push(datetime_local);
    }

    dtvec
}
