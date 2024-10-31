use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

use crate::state::server::playlist::PlaylistPage;

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Musiclist {
    pub playlist_page: PlaylistPage::State,
    pub list_items: MusiclistItems::State,
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MusiclistItems {
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub total_results: usize,
    pub results_per_page: usize,
    pub items: Vec<MusiclistItem::State>,    
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MusiclistItem {
    pub video_id: String,
    pub title: String,
}
