use std::any::Any;

use serde::{Deserialize, Serialize};
use yew::{BaseComponent, Callback, Context, Properties};

use crate::{vec_pushed, State, StateMessage};

impl<V: Default + Clone + PartialEq> State for Vec<V> {
    type Message = VecStateMessage<V>;
    type Property = VecStateProperty<V>;

    fn apply(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(value) => *self = value,
            Self::Message::Len(len) => self.resize(len, V::default()),
            Self::Message::Item((index, value)) => self[index] = value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VecStateMessage<V: Clone> {
    Error(String),
    State(Vec<V>),
    Len(usize),
    Item((usize, V)),
}

impl<V: Clone + 'static> StateMessage for VecStateMessage<V> {
    fn try_new(
        ids: &[usize],
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn Any>,
    ) -> Result<Self, Box<dyn Any>> {
        match ids[index] {
            1 => Ok(Self::State(*value.downcast()?)),
            2 => Ok(Self::Len(*value.downcast()?)),
            3 => Ok(Self::Item((ids[index + 1], *value.downcast()?))),
            _ => Err(value),
        }        
    }
    
    fn error(err: String) -> Self {
        Self::Error(err)
    }
}

#[derive(Clone, Properties, PartialEq, Default)]
pub struct VecStateProperty<V: Clone + PartialEq> {
    pub callback: VecStatePropertyCallback<V>,
}

impl<V: Default + Clone + PartialEq + 'static> VecStateProperty<V> {
    pub fn items(&self) -> &Vec<V> { &self.callback.items }
    pub fn item(&self, index: usize) -> &V { &self.callback.items[index] }

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
            callback: VecStatePropertyCallback::new(ids, context), 
        }
    }

    pub fn applied(&self, items: &Vec<V>) -> Self {
        Self {
            callback: self.callback.applied(items),        
        }
    }

    pub fn callback(&self) -> &Callback<Vec<V>> {
        self.callback.callback()
    }

    pub fn callback_len(&self) -> &Callback<usize> {
        self.callback.callback_len()
    }

    pub fn callback_item(&self) -> &Callback<(usize, V)> {
        self.callback.callback_item()
    }

    pub fn emit(&self, items: Vec<V>) {
        self.callback.emit(items)
    }

    pub fn emit_len(&self, len: usize) {
        self.callback.emit_len(len)
    }

    pub fn emit_item(&self, index: usize, item: V) {
        self.callback.emit_item(index, item)
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct VecStatePropertyCallback<V: Clone> {
    ids: Vec<usize>,
    callback: Callback<Vec<V>>,
    callback_len: Callback<usize>,
    callback_item: Callback<(usize, V)>,
    items: Vec<V>,
}

impl<V> VecStatePropertyCallback<V> 
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
            Some(context) => context.link().callback(move |items: Vec<V>| 
                Msg::new(callback_ids.as_slice(), 0, Box::new(items))
            ),
            None => Default::default(),
        };

        let len_callback_ids = vec_pushed(&ids, 2);
        let len_callback = match context {
            Some(context) => context.link().callback(move |len: usize| 
                Msg::new(len_callback_ids.as_slice(), 0, Box::new(len))
            ),
            None => Default::default(),
        };

        let item_callback_ids = vec_pushed(&ids, 3);
        let item_callback = match context {
            Some(context) => context.link().callback(move |(index, value): (usize, V)| 
                Msg::new(vec_pushed(&item_callback_ids, index).as_slice(), 0, Box::new(value))
            ),
            None => Default::default(),
        };

        Self { 
            ids, 
            callback, 
            callback_len: len_callback, 
            callback_item: item_callback,
            items: Vec::default(),
        }
    }

    pub fn applied(&self, items: &Vec<V>) -> Self {        
        Self { 
            ids: self.ids.clone(), 
            callback: self.callback.clone(), 
            callback_len: self.callback_len.clone(), 
            callback_item: self.callback_item.clone(), 
            items: items.clone(),
        }
    }

    pub fn callback(&self) -> &Callback<Vec<V>> {
        &self.callback
    }

    pub fn callback_len(&self) -> &Callback<usize> {
        &self.callback_len
    }

    pub fn callback_item(&self) -> &Callback<(usize, V)> {
        &self.callback_item
    }

    pub fn emit(&self, items: Vec<V>) {
        self.callback.emit(items)
    }

    pub fn emit_len(&self, len: usize) {
        self.callback_len.emit(len)
    }

    pub fn emit_item(&self, index: usize, item: V) {
        self.callback_item.emit((index, item))
    }
}