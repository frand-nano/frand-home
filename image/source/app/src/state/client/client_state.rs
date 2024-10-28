use frand_home_state::PropertyState;
use serde::{Deserialize, Serialize};

use super::{task_bar::TaskBarState, user_state::UserState};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct ClientState {
    pub user: UserState,
    pub task_bar: TaskBarState,    
    pub music: frand_home_music::ClientState,
}
