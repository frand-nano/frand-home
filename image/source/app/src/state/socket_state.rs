use frand_home_state::PropertyStateRoot;
use serde::{Deserialize, Serialize};

use super::{client::client_state::ClientState, server::server_state::ServerState};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PropertyStateRoot)]
pub struct SocketState {
    pub server: ServerState,
    pub client: ClientState,
    pub opened: (),
    pub closed: (),
    pub alert: String,
}

