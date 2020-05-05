use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, FixedOffset, Utc};

pub mod json_request;

fn utc_time() -> chrono::DateTime<chrono::Utc> {
  let start = SystemTime::now();
  let since_the_epoch = start.duration_since(UNIX_EPOCH)
      .expect("Time went backwards");
  println!("{:?}", since_the_epoch);

  let start_in_ms = since_the_epoch.as_millis();
  println!("Current time in millis: {:?}", start_in_ms);

  let utc_time: DateTime<Utc> = Utc::now();

  utc_time
}

pub fn get_utc_timestamp() -> String {
  let current_timestamp = utc_time().to_rfc2822();

  current_timestamp
}

pub fn get_beijing_timestamp() -> String {
  let beijing_timezone = FixedOffset::east(8 * 3600);
  let beijing_timestamp = utc_time().with_timezone(&beijing_timezone).to_rfc2822();

  beijing_timestamp
}