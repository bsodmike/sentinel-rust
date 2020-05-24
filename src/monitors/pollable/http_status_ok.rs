use crate::monitors::{
  AlertContext, Monitor, Monitorable, PollAlert, PollHTTPStatusOk, Pollable, State,
};
use crate::wrappers::chrono::WrappedDateTime;
use std::error;
use std::fmt;

// PollHTTPStatusOk
impl fmt::Debug for Monitor<PollHTTPStatusOk> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Monitor<PollHTTPStatusOk>")
      .field("current_state", &self.current_state)
      .field("previous_state", &self.previous_state)
      .field("current_state_timestamp", &self.current_state_timestamp)
      .field("previous_state_timestamp", &self.previous_state_timestamp)
      .field("context", &self.context)
      .finish()
  }
}

impl fmt::Debug for PollAlert<PollHTTPStatusOk> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("PollAlert<PollHTTPStatusOk>")
      .field("alert_context", &self.alert_context)
      .field("cx", &self.cx)
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
      current_state: State::default(),
      previous_state: State::default(),
      current_state_timestamp: WrappedDateTime::default(),
      previous_state_timestamp: WrappedDateTime::default(),
    }
  }
}

impl Pollable for PollAlert<PollHTTPStatusOk> {
  fn debug(&self) -> String {
    format!("{:#?}", self)
  }
}

impl Monitorable for Monitor<PollHTTPStatusOk> {
  fn info(&self) -> String {
    String::from("Monitor<PollHTTPStatusOk>")
  }
  fn poll<'a>(&mut self) -> Box<dyn Pollable> {
    println!("poll() for {:#?}", self);

    let new_state = State::Down;
    let alert: PollAlert<PollHTTPStatusOk> = PollAlert {
      alert_context: AlertContext::StateChange(new_state),
      cx: *self,
    };

    self.update_state(new_state);
    Box::new(alert)
  }
}

impl Monitor<PollHTTPStatusOk> {
  pub async fn new() -> Result<Monitor<PollHTTPStatusOk>, Box<dyn error::Error>> {
    let monitor: Monitor<PollHTTPStatusOk> = Monitor::<PollHTTPStatusOk>::default();
    Ok(monitor)
  }

  fn update_state(&mut self, new_state: State) {
    self.previous_state = self.current_state;
    self.current_state = new_state;

    self.previous_state_timestamp = self.current_state_timestamp;
    self.current_state_timestamp = WrappedDateTime::default();
  }
}
