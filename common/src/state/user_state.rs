use frand_home_base::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct UserState {
    pub name: String,
}

#[derive(Default, Clone, PartialEq, frand_home_base::yew::Properties)]
pub struct UserStateProperty {
    pub state: Node<UserState>,
    pub name: Node<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum UserStateMessage {
    Error(String),
    State(UserState),
    Name(String),
}

impl frand_home_base::State for UserState {
    type Property = UserStateProperty;
    type Message = UserStateMessage;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => *self = value,
            Self::Message::Name(value) => self.name = value,
        }
    }

    fn export_to(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(value) => *value = self.clone(),
            Self::Message::Name(value) => *value = self.name.clone(),
        }
    }
}

impl frand_home_base::StateProperty for UserStateProperty {
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

impl frand_home_base::StateMessage for UserStateMessage {
    fn error(err: String) -> Self { Self::Error(err) }
}