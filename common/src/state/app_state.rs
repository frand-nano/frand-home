use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::socket_state::SocketState;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct AppState {
    pub send: SocketState,
    pub receive: SocketState,
}
