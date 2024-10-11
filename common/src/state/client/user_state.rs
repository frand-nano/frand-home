use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct UserState {
    pub name: String,
}

#[derive(Default, Clone, PartialEq, frand_home_base::yew::Properties)]
pub struct UserStateProperty {
    pub state: frand_home_base::Node<UserState>,
    pub name: frand_home_base::Node<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum UserStateMessage {
    Error(String),
    State(UserState),
    Name(String),
}

impl frand_home_base::NodeValue for UserState {}

impl frand_home_base::State for UserState {
    type Property = UserStateProperty;
    type Message = UserStateMessage;
}

impl frand_home_base::StateProperty for UserStateProperty {
    type Message = UserStateMessage;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => {
                self.name.apply(value.name.clone());
                self.state.apply(value);
            },
            Self::Message::Name(value) => self.name.apply(value),
        }
    }

    fn export_to(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(value) => *value = self.state.value().clone(),
            Self::Message::Name(value) => *value = self.name.value().clone(),
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
            name: frand_home_base::Node::new(
                frand_home_base::vec_pushed(&ids, 2), 
                context,
            ),
        }
    }
}

impl frand_home_base::StateMessage for UserStateMessage {
    fn error(err: String) -> Self { Self::Error(err) }
    
    fn try_new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn std::any::Any>,
    ) -> Result<Self, Box<dyn std::any::Any>> {
        match ids[index] {
            1 => Ok(Self::State(*value.downcast()?)),
            2 => Ok(Self::Name(*value.downcast()?)),
            _ => Err(value),
        }        
    }
}