extern crate glob;
extern crate config;
extern crate hyper;
extern crate serde_json;
extern crate rustc_serialize;
extern crate tokio;
extern crate futures;
extern crate once_cell;
extern crate sqlx;
extern crate async_trait;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate regex;
extern crate url;

pub mod configure;
pub mod opts;
pub mod errors;
pub mod utils;
pub mod dbslave;
pub mod services;
pub mod monitor;
pub mod alerts;
pub mod wrappers;

use config::*;
use glob::glob;
use once_cell::sync::{Lazy};

pub static CONFIG: Lazy<config::Config> = Lazy::new(|| {
  let mut glob_path = "conf/development/*";
  let mut settings = Config::default();

  let run_mode = match std::env::var("RUST_ENV") {
    Ok(value) => value,
    Err(_) => String::new()
  };

  if run_mode.eq("production") {
    glob_path = "conf/production/*";
    println!("RUST_ENV={}", run_mode);
  }
  
  settings
      .merge(glob(glob_path)
                  .unwrap()
                  .map(|path| File::from(path.unwrap()))
                  .collect::<Vec<_>>())
      .unwrap();
  settings
});