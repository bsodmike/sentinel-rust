use std::fmt;
use crate::errors::Error;
use crate::dbslave;
use chrono::{DateTime, Utc};
use crate::utils::time;

#[derive(Default)]
struct AlertQueue<DataType> {
  queue: Vec<Alert<DataType>>
}

#[derive(Debug)]
struct Alert<DataType> {
  data: DataType,
  created_at: DateTime<Utc>,
}

// impl std::fmt::Display for AlertQueue<DataType> {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     write!(f, "({:#?})", self.queue)
//   }
// }

// impl<DataType> fmt::Debug for std::vec::Vec<Alert<DataType>>
// {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     // self.fmt.debug_set().entries(self.0.iter()).finish()
//     f.debug_set().entries(self.as_mut().into_iter()).finish()
//   }
// }

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
  fn add(&mut self, item: Alert<DataType>) {
    self.queue.insert(0, item);
  }
}

impl<DataType> Alert<DataType> {
  fn info(&self) -> Result<(), Error> {
    println!("Debugging in info");

    Ok(())
  }
}

pub async fn add<DataType>(data: DataType) -> Result<(), Error>
where
  DataType: Default + fmt::Debug
{
  let alert = Alert {
    data: data,
    created_at: time::get_utc_time()
  };
  // println!("{:#?}", alert);

  let mut main_queue: AlertQueue<DataType> = AlertQueue::default();
  main_queue.add(alert);


  println!("{:#?}", main_queue);
  // alert.info()?;

  println!("Debugging in add");
  Ok(())
}