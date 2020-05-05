use std::env;
use getopts::{Options};
use crate::errors::Error;

const PROGRAM: &str = "sentinel";

pub struct Config {
  pub slack_token: String,
}

// Implement `Display`
impl std::fmt::Display for Config {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "(slack_token: {})", self.slack_token)
  }
}

fn print_usage(opts: Options) {
  let brief = format!("Usage: ./{} [options]", PROGRAM);
  print!("{}", opts.usage(&brief));
}

pub fn parse_args() -> Result<Config, Error> {
  let args: Vec<String> = env::args().collect();

  let mut opts = Options::new();

  opts.reqopt("t", "token", "You must provide the access token.", "TOKEN");
  opts.optflag("h", "help", "Print this help menu.");

  let matches = match opts.parse(&args[1..]) {
    Ok(opt) => opt,
    Err(error) => {
      print_usage(opts);
      panic!("Error: {:?}", error);
    }
  };

  if matches.opt_present("h") {
    print_usage(opts);
    return Err(Error::HelpMenuRequested);
}

  let conf = Config {
    slack_token: matches.opt_str("t").ok_or(Error::InvalidArgError)?,
  };

  Ok(conf)
}