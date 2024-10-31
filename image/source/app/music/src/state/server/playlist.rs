use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Playlist {
    pub list_items: PlaylistItems::State,
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaylistItems {
    pub items: Vec<PlaylistItem::State>,    
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaylistItem {
    pub playlist_id: String,
    pub title: String,
    pub refresh: bool,
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaylistPage {
    pub playlist_id: String,
    pub page_token: Option<String>,
}