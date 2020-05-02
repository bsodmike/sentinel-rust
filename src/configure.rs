use std::collections::HashMap;
use crate::errors::Error;

pub struct ConfigInfo<V> {
  flag: V
}


pub trait FetchFromConfig<T> {
  fn fetch_config(&self, config: &HashMap<String, String>) -> T;
}

impl<V> FetchFromConfig<bool> for ConfigInfo<V>
where
  V: std::fmt::Debug + std::fmt::Display + std::cmp::Eq + std::hash::Hash
{
  fn fetch_config(&self, config: &HashMap<String, String>) -> bool {
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
where
  V: std::fmt::Debug + std::fmt::Display + std::cmp::Eq + std::hash::Hash
{
  fn fetch_config(&self, config: &HashMap<String, String>) -> String {
    let value = match config.get(&self.flag.to_string()) {
      Some(value) => value.to_string(),
      None => String::new()
    };

    value
  }
}

pub fn fetch<T>(flag: std::string::String) -> Result<T, Error> 
where
  ConfigInfo<String>: FetchFromConfig<T>
{
  let config = match crate::CONFIG.clone().try_into::<HashMap<String, String>>() {
    Ok(config) => config,
    Err(error) => panic!("Error: {:?}", error)
  };

  let mut cli_info = ConfigInfo {
    flag: String::from(flag)
  };

  Ok(cli_info.fetch_config(&config))
}

