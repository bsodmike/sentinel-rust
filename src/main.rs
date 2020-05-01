extern crate glob;
extern crate config;
extern crate hyper;
extern crate serde_json;
extern crate rustc_serialize;
extern crate tokio;
extern crate futures;

use config::*;
use glob::glob;
use std::collections::HashMap;

mod opts;
mod errors;
mod utils;

#[tokio::main]
async fn main() {
    let mut settings = Config::default();
    settings
        .merge(glob("conf/*")
                   .unwrap()
                   .map(|path| File::from(path.unwrap()))
                   .collect::<Vec<_>>())
        .unwrap();
        
    let config = match settings.try_into::<HashMap<String, String>>() {
      Ok(config) => config,
      Err(error) => panic!("Error: {:?}", error)
    };

    let _enable_cli_options = match config.get("cli_options") {
      Some(value) => value.to_string(),
      None => String::new()
    };
    let enable_cli_options = _enable_cli_options.parse::<bool>().unwrap();
    println!("Value: {:?}", enable_cli_options);

    // Load options from CLI
    if enable_cli_options {
      let _conf = match opts::parse_args() {
        Ok(conf) => conf,
        Err(error) => panic!("Error: {:?}", error)
      };
      
      println!("Conf: {}", _conf);
    }

    // Main execution
    let run_mode = std::env::var("RUN_MODE");
    match run_mode {
      Ok(v) => println!("Run mode: {:?}", v),
      Err(e) => println!("Run mode error: {:?}", e)
    }

    let mut url = "https://jsonplaceholder.typicode.com/todos/1";
    let response: serde_json::Value = match utils::get(url).await {
      Ok(result) => result,
      Err(error) => panic!("Error whilst fetching url: {}, error: {:?}", url, error)
    };

    println!("Response: {:?}", response);

    url = "https://jsonplaceholder.typicode.com/posts";
    let data =  serde_json::json!({
      "title": "John Doe",
      "body": "bar",
      "userId": 1
    });
    let payload = hyper::Body::from(data.to_string());
    
    let resp = match utils::post(url, payload).await {
      Ok(result) => result,
      Err(error) => panic!("Error whilst posting JSON to url: {}, error: {:?}", url, error)
    };
    println!("Response: {:?}", resp);

}