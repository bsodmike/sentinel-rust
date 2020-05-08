use chrono::{DateTime, Utc, NaiveDateTime, Duration};
use crate::utils;

#[derive(Debug)]
pub struct WrappedDateTime(chrono::DateTime<chrono::Utc>);

impl WrappedDateTime {
  pub fn new(dt: chrono::DateTime<chrono::Utc>) -> WrappedDateTime {
    WrappedDateTime(dt)
  }
}

impl std::default::Default for WrappedDateTime {
  fn default() -> WrappedDateTime {
    let utc = Utc::now().with_timezone(&Utc);
    WrappedDateTime::new(utc)
  }
}

impl WrappedDateTime {
  pub fn to_rfc2822(&self) -> String {
    self.0.to_rfc2822()
  }
}

impl WrappedDateTime {
  pub fn to_rfc3339(&self) -> String {
    self.0.to_rfc3339()
  }
}

impl WrappedDateTime {
  pub fn naive_utc(&self) -> NaiveDateTime {
    self.0.naive_utc()
  }
}

impl WrappedDateTime {
  pub fn add_minutes(&self, mins: i64) -> WrappedDateTime {
    let naive_dt = self.0.naive_utc() + Duration::minutes(mins);
    let dt = utils::time::to_rfc_rfc3339(naive_dt).unwrap();
    let with_tz = dt.with_timezone(&Utc);

    WrappedDateTime::new(with_tz)
  }
}

#[test]
fn test_add_minutes() {
  let mins: i64 = 30;
  let naive_dt = Utc::now().naive_utc();
  let naive_wrapped = WrappedDateTime::default()
    .add_minutes(mins).
    naive_utc();

  let duration = naive_wrapped.signed_duration_since(naive_dt);
  assert_eq!(duration.num_minutes(), mins);
}