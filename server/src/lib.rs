use crate::db::connect_pool;
use crate::error::InitError;
use crate::http::router::build_router;

pub mod config;
mod db;
mod error;
pub mod http;
mod matches;
mod reference;
mod standings;
pub mod state;
mod sync;

pub async fn run() -> Result<(), InitError> {
    let config = config::Config::load_from_env()?;

    let pool = connect_pool(config.db_max_connections.get(), &config.database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    sync::start(pool.clone(), &config)?;

    let listener = tokio::net::TcpListener::bind(&config.bind_addr)
        .await
        .map_err(|source| InitError::Bind {
            address: config.bind_addr.clone(),
            source,
        })?;

    let state = state::AppState::new(pool, config);
    let router = build_router(state);
    axum::serve(listener, router).await?;

    Ok(())
}
