use crate::sync::dto::MatchResponse;
use crate::sync::error::SyncError;
use reqwest::Url;
use std::time::Duration;

pub(super) struct Client {
    client: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub(super) fn new(base_url: Url, timeout: Duration) -> Result<Self, SyncError> {
        let client = reqwest::Client::builder().timeout(timeout).build()?;

        Ok(Self { client, base_url })
    }

    pub(super) async fn matches(
        &self,
        shortcut: &str,
        season: i32,
    ) -> Result<Vec<MatchResponse>, SyncError> {
        let url = self
            .base_url
            .join(&format!("getmatchdata/{shortcut}/{season}"))?;

        let matches = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<MatchResponse>>()
            .await?;

        Ok(matches)
    }
}
