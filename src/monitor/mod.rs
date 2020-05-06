use super::errors::Error;
use std::{thread, time};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use chrono::{DateTime, Utc, NaiveDateTime};
use crate::dbslave;
use crate::dbslave::alertable;
use crate::utils;
use crate::alerts;

#[macro_use]
use crate::log;

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
  template: String,
  created_at: WrappedDateTime,
}

/// Used for passing messages in channels
#[derive(Debug)]
enum WsMessage {
    Close,
    Text(String),
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

    println!("{}", template);

    Ok(template)
}

pub async fn begin_watch() -> Result<(), Error>{
  // "Night gathers, and now my watch begins. It shall not end until my death. I shall take no wife, hold no lands, father no children. I shall wear no crowns and win no glory. I shall live and die at my post. I am the sword in the darkness. I am the watcher on the walls. I am the shield that guards the realms of men. I pledge my life and honor to the Night's Watch, for this night and all the nights to come."
  // ―The Night's Watch oath

  // Enabling mock data PREVENTS making actual calls to a live dbslave server.
  let enable_mock_data = true;

  let query_data: Vec<dbslave::DBSlaveStatus>;
  
  if enable_mock_data {
    query_data = dbslave::fetch_mocked::<dbslave::ConnectorMysql, Result<Vec<dbslave::DBSlaveStatus>, Error>>(dbslave::ConnectorMysql{}).await.unwrap();
  } else {
    query_data = dbslave::fetch::<dbslave::ConnectorMysql, Result<Vec<dbslave::DBSlaveStatus>, Error>>(dbslave::ConnectorMysql{}).await.unwrap();
  }

  let (data, message) = check_dbslave(query_data).await.unwrap();
  // let trigger_alert = alertable::run(data.clone()).await?;
  
  // let initial = vec![Alert::<dbslave::DBSlaveStatus>::default()];
  let mut queue = alerts::queue::add::<dbslave::DBSlaveStatus>(data.clone()).await.unwrap();

  let created_at = WrappedDateTime(
    utils::time::get_utc_time()
  );
  let created_at2 = WrappedDateTime(
    utils::time::get_utc_time()
  );
  let alert = Alert {
    data: data.clone(),
    template: dbslave_notification_template(&message).await.unwrap(),
    created_at
  };
  let alert2 = Alert {
    data: data.clone(),
    template: dbslave_notification_template(&message).await.unwrap(),
    created_at: created_at2
  };
  queue.add(alert).await?;
  queue.add(alert2).await?;


  // Threads.
  let delay = time::Duration::from_millis(1);
  let now = time::Instant::now();

  let mut handler = Handler;
  let r_client = RtmClient::get_client(&mut handler).unwrap();

  let mut loop_done = false;
  while !loop_done {
    println!("🚀🚀🚀 Inside queue reduction loop. Elapsed {:#?}", now.elapsed());
    let queue_len = queue.len().unwrap();
    if queue_len > 0 {
      let current_alert = queue.queue.remove(0);

      let sender = r_client.sender().clone();
      // Thread
      let handle = thread::spawn(move || {
        println!("spawn thread: ...");
        
        sender.send_message(current_alert);
        println!("🚀🚀🚀 Added alert to channel-queue. Elapsed {:#?}", now.elapsed());
      });
      handle.join().unwrap();
    } else if queue_len <= 0 {
      loop_done = true;
    }
  }

  println!("🚀🚀🚀 Queue is now empty! et voila! Elapsed {:#?}\n\n{:#?}", now.elapsed(), queue);

  // Main thread
  for i in 1..10 {
    println!("Main thread: {}!", i);
    thread::sleep(time::Duration::from_millis(1));
  }
  println!("Main thread loop end: {:#?}", now.elapsed());

  // Notification processing loop
  let done = false;
  while !done {
    for alert in r_client.rx.try_iter() {
      println!("Received queue item {:#?}, elapsed {:#?}", alert, now.elapsed());
      
      // Send alert here.
      // notify::notify_slack(&alert.template).await;
    }
  }


  println!("Elapsed: {:#?}", now.elapsed());

  Ok(())
}