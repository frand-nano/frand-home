use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub youtube_api_key: String,
    pub youtube_playlists_max_results: u32,
    pub youtube_playlist_items_max_results: u32,
    pub youtube_playlists: String,
    pub youtube_playlist_items: String,
    pub playlists: Vec<String>,
}
