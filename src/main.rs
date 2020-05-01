extern crate glob;
extern crate config;
extern crate hyper;
extern crate serde_json;
extern crate rustc_serialize;
extern crate tokio;
extern crate futures;

use std::collections::HashMap;
use config::*;
use glob::glob;

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

    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------",
             settings.try_into::<HashMap<String, String>>().unwrap());

    let run_mode = std::env::var("RUN_MODE");
    match run_mode {
      Ok(v) => println!("Run mode: {:?}", v),
      Err(e) => println!("Run mode error: {:?}", e)
    }

    let url = "http://jsonplaceholder.typicode.com/todos/1";
    let response: serde_json::Value = match utils::get(url).await {
      Ok(result) => result,
      Err(error) => panic!("Error whilst fetching url: {}, error: {:?}", url, error)
    };

    println!("Response: {:?}", response);
}