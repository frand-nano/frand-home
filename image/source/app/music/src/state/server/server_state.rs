use frand_home_node::NodeState;
use serde::{Deserialize, Serialize};

use super::playlist_state::PlaylistState;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, NodeState)]
pub struct ServerState {
    pub playlist: PlaylistState,
}
