use std::fmt;
use crate::errors::Error;
use crate::dbslave;
use chrono::{DateTime, Utc, NaiveDateTime};
use crate::utils::time;

#[derive(Debug)]
struct WrappedDateTime(chrono::DateTime<chrono::Utc>);

impl std::default::Default for WrappedDateTime {
  fn default() -> Self {
    return WrappedDateTime(
      DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
    );
  }
}

#[derive(Default)]
struct AlertQueue<DataType> {
  queue: Vec<Alert<DataType>>
}

#[derive(Default, Debug)]
pub struct Alert<DataType> {
  data: DataType,
  created_at: WrappedDateTime,
}


impl<DataType> fmt::Debug for AlertQueue<DataType>
where
  DataType: fmt::Debug
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("AlertQueue<DataType>")
      .field("queue", &self.queue)
      .finish()
  }
}

impl<DataType> AlertQueue<DataType> {
  fn add(&mut self, item: Alert<DataType>) -> Result<(), Error> {
    self.queue.insert(0, item);

    Ok(())
  }
}

pub async fn add<DataType>(data: DataType, current_queue: Vec<Alert<DataType>>) -> Result<Vec<Alert<DataType>>, Error>
where
  DataType: Default + fmt::Debug
{
  let created_at = WrappedDateTime(
    time::get_utc_time()
  );

  let alert = Alert {
    data: data,
    created_at: created_at
  };
  println!("{:#?}", alert);

  let mut main_queue = AlertQueue::<DataType> {
    queue:   current_queue,
  };

  // let mut main_queue: AlertQueue<DataType> = AlertQueue::default();
  main_queue.add(alert)?;
  println!("{:#?}", main_queue);

  Ok(main_queue.queue)
}