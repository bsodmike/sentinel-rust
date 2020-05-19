use super::errors::Error;
use crate::configure;
use crate::log::LevelFilter;
use crate::log4rs::append::file::FileAppender;
use crate::log4rs::config::{Appender, Config, Root};
use crate::log4rs::encode::pattern::PatternEncoder;
use crate::utils;
use crate::wrappers;
use ::chrono::Utc;
use std::collections::VecDeque;
use std::error;

use std::time::Duration;
use std::{thread, time};

#[derive(Default)]
pub struct Monitors {
    pub enabled: Vec<Monitor>,
}

#[derive(Copy, Clone)]
enum MonitorState {
    Good,
    Failed,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct Monitor {
    state: MonitorState,
}

impl Monitors {
    pub async fn new() -> Result<Monitors, Box<dyn error::Error>> {
        Ok(Monitors::default())
    }

    pub async fn add(&mut self, monitor: &Monitor) -> Result<(), Box<dyn error::Error>> {
        self.enabled.push(*monitor);

        Ok(())
    }
}

pub async fn run() -> Result<(), Box<dyn error::Error>> {
    let mut monitors = Monitors::new().await?;
    let new_monitor = Monitor {
        state: MonitorState::Unknown,
    };
    monitors.add(&new_monitor).await?;

    println!("Enabled monitor count: {}", monitors.enabled.len());

    info!("monitors run");
    Ok(())
}
