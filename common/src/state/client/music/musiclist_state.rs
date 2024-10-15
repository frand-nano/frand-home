use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::playlist_state::PlaylistPageState;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct MusiclistState {
    pub playlist_page: PlaylistPageState,
    pub list_items: MusiclistItemsState,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct MusiclistItemsState {
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub total_results: usize,
    pub results_per_page: usize,
    pub items: Vec<MusiclistItemState>,    
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct MusiclistItemState {
    pub video_id: String,
    pub title: String,
}