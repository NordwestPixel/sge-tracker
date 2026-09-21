use dotenvy::dotenv;
use server::run;
use std::process::ExitCode;
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> ExitCode {
    let env_status = dotenv();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("LOG_LEVEL")
                .from_env_lossy(),
        )
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
        error!("{err:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
