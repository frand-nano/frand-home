use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::{music::client_music_state::ClientMusicState, user_state::UserState, view::task_bar::TaskBarState};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PropertyState)]
pub struct ClientState {
    pub user: UserState,
    pub task_bar: TaskBarState,
    pub music: ClientMusicState,
}
