use frand_home_node::NodeState;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, NodeState)]
pub struct YoutubePlayerState {
    pub video_id: String,
}
