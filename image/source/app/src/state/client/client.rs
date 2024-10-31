use frand_home_node::node_state;
use serde::{Deserialize, Serialize};
use frand_home_music::state::client::music_client::MusicClient;

use crate::state::client::{
    user::User,
    task_bar::TaskBar,
};

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Client {
    pub user: User::State,
    pub task_bar: TaskBar::State,    
    pub music: MusicClient::State,
}
