use crate::errors::Error;
use crate::dbslave;
use chrono::{Utc};
use std::panic;
use crate::utils;

pub async fn run(slave_data: &mut dbslave::DBSlaveStatus) -> Result<(bool, String), Error> {
  let mut alertable: bool = false;
  let beijing_timestamp = utils::time::get_beijing_timestamp_as_rfc2822();
  let behind_master_max: u64 = 300;

  // Build status report
  let data = &slave_data;
  let parsed_seconds_behind_master = data.seconds_behind_master.parse::<u64>().unwrap();

  info!("Current time: {}", Utc::now());
  info!("💾 Slave IO running: {:#?}", data.slave_io_running);
  info!("💾 Slave SQL running: {:#?}", data.slave_sql_running);
  info!("💾 Slave seconds behind master: {:#?}", data.seconds_behind_master);

  let message = String::new() +
    &format!("\\n\\n*Timestamp (Beijing)*: {}\\n\\n", beijing_timestamp) +
    &format!("Master host: {}\\n", data.master_host) +
    &format!("Master user: {}\\n", data.master_user) +
    &format!("Slave IO running: {}\\n", data.slave_io_running) +
    &format!("Slave SQL running: {}\\n", data.slave_sql_running) +
    &format!("Master log file: {}\\n", data.master_log_file) +
    &format!("Master log pos: {}\\n", data.read_master_log_pos) +
    &format!("Relay log file: {}\\n", data.relay_log_file) +
    &format!("Relay log pos: {}\\n", data.relay_log_pos) +
    &format!("Relay master log file: {}\\n", data.relay_master_log_file) +
    &format!("Slave seconds behind master: {}\\n\\n", data.seconds_behind_master);

  if data.slave_io_running == String::from("No")
    || data.slave_sql_running == String::from("No")
    || parsed_seconds_behind_master > behind_master_max
  {
    alertable = true;
  }

  info!("alertable::run(): notify_now? {}", alertable);
  Ok((alertable, message))
}