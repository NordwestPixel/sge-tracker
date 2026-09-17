use std::env;
use std::num::{NonZeroU32, NonZeroU64, ParseIntError};
use std::str::{FromStr, ParseBoolError};
use thiserror::Error;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub db_max_connections: NonZeroU32,
    pub sync_interval_secs: NonZeroU64,
    pub openligadb_url: String,
    pub team_id: i32,
    pub league_list: Vec<TrackedLeague>,
}

#[derive(Clone)]
pub struct TrackedLeague {
    pub shortcut: String,
    pub season: i32,
    pub league_id: i32,
    pub has_table: bool,
}

impl Config {
    pub fn load_from_env() -> Result<Config, ConfigError> {
        let database_url = get_var("DATABASE_URL")?;
        let bind_addr = get_var("BIND_ADDR")?;
        let db_max_connections = get_parse_var("DB_MAX_CONNECTIONS")?;
        let sync_interval_secs = get_parse_var("SYNC_INTERVAL_SECS")?;
        let openligadb_url = get_var("OPENLIGADB_BASE_URL")?;
        let team_id = get_parse_var("TEAM_ID")?;
        let league_list = get_league_list("LEAGUE_LIST")?;

        Ok(Config {
            database_url,
            bind_addr,
            db_max_connections,
            sync_interval_secs,
            openligadb_url,
            team_id,
            league_list,
        })
    }
}

fn get_var(name: &'static str) -> Result<String, ConfigError> {
    env::var(name).map_err(|source| ConfigError::Missing { name, source })
}

fn get_parse_var<F: FromStr<Err=ParseIntError>>(name: &'static str) -> Result<F, ConfigError> {
    get_var(name)?
        .parse()
        .map_err(|source| ConfigError::Invalid { name, source })
}

fn get_league_list(name: &'static str) -> Result<Vec<TrackedLeague>, ConfigError> {
    let raw = get_var(name)?;
    let mut leagues = Vec::new();

    for entry in raw.split(',') {
        let entry = entry.trim();
        let parts = entry.split(':').collect::<Vec<_>>();

        let [shortcut, season, league_id, has_table] = parts[..] else {
            return Err(ConfigError::InvalidEntry {
                name,
                entry: entry.to_string(),
                reason: "expected shortcut:season:league_id:has_table".to_string(),
            });
        };

        leagues.push(TrackedLeague {
            shortcut: shortcut.to_string(),
            season: season.parse().map_err(|e: ParseIntError| ConfigError::InvalidEntry {
                name,
                entry: entry.to_string(),
                reason: e.to_string(),
            })?,
            league_id: league_id.parse().map_err(|e: ParseIntError| ConfigError::InvalidEntry {
                name,
                entry: entry.to_string(),
                reason: e.to_string(),
            })?,
            has_table: has_table.parse().map_err(|e: ParseBoolError| ConfigError::InvalidEntry {
                name,
                entry: entry.to_string(),
                reason: e.to_string(),
            })?,
        });
    }

    Ok(leagues)
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{name} is not set")]
    Missing {
        name: &'static str,
        source: env::VarError,
    },
    #[error("{name} is not a valid number: {source}")]
    Invalid {
        name: &'static str,
        source: ParseIntError,
    },
    #[error("{name} entry {entry} is invalid: {reason}")]
    InvalidEntry {
        name: &'static str,
        entry: String,
        reason: String,
    },
}
