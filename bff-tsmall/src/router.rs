use std::{future::ready, time::Duration};

use anyhow::Context;
use axum::{error_handling::HandleErrorLayer, http::HeaderValue, middleware, routing::get, Router};
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use tower::ServiceBuilder;
use tower_http::{cors::Any, cors::CorsLayer, trace::TraceLayer};

use crate::presentation::endpoints::supports_endpoints::{handle_timeout_error, ping, track_metrics};
use crate::register::ServiceRegister;
use crate::routes::{ArticlesRouter, ProfilesRouter, SuppliersRouter, TagsRouter, UsersRouter};

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct AppRouter;

impl AppRouter {
    pub async fn router(cors_origin: &str, service_register: ServiceRegister) -> anyhow::Result<Router> {
        // pub async fn serve(port: u32, cors_origin: &str, service_register: ServiceRegister) ->  {
        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_requests_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .context("could not setup buckets for metrics, verify matchers are correct")?
            .install_recorder()
            .context("could not install metrics recorder")?;

        let router = Router::new()
            .nest("/api", SuppliersRouter::new_router(service_register.clone()))
            .nest("/api", UsersRouter::new_router(service_register.clone()))
            .nest("/api", ProfilesRouter::new_router(service_register.clone()))
            .nest("/api", TagsRouter::new_router(service_register.clone()))
            .nest("/api", ArticlesRouter::new_router(service_register.clone()))
            .route("/api/ping", get(ping))
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT)),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .route_layer(middleware::from_fn(track_metrics));

        Ok(router)
    }
}
