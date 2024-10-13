use serde::{Deserialize, Serialize};
use yew::{Callback, Properties};

use crate::{node::{Node, NodeValue}, state::{State, StateMessage, StateProperty}, vec_pushed};

#[derive(Default, Clone, PartialEq, Properties)]
pub struct OptionNode<V: NodeValue> {
    value: Node<Option<V>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OptionMessage<V: NodeValue> {
    Error(String),
    State(Option<V>),
}

impl<V: NodeValue> OptionNode<V> {
    pub fn value(&self) -> &Option<V> { self.value.value() }
    pub fn callback(&self) -> &Callback<Option<V>> { self.value.callback() }

    fn apply(&mut self, value: Option<V>) {
        self.value.apply(value);
    }

    pub fn emit(&self, value: Option<V>) {
        self.value.emit(value);
    }
}

impl<V: NodeValue> NodeValue for Option<V> {}

impl<V: NodeValue> State for Option<V> {
    type Property = OptionNode<V>;
    type Message = OptionMessage<V>;
}

impl<V: NodeValue> StateProperty for OptionNode<V> {
    type Message = OptionMessage<V>;

    fn apply_message(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => self.apply(value),
        }
    }

    fn export_message(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(value) => *value = self.value().clone(),
        }
    }

    fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: Option<&yew::Context<Comp>>,
    ) -> Self    
    where
        Comp: yew::BaseComponent,
        Msg: StateMessage,
        <Comp as yew::BaseComponent>::Message: From<Msg>,
    {
        Self { 
            value: Node::new(
                vec_pushed(&ids, 1), 
                context,
            ),
        }
    }
}

impl<V: NodeValue> StateMessage for OptionMessage<V> {
    fn error(err: String) -> Self { Self::Error(err) }
    
    fn try_new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn std::any::Any>,
    ) -> Result<Self, Box<dyn std::any::Any>> {
        match ids[index] {
            1 => Ok(Self::State(*value.downcast()?)),
            _ => Err(value),
        }        
    }
}