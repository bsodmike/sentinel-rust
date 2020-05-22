use crate::monitors::{AlertContext, Monitor, Monitorable, PollAlert, PollHTTPBodyContent, State};
use std::error;
use std::fmt;

// PollHTTPBodyContent
impl fmt::Debug for Monitor<PollHTTPBodyContent> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Monitor<PollHTTPBodyContent>")
      .field("current_state", &self.current_state)
      .field("previous_state", &self.previous_state)
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
      current_state: State::default(),
      previous_state: State::default(),
    }
  }
}

impl Monitorable for Monitor<PollHTTPBodyContent> {
  fn info(&self) -> String {
    String::from("Monitor<PollHTTPBodyContent>")
  }
  fn poll<'a>(&mut self, alert: &'a mut PollAlert) -> &'a mut PollAlert {
    println!("poll() for {:#?}", self);

    let new_state = State::Up;
    self.update_state(new_state);
    alert.alert_context = AlertContext::StateChange(new_state);
    alert
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
  }
}
