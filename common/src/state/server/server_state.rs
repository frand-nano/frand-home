use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::music::server_music_state::ServerMusicState;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct ServerState {
    pub music: ServerMusicState,
}
