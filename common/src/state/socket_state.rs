use frand_home_base::{JsonConvert, PropertyState};
use serde::{Deserialize, Serialize};

use super::{client::client_state::ClientState, server::server_state::ServerState};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PropertyState, JsonConvert)]
pub struct SocketState {
    pub server: ServerState,
    pub client: ClientState,
    pub opened: (),
    pub closed: (),
}
