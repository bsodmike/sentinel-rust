use crate::monitors::{AlertContext, Monitor, Monitorable, PollAlert, PollHTTPStatusOk, State};
use std::error;
use std::fmt;

// PollHTTPStatusOk
impl fmt::Debug for Monitor<PollHTTPStatusOk> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Monitor<PollHTTPStatusOk>")
      .field("current_state", &self.current_state)
      .field("previous_state", &self.previous_state)
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
      current_state: State::default(),
      previous_state: State::default(),
    }
  }
}

impl Monitorable for Monitor<PollHTTPStatusOk> {
  fn info(&self) -> String {
    String::from("Monitor<PollHTTPStatusOk>")
  }
  fn poll<'a>(&mut self, alert: &'a mut PollAlert) -> &'a mut PollAlert {
    println!("poll() for {:#?}", self);

    let new_state = State::Down;
    self.update_state(new_state);
    alert.alert_context = AlertContext::StateChange(new_state);
    alert
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
  }
}
