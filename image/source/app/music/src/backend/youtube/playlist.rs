use anyhow::anyhow;
use awc::Client;
use serde::{Deserialize, Serialize};

use crate::backend::config::Config;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub items: Vec<PlaylistItem>,    
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

impl Playlist {
    pub async fn youtube_get(
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let playlists = config.playlists.iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join(",");

        let params = [
            ("part", "snippet"),
            ("id", &playlists),
            ("key", &config.youtube_api_key),
            ("maxResults", &config.youtube_playlists_max_results.to_string()),
        ];

        let mut response = client
        .get(&config.youtube_playlists)
        .query(&params)?
        .send().await
        .map_err(|err| anyhow!("{err}"))?;

        let result = if response.status().is_success() {
            log::info!("🔎 Playlist::youtube_get playlists: {}",
                playlists,
            );

            response.json::<Self>().await
            .map_err(|err| err.into())
        } else {
            log::error!(" Playlist::youtube_get 
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