use std::sync::Arc;

// use anyhow::Context;
use clap::Parser;
use tracing::info;
use tracing_appender::{non_blocking, rolling};
// use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::config::AppConfig;
use crate::infrastructure::mysql_connection_pool::MySqlConnectionManager;

mod app;
mod application;
mod domain;
mod infrastructure;
mod presentation;
mod register;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::from_filename("config/tsadmin.env").ok();
    let config = Arc::new(AppConfig::parse());

    let log_path = &config.log_path;
    // let rust_log = &config.rust_log;
    // app::AppServer::logger(&rust_log, &log_path).await?;

    let file_appender = rolling::never(log_path, "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .with(file_layer)
        .init();

    let mysql_manager = MySqlConnectionManager::new(config.clone());
    let service_register = register::ServiceRegister::new(&mysql_manager, config.clone()).await;

    // info!("migrations successfully ran, initializing axum server...");
    // if config.seed {
    //     info!("seeding enabled, creating test data...");
    //     AppSeedService::new(service_register.clone())
    //         .seed()
    //         .await
    //         .expect("unexpected error occurred while seeding application data");
    // }

    let port = config.port;
    let host = &config.host;
    let router = app::AppServer::router(&config.cors_origin, service_register).await?;

    // run our app with hyper, listening globally on port 3000
    info!("routes initialized, listening on http://{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
