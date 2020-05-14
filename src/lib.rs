extern crate async_trait;
extern crate chrono;
extern crate config;
extern crate futures;
extern crate glob;
extern crate hyper;
extern crate once_cell;
extern crate rustc_serialize;
extern crate serde_json;
extern crate sqlx;
extern crate tokio;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate regex;
extern crate url;

pub mod configure;
pub mod errors;
pub mod monitor;
pub mod opts;
pub mod runner;
pub mod services;
pub mod utils;
pub mod wrappers;

use config::*;
use glob::glob;
use once_cell::sync::Lazy;
use std::error;
#[macro_use]
use crate::log::LevelFilter;
use crate::log4rs::append::file::FileAppender;
use crate::log4rs::config::{Appender, Config, Root};
use crate::log4rs::encode::pattern::PatternEncoder;

pub static CONFIG: Lazy<config::Config> = Lazy::new(|| {
    let mut glob_path = "conf/development/*";
    let mut settings = config::Config::default();

    let run_mode = match std::env::var("RUST_ENV") {
        Ok(value) => value,
        Err(_) => String::new(),
    };

    if run_mode.eq("production") {
        glob_path = "conf/production/*";
        println!("RUST_ENV={}", run_mode);
    }

    settings
        .merge(
            glob(glob_path)
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        )
        .unwrap();
    settings
});

pub async fn run() -> Result<(), Box<dyn error::Error>> {
    // Prep Logging
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{l}: {d(%Y-%m-%d %H:%M:%S %Z)(utc)} - Line {L} File {f} - {m}\n",
        )))
        .build("log/info.log")?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;
    log4rs::init_config(config);

    crate::runner::run().await;
    Ok(())
}
