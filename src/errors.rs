use std::io;
use serde_json::error;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
  Chrono(chrono::ParseError),
  Hyper(hyper::Error),
  HyperHTTP(hyper::http::Error),
  Serde(error::Error),
  Sqlx(sqlx::Error),
  Log4rs(log4rs::config::Errors),
  Log(log::SetLoggerError),
  /// Errors that do not fit under the other types
  Internal(String),
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

impl From<sqlx::Error> for Error {
  fn from(err: sqlx::Error) -> Error {
    Error::Sqlx(err)
  }
}

impl From<chrono::ParseError> for Error {
  fn from(err: chrono::ParseError) -> Error {
    Error::Chrono(err)
  }
}

impl From<log4rs::config::Errors> for Error {
  fn from(err: log4rs::config::Errors) -> Error {
    Error::Log4rs(err)
  }
}

impl From<log::SetLoggerError> for Error {
  fn from(err: log::SetLoggerError) -> Error {
    Error::Log(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::Internal(format!("{:?}", err))
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

// impl fmt::Display for Error {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       match *self {
//         Error::Internal(ref st) => write!(f, "Internal Error: {:?}", st),
//       }
//   }
// }