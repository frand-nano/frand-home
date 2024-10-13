use anyhow::anyhow;
use awc::Client;
use frand_home_common::state::client::music::playlist_state::{PlaylistItemState, PlaylistItemsState};
use serde::{Deserialize, Serialize};

use crate::CONFIG;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub items: Vec<PlaylistItem>,    
}

impl Playlist {
    pub async fn youtube_get(
        client: &Client,
        playlist_ids: &Vec<String>,
    ) -> anyhow::Result<Self> {
        let playlist_ids = playlist_ids.join(",");
        let params = [
            ("part", "snippet"),
            ("id", &playlist_ids),
            ("key", &CONFIG.keys.youtube_api_key),
            ("maxResults", &CONFIG.settings.youtube_playlists_max_results.to_string()),
        ];
        let mut response = client
        .get(&CONFIG.uris.youtube_playlists)
        .query(&params)?
        .send().await
        .map_err(|err| anyhow!("{err}"))?;

        if response.status().is_success() {
            response.json::<Self>().await
            .map_err(|err| err.into())
        } else {
            log::error!("❗ Playlist::youtube_get 
                playlist_ids: {}, 
                response.json(): {:#?},
                ",
                playlist_ids,
                response.json::<serde_json::Value>().await?,
            );
            Err(anyhow!("response.status(): {}", response.status()))
        }
    }
}

impl From<Playlist> for PlaylistItemsState {
    fn from(value: Playlist) -> Self {
        Self { 
            items: value.items.into_iter().map(|item| item.into()).collect(), 
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    pub id: String,
    pub snippet: PlaylistItemSnippet,
}

impl From<PlaylistItem> for PlaylistItemState {
    fn from(value: PlaylistItem) -> Self {
        Self {
            playlist_id: value.id,
            title: value.snippet.title,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippet {
    pub title: String,
}