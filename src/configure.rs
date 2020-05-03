use std::collections::HashMap;
use crate::errors::Error;
use crate::configure;

pub struct ConfigInfo<V> {
  flag: V
}

pub trait FetchFromConfig<T>: private::Sealed {
  fn fetch_config(&self, config: &HashMap<String, String>) -> T;
}

impl<V> FetchFromConfig<bool> for ConfigInfo<V>
where
  V: std::fmt::Debug + std::fmt::Display
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
  V: std::fmt::Debug + std::fmt::Display
{
  fn fetch_config(&self, config: &HashMap<String, String>) -> String {
    let value = match config.get(&self.flag.to_string()) {
      Some(value) => value.to_string(),
      None => String::new()
    };

    value
  }
}

// Prevent users from implementing the FetchFromConfig trait.
mod private {
  pub trait Sealed {}
  impl<V> Sealed for super::ConfigInfo<V> {}
}

pub fn fetch<T>(flag: std::string::String) -> Result<T, Error> 
where
  // Trait bound to implement FetchFromConfig<T> for ConfigInfo<String>,
  // to allow it to call `fetch_config` as defined by the trait, returning T.
  ConfigInfo<String>: FetchFromConfig<T>
{
  let config = match crate::CONFIG.clone().try_into::<HashMap<String, String>>() {
    Ok(config) => config,
    Err(error) => panic!("Error: {:?}", error)
  };

  let cli_info = ConfigInfo {
    flag: String::from(flag)
  };

  Ok(cli_info.fetch_config(&config))
}

