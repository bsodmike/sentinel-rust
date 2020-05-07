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
use crate::log::LevelFilter;
use crate::log4rs::append::file::FileAppender;
use crate::log4rs::encode::pattern::PatternEncoder;
use crate::log4rs::config::{Appender, Config, Root};

mod notify;

#[derive(Debug)]
pub struct WrappedDateTime(chrono::DateTime<chrono::Utc>);

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
  pub fn naive_utc(&self) -> NaiveDateTime {
    self.0.naive_utc()
  }
}

impl WrappedDateTime {
  pub fn add_minutes(&self, mins: i64) -> WrappedDateTime {
    let naive_dt = self.0.naive_utc() + Duration::minutes(mins);
    let dt = utils::time::to_rfc_rfc3339(naive_dt).unwrap();
    let with_tz = dt.with_timezone(&Utc);

    WrappedDateTime::new(with_tz)
  }
}

#[test]
fn test_add_minutes() {
  let mins: i64 = 30;
  let naive_dt = Utc::now().naive_utc();
  let naive_wrapped = WrappedDateTime::default()
    .add_minutes(mins).
    naive_utc();

  let duration = naive_wrapped.signed_duration_since(naive_dt);
  assert_eq!(duration.num_minutes(), mins);
}


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

  // Prep Logging
  let logfile = FileAppender::builder()
  .encoder(Box::new(PatternEncoder::new("{l}: {d(%Y-%m-%d %H:%M:%S %Z)(utc)} - Line {L} File {f} - {m}\n")))
  .build("log/info.log")?;

  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder()
              .appender("logfile")
              .build(LevelFilter::Info))?;

  log4rs::init_config(config)?;
  // Logging END

  // Initialise main queue
  let mut queue = alerts::queue::add::<dbslave::DBSlaveStatus>().await.unwrap();
  info!("Queue initialised: {:#?}", queue);

  // Inititalise sent queue
  let mut sent_queue = SentAlerts::initialise().await.unwrap();
  info!("Sent Queue initialised {:#?}", sent_queue);

  let mut loop_counter: i64 = 0;

  // Primary run-loop
  loop {
    let now = time::Instant::now();
    info!("MAIN Loop Start 🐶🐶🐶🐶🐶🐶 {}", loop_counter);

    // Enabling mock data PREVENTS making actual calls to a live dbslave server.
    let enable_mock_data = true;

    let query_data: Vec<dbslave::DBSlaveStatus>;
    
    if enable_mock_data {
      query_data = dbslave::fetch_mocked::<dbslave::ConnectorMysql, Result<Vec<dbslave::DBSlaveStatus>, Error>>(dbslave::ConnectorMysql{}).await.unwrap();
    } else {
      query_data = dbslave::fetch::<dbslave::ConnectorMysql, Result<Vec<dbslave::DBSlaveStatus>, Error>>(dbslave::ConnectorMysql{}).await.unwrap();
    }

    let (data, db_status) = check_dbslave(query_data).await.unwrap();
    let (notify_now, slave_data) = alertable::run(data.clone()).await?;
  
    if notify_now {
      if sent_queue.sent_queue.len() <= 0 {
        info!("🐤🐤🐤🐤🐤🐤🐤🐤🐤 Sent queue is empty.");

        // Sent queue is empty, good to notify now.
        let mut alert = Alert {
          data: slave_data.clone(),
          template: dbslave_notification_template(&db_status).await.unwrap(),
          created_at: WrappedDateTime::default().to_rfc3339(),
        };  
  
        queue.add(alert).await?;
        info!("BR1: Main Queue: 🚀🚀🚀 Added alert to queue.");

        alert = Alert {
          data: slave_data,
          template: dbslave_notification_template(&db_status).await.unwrap(),
          created_at: WrappedDateTime::default().to_rfc3339(),
        };    
        sent_queue.add(alert).await?;
        info!("BR1: Sent Queue: 📦📦📦 Added sent alert to queue.");
        info!("BR1: 📦📦📦 Sent queue length {:#?}", sent_queue.sent_queue.len());
      } else {
        // Need to check last sent item to prevent spamming notifications
        info!("BR2: 📦📦📦 Sent queue length {:#?}", sent_queue.sent_queue.len());
        info!("BR2: About to `pop_back()` in Send queue");

        // NOTE: Notice a call is made to `VecDeque::pop_back()` and any calls to
        // `push_back()` will have a circular effect, i.e. the accumulator will
        // not grow, but certainly the most recently pushed will be stored.
        match sent_queue.sent_queue.pop_back() {
          Some(queue_item) => {
            info!("🐷🐷🐷🐷🐷🐷🐷 Sent Queue Inside SOME\n{:#?}", queue_item);
            info!("🐷🐷🐷🐷🐷🐷🐷 Some: Loop count {}", loop_counter);

            let parsed = utils::time::from_rfc_rfc3339(&queue_item.created_at);
            let parsed_ref = parsed.unwrap();
            let current_time =  utils::time::parse_utc_time_to_rfc_rfc3339(Utc::now());
            let alert_timestamp = parsed_ref;
  
            info!("Current time {:#?}", current_time.to_rfc2822());
            info!("Alert parsed timestamp {:#?}", parsed_ref.to_rfc2822());
  
            let threshold: i64 = 30;
            let process_alerts = utils::time::occurred_more_than_mins_ago(alert_timestamp, current_time, 30);
            info!("Alert occured before threshold({} mins)? {}", threshold, process_alerts);
  
            info!("🐷🐷🐷🐷🐷🐷🐷 Some: Process??? {}", process_alerts);

           
            // if process_alerts {
            if true {
              let mut alert = Alert {
                data: slave_data.clone(),
                template: dbslave_notification_template(&db_status).await.unwrap(),
                created_at: WrappedDateTime::default().to_rfc3339(),
              };    
      
              queue.add(alert).await.unwrap();
              info!("Some: Main Queue: 🚀🚀🚀 Added alert to queue.");
      
              alert = Alert {
                data: slave_data,
                template: dbslave_notification_template(&db_status).await.unwrap(),
                created_at: WrappedDateTime::default().to_rfc3339(),
              };    

              info!("Some: Sent Queue: 📦📦📦 Sent queue length {:#?}", sent_queue.sent_queue.len());
              sent_queue.add(alert).await.unwrap();
              info!("Some: Sent Queue: 📦📦📦 Added alert to SENT queue.");
              info!("Some: Sent Queue: 📦📦📦 Sent queue length {:#?}", sent_queue.sent_queue.len());
            }
          },
          None => {
            panic!("In none...")
          }
        }

      }
    }
 
    // 🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀

    // Threads handling
    let mut handler = Handler;
    let r_client = RtmClient::get_client(&mut handler).unwrap();

    let mut loop_done = false;
    while !loop_done {
      let queue_len = queue.len().unwrap();
      info!("🚀🚀🚀 Inside queue reduction loop. Queue length: {:#?} / Elapsed {:#?}", queue_len, now.elapsed());

      if queue_len > 0 {
        let current_alert = queue.queue.remove(0);

        let sender = r_client.sender().clone();
        // Thread
        let handle = thread::spawn(move || {
          info!("spawn thread: ...");
          
          match sender.send_message(current_alert) {
            Ok(value) => value,
            Err(error) => panic!("Err: {:#?}", error)
          };
          info!("🚀🚀🚀 Added alert to channel-queue. Elapsed {:#?}", now.elapsed());
        });

        // Disabled to prevent blocking the Main thread.
        // handle.join().unwrap();
      } else if queue_len <= 0 {
        loop_done = true;
      }
    }

    info!("🚀🚀🚀 Queue is now empty! et voilà! Elapsed {:#?}\n\n{:#?}", now.elapsed(), queue);

    // Main thread
    for i in 1..20 {
      // println!("Main thread: {}!", i);
      thread::sleep(time::Duration::from_millis(1));
    }
    info!("Main thread loop end: {:#?}", now.elapsed());

    // Notification processing loop
    let mut done = false;
    while !done {
      for alert in r_client.rx.try_iter() {
        info!("Received queue item {:#?}, elapsed {:#?}", alert, now.elapsed());
        
        // Send alert here.
        // notify::notify_slack(&alert.template).await;
      }

      done = true;
    }

    let pause_main: u64 = 5000;
    info!("🚀 Pausing main loop. Elapsed: {:#?}", now.elapsed());
    thread::sleep(time::Duration::from_millis(pause_main));
    info!("🚀 Continuing main loop. Elapsed: {:#?}", now.elapsed());

    info!("MAIN Loop Bottom 😸😸😸😸😸😸😸😸😸😸😸😸 {}", loop_counter);

    loop_counter = loop_counter + 1;
  }
}