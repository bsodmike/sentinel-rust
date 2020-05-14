use crate::errors::Error;
use crate::regex::Regex;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_utc_time() -> chrono::DateTime<chrono::Utc> {
    Utc::now()
}

/// Returns a `chrono::DateTime<chrono::FixedOffset>`
///
/// # Example
///
/// ~~~~
/// use sentinel::utils::time::get_utc_time;
/// use sentinel::utils::time::parse_utc_time_to_rfc_rfc3339;
///
/// parse_utc_time_to_rfc_rfc3339(get_utc_time());
/// ~~~~
pub fn parse_utc_time_to_rfc_rfc3339(utc: DateTime<Utc>) -> DateTime<FixedOffset> {
    from_rfc_rfc3339(&utc.to_rfc3339()[..]).unwrap()
}

pub fn from_rfc_rfc3339(timestamp: &str) -> Result<DateTime<FixedOffset>, Error> {
    Ok(DateTime::parse_from_rfc3339(timestamp)?)
}

pub fn to_rfc_rfc3339(naive_dt: chrono::NaiveDateTime) -> Result<DateTime<FixedOffset>, Error> {
    let naive_string = naive_dt.to_string();
    let re = Regex::new(r"[\s]+").unwrap();
    let mut corrected = String::from(re.replace(&naive_string[..], "T"));
    corrected.push_str("Z");

    let dt = match DateTime::parse_from_rfc3339(&corrected[..]) {
        Ok(value) => value,
        Err(e) => panic!("Err: {}", e),
    };

    let rfc3339 = dt.with_timezone(&Utc).to_rfc3339();
    let result = from_rfc_rfc3339(&rfc3339[..]).unwrap();

    Ok(result)
}

pub fn occurred_more_than_mins_ago(
    timestamp: DateTime<FixedOffset>,
    now: DateTime<FixedOffset>,
    mins: i64,
) -> bool {
    // timestamp is in the past
    let timestamp_naive = timestamp.naive_utc();
    let now_naive = now.naive_utc();
    let past_max = timestamp_naive + Duration::minutes(mins);

    now_naive > past_max
}

pub fn is_greater(current: DateTime<FixedOffset>, previous: DateTime<FixedOffset>) -> bool {
    let mut result = false;
    let current_naive = current.naive_utc();
    let previous_naive = previous.naive_utc();

    if current_naive > previous_naive {
        result = true;
    }

    result
}

pub fn get_utc_timestamp_as_rfc2822() -> String {
    get_utc_time().to_rfc2822()
}

pub fn get_beijing_timestamp_as_rfc2822() -> String {
    let beijing_timezone = FixedOffset::east(8 * 3600);

    get_utc_time().with_timezone(&beijing_timezone).to_rfc2822()
}

pub fn timestamp_as_rfc2822_from_utc(utc: DateTime<Utc>) -> String {
    utc.to_rfc2822()
}

#[allow(dead_code)]
fn time_since_epoch_in_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_greater() {
        let previous = from_rfc_rfc3339("1996-12-19T16:39:57-08:00").unwrap();
        let current = from_rfc_rfc3339("2018-12-19T16:39:57-08:00").unwrap();

        assert_eq!(true, is_greater(current, previous));
    }

    #[test]
    fn test_less_than_mins_ago() {
        let mins = 30;

        let now = from_rfc_rfc3339(&Utc::now().to_rfc3339()).unwrap();
        let ts = Utc::now() - Duration::minutes(mins - 1);
        let timestamp = from_rfc_rfc3339(&ts.to_rfc3339()).unwrap();

        assert_eq!(false, occurred_more_than_mins_ago(timestamp, now, mins));
    }
}
