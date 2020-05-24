use crate::wrappers::chrono::WrappedDateTime;
use std::default::Default;
use std::error;
use std::fmt;
use std::marker::Copy;

mod pollable;

#[derive(Debug, Copy, Clone)]
pub enum State {
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum AlertContext {
    PollFailure,
    StateChange(State),
    UnknownFailure,
}

#[derive(Copy, Clone)]
pub struct PollAlert<T> {
    alert_context: AlertContext,
    cx: Monitor<T>,
}

#[derive(Debug, Copy, Clone)]
pub struct PollHTTPBodyContent;

#[derive(Debug, Copy, Clone)]
pub struct PollHTTPStatusOk;

#[derive(Debug, Copy, Clone)]
pub struct PollMySQLDBSlave;

pub trait Pollable {
    fn debug(&self) -> String;
}

pub trait Monitorable {
    fn info(&self) -> String;
    fn poll<'a>(&mut self) -> Box<dyn Pollable>;
}

#[derive(Copy, Clone)]
pub struct Monitor<T> {
    context: T,
    current_state: State,
    previous_state: State,
    current_state_timestamp: WrappedDateTime,
    previous_state_timestamp: WrappedDateTime,
}

pub struct Monitored {
    pub enabled: Vec<Box<dyn Monitorable>>,
}

impl Default for State {
    fn default() -> Self {
        State::Unknown
    }
}

impl Default for Monitored {
    fn default() -> Monitored {
        Monitored {
            enabled: Vec::new(),
        }
    }
}

impl fmt::Display for AlertContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Monitored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.enabled).finish()
    }
}

impl fmt::Debug for dyn Monitorable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Monitorable")
            .field("info", &self.info())
            .finish()
    }
}

impl Monitored {
    pub async fn new() -> Result<Monitored, Box<dyn error::Error>> {
        Ok(Monitored::default())
    }

    pub async fn add<T: 'static + Monitorable>(
        &mut self,
        item: Box<T>,
    ) -> Result<(), Box<dyn error::Error>> {
        self.enabled.push(item);

        Ok(())
    }
}

pub async fn run() -> Result<(), Box<dyn error::Error>> {
    let mut monitored = Monitored::new().await?;
    let new_monitor: Monitor<PollHTTPBodyContent> = Monitor::<PollHTTPBodyContent>::new().await?;
    let new_monitor2: Monitor<PollHTTPStatusOk> = Monitor::<PollHTTPStatusOk>::new().await?;
    monitored.add(Box::new(new_monitor)).await?;
    monitored.add(Box::new(new_monitor2)).await?;

    println!("Monitored: {:#?}", &monitored);
    println!("Enabled monitor count: {}", &monitored.enabled.len());

    for item in monitored.enabled.iter_mut() {
        item.poll();
    }

    // Simulate a state change
    for item in monitored.enabled.iter_mut() {
        let alert = item.poll();
        println!("Alert: {}", &alert.debug());
    }

    info!("monitors::run()");
    Ok(())
}
