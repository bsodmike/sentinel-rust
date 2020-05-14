use super::errors::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;

pub struct Message;

// Handler
pub struct Handler;

/// The actual messaging client.
pub struct RtmClient {
  sender: Sender,
  rx: mpsc::Receiver<Message>,
}

/// Thread-safe API for sending messages asynchronously
#[derive(Clone)]
pub struct Sender {
  tx: mpsc::Sender<Message>,
  msg_num: Arc<AtomicUsize>,
}

pub trait EventHandler {}
impl EventHandler for Handler {}

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
  pub fn send(&self, raw: Message) -> Result<(), Error> {
    self
      .tx
      .send(raw)
      .map_err(|err| Error::Internal(format!("{}", err)))?;
    Ok(())
  }

  /// Send a message
  ///
  /// Only valid after `RtmClient::run`.
  pub fn send_message(&self, msg: Message) -> Result<usize, Error> {
    let n = self.get_msg_uid();

    self
      .send(msg)
      .map_err(|err| Error::Internal(format!("{}", err)))?;

    Ok(n)
  }
}

impl RtmClient {
  /// Runs the message receive loop
  pub fn get_client<T: EventHandler>(_: &mut T) -> Result<RtmClient, Error> {
    // setup channels for passing messages
    let (tx, rx) = mpsc::channel::<Message>();
    let sender = Sender {
      tx,
      msg_num: Arc::new(AtomicUsize::new(0)),
    };

    let client = RtmClient { sender, rx };

    Ok(client)
  }

  /// Get a reference thread-safe cloneable message `Sender`
  pub fn sender(&self) -> &Sender {
    &self.sender
  }
}
