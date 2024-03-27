use axum::body::Body;
use axum::extract::MatchedPath;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{BoxError, Json};
use lazy_static::lazy_static;
use serde_json::json;
use std::time::Instant;
use tracing::info;

use crate::domain::PingResponse;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
}

/// Adds a custom handler for tower's `TimeoutLayer`, see https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware.
pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "error":
                    format!(
                        "request took longer than the configured {} second timeout",
                        *HTTP_TIMEOUT
                    )
            })),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("unhandled internal error: {}", err) })),
        )
    }
}

pub async fn track_metrics(request: Request<Body>, next: Next) -> impl IntoResponse {
    let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        request.uri().path().to_owned()
    };

    let start = Instant::now();
    let method = request.method().clone();
    let response = next.run(request).await;
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [("method", method.to_string()), ("path", path), ("status", status)];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}

pub async fn ping() -> Json<PingResponse> {
    info!("received ping request");
    Json(PingResponse::default())
}
