use hyper;
use std::io;
use serde_json::error;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
  Hyper(hyper::Error),
  HyperHTTP(hyper::http::Error),
  Serde(error::Error),
  Io(io::Error),
  UnexpectedJson,
  NoResult,
  NoMembers,
  UserNotFound,
  InvalidTokenError,
  InvalidArgError,
  HelpMenuRequested,
  CantConvertJsonToObj,
  NotImplementedError,
}

impl From<hyper::Error> for Error {
  fn from(err: hyper::Error) -> Error {
      Error::Hyper(err)
  }
}

impl From<hyper::http::Error> for Error {
  fn from(err: hyper::http::Error) -> Error {
      Error::HyperHTTP(err)
  }
}

impl From<error::Error> for Error {
  fn from(err: error::Error) -> Error {
      Error::Serde(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
      Error::Io(err)
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    "Unknown error!"
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Unknown error!")
  }
}
