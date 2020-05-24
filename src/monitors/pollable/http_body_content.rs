use crate::monitors::{
  AlertContext, Monitor, Monitorable, PollAlert, PollHTTPBodyContent, Pollable, State,
};
use crate::wrappers::chrono::WrappedDateTime;
use std::error;
use std::fmt;

// PollHTTPBodyContent
impl fmt::Debug for Monitor<PollHTTPBodyContent> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Monitor<PollHTTPBodyContent>")
      .field("current_state", &self.current_state)
      .field("previous_state", &self.previous_state)
      .field("current_state_timestamp", &self.current_state_timestamp)
      .field("previous_state_timestamp", &self.previous_state_timestamp)
      .field("context", &self.context)
      .finish()
  }
}

impl fmt::Debug for PollAlert<PollHTTPBodyContent> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("PollAlert<PollHTTPBodyContent>")
      .field("alert_context", &self.alert_context)
      .field("cx", &self.cx)
      .finish()
  }
}

impl fmt::Display for Monitor<PollHTTPBodyContent> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Default for Monitor<PollHTTPBodyContent>
where
  State: Default,
{
  fn default() -> Monitor<PollHTTPBodyContent> {
    Monitor::<PollHTTPBodyContent> {
      context: PollHTTPBodyContent {},
      current_state: State::default(),
      previous_state: State::default(),
      current_state_timestamp: WrappedDateTime::default(),
      previous_state_timestamp: WrappedDateTime::default(),
    }
  }
}

impl Pollable for PollAlert<PollHTTPBodyContent> {
  fn debug(&self) -> String {
    format!("{:#?}", self)
  }
}

impl Monitorable for Monitor<PollHTTPBodyContent> {
  fn info(&self) -> String {
    String::from("Monitor<PollHTTPBodyContent>")
  }
  fn poll<'a>(&mut self) -> Box<dyn Pollable> {
    println!("poll() for {:#?}", self);

    let new_state = State::Up;
    let alert: PollAlert<PollHTTPBodyContent> = PollAlert {
      alert_context: AlertContext::StateChange(new_state),
      cx: *self,
    };

    self.update_state(new_state);
    Box::new(alert)
  }
}

impl Monitor<PollHTTPBodyContent> {
  pub async fn new() -> Result<Monitor<PollHTTPBodyContent>, Box<dyn error::Error>> {
    let monitor: Monitor<PollHTTPBodyContent> = Monitor::<PollHTTPBodyContent>::default();
    Ok(monitor)
  }

  fn update_state(&mut self, new_state: State) {
    self.previous_state = self.current_state;
    self.current_state = new_state;

    self.previous_state_timestamp = self.current_state_timestamp;
    self.current_state_timestamp = WrappedDateTime::default();
  }
}
