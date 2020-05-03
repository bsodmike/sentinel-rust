extern crate glob;
extern crate config;
extern crate hyper;
extern crate serde_json;
extern crate rustc_serialize;
extern crate tokio;
extern crate futures;
extern crate once_cell;
extern crate mysql;

use config::*;
use glob::glob;
use once_cell::sync::{Lazy};

mod opts;
mod errors;
mod configure;
mod utils;
mod slack;
mod database;

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
  let mut glob_path = "conf/production/*";
  let mut settings = Config::default();

  let key = "RUST_ENV";
  let run_mode = match std::env::var(key) {
    Ok(value) => value,
    Err(_) => String::new()
  };

  if run_mode.eq("development") {
    glob_path = "conf/development/*";
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


    let result1 = database::fetch::<database::ConnectorMysql, Vec<String>>(&database::ConnectorMysql{});

    let mut message = String::new();
    for item in &result1 {
      message.push_str(item);
      message.push_str(", ");
    }

    println!("MySql parsed message: {}", message);

    let result2 = database::fetch::<database::ConnectorPostgres, String>(&database::ConnectorPostgres{});

    print!("Postgres Result: {:#?}", result2);

    

    // let mut url = "https://jsonplaceholder.typicode.com/todos/1";
    // let response: serde_json::Value = match utils::get(url).await {
    //   Ok(result) => result,
    //   Err(error) => panic!("Error whilst fetching url: {}, error: {:?}", url, error)
    // };

    // println!("Response: {:?}", response);

    let data =  serde_json::json!({
      "blocks": [
        {
          "type": "section",
          "text": {
            "type": "mrkdwn",
            "text": "Hello, this is a test broadcast from your friendly *Sentinel*.\n"
          }
        }
      ]
    });
    let (_, body_json): (hyper::Response<hyper::Body>, serde_json::Value) = 
      match slack::notify(&data).await {
      Ok(result) => result,
      Err(error) => panic!("Error: {:#?}", error)
    };

    println!("Slack response: {:#?}", body_json);
}
