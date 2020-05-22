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

#[derive(Copy, Clone)]
pub enum Alerts {
    PollFailure,
    StateChange,
    UnknownFailure,
}

#[derive(Debug, Copy, Clone)]
pub struct PollHTTPBodyContent;

#[derive(Debug, Copy, Clone)]
pub struct PollHTTPStatusOk;

#[derive(Debug, Copy, Clone)]
pub struct PollMySQLDBSlave;

pub trait Monitorable {
    fn info(&self) -> String;
    fn poll(&self);
}

pub struct Monitor<T> {
    context: T,
    state: State,
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

    for item in monitored.enabled.iter() {
        item.poll();
    }

    info!("monitors::run()");
    Ok(())
}
