// Module for dealing with times/dates of fixtures, taking care of timezones etc
// Author: Charlie Giese
//

use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Europe::London;

pub fn uk_time_to_utc(uk_time: &str, format: &str) -> chrono::DateTime<chrono::Utc> {
    // Parse the input time string as a NaiveDateTime
    let naive_time = NaiveDateTime::parse_from_str(uk_time, format).expect("Invalid date format");

    // Use the London timezone, which handles both BST and GMT transitions
    let london_time = London.from_local_datetime(&naive_time).unwrap();

    // Convert to UTC
    london_time.with_timezone(&chrono::Utc)
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDateTime, Utc};

    #[test]
    fn test_uk_time_to_utc_winter() {
        // Test a time in GMT (winter time, no offset from UTC)
        let uk_time = "2024-12-25 15:00:00"; // Christmas, should be GMT
        let format = "%Y-%m-%d %H:%M:%S";
        let expected_utc = "2024-12-25 15:00:00";

        let utc_time = uk_time_to_utc(uk_time, format);
        let expected_utc_time = NaiveDateTime::parse_from_str(expected_utc, format)
            .unwrap()
            .and_utc();

        assert_eq!(utc_time, expected_utc_time);
    }

    #[test]
    fn test_uk_time_to_utc_summer() {
        // Test a time in BST (summer time, +1 hour offset from UTC)
        let uk_time = "2024-07-15 15:00:00"; // Should be BST
        let format = "%Y-%m-%d %H:%M:%S";
        let expected_utc = "2024-07-15 14:00:00"; // 1 hour behind BST

        let utc_time = uk_time_to_utc(uk_time, format);
        let expected_utc_time = NaiveDateTime::parse_from_str(expected_utc, format)
            .unwrap()
            .and_utc();

        assert_eq!(utc_time, expected_utc_time);
    }
}

