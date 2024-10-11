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
    pub send: <SocketState as frand_home_base::State>::Property,
    pub receive: <SocketState as frand_home_base::State>::Property,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AppStateMessage {
    Error(String),
    State(AppState),
    Send(SocketStateMessage),
    Receive(SocketStateMessage),
}

impl frand_home_base::NodeValue for AppState {}

impl frand_home_base::State for AppState {
    type Property = AppStateProperty;
    type Message = AppStateMessage;
}

impl frand_home_base::StateProperty for AppStateProperty {
    type Message = AppStateMessage;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => {
                self.send.apply(
                    <SocketState as frand_home_base::State>::Message::State(
                        value.send.clone()
                    ), 
                );
                self.receive.apply(
                    <SocketState as frand_home_base::State>::Message::State(
                        value.receive.clone()
                    ), 
                );
                self.state.apply(value);
            },
            Self::Message::Send(message) => self.send.apply(message),
            Self::Message::Receive(message) => self.receive.apply(message),
        }
    }

    fn export_to(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(value) => *value = self.state.value().clone(),
            Self::Message::Send(message) => self.send.export_to(message),
            Self::Message::Receive(message) => self.receive.export_to(message),
        }
    }

    fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: Option<&frand_home_base::yew::Context<Comp>>,
    ) -> Self    
    where
        Comp: frand_home_base::yew::BaseComponent,
        Msg: frand_home_base::StateMessage,
        <Comp as frand_home_base::yew::BaseComponent>::Message: From<Msg>,
    {
        Self { 
            state: frand_home_base::Node::new(
                frand_home_base::vec_pushed(&ids, 1), 
                context,
            ),
            send: <SocketState as frand_home_base::State>::Property::new(
                frand_home_base::vec_pushed(&ids, 2), 
                context,
            ),
            receive: <SocketState as frand_home_base::State>::Property::new(
                frand_home_base::vec_pushed(&ids, 3), 
                context,
            ),
        }
    }
}

impl frand_home_base::StateMessage for AppStateMessage {
    fn error(err: String) -> Self { Self::Error(err) }
    
    fn try_new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn std::any::Any>,
    ) -> Result<Self, Box<dyn std::any::Any>> {
        match ids[index] {
            1 => Ok(Self::State(*value.downcast()?)),
            2 => Ok(Self::Send(
                <SocketState as frand_home_base::State>::Message::new(ids, index+1, value)
            )),
            3 => Ok(Self::Receive(
                <SocketState as frand_home_base::State>::Message::new(ids, index+1, value)
            )),
            _ => Err(value),
        }        
    }
}