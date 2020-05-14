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

pub async fn run() -> Result<(), Box<dyn error::Error>> {
    info!("monitor run");
    Ok(())
}
