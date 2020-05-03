use crate::mysql::*;
use crate::mysql::prelude::*;
use crate::errors::Error;

#[derive(Debug)]
pub struct ConnectorMysql {

}

#[derive(Debug)]
pub struct ConnectorPostgres {
  
}

#[derive(Debug)]
struct Connection {

}

pub trait Fetch<ReturnType> {
  fn call_db(&self) -> ReturnType;
}

impl Fetch<Vec<String>> for ConnectorMysql
{
  fn call_db(&self) -> Vec<String> {
    let url = "mysql://root:a@127.0.0.1:3306/";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
  
    let query: &str = r#"SHOW DATABASES"#;
    let result = match conn.query::<String, &str>(query) {
      Ok(value) => value,
      Err(error) => panic!("Err: {} making query {}", error, query)
    };

    result
  }
} 

impl Fetch<String> for ConnectorPostgres
{
  fn call_db(&self) -> String {
    panic!("Err: {:#?}", Error::NotImplementedError)
  }
}

pub fn fetch<ConnectorType, ReturnType>(connector: &ConnectorType) -> ReturnType
where
  ConnectorType: Fetch<ReturnType>
{
  connector.call_db()
}