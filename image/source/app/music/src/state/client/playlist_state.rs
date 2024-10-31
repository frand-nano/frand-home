use frand_home_node::NodeState;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, NodeState)]
pub struct PlaylistState {
    pub list_items: PlaylistItemsState,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, NodeState)]
pub struct PlaylistItemsState {
    pub items: Vec<PlaylistItemState>,    
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, NodeState)]
pub struct PlaylistItemState {
    pub playlist_id: String,
    pub title: String,
    pub refresh: bool,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, NodeState)]
pub struct PlaylistPageState {
    pub playlist_id: String,
    pub page_token: Option<String>,
}