use std::any::Any;

use serde::{Deserialize, Serialize};
use yew::{BaseComponent, Callback, Context, Properties};

use crate::{vec_pushed, State, StateMessage};

impl<V: Default + Clone + PartialEq> State for Option<V> {
    type Message = OptionStateMessage<V>;
    type Property = OptionStateProperty<V>;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => *self = value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OptionStateMessage<V: Clone> {
    Error(String),
    State(Option<V>),
}

impl<V: Clone + 'static> StateMessage for OptionStateMessage<V> {
    fn try_new(
        ids: &[usize],
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn Any>,
    ) -> Result<Self, Box<dyn Any>> {
        match ids[index] {
            1 => Ok(Self::State(*value.downcast()?)),
            _ => Err(value),
        }        
    }
    
    fn error(err: String) -> Self {
        Self::Error(err)
    }
}

#[derive(Clone, Properties, PartialEq, Default)]
pub struct OptionStateProperty<V: Clone + PartialEq> {
    pub callback: OptionStatePropertyCallback<V>,
}

impl<V: Default + Clone + PartialEq + 'static> OptionStateProperty<V> {
    pub fn value(&self) -> Option<&V> { self.callback.value.as_ref() }

    pub fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self     
    where
        Comp: BaseComponent,
        Msg: StateMessage,
        <Comp as BaseComponent>::Message: From<Msg>,
    {
        Self { 
            callback: OptionStatePropertyCallback::new(ids, context), 
        }
    }

    pub fn applied(&self, value: &Option<V>) -> Self {
        Self {
            callback: self.callback.applied(value),        
        }
    }

    pub fn callback(&self) -> &Callback<Option<V>> {
        self.callback.callback()
    }

    pub fn emit(&self, value: Option<V>) {
        self.callback.emit(value)
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct OptionStatePropertyCallback<V: Clone> {
    ids: Vec<usize>,
    callback: Callback<Option<V>>,
    value: Option<V>,
}

impl<V> OptionStatePropertyCallback<V> 
where V: Clone + Default + 'static
{
    pub fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self 
    where
        Comp: BaseComponent,
        Msg: StateMessage,
        <Comp as BaseComponent>::Message: From<Msg>,
    {
        let callback_ids = vec_pushed(&ids, 1);
        let callback = match context {
            Some(context) => context.link().callback(move |items: Option<V>| 
                Msg::new(callback_ids.as_slice(), 0, Box::new(items))
            ),
            None => Default::default(),
        };

        Self { 
            ids, 
            callback, 
            value: None,
        }
    }

    pub fn applied(&self, value: &Option<V>) -> Self {        
        Self { 
            ids: self.ids.clone(), 
            callback: self.callback.clone(), 
            value: value.clone(),
        }
    }

    pub fn callback(&self) -> &Callback<Option<V>> {
        &self.callback
    }

    pub fn emit(&self, value: Option<V>) {
        self.callback.emit(value)
    }
}