use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

use crate::state::server::playlist::PlaylistPage;

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Musiclist {
    pub visible: bool,
    pub page: PlaylistPage::State,
    pub pages: Vec<PlaylistPage::State>,
    pub items: Vec<MusiclistItem::State>,
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MusiclistItem {
    pub music_id: String,
    pub datetime: String,
    pub youtube_title: String,
    pub title: String,
    pub artist: String,
    pub upload_by: String,
    pub lyrics_by: String,
    pub info: String,
    pub tag: String,
    pub volume: i32,
}
