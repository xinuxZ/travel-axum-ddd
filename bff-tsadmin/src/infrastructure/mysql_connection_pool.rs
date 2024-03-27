use std::sync::Arc;

use anyhow::Context;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

use super::config::AppConfig;

pub type MySqlConnectionPool = Pool<MySql>;

#[derive(Clone)]
pub struct MySqlConnectionManager {
    config: Arc<AppConfig>,
    // pub host: String,
    // pub port: u16,
    // pub username: String,
    // pub password: String,
}

impl MySqlConnectionManager {
    pub fn new(config: Arc<AppConfig>) -> Self {
        // let config = cfg.clone();
        Self {
            config, // host: config.mysql_host,
                    // port: config.mysql_port,
                    // username: config.mysql_username,
                    // password: config.mysql_password,
        }
    }
    // pub async fn pool(connection_string: &str) -> anyhow::Result<MySqlConnectionPool> {
    //     let pool = MySqlPoolOptions::new()
    //         .max_connections(5)
    //         .connect(connection_string) //"mysql://username:password@hostname:port/database")
    //         .await
    //         .context("error while initializing the mysql connection pool")?;

    //     Ok(pool)
    // }

    pub async fn pool_with_db(self, database: &str) -> anyhow::Result<MySqlConnectionPool> {
        let conn_option = MySqlConnectOptions::new()
            .host(&self.config.mysql_host)
            .port(self.config.mysql_port)
            .username(&self.config.mysql_username)
            .password(&self.config.mysql_password)
            .database(database);

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect_with(conn_option) //"mysql://username:password@hostname:port/database")
            .await
            .context("error while initializing the mysql connection pool")?;

        Ok(pool)
    }
}
