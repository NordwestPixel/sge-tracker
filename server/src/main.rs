use crate::db::connect_pool;
use crate::http::router::build_router;
use dotenvy::dotenv;
use std::process::ExitCode;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod config;
mod db;
mod error;
mod http;
mod matches;
mod standings;
mod state;
mod sync;

#[tokio::main]
async fn main() -> ExitCode {
    let env_status = dotenv();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .init();

    match env_status {
        Ok(path) => info!(".env loaded from: {}", path.display()),
        Err(e) => {
            if e.not_found() {
                info!(".env not found, process environment is used.");
            } else {
                error!("Failed to load .env: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    let run_result = run().await;
    if let Err(err) = run_result {
        error!("{err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::load_from_env()?;

    let pool = connect_pool(config.db_max_connections.get(), &config.database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = state::AppState::new(pool, config.clone());

    let router = build_router(state);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
