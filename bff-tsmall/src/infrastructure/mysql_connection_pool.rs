use anyhow::Context;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
// use tracing::instrument::WithSubscriber;
// use tracing::info;

pub type MySqlConnectionPool = Pool<MySql>;

pub struct MySqlConnectionManager;

impl MySqlConnectionManager {
    pub async fn pool(connection_string: &str) -> anyhow::Result<MySqlConnectionPool> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(connection_string) //"mysql://username:password@hostname:port/database")
            // .connect("mysql://root:xinuxZ@127.0.0.1:3306/saas_srm")
            .await
            .context("error while initializing the mysql connection pool")?;

        Ok(pool)
    }
}
