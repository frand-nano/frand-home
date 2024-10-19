use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::{music_queue_state::MusicQueueState, server_player_state::ServerPlayerState};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct ServerMusicState {
    pub server_player: ServerPlayerState,    
    pub music_queue: MusicQueueState,
}
