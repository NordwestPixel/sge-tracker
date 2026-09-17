use axum::Router;
use tower_http::trace::TraceLayer;
use crate::http::routes;
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::load_root_routes())
        .nest("/api", routes::load_api_routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
