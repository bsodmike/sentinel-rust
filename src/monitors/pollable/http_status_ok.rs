use crate::monitors::{Monitor, Monitorable, PollHTTPStatusOk, State};
use std::clone;
use std::error;
use std::fmt;

// PollHTTPStatusOk
impl fmt::Debug for Monitor<PollHTTPStatusOk> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Monitor<PollHTTPStatusOk>")
      .field("state", &self.state)
      .field("context", &self.context)
      .finish()
  }
}

impl Default for Monitor<PollHTTPStatusOk>
where
  State: Default,
{
  fn default() -> Monitor<PollHTTPStatusOk> {
    Monitor::<PollHTTPStatusOk> {
      context: PollHTTPStatusOk {},
      state: State::default(),
    }
  }
}

impl Copy for Monitor<PollHTTPStatusOk> {}

impl clone::Clone for Monitor<PollHTTPStatusOk> {
  fn clone(&self) -> Self {
    *self
  }
}

impl Monitorable for Monitor<PollHTTPStatusOk> {
  fn info(&self) -> String {
    String::from("Monitor<PollHTTPStatusOk>")
  }
  fn poll(&self) {
    println!("poll() for {:#?}", self);
  }
}

impl Monitor<PollHTTPStatusOk> {
  pub async fn new() -> Result<Monitor<PollHTTPStatusOk>, Box<dyn error::Error>> {
    let monitor: Monitor<PollHTTPStatusOk> = Monitor::<PollHTTPStatusOk>::default();
    Ok(monitor)
  }
}
