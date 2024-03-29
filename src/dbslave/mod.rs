use crate::alerts;
use crate::monitor::{Alert, SentAlerts};
use crate::configure;
use crate::errors::Error;
use crate::sqlx::Cursor;
use crate::sqlx::Row;
use async_trait::async_trait;
use std::collections::VecDeque;
use std::panic;

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
    pub seconds_behind_master: String,
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
            seconds_behind_master: String::from("0"),
        }
    }
}

#[async_trait]
pub trait Fetch<T> {
    async fn fetch_dbslave_status<'a>(&'a self) -> T;
}

#[async_trait]
impl Fetch<Result<DBSlaveStatus, Error>> for ConnectorMysql {
    async fn fetch_dbslave_status(&self) -> Result<DBSlaveStatus, Error> {
        let mysql_url: String = configure::fetch::<String>(String::from("mysql_url")).unwrap();
        let pool = sqlx::MySqlPool::builder().build(&mysql_url[..]).await?;
        // println!("Pool: {:#?}", pool);

        let sql = "SHOW SLAVE STATUS";
        let mut cursor = sqlx::query(sql).fetch(&pool);
        let mut result: DBSlaveStatus = DBSlaveStatus::default();

        while let Some(row) = cursor.next().await? {
            let mut alert_state: bool = false;
            let seconds_behind_master: String = String::from("0");
            let read_behind_master = match row.try_get::<String, &str>("Seconds_Behind_Master") {
                Ok(val) => val,
                _ => {
                    // When DB Slave is disabled with `STOP SLAVE;` it returns
                    // Seconds_Behind_Master: NULL and sqlx raises a
                    // `UnexpectedNullError`

                    // TODO: handle this as a `QueryAlert`
                    let _ = alerts::QueryAlert {
                        warning: String::from("DB Slave returned `Seconds_Behind_Master: NULL`"),
                    };
                    alert_state = true;

                    seconds_behind_master
                }
            };

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
                seconds_behind_master: read_behind_master,
            };

            result = data;
        }

        Ok(result)
    }
}

#[async_trait]
impl Fetch<Result<String, Error>> for ConnectorPostgres {
    async fn fetch_dbslave_status(&self) -> Result<String, Error> {
        unimplemented!()
    }
}

pub async fn fetch<T: 'static, U>(connector: T) -> U
where
    T: Fetch<U>,
{
    connector.fetch_dbslave_status().await
}

// MOCKED
// TODO: Make this an integration test.
#[async_trait]
pub trait FetchMock<T> {
    async fn fetch_mock_status<'a>(&'a self) -> T;
}

#[async_trait]
impl FetchMock<Result<DBSlaveStatus, Error>> for ConnectorMysql {
    async fn fetch_mock_status(&self) -> Result<DBSlaveStatus, Error> {
        let mut status = DBSlaveStatus::default();
        status.slave_io_running = String::from("Yes");
        status.slave_sql_running = String::from("Yes");
        status.seconds_behind_master = String::from("320");

        Ok(status)
    }
}

#[async_trait]
impl FetchMock<Result<String, Error>> for ConnectorPostgres {
    async fn fetch_mock_status(&self) -> Result<String, Error> {
        unimplemented!()
    }
}

pub async fn fetch_mocked<T: 'static, U>(
    connector: T,
) -> U
where
    T: FetchMock<U>,
{
    connector.fetch_mock_status().await
}

impl<DBSlaveStatus> SentAlerts<DBSlaveStatus> {
    pub async fn initialise() -> Result<SentAlerts<DBSlaveStatus>, Error> {
        let vec: VecDeque<Alert<DBSlaveStatus>> = VecDeque::new();

        let sent_queue = SentAlerts { sent_queue: vec };

        Ok(sent_queue)
    }

    pub async fn add(&mut self, alert: Alert<DBSlaveStatus>) -> Result<(), Error> {
        self.sent_queue.push_back(alert);

        Ok(())
    }

    pub async fn sent(&mut self) -> Result<&VecDeque<Alert<DBSlaveStatus>>, Error> {
        Ok(&self.sent_queue)
    }
}