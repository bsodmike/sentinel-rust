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

use config::*;
use glob::glob;
use once_cell::sync::{Lazy};
use errors::Error;

mod opts;
mod errors;
mod configure;
mod utils;
mod slack;
mod dbslave;
mod monitor;

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
  let mut glob_path = "conf/development/*";
  let mut settings = Config::default();

  let key = "RUST_ENV";
  let run_mode = match std::env::var(key) {
    Ok(value) => value,
    Err(_) => String::new()
  };

  if run_mode.eq("production") {
    glob_path = "conf/production/*";
    println!("Run mode {}", run_mode);
  }
  
  settings
      .merge(glob(glob_path)
                  .unwrap()
                  .map(|path| File::from(path.unwrap()))
                  .collect::<Vec<_>>())
      .unwrap();
  settings
});

#[tokio::main]
async fn main() {
    let enable_cli_options: bool = configure::fetch::<bool>(String::from("cli_options"))
      .unwrap_or(false);
    
    // Load options from CLI
    if enable_cli_options {
      let _conf = opts::parse_args().unwrap();
      println!("Conf: {}", _conf);
    }
   
    monitor::begin_watch().await;
}
