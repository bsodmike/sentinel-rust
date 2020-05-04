#[macro_use]
use sqlx::mysql::MySqlPool;
use crate::sqlx::prelude::MySqlQueryAs;
use crate::sqlx::Cursor;
use crate::sqlx::Row;
use crate::errors::Error;
use async_trait::async_trait;
use crate::configure;

#[derive(Debug)]
pub struct ConnectorMysql {

}

#[derive(Debug)]
pub struct ConnectorPostgres {
  
}

#[derive(Debug)]
struct Connection {

}

#[async_trait]
pub trait Fetch<ReturnType> {
  async fn call_db<'a>(&'a self) -> ReturnType;
}

#[derive(Debug)]
pub struct Data {
  pub master_host: String,
  pub master_user: String,
  pub slave_io_running: String,
  pub slave_sql_running: String,
  pub master_log_file: String,
  pub read_master_log_pos: u64,
  pub relay_log_file: String,
  pub relay_log_pos: u64,
  pub relay_master_log_file: String,
  pub seconds_behind_master: u64,
}

#[async_trait]
impl Fetch<Result<Vec<Data>, Error>> for ConnectorMysql
{
  async fn call_db<'a>(&'a self) -> Result<Vec<Data>, Error> {
    let mysql_url: String = configure::fetch::<String>(String::from("mysql_url")).unwrap();
    let pool = sqlx::MySqlPool::builder()
      .build(&mysql_url[..]).await?;
    // println!("Pool: {:#?}", pool);

    let sql = "SHOW SLAVE STATUS";
    let mut cursor = sqlx::query(sql).fetch(&pool);
    let mut result = Vec::new();
    while let Some(row) = cursor.next().await? {
        let data = Data {
          master_host: row.get("Master_Host"),
          master_user: row.get("Master_User"),
          slave_io_running: row.get("Slave_IO_Running"),
          slave_sql_running: row.get("Slave_SQL_Running"),
          master_log_file: row.get("Master_Log_File"),
          read_master_log_pos: row.get("Read_Master_Log_Pos"),
          relay_log_file: row.get("Relay_Log_File"),
          relay_log_pos: row.get("Relay_Log_Pos"),
          relay_master_log_file: row.get("Relay_Master_Log_File"),
          seconds_behind_master: row.get("Seconds_Behind_Master"),
        };
        result.push(data);
    }
    
    Ok(result)
  }
} 

#[async_trait]
impl Fetch<Result<String, Error>> for ConnectorPostgres
{
  async fn call_db<'a>(&'a self) -> Result<String, Error> {
    panic!("Err: {:#?}", Error::NotImplementedError)
  }
}

pub async fn fetch<ConnectorType: 'static, ReturnType>(connector: ConnectorType) -> ReturnType
where
  ConnectorType: Fetch<ReturnType>
{
  let result = connector.call_db().await;

  result
}