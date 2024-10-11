use frand_home_macro::{JsonConvert, PropertyState};
use serde::{Deserialize, Serialize};

use super::client::client_state::ClientState;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PropertyState, JsonConvert)]
pub struct SocketState {
    pub client: ClientState,
    pub opened: (),
    pub closed: (),
}
