use super::errors::Error;
use std::{thread, time};
use std::sync::Arc;
use chrono::{DateTime, Utc, NaiveDateTime};
use crate::dbslave;
use crate::dbslave::alertable;
use crate::utils;
use crate::alerts;

mod notify;

#[derive(Debug)]
struct WrappedDateTime(chrono::DateTime<chrono::Utc>);

impl std::default::Default for WrappedDateTime {
  fn default() -> Self {
    WrappedDateTime(
      DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
    )
  }
}

#[derive(Default, Debug)]
pub struct Alert<DataType> {
  data: DataType,
  created_at: WrappedDateTime,
}

async fn check_dbslave() -> Result<(dbslave::DBSlaveStatus, String), Error> {
  let beijing_timestamp = utils::time::get_beijing_timestamp_as_rfc2822();

  let result = dbslave::fetch::<dbslave::ConnectorMysql, Result<Vec<dbslave::DBSlaveStatus>, Error>>(dbslave::ConnectorMysql{})
    .await
    .unwrap();
  // println!("dbslave Result: {:#?}", result);

  let mut message = String::new();
  let data = &result[0];
  message.push_str(&format!("\\n\\n*Timestamp (Beijing)*: {}\\n\\n", beijing_timestamp)[..]);
  message.push_str(&(format!("Master host: {}\\n", &data.master_host[..]))[..]);
  message.push_str(&(format!("Master user: {}\\n", &data.master_user[..]))[..]);
  message.push_str(&(format!("Slave IO running: {}\\n", &data.slave_io_running[..]))[..]);
  message.push_str(&(format!("Slave SQL running: {}\\n", &data.slave_sql_running[..]))[..]);
  message.push_str(&(format!("Master log file: {}\\n", &data.master_log_file[..]))[..]);
  message.push_str(&(format!("Master log pos: {}\\n", data.read_master_log_pos))[..]);
  message.push_str(&(format!("Relay log file: {}\\n", &data.relay_log_file[..]))[..]);
  message.push_str(&(format!("Relay log pos: {}\\n", data.relay_log_pos))[..]);
  message.push_str(&(format!("Relay master log file: {}\\n", &data.relay_master_log_file[..]))[..]);
  message.push_str(&(format!("Slave seconds behind master: {}\\n\\n", data.seconds_behind_master))[..]);

  let returned_data = data.clone();
  Ok((returned_data, message))
}

async fn dbslave_notification_template(message: &str) -> Result<String, Error>{
    let mut template = String::new();
    template.push_str(&String::from(r#"
    {
      "blocks": [
        {
          "type": "section",
          "text": {
            "type": "mrkdwn",
            "text":  "Hello, this is a test broadcast from your friendly *Sentinel*."#));
    template.push_str(&message);
    template.push_str(&String::from(r#""
          }
        }
      ]
    }"#));

    println!("{}", template);

    Ok(template)
}

pub async fn begin_watch() -> Result<(), Error>{
  // "Night gathers, and now my watch begins. It shall not end until my death. I shall take no wife, hold no lands, father no children. I shall wear no crowns and win no glory. I shall live and die at my post. I am the sword in the darkness. I am the watcher on the walls. I am the shield that guards the realms of men. I pledge my life and honor to the Night's Watch, for this night and all the nights to come."
  // ―The Night's Watch oath

  let (data, message) = check_dbslave().await.unwrap();
  let trigger_alert = alertable::run(data.clone()).await?;
  
  // let initial = vec![Alert::<dbslave::DBSlaveStatus>::default()];
  let mut queue = alerts::queue::add::<dbslave::DBSlaveStatus>(data.clone()).await?;

  let created_at = WrappedDateTime(
    utils::time::get_utc_time()
  );
  let created_at2 = WrappedDateTime(
    utils::time::get_utc_time()
  );
  let alert = Alert {
    data: data.clone(),
    created_at
  };
  let alert2 = Alert {
    data: data.clone(),
    created_at: created_at2
  };
  queue.add(alert).await?;
  queue.add(alert2).await?;

  println!("{:#?}", queue);
  // println!("Output: {}", data.slave_io_running);

  let delay = time::Duration::from_millis(1);
  let now = time::Instant::now();

  let thread_delay = Arc::new(1);

  let mut done = false;
  while !done {
    let thread_delay = Arc::clone(&thread_delay);

    let handle = thread::spawn(move || {
      for j in 1..20 {
        println!("hi number {} from the spawned thread!", j);
        thread::sleep(time::Duration::from_millis(*thread_delay));
      }
      println!("Thread end: {:#?}", now.elapsed());
    });

    for i in 1..10 {
      println!("hi number {} from the main thread!", i);
      thread::sleep(time::Duration::from_millis(1));
    }
    println!("Loop end: {:#?}", now.elapsed());

    handle.join().unwrap();
    done = true;
  }

  println!("Elapsed: {:#?}", now.elapsed());

  let template = dbslave_notification_template(&message).await.unwrap();
  // notify::notify_slack(&template).await;

  Ok(())
}