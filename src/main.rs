extern crate glob;
extern crate config;
extern crate hyper;
extern crate serde_json;
extern crate rustc_serialize;
extern crate tokio;
extern crate futures;
extern crate once_cell;

use config::*;
use glob::glob;
use std::collections::HashMap;
use once_cell::sync::{Lazy};

mod opts;
mod errors;
mod utils;
mod slack;

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
  let mut settings = Config::default();
  settings
      .merge(glob("conf/*")
                  .unwrap()
                  .map(|path| File::from(path.unwrap()))
                  .collect::<Vec<_>>())
      .unwrap();
  settings
});

trait FetchFromConfig<U> {
  fn fetch_boolean(&self, config: &HashMap<String, String>) -> U;
}

struct ConfigInfo<V> {
  flag: V
}

impl<V> FetchFromConfig<bool> for ConfigInfo<V>
where V: std::fmt::Debug + std::fmt::Display + std::cmp::Eq + std::hash::Hash {
  fn fetch_boolean(&self, config: &HashMap<String, String>) -> bool {
    let value = match config.get(&self.flag.to_string()) {
      Some(value) => value.to_string(),
      None => String::new()
    };

    let mut result: bool = false;

    if value.eq("true") || value.eq("false") {
      result = match value.parse::<bool>() {
        Ok(value) => value,
        Err(error) => panic!(
          "Unknown error parsing configuration flag {}. Err: {:#?}", 
          &self.flag, error
        )
      };
    }

  result
  }
}

impl<V> FetchFromConfig<String> for ConfigInfo<V>
where V: std::fmt::Debug + std::fmt::Display + std::cmp::Eq + std::hash::Hash {
  fn fetch_boolean(&self, config: &HashMap<String, String>) -> String {
    let value = match config.get(&self.flag.to_string()) {
      Some(value) => value.to_string(),
      None => String::new()
    };

    value
  }
}


#[tokio::main]
async fn main() {
    let config = match CONFIG.clone().try_into::<HashMap<String, String>>() {
      Ok(config) => config,
      Err(error) => panic!("Error: {:?}", error)
    };

    let mut cli_info = ConfigInfo {
      flag: String::from("cli_options")
    };
    let enable_cli_options: bool = cli_info.fetch_boolean(&config);
    
    // Load options from CLI
    if enable_cli_options {
      let _conf = match opts::parse_args() {
        Ok(conf) => conf,
        Err(error) => panic!("Error: {:?}", error)
      };
      
      println!("Conf: {}", _conf);
    }

    cli_info = ConfigInfo {
      flag: String::from("slack_key")
    };
    let slack_key: String = cli_info.fetch_boolean(&config);
    println!("Slack key: {:#?}", slack_key);

    // Main execution
    let run_mode = std::env::var("RUN_MODE");
    match run_mode {
      Ok(v) => println!("Run mode: {:?}", v),
      Err(e) => println!("Run mode error: {:?}", e)
    }

    // let mut url = "https://jsonplaceholder.typicode.com/todos/1";
    // let response: serde_json::Value = match utils::get(url).await {
    //   Ok(result) => result,
    //   Err(error) => panic!("Error whilst fetching url: {}, error: {:?}", url, error)
    // };

    // println!("Response: {:?}", response);

    // url = "https://jsonplaceholder.typicode.com/posts";
    // let data =  serde_json::json!({
    //   "title": "John Doe",
    //   "body": "bar",
    //   "userId": 1
    // });
    // let payload = hyper::Body::from(data.to_string());
    
    // let (resp, body_json): (hyper::Response<hyper::Body>, serde_json::Value) = match utils::post(url, payload).await {
    //   Ok(result) => result,
    //   Err(error) => panic!("Error whilst posting JSON to url: {}, error: {:?}", url, error)
    // };
    // println!("Response: {:#?}", resp);
    // println!("Body: {:#?}", body_json);

    slack::run();
}
