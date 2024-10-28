use frand_home_state::PropertyState;
use serde::{Deserialize, Serialize};

use super::playlist_state::PlaylistState;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct ServerState {
    pub playlist: PlaylistState,
}
