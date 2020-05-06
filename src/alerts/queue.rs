use std::fmt;
use std::collections::VecDeque;
use crate::errors::Error;
use crate::monitor;

// #[derive(Debug)]
// struct WrappedDateTime(chrono::DateTime<chrono::Utc>);

// impl std::default::Default for WrappedDateTime {
//   fn default() -> Self {
//     return WrappedDateTime(
//       DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
//     );
//   }
// }

#[derive(Default)]
pub struct AlertQueue<DataType> {
  pub queue: Vec<monitor::Alert<DataType>>
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
  pub async fn add(&mut self, item: monitor::Alert<DataType>) -> Result<(), Error> {
    self.queue.insert(0, item);

    Ok(())
  }

  pub fn len(&self) -> Result<usize, Error> {
    Ok(self.queue.len())
  }

  pub fn take_first(&mut self) -> Result<monitor::Alert<DataType>, Error> {
    let first = self.queue.remove(0);

    Ok(first)
  }
}

pub async fn add<DataType>(data: DataType) -> Result<AlertQueue::<DataType>, Error>
where
  DataType: Default + fmt::Debug
{
  let main_queue: AlertQueue<DataType> = AlertQueue::default();

  Ok(main_queue)
}