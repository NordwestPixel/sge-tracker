use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("OpenLigaDB request failed")]
    Http(#[source] reqwest::Error),
    #[error("unexpected response shape from OpenLigaDB")]
    Decode(#[source] reqwest::Error),
    #[error("OPENLIGADB_BASE_URL is not a valid URL")]
    ParseUrl(#[from] url::ParseError),
}

impl From<reqwest::Error> for SyncError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_decode() {
            Self::Decode(e)
        } else {
            Self::Http(e)
        }
    }
}
