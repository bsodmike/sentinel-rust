use crate::errors::Error;
use crate::dbslave;
use chrono::{Utc};

pub async fn run(slave_data: &dbslave::DBSlaveStatus) -> Result<bool, Error> {
  let mut alertable: bool = false;

  let behind_master: u64 = 300;

  info!("Current time: {}", Utc::now());
  info!("💾 Slave IO running: {:#?}", slave_data.slave_io_running);
  info!("💾 Slave SQL running: {:#?}", slave_data.slave_sql_running);
  info!("💾 Slave seconds behind master: {:#?}", slave_data.seconds_behind_master);

  if slave_data.slave_io_running == String::from("No") || slave_data.slave_sql_running == String::from("No") ||
    slave_data.seconds_behind_master > behind_master {
    alertable = true;
  }

  info!("alertable::run(): notify_now? {}", alertable);
  Ok(alertable)
}