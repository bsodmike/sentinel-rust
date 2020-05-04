extern crate glob;
extern crate config;
extern crate hyper;
extern crate serde_json;
extern crate rustc_serialize;
extern crate tokio;
extern crate futures;
extern crate once_cell;
extern crate sqlx;
extern crate async_trait;

use config::*;
use glob::glob;
use once_cell::sync::{Lazy};
use errors::Error;

mod opts;
mod errors;
mod configure;
mod utils;
mod slack;
mod database;

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
  let mut glob_path = "conf/development/*";
  let mut settings = Config::default();

  let key = "RUST_ENV";
  let run_mode = match std::env::var(key) {
    Ok(value) => value,
    Err(_) => String::new()
  };

  if run_mode.eq("production") {
    glob_path = "conf/production/*";
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
   

    let result1 = database::fetch::<database::ConnectorMysql, Result<Vec<database::Data>, Error>>(database::ConnectorMysql{}).await.unwrap();
    // println!("MySQL Result: {:#?}", result1);

    let mut message = String::new();
    let data = &result1[0];
    message.push_str(&String::from(format!("\\n\\nMaster host: {}\\n", &data.master_host[..])));
    message.push_str(&String::from(format!("Master user: {}\\n", &data.master_user[..])));
    message.push_str(&String::from(format!("Slave IO running: {}\\n", &data.slave_io_running[..])));
    message.push_str(&String::from(format!("Slave SQL running: {}\\n", &data.slave_sql_running[..])));
    message.push_str(&String::from(format!("Master log file: {}\\n", &data.master_log_file[..])));
    message.push_str(&String::from(format!("Master log pos: {}\\n", data.read_master_log_pos)));
    message.push_str(&String::from(format!("Relay log file: {}\\n", &data.relay_log_file[..])));
    message.push_str(&String::from(format!("Relay log pos: {}\\n", data.relay_log_pos)));
    message.push_str(&String::from(format!("Relay master log file: {}\\n", &data.relay_master_log_file[..])));
    message.push_str(&String::from(format!("Slave seconds behind master: {}\\n\\n", data.seconds_behind_master)));

    println!("{}", message);

    // let result2 = database::fetch::<database::ConnectorPostgres, Result<String, Error>>(database::ConnectorPostgres{}).await;
    // print!("Postgres Result: {:#?}", result2.unwrap());  

    // let mut url = "https://jsonplaceholder.typicode.com/todos/1";
    // let response: serde_json::Value = match utils::get(url).await {
    //   Ok(result) => result,
    //   Err(error) => panic!("Error whilst fetching url: {}, error: {:?}", url, error)
    // };

    // println!("Response: {:?}", response);

    let mut template = String::new();
    template.push_str(&String::from(r#"{
      "blocks": [
        {
          "type": "section",
          "text": {
            "type": "mrkdwn",
            "text":  "Hello, this is a test broadcast from your friendly *Sentinel*."#));
    template.push_str(&message);
    template.push_str(&String::from(r#""
          }
        }
      ]
    }
    "#));

    println!("{}", template);


    // let data =  serde_json::json!({
    //   "blocks": [
    //     {
    //       "type": "section",
    //       "text": {
    //         "type": "mrkdwn",
    //         "text": "Hello, this is a test broadcast from your friendly *Sentinel*.\n"
    //       }
    //     }
    //   ]
    // });
    let data: serde_json::Value = serde_json::from_str(&template).unwrap();
    let (_, body_json): (hyper::Response<hyper::Body>, serde_json::Value) = 
      match slack::notify(&data).await {
      Ok(result) => result,
      Err(error) => panic!("Error: {:#?}", error)
    };

    println!("Slack response: {:#?}", body_json);
}
