use frand_home_node::node_state;
use serde::{Deserialize, Serialize};
use crate::state::client::musiclist::MusiclistItem;

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct YoutubePlayer {
    pub music: MusiclistItem::State,
}
