use crate::errors::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, FixedOffset, Utc};

pub fn get_utc_time() -> chrono::DateTime<chrono::Utc> {
  let utc_time: DateTime<Utc> = Utc::now();

  utc_time
}

/// Returns a `chrono::DateTime<chrono::FixedOffset>`
///
/// # Example
///
/// ~~~~
/// let previous = utils::parse_utc_time_to_rfc_rfc3339(utils::get_utc_time());
/// ~~~~
pub fn parse_utc_time_to_rfc_rfc3339(utc: DateTime<Utc>) -> DateTime<FixedOffset> {
  from_rfc_rfc3339(&utc.to_rfc3339()[..]).unwrap()
}

pub fn from_rfc_rfc3339(timestamp: &str) -> Result<DateTime<FixedOffset>, Error> {
  let result = DateTime::parse_from_rfc3339(timestamp)?;

  Ok(result)
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
  let current_timestamp = get_utc_time().to_rfc2822();

  current_timestamp
}

pub fn get_beijing_timestamp_as_rfc2822() -> String {
  let beijing_timezone = FixedOffset::east(8 * 3600);
  let beijing_timestamp = get_utc_time().with_timezone(&beijing_timezone).to_rfc2822();

  beijing_timestamp
}

pub fn timestamp_as_rfc2822_from_utc(utc: DateTime<Utc>) -> String {
  let current_timestamp = utc.to_rfc2822();

  current_timestamp
}

fn time_since_epoch_in_millis() -> u128 {
  let start = SystemTime::now();
  let since_the_epoch = start.duration_since(UNIX_EPOCH)
      .expect("Time went backwards");

  let since_the_epoch_in_ms = since_the_epoch.as_millis();

  since_the_epoch_in_ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_greater() {
      let previous = from_rfc_rfc3339("1996-12-19T16:39:57-08:00")
        .unwrap();
      let current = from_rfc_rfc3339("2018-12-19T16:39:57-08:00")
        .unwrap();
        
      assert_eq!(true, is_greater(current, previous));
    }
}