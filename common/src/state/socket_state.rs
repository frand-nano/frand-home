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
    pub client: <ClientState as frand_home_base::State>::Property,
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

impl frand_home_base::NodeValue for SocketState {}

impl frand_home_base::State for SocketState {
    type Property = SocketStateProperty;
    type Message = SocketStateMessage;
}

impl frand_home_base::StateProperty for SocketStateProperty {
    type Message = SocketStateMessage;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => {
                self.client.apply(
                    <ClientState as frand_home_base::State>::Message::State(
                        value.client.clone()
                    ), 
                );
                self.opened.apply(value.opened.clone());
                self.closed.apply(value.closed.clone());
                self.state.apply(value);
            },
            Self::Message::Client(message) => self.client.apply(message),
            Self::Message::Opened(value) => self.opened.apply(value),
            Self::Message::Closed(value) => self.closed.apply(value),
        }
    }

    fn export_to(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(value) => *value = self.state.value().clone(),
            Self::Message::Client(message) => self.client.export_to(message),
            Self::Message::Opened(value) => *value = self.opened.value().clone(),
            Self::Message::Closed(value) => *value = self.closed.value().clone(),
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
            client: <ClientState as frand_home_base::State>::Property::new(
                frand_home_base::vec_pushed(&ids, 2), 
                context,
            ),
            opened: frand_home_base::Node::new(
                frand_home_base::vec_pushed(&ids, 3), 
                context,
            ),
            closed: frand_home_base::Node::new(
                frand_home_base::vec_pushed(&ids, 4), 
                context,
            ),
        }
    }
}

impl frand_home_base::StateMessage for SocketStateMessage {
    fn error(err: String) -> Self { Self::Error(err) }
    
    fn try_new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn std::any::Any>,
    ) -> Result<Self, Box<dyn std::any::Any>> {
        match ids[index] {
            1 => Ok(Self::State(*value.downcast()?)),
            2 => Ok(Self::Client(
                <ClientState as frand_home_base::State>::Message::new(ids, index+1, value)
            )),
            3 => Ok(Self::Opened(*value.downcast()?)),
            4 => Ok(Self::Closed(*value.downcast()?)),
            _ => Err(value),
        }        
    }
}
