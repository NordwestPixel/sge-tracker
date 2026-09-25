use crate::sync::dto::{GroupResponse, MatchResponse, StandingsResponse};
use crate::sync::error::SyncError;
use chrono::NaiveDateTime;
use reqwest::Url;
use serde::de::DeserializeOwned;
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
        let matches = self
            .get_json(&format!("getmatchdata/{shortcut}/{season}"))
            .await?;

        Ok(matches)
    }

    pub(super) async fn bltable(
        &self,
        shortcut: &str,
        season: i32,
    ) -> Result<Vec<StandingsResponse>, SyncError> {
        let table = self
            .get_json(&format!("getbltable/{shortcut}/{season}"))
            .await?;

        Ok(table)
    }

    pub(super) async fn current_group(&self, shortcut: &str) -> Result<GroupResponse, SyncError> {
        let group = self
            .get_json(&format!("getcurrentgroup/{shortcut}"))
            .await?;

        Ok(group)
    }

    pub(super) async fn last_change_date(
        &self,
        shortcut: &str,
        season: i32,
        group_order_id: i32,
    ) -> Result<NaiveDateTime, SyncError> {
        let date = self
            .get_json(&format!(
                "getlastchangedate/{shortcut}/{season}/{group_order_id}"
            ))
            .await?;

        Ok(date)
    }

    async fn get_json<T>(&self, path: &str) -> Result<T, SyncError>
    where
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;

        let body = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn live_client() -> Client {
        Client::new(
            Url::parse("https://api.openligadb.de").unwrap(),
            Duration::from_secs(10),
        )
        .unwrap()
    }

    #[tokio::test]
    #[ignore = "hits the live OpenLigaDB API"]
    async fn decodes_live_matches() {
        let client = live_client();

        for (shortcut, season) in [("bl1", 2026), ("dfb", 2026), ("ucl", 2025)] {
            let matchess = client.matches(shortcut, season).await.unwrap();
            assert!(
                !matchess.is_empty(),
                "expected matches for {shortcut}/{season}"
            );
        }
    }

    #[tokio::test]
    #[ignore = "hits the live OpenLigaDB API"]
    async fn decodes_live_table() {
        let table = live_client().bltable("bl1", 2026).await.unwrap();

        assert_eq!(table.len(), 18, "expected 18 teams in bl1/2026");
    }

    #[tokio::test]
    #[ignore = "hits the live OpenLigaDB API"]
    async fn decodes_live_current_group_and_last_change_date() {
        let client = live_client();

        let group = client.current_group("bl1").await.unwrap();
        assert!(group.group_order_id > 0);

        client
            .last_change_date("bl1", 2026, group.group_order_id)
            .await
            .unwrap();
    }
}
