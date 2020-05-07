use crate::errors::Error;
use crate::dbslave;


pub async fn run(slave_data: &dbslave::DBSlaveStatus) -> Result<bool, Error> {
  let mut alertable: bool = false;

  if slave_data.slave_io_running == "No" ||
    slave_data.slave_sql_running == "No" ||
    slave_data.seconds_behind_master > 300 {
    alertable = true;
  }

  Ok(alertable)
}