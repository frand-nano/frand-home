use anyhow::anyhow;
use awc::Client;
use serde::{Deserialize, Serialize};

use crate::backend::config::Config;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItems {
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub page_info: PlaylistItemsPageInfo,
    pub items: Vec<PlaylistItemsItem>,    
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsPageInfo {
    pub total_results: usize,
    pub results_per_page: usize,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsItem {
    pub snippet: PlaylistItemsItemSnippet,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsItemSnippet {
    pub title: String,
    pub resource_id: PlaylistItemsItemSnippetResourceId,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsItemSnippetResourceId {
    pub video_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaylistPage {
    pub playlist_id: String,
    pub page_token: Option<String>,
}

impl PlaylistItems {
    pub async fn youtube_get(
        client: &Client,
        config: &Config,
        playlist_page: &PlaylistPage,
    ) -> anyhow::Result<Self> {
        let params = [
            ("part", "snippet"),
            ("playlistId", playlist_page.playlist_id.as_str()),
            ("pageToken", &playlist_page.page_token.clone().unwrap_or_default()),
            ("key", &config.youtube_api_key),
            ("maxResults", &config.youtube_playlist_items_max_results.to_string()),
        ];
        let mut response = client
        .get(&config.youtube_playlist_items)
        .query(&params)?
        .send().await
        .map_err(|err| anyhow!("{err}"))?;

        let result = if response.status().is_success() {
            log::info!("🔎 PlaylistItems::youtube_get playlist_page: {:#?}",
                playlist_page,
            );
            response.json::<Self>().await
            .map_err(|err| err.into())
        } else {
            log::error!(" PlaylistItems::youtube_get 
                playlist_page: {:#?}, 
                response.json(): {:#?},
                ",
                playlist_page,
                response.json::<serde_json::Value>().await?,
            );
            Err(anyhow!("response.status(): {}", response.status()))
        }?;

        Ok(result)
    }
}