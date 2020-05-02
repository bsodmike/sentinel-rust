use std::collections::HashMap;
use crate::configure;

pub fn run() {
  let config = match crate::CONFIG.clone().try_into::<HashMap<String, String>>() {
    Ok(config) => config,
    Err(error) => panic!("Error: {:?}", error)
  };

  let slack_key: String = match configure::fetch::<String>(String::from("slack_key")) {
    Ok(value) => value,
    Err(error) => panic!(),
  };
  
  println!("Slack key: {:#?}", slack_key);
}