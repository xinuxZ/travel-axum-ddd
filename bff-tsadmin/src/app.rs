use std::{future::ready, time::Duration};

use anyhow::Context;
use axum::{error_handling::HandleErrorLayer, http::HeaderValue, middleware, routing::get, Router};
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use tower::ServiceBuilder;
use tower_http::{cors::Any, cors::CorsLayer, trace::TraceLayer};

use crate::presentation::endpoints::supports_endpoints::{handle_timeout_error, ping, track_metrics};
use crate::register::ServiceRegister;
use crate::routes::ProductRouter;
use crate::routes::SuppliersRouter;

// use tracing::info;
// use tracing_appender::{non_blocking, rolling};
// use tracing_error::ErrorLayer;
// use tracing_subscriber::filter::EnvFilter;
// use tracing_subscriber::{fmt, layer::SubscriberExt};

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct AppServer;

impl AppServer {
    // pub async fn logger(rust_log: &str, log_path: &str) -> anyhow::Result<()> {
    //     // 将日志输出到控制台
    //     let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);
    //     let env_filter = EnvFilter::new(rust_log);
    //     let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    //     // 将日志写入文件
    //     let file_appender = rolling::never(log_path, "app.log");
    //     let (non_blocking_appender, _guard) = non_blocking(file_appender);
    //     let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking_appender);

    //     tracing_subscriber::registry()
    //         // .with(formatting_layer)
    //         // .with(env_filter)
    //         // .with(tracing_subscriber::EnvFilter::new(rust_log))
    //         // .with(ErrorLayer::default())
    //         // .with(file_layer)
    //         .with(tracing_subscriber::fmt::layer())
    //         .init();

    //     Ok(())
    // }

    pub async fn router(cors_origin: &str, service_register: ServiceRegister) -> anyhow::Result<Router> {
        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_requests_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .context("could not setup buckets for metrics, verify matchers are correct")?
            .install_recorder()
            .context("could not install metrics recorder")?;

        let router = Router::new()
            .nest("/api", SuppliersRouter::new_router(service_register.suppliers_service))
            .nest("/api", ProductRouter::new_router(service_register.product_service))
            .route("/api/ping", get(ping))
            .route("/metrics", get(move || ready(recorder_handle.render())))
            // .layer(
            //     ServiceBuilder::new()
            //         .layer(TraceLayer::new_for_http())
            //         .layer(HandleErrorLayer::new(handle_timeout_error))
            //         .timeout(Duration::from_secs(*HTTP_TIMEOUT)),
            // )
            // .layer(
            //     CorsLayer::new()
            //         .allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
            //         .allow_methods(Any)
            //         .allow_headers(Any),
            // )
            .route_layer(middleware::from_fn(track_metrics));

        return Ok(router);
    }
}
