use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
// use tracing_appender::{non_blocking, rolling};
// use tracing_subscriber::{fmt,layer::SubscriberExt,util::SubscriberInitExt}
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::config::AppConfig;
use crate::infrastructure::mysql_connection_pool::MySqlConnectionManager;
use crate::infrastructure::pg_connection_pool::PgConnectionManager;
use crate::infrastructure::seed_service::AppSeedService;

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod register;
mod router;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::from_filename("config/tsmall.env").ok();
    let config = Arc::new(AppConfig::parse());
    // let log_path = &config.log_path;

    // let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);
    // // // let env_filter = EnvFilter::new(&config.rust_log);
    // // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // let file_appender = rolling::never(log_path, "app.log");
    // let (non_blocking_appender, _guard) = non_blocking(file_appender);
    // let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking_appender);

    // tracing_subscriber::registry()
    //     .with(formatting_layer)
    //     // .with(env_filter)
    //     // .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
    //     .with(tracing_subscriber::fmt::layer())
    //     .with(ErrorLayer::default())
    //     .with(file_layer)
    //     .init();
    tracing_subscriber::registry()
        // .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mysql_pool = MySqlConnectionManager::pool(&config.mysql_url)
        .await
        .expect("could not initialize the mysql connection pool");

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = PgConnectionManager::new_pool(&config.postgresql_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");
    let service_register = register::ServiceRegister::new(pg_pool, mysql_pool, config.clone());

    // if config.seed {
    //     info!("seeding enabled, creating test data...");
    //     AppSeedService::new(service_register.clone())
    //         .seed()
    //         .await
    //         .expect("unexpected error occurred while seeding application data");
    // }

    // info!("migrations successfully ran, initializing axum server...");
    // router::AppServer::serve(host, port, &config.cors_origin, service_register, mysql_pool.clone())
    //     .await
    //     .context("could not initialize application routes")?;

    let port = config.port;
    let host = &config.host;
    let router = router::AppRouter::router(&config.cors_origin, service_register).await?;

    info!("routes initialized, listening on port {}", port);
    axum::Server::bind(&format!("{}:{}", host, port).parse().unwrap())
        .serve(router.into_make_service())
        .await
        .context("error while starting API server")?;

    Ok(())
}
