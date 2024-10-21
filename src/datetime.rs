// Module for dealing with times/dates of fixtures, taking care of timezones etc
// Author: Charlie Giese
//

use chrono::{NaiveDateTime, TimeZone, DateTime, Utc, NaiveDate, NaiveTime};
use chrono_tz::Europe::London;

fn uk_naive_to_utc(naive_datetime: NaiveDateTime) -> DateTime<Utc> {
    // Step 1: Convert NaiveDateTime to DateTime<London> (timezone-aware)
    let london_datetime = London.from_local_datetime(&naive_datetime)
                                .single()
                                .expect("Invalid or ambiguous date (due to DST)");

    // Step 2: Convert London DateTime to UTC
    let utc_datetime = london_datetime.with_timezone(&Utc);

    utc_datetime
}

pub fn format_datetimes(times : Vec<String>, dates : Vec<String>, year : i32) -> Vec<DateTime<Utc>> {
    
    let mut dtvec = Vec::<DateTime<Utc>>::new();

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

        let datetime_uk =  NaiveDateTime::new(parsed_date, parsed_time);
        let datetime_utc = uk_naive_to_utc(datetime_uk);

        dtvec.push(datetime_utc);
    }

    dtvec
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_uk_naive_to_utc_bst() {
        // Date during BST (British Summer Time, UTC+1)
        let naive_datetime = NaiveDateTime::parse_from_str("2024-07-10 15:30:00", "%Y-%m-%d %H:%M:%S")
            .expect("Invalid datetime format");

        // Convert UK time to UTC
        let utc_datetime = uk_naive_to_utc(naive_datetime);

        // Assert that the UTC time is one hour behind (BST = UTC+1)
        assert_eq!(utc_datetime.to_string(), "2024-07-10 14:30:00 UTC");
    }

    #[test]
    fn test_uk_naive_to_utc_gmt() {
        // Date during GMT (Greenwich Mean Time, UTC+0)
        let naive_datetime = NaiveDateTime::parse_from_str("2024-12-10 15:30:00", "%Y-%m-%d %H:%M:%S")
            .expect("Invalid datetime format");

        // Convert UK time to UTC
        let utc_datetime = uk_naive_to_utc(naive_datetime);

        // Assert that the UTC time is the same (GMT = UTC+0)
        assert_eq!(utc_datetime.to_string(), "2024-12-10 15:30:00 UTC");
    }
}


