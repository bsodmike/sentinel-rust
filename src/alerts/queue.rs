use std::fmt;
use std::collections::VecDeque;
use crate::errors::Error;
use crate::monitor;
use crate::dbslave;

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

pub async fn add<DataType>() -> Result<AlertQueue::<DataType>, Error>
where
  DataType: Default + fmt::Debug
{
  let main_queue: AlertQueue<DataType> = AlertQueue::default();

  Ok(main_queue)
}

impl monitor::SentAlerts {
  pub async fn initialise() -> Result<monitor::SentAlerts, Error> {
    let vec: VecDeque<monitor::Alert<dbslave::DBSlaveStatus>> = VecDeque::new();

    let sent_queue = monitor::SentAlerts {
      sent_queue: vec
    };
    
    Ok(sent_queue)
  }

  pub async fn add(&mut self, alert: monitor::Alert<dbslave::DBSlaveStatus>) -> Result<(), Error> {
    self.sent_queue.push_back(alert);

    Ok(())
  }

  pub async fn sent(&mut self) -> Result<&VecDeque<monitor::Alert<dbslave::DBSlaveStatus>>, Error> {
    Ok(&self.sent_queue)
  }
}