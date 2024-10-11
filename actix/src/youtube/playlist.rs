use anyhow::anyhow;
use awc::Client;
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


#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    pub id: String,
    pub snippet: PlaylistItemSnippet,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippet {
    pub title: String,
}