use frand_home_base::{JsonConvert, PropertyState};
use serde::{Serialize, Deserialize};

use super::{client::client_state::ClientState, request_state::RequestState, server::server_state::ServerState};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState, JsonConvert)]
pub struct SocketState {
    pub server: ServerState,
    pub client: ClientState,
    pub request: RequestState,
    pub opened: (),
    pub closed: (),
}