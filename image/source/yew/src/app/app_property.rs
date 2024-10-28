use frand_home_app::state::socket_state::{SocketStateMessage, SocketStateProperty};
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Default, Clone, PartialEq, Properties)]
pub struct AppProperty {
    pub socket: SocketStateProperty,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AppMessage {
    Send(SocketStateMessage),
    Receive(SocketStateMessage),
}

impl From<SocketStateMessage> for AppMessage {
    fn from(value: SocketStateMessage) -> Self {
        Self::Send(value)
    }
}