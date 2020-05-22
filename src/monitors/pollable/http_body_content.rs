use crate::monitors::{Monitor, Monitorable, PollHTTPBodyContent, State};
use std::clone;
use std::error;
use std::fmt;

// PollHTTPBodyContent
impl fmt::Debug for Monitor<PollHTTPBodyContent> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Monitor<PollHTTPBodyContent>")
      .field("state", &self.state)
      .field("context", &self.context)
      .finish()
  }
}

impl Default for Monitor<PollHTTPBodyContent>
where
  State: Default,
{
  fn default() -> Monitor<PollHTTPBodyContent> {
    Monitor::<PollHTTPBodyContent> {
      context: PollHTTPBodyContent {},
      state: State::default(),
    }
  }
}

impl Copy for Monitor<PollHTTPBodyContent> {}

impl clone::Clone for Monitor<PollHTTPBodyContent> {
  fn clone(&self) -> Self {
    *self
  }
}

impl Monitorable for Monitor<PollHTTPBodyContent> {
  fn info(&self) -> String {
    String::from("Monitor<PollHTTPBodyContent>")
  }
  fn poll(&self) {
    println!("poll() for {:#?}", self);
  }
}

impl Monitor<PollHTTPBodyContent> {
  pub async fn new() -> Result<Monitor<PollHTTPBodyContent>, Box<dyn error::Error>> {
    let monitor: Monitor<PollHTTPBodyContent> = Monitor::<PollHTTPBodyContent>::default();
    Ok(monitor)
  }
}
