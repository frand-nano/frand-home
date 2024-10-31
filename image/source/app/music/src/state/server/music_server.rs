use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

use crate::state::server::playlist::Playlist;

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MusicServer {
    pub playlist: Playlist::State,
}
