use crate::sqlx::Cursor;
use crate::sqlx::Row;
use crate::errors::Error;
use async_trait::async_trait;
use crate::configure;

pub mod alertable;

#[derive(Debug)]
pub struct ConnectorMysql;

#[derive(Debug)]
pub struct ConnectorPostgres;

#[derive(Debug)]
struct Connection;

#[derive(Debug, Clone)]
pub struct DBSlaveStatus {
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

impl std::convert::AsRef<DBSlaveStatus> for DBSlaveStatus {
  fn as_ref(&self) -> &DBSlaveStatus {
    self
  }
}

impl Default for DBSlaveStatus {
  fn default() -> Self {
    Self {
      master_host: String::new(),
      master_user: String::new(),
      slave_io_running: String::new(),
      slave_sql_running: String::new(),
      master_log_file: String::new(),
      read_master_log_pos: 0,
      relay_log_file: String::new(),
      relay_log_pos: 0,
      relay_master_log_file: String::new(),
      seconds_behind_master: 0,
    }
  }
}

#[async_trait]
pub trait Fetch<ReturnType> {
  async fn fetch_dbslave_status<'a>(&'a self) -> ReturnType;
}

#[async_trait]
impl Fetch<Result<Vec<DBSlaveStatus>, Error>> for ConnectorMysql
{
  async fn fetch_dbslave_status(&self) -> Result<Vec<DBSlaveStatus>, Error> {
    let mysql_url: String = configure::fetch::<String>(String::from("mysql_url")).unwrap();
    let pool = sqlx::MySqlPool::builder()
      .build(&mysql_url[..]).await?;
    // println!("Pool: {:#?}", pool);

    let sql = "SHOW SLAVE STATUS";
    let mut cursor = sqlx::query(sql).fetch(&pool);
    let mut result = Vec::new();
    while let Some(row) = cursor.next().await? {
        let data = DBSlaveStatus {
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
  async fn fetch_dbslave_status(&self) -> Result<String, Error> {
    unimplemented!()
  }
}

pub async fn fetch<ConnectorType: 'static, ReturnType>(connector: ConnectorType) -> ReturnType
where
  ConnectorType: Fetch<ReturnType>
{
  connector.fetch_dbslave_status().await
}

// MOCKED
// TODO: Make this an integration test.
#[async_trait]
pub trait FetchMock<ReturnType> {
  async fn fetch_mock_status<'a>(&'a self) -> ReturnType;
}

#[async_trait]
impl FetchMock<Result<Vec<DBSlaveStatus>, Error>> for ConnectorMysql
{
  async fn fetch_mock_status(&self) -> Result<Vec<DBSlaveStatus>, Error> {
    let mut status = DBSlaveStatus::default();
    status.slave_io_running = String::from("Yes");
    status.slave_sql_running = String::from("No");
    status.seconds_behind_master = 300;

    Ok(vec![status])
  }
}

#[async_trait]
impl FetchMock<Result<String, Error>> for ConnectorPostgres
{
  async fn fetch_mock_status(&self) -> Result<String, Error> {
    unimplemented!()
  }
}

pub async fn fetch_mocked<ConnectorType: 'static, ReturnType>(connector: ConnectorType) -> ReturnType
where
  ConnectorType: FetchMock<ReturnType>
{
  connector.fetch_mock_status().await
}
