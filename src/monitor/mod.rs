use super::errors::Error;
use std::{thread, time};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::collections::VecDeque;
use chrono::{DateTime, Utc, NaiveDateTime, Duration};
use crate::dbslave;
use crate::dbslave::alertable;
use crate::utils;
use crate::alerts;

#[macro_use]
use crate::log;

mod notify;

#[derive(Debug)]
struct WrappedDateTime(chrono::DateTime<chrono::Utc>);

impl WrappedDateTime {
  pub fn new(dt: chrono::DateTime<chrono::Utc>) -> WrappedDateTime {
    WrappedDateTime(dt)
  }
}

impl std::default::Default for WrappedDateTime {
  fn default() -> WrappedDateTime {
    let utc = Utc::now().with_timezone(&Utc);
    WrappedDateTime::new(utc)
  }
}

impl WrappedDateTime {
  pub fn to_rfc2822(&self) -> String {
    self.0.to_rfc2822()
  }
}

impl WrappedDateTime {
  pub fn to_rfc3339(&self) -> String {
    self.0.to_rfc3339()
  }
}

impl WrappedDateTime {
  pub fn add_minutes(&self, mins: i64) -> WrappedDateTime {
    let naive_dt = self.0.naive_utc() + Duration::minutes(mins);
    let dt = utils::time::to_rfc_rfc3339(naive_dt).unwrap();
    let with_tz = dt.with_timezone(&Utc);
    
    println!(">>>>>>> 🚀 dt.with_timezone(&Utc) {:#?}", with_tz.to_rfc2822());
    println!(">>>>>>>>>> add {} mins", -(mins));
    println!(">>>>>>> NOW {:#?}", Utc::now().to_rfc2822());


    WrappedDateTime::new(with_tz)
  }
}

// impl std::convert::Into<String> for WrappedDateTime {
//   fn into(self) -> String {
//     self
//   }
// }

#[derive(Default, Debug)]
pub struct Alert<DataType> {
  data: DataType,
  template: String,
  pub created_at: String,
}

/// FIXME needs usage
// #[derive(Default, Debug)]
// pub enum Alerts {
//   DBSlave(Alert<dbslave::DBSlaveStatus>),
// }

#[derive(Default, Debug)]
pub struct SentAlerts {
  pub sent_queue: VecDeque<Alert<dbslave::DBSlaveStatus>>
}

// Handler
pub struct Handler;

/// The actual messaging client.
pub struct RtmClient {
  sender: Sender,
  rx: mpsc::Receiver<Alert<dbslave::DBSlaveStatus>>,
}

/// Thread-safe API for sending messages asynchronously
#[derive(Clone)]
pub struct Sender {
  tx: mpsc::Sender<Alert<dbslave::DBSlaveStatus>>,
  msg_num: Arc<AtomicUsize>,
}

pub trait EventHandler {
}
impl EventHandler for Handler {
}

impl Sender {
  /// Get the next message id
  ///
  /// A value returned from this method *must* be included in the JSON payload
  /// (the `id` field) when constructing your own message.
  pub fn get_msg_uid(&self) -> usize {
      self.msg_num.fetch_add(1, Ordering::SeqCst)
  }

  /// Send a raw message
  ///
  /// Must set `message.id` using result of `get_msg_id()`.
  pub fn send(&self, raw: Alert<dbslave::DBSlaveStatus>) -> Result<(), Error> {
      self.tx
          .send(raw)
          .map_err(|err| Error::Internal(format!("{}", err)))?;
      Ok(())
  }

  /// Send a message
  /// 
  /// Only valid after `RtmClient::run`.
  pub fn send_message(&self, msg: Alert<dbslave::DBSlaveStatus>) -> Result<usize, Error> {
    let n = self.get_msg_uid();

    self.send(msg)
        .map_err(|err| Error::Internal(format!("{}", err)))?;

    Ok(n)
  }
}

impl RtmClient {
  /// Runs the message receive loop
  pub fn get_client<T: EventHandler>(handler: &mut T) -> Result<RtmClient, Error> {
    // setup channels for passing messages
    let (tx, rx) = mpsc::channel::<Alert<dbslave::DBSlaveStatus>>();
    let sender = Sender {
        tx,
        msg_num: Arc::new(AtomicUsize::new(0)),
    };

    let client = RtmClient {
      sender,
      rx,
    };

    Ok(client)
  }

  /// Get a reference thread-safe cloneable message `Sender`
  pub fn sender(&self) -> &Sender {
    &self.sender
  }
}

async fn check_dbslave(query_data: Vec<dbslave::DBSlaveStatus>) -> Result<(dbslave::DBSlaveStatus, String), Error> {
  let beijing_timestamp = utils::time::get_beijing_timestamp_as_rfc2822();

  let result = query_data;
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

    // println!("{}", template);

    Ok(template)
}

pub async fn begin_watch() -> Result<(), Error>{
  // "Night gathers, and now my watch begins. It shall not end until my death. I shall take no wife, hold no lands, father no children. I shall wear no crowns and win no glory. I shall live and die at my post. I am the sword in the darkness. I am the watcher on the walls. I am the shield that guards the realms of men. I pledge my life and honor to the Night's Watch, for this night and all the nights to come."
  // ―The Night's Watch oath

  // Initialise main queue
  let mut queue = alerts::queue::add::<dbslave::DBSlaveStatus>().await.unwrap();
  let mut sent_queue = SentAlerts::initialise().await.unwrap();
  println!("Queue initialised: {:#?}", queue);
  println!("Sent queue initialised: {:#?}", sent_queue);

  // Primary run-loop
  loop {

    // Enabling mock data PREVENTS making actual calls to a live dbslave server.
    let enable_mock_data = true;

    let query_data: Vec<dbslave::DBSlaveStatus>;
    
    if enable_mock_data {
      query_data = dbslave::fetch_mocked::<dbslave::ConnectorMysql, Result<Vec<dbslave::DBSlaveStatus>, Error>>(dbslave::ConnectorMysql{}).await.unwrap();
    } else {
      query_data = dbslave::fetch::<dbslave::ConnectorMysql, Result<Vec<dbslave::DBSlaveStatus>, Error>>(dbslave::ConnectorMysql{}).await.unwrap();
    }

    let (data, db_status) = check_dbslave(query_data).await.unwrap();
    let (alertable, slave_data) = alertable::run(data.clone()).await?;
    let mut process_alert = false;

    // let timestamp = utils::time::parse_utc_time_to_rfc_rfc3339(Utc::now());
    // let  = Utc::now() + Duration::minutes(30);

    let dummy_created_at = WrappedDateTime::default().add_minutes(-29);
    let dummy_created_at_rfc3339 = dummy_created_at.to_rfc3339();
    println!("Dummy created at {:#?}", dummy_created_at.to_rfc2822());
    println!("Dummy created at as RFC3339 {:#?}", dummy_created_at_rfc3339);

    // TEMP
    let mut test_alert = Alert {
      data: slave_data.clone(),
      template: dbslave_notification_template(&db_status).await.unwrap(),
      created_at: dummy_created_at_rfc3339
    };
    
    println!("-----FIELD {:#?}", test_alert.created_at);
    let parsed = utils::time::from_rfc_rfc3339(&test_alert.created_at).unwrap();
    println!("-----parsed {:#?}", parsed.to_rfc2822());

    sent_queue.add(test_alert).await.unwrap();
    println!("🎃🎃🎃🎃🎃🎃 Sent Queue: Added sent alert to queue.");
    // TEMP

    let sent_queue_lenth = sent_queue.sent_queue.len();
    println!("🎃🎃🎃 Sent Queue LENGTH {}", sent_queue_lenth);

    if true {
      match sent_queue.sent_queue.pop_back() {
        Some(queue_item) => {
          let parsed = utils::time::from_rfc_rfc3339(&queue_item.created_at);
          let parsed_ref = parsed.unwrap();
          let current_time =  utils::time::parse_utc_time_to_rfc_rfc3339(Utc::now());
          let alert_timestamp = parsed_ref;

          println!("Current time {:#?}", current_time.to_rfc2822());
          println!("Alert parsed timestamp {:#?}", parsed_ref.to_rfc2822());

          let process_alerts_if_false = utils::time::occurred_more_than_mins_ago(alert_timestamp, current_time, 30);
          println!("Alert occured before threshold? {}", process_alerts_if_false);

          if !process_alerts_if_false {
            let mut alert = Alert {
              data: slave_data.clone(),
              template: dbslave_notification_template(&db_status).await.unwrap(),
              created_at: WrappedDateTime::default().to_rfc3339(),
            };    
    
            queue.add(alert).await?;
            println!("🚀🚀🚀 Added alert to queue.");
            println!("Queue: {:#?}", queue);
    
            alert = Alert {
              data: slave_data,
              template: dbslave_notification_template(&db_status).await.unwrap(),
              created_at: WrappedDateTime::default().to_rfc3339(),
            };    
            sent_queue.add(alert);
            println!("Sent Queue: Added sent alert to queue.");
            println!("Sent queue {:#?}", sent_queue.sent().await.unwrap());
            println!("Sent queue length {:#?}", sent_queue.sent_queue.len());
          }
        },
        None => {}
      }


    }
    // 🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀

    // Threads handling
    let mut handler = Handler;
    let r_client = RtmClient::get_client(&mut handler).unwrap();
    let now = time::Instant::now();

    let mut loop_done = false;
    while !loop_done {
      let mut queue_len = queue.len().unwrap();
      println!("🚀🚀🚀 Inside queue reduction loop. Queue length: {:#?} / Elapsed {:#?}", queue_len, now.elapsed());

      if queue_len > 0 {
        let current_alert = queue.queue.remove(0);

        let sender = r_client.sender().clone();
        // Thread
        let handle = thread::spawn(move || {
          println!("spawn thread: ...");
          
          thread::sleep(time::Duration::from_millis(60000));
          sender.send_message(current_alert);
          println!("🚀🚀🚀 Added alert to channel-queue. Elapsed {:#?}", now.elapsed());
        });

        // Disabled to prevent blocking the Main thread.
        // handle.join().unwrap();
      } else if queue_len <= 0 {
        loop_done = true;
      }
    }

    println!("🚀🚀🚀 Queue is now empty! et voilà! Elapsed {:#?}\n\n{:#?}", now.elapsed(), queue);

    // Main thread
    for i in 1..20 {
      println!("Main thread: {}!", i);
      thread::sleep(time::Duration::from_millis(1));
    }
    println!("Main thread loop end: {:#?}", now.elapsed());

    // Notification processing loop
    let mut done = false;
    while !done {
      for alert in r_client.rx.try_iter() {
        println!("Received queue item {:#?}, elapsed {:#?}", alert, now.elapsed());
        
        // Send alert here.
        // notify::notify_slack(&alert.template).await;
      }

      done = true;
    }

    println!("🚀 Pausing main loop. Elapsed: {:#?}", now.elapsed());
    thread::sleep(time::Duration::from_millis(10000));
    println!("Continuing now. Elapsed {:#?}", now.elapsed());
  }
}