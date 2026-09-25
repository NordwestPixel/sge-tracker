use crate::config::Config;
use crate::sync::client::Client;
use crate::sync::error::SyncError;
use reqwest::Url;
use sqlx::PgPool;
use std::time::Duration;

mod client;
mod dto;
pub(crate) mod error;
mod mapper;

pub fn start(pool: PgPool, config: &Config) -> Result<(), SyncError> {
    let url = Url::parse(&config.openligadb_url)?;

    Client::new(url, Duration::from_secs(config.http_timeout_secs.get()))?;

    Ok(())
}
