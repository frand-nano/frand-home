use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{backend::component::Music, state::server::playlist_state::{PlaylistItemState, PlaylistItemsState}};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub items: Vec<PlaylistItem>,    
}

impl Playlist {
    pub async fn youtube_get(
        music: &Music,
    ) -> anyhow::Result<Self> {
        let playlists = music.config.playlists.join(",");
        let params = [
            ("part", "snippet"),
            ("id", &playlists),
            ("key", &music.config.youtube_api_key),
            ("maxResults", &music.config.youtube_playlists_max_results.to_string()),
        ];
        let mut response = music.client
        .get(&music.config.youtube_playlists)
        .query(&params)?
        .send().await
        .map_err(|err| anyhow!("{err}"))?;

        let result = if response.status().is_success() {
            response.json::<Self>().await
            .map_err(|err| err.into())
        } else {
            log::error!("❗ Playlist::youtube_get 
                playlists: {}, 
                response.json(): {:#?},
                ",
                playlists,
                response.json::<serde_json::Value>().await?,
            );
            Err(anyhow!("response.status(): {}", response.status()))
        };

        result
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
            refresh: false,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippet {
    pub title: String,
}