use crate::state::AppState;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;

pub fn load_root_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_handler))
}

pub fn load_api_routes() -> Router<AppState> {
    Router::new()
        .route("/home", get(|| async { "placeholder" }))
        .route("/matches/{id}", get(|| async { "placeholder" }))
        .route("/matches", get(|| async { "placeholder" }))
        .route("/standings", get(|| async { "placeholder" }))
}

async fn health_handler() -> StatusCode {
    StatusCode::OK
}
