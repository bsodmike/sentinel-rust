use std::collections::HashMap;

pub fn run() {
  let config = match crate::CONFIG.clone().try_into::<HashMap<String, String>>() {
    Ok(config) => config,
    Err(error) => panic!("Error: {:?}", error)
  };

  let _enable_cli_options = match config.get("cli_options") {
    Some(value) => value.to_string(),
    None => String::new()
  };
  let enable_cli_options = _enable_cli_options.parse::<bool>().unwrap();
  println!("Value: {:?}", enable_cli_options);
}