use anyhow::anyhow;
use serde::Deserialize;

use crate::state::server::playlist::PlaylistId;

#[derive(Deserialize)]
pub struct Config {
    pub youtube_playlists: String,
    pub youtube_playlist_items: String,
    pub youtube_api_key: String,
    pub youtube_playlists_max_results: u32,
    pub youtube_playlist_items_max_results: u32,
    pub playlist_items_max_results: usize,
    pub playlists: Vec<PlaylistId>,
}

impl Config {
    pub fn playlist_id(&self, playlist_id: &str) -> anyhow::Result<PlaylistId> {
        self.playlists.iter()
        .find(|t| t.as_str() == playlist_id)
        .map(|t| t.clone())
        .ok_or_else(|| anyhow!("❗ Config.playlists not contains {playlist_id}"))
    }
}
