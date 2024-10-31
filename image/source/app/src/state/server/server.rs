use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

use frand_home_music::state::server::music_server::MusicServer;

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Server {
    pub music: MusicServer::State,    
}
