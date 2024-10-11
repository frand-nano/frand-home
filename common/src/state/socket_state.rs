use frand_home_base::Node;
use frand_home_macro::JsonConvert;
use serde::{Deserialize, Serialize};

use super::client::client_state::{ClientState, ClientStateMessage};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, JsonConvert)]
pub struct SocketState {
    pub client: ClientState,
    pub opened: (),
    pub closed: (),
}

#[derive(Default, Clone, PartialEq, frand_home_base::yew::Properties)]
pub struct SocketStateProperty {
    pub state: Node<SocketState>,
    pub client: Node<ClientState>,
    pub opened: Node<()>,
    pub closed: Node<()>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SocketStateMessage {
    Error(String),
    State(SocketState),
    Client(ClientStateMessage),
    Opened(()),
    Closed(()),
}

impl frand_home_base::State for SocketState {
    type Property = SocketStateProperty;
    type Message = SocketStateMessage;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => *self = value,
            Self::Message::Client(value) => self.client.apply(value),
            Self::Message::Opened(value) => self.opened = value,
            Self::Message::Closed(value) => self.closed = value,
        }
    }

    fn export_to(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(value) => *value = self.clone(),
            Self::Message::Client(value) => self.client.export_to(value),
            Self::Message::Opened(value) => *value = self.opened.clone(),
            Self::Message::Closed(value) => *value = self.closed.clone(),
        }
    }
}

impl frand_home_base::StateProperty for SocketStateProperty {
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

impl frand_home_base::StateMessage for SocketStateMessage {
    fn error(err: String) -> Self { Self::Error(err) }
}
