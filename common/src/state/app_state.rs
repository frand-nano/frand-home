use frand_home_base::Node;
use serde::{Deserialize, Serialize};

use super::socket_state::{SocketState, SocketStateMessage};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct AppState {
    pub send: SocketState,
    pub receive: SocketState,
}

#[derive(Default, Clone, PartialEq, frand_home_base::yew::Properties)]
pub struct AppStateProperty {
    pub state: Node<AppState>,
    pub send: Node<SocketState>,
    pub receive: Node<SocketState>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AppStateMessage {
    Error(String),
    State(AppState),
    Send(SocketStateMessage),
    Receive(SocketStateMessage),
}

impl frand_home_base::State for AppState {
    type Property = AppStateProperty;
    type Message = AppStateMessage;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => *self = value,
            Self::Message::Send(value) => self.send.apply(value),
            Self::Message::Receive(value) => self.receive.apply(value),
        }
    }

    fn export_to(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(value) => *value = self.clone(),
            Self::Message::Send(value) => self.send.export_to(value),
            Self::Message::Receive(value) => self.receive.export_to(value),
        }
    }
}

impl frand_home_base::StateProperty for AppStateProperty {
    fn new<Comp, State, Msg>(
        ids: Vec<usize>,
        state: &State,
        context: Option<&yew::Context<Comp>>,
    ) -> Self    
    where
        Comp: yew::BaseComponent,
        Msg: frand_home_base::StateMessage,
        <Comp as yew::BaseComponent>::Message: From<Msg>,
     {
        todo!()
    }
}

impl frand_home_base::StateMessage for AppStateMessage {
    fn error(err: String) -> Self { Self::Error(err) }
}