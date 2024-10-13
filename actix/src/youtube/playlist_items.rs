use anyhow::anyhow;
use awc::Client;
use frand_home_common::state::client::music::musiclist_state::{MusiclistItemState, MusiclistItemsState};
use serde::{Deserialize, Serialize};

use crate::CONFIG;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItems {
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub page_info: PlaylistItemsPageInfo,
    pub items: Vec<PlaylistItem>,    
}

impl PlaylistItems {
    pub async fn youtube_get(
        client: &Client,
        playlist_id: &str,
    ) -> anyhow::Result<Self> {
        let params = [
            ("part", "snippet"),
            ("playlistId", playlist_id),
            ("key", &CONFIG.keys.youtube_api_key),
        ];
        let mut response = client
        .get(&CONFIG.uris.youtube_playlist_items)
        .query(&params)?
        .send().await
        .map_err(|err| anyhow!("{err}"))?;

        if response.status().is_success() {
            response.json::<Self>().await
            .map_err(|err| err.into())
        } else {
            log::error!("❗ PlaylistItems::youtube_get 
                playlist_id: {}, 
                response.json(): {:#?},
                ",
                playlist_id,
                response.json::<serde_json::Value>().await?,
            );
            Err(anyhow!("response.status(): {}", response.status()))
        }
    }
}

impl From<PlaylistItems> for MusiclistItemsState {
    fn from(value: PlaylistItems) -> Self {
        Self { 
            next_page_token: value.next_page_token, 
            prev_page_token: value.prev_page_token, 
            total_results: value.page_info.total_results,
            results_per_page: value.page_info.results_per_page,
            items: value.items.into_iter().map(|item| item.into()).collect(), 
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsPageInfo {
    pub total_results: usize,
    pub results_per_page: usize,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    pub snippet: PlaylistItemSnippet,
}

impl From<PlaylistItem> for MusiclistItemState {
    fn from(value: PlaylistItem) -> Self {
        Self {
            video_id: value.snippet.resource_id.video_id,
            title: value.snippet.title,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippet {
    pub title: String,
    pub resource_id: PlaylistItemSnippetResourceId,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippetResourceId {
    pub video_id: String,
}