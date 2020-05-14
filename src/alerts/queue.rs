use crate::dbslave::DBSlaveStatus;
use crate::errors::Error;
use crate::monitor::{Alert, SentAlerts};
use std::collections::VecDeque;
use std::fmt;

#[derive(Default)]
pub struct AlertQueue<DataType> {
    pub queue: Vec<Alert<DataType>>,
}

impl<DataType> fmt::Debug for AlertQueue<DataType>
where
    DataType: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AlertQueue<DataType>")
            .field("queue", &self.queue)
            .finish()
    }
}

impl<DataType> AlertQueue<DataType> {
    pub async fn add(&mut self, item: Alert<DataType>) -> Result<(), Error> {
        self.queue.insert(0, item);

        Ok(())
    }

    pub fn len(&self) -> Result<usize, Error> {
        Ok(self.queue.len())
    }

    pub fn take_first(&mut self) -> Result<Alert<DataType>, Error> {
        let first = self.queue.remove(0);

        Ok(first)
    }
}

pub async fn add<DataType>() -> Result<AlertQueue<DataType>, Error>
where
    DataType: Default + fmt::Debug,
{
    let main_queue: AlertQueue<DataType> = AlertQueue::default();

    Ok(main_queue)
}

impl SentAlerts {
    pub async fn initialise() -> Result<SentAlerts, Error> {
        let vec: VecDeque<Alert<DBSlaveStatus>> = VecDeque::new();

        let sent_queue = SentAlerts { sent_queue: vec };

        Ok(sent_queue)
    }

    pub async fn add(&mut self, alert: Alert<DBSlaveStatus>) -> Result<(), Error> {
        self.sent_queue.push_back(alert);

        Ok(())
    }

    pub async fn sent(&mut self) -> Result<&VecDeque<Alert<DBSlaveStatus>>, Error> {
        Ok(&self.sent_queue)
    }
}
