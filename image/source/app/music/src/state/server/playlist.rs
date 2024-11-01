use frand_home_node::{impl_message_state_for, node_state};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Range};

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Playlist {
    pub items: Vec<PlaylistItem::State>,  
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaylistItem {
    pub youtube_title: String,
    pub update: bool,
    pub playlist_id: PlaylistId,
    pub pages: Vec<PlaylistPage::State>,
}

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaylistPage {
    pub id: PlaylistId,
    pub range: Range<usize>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlaylistId {
    value: String,
}

impl Display for PlaylistId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

impl From<PlaylistId> for String {
    fn from(playlist_id: PlaylistId) -> Self {
        playlist_id.value
    }
}

impl PlaylistId {
    pub fn as_str(&self) -> &str { &self.value }
}

impl_message_state_for!(PlaylistId);
