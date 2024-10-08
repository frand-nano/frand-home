use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::{client_music_state::ClientMusicState, user_state::UserState};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct ClientState {
    pub music: ClientMusicState,
    pub user: UserState,
}
