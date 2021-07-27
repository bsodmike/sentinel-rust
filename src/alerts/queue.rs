use crate::dbslave::DBSlaveStatus;
use crate::errors::Error;
use crate::monitor::{Alert, SentAlerts};
use std::fmt;

#[derive(Default)]
pub struct AlertQueue<T> {
    pub queue: Vec<Alert<T>>,
}

impl<T> fmt::Debug for AlertQueue<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AlertQueue<T>")
            .field("queue", &self.queue)
            .finish()
    }
}

impl<T> AlertQueue<T> {
    pub async fn add(&mut self, item: Alert<T>) -> Result<(), Error> {
        self.queue.insert(0, item);

        Ok(())
    }

    pub fn len(&self) -> Result<usize, Error> {
        Ok(self.queue.len())
    }

    pub fn take_first(&mut self) -> Result<Alert<T>, Error> {
        let first = self.queue.remove(0);

        Ok(first)
    }
}

pub async fn add<T>() -> Result<AlertQueue<T>, Error>
where
    T: Default + fmt::Debug,
{
    let main_queue: AlertQueue<T> = AlertQueue::default();

    Ok(main_queue)
}


