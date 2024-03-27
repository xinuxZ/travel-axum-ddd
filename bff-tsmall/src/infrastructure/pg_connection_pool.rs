use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

pub type PgConnectionPool = Pool<Postgres>;

pub struct PgConnectionManager;

impl PgConnectionManager {
    pub async fn new_pool(connection_string: &str, run_migrations: bool) -> anyhow::Result<PgConnectionPool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("error while initializing the pgsql connection pool")?;

        if run_migrations {
            info!("migrations enabled, running...");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .context("error while running pgsql migrations")?;
        }

        Ok(pool)
    }
}
