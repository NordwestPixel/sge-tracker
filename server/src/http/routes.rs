use axum::Router;
use axum::routing::get;
use crate::state::AppState;

pub fn load_root_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(|| async { "placeholder" }))
}

pub fn load_api_routes() -> Router<AppState> {
    Router::new()
        .route("/home", get(|| async { "placeholder" }))
        .route("/matches/{id}", get(|| async { "placeholder" }))
        .route("/matches", get(|| async { "placeholder" }))
        .route("/standings", get(|| async { "placeholder" }))
}