use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct YoutubePlayer {
    pub video_id: String,
}
