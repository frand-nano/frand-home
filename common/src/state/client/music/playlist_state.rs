use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct PlaylistState {
    pub visible: bool,
    pub list_items: PlaylistItemsState,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct PlaylistItemsState {
    pub items: Vec<PlaylistItemState>,    
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct PlaylistItemState {
    pub playlist_id: String,
    pub title: String,
}