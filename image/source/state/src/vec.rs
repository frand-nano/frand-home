use serde::{Deserialize, Serialize};
use yew::{Callback, Properties};

use crate::{node::NodeValue, state::{State, StateMessage, StateProperty}, vec_pushed};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct VecNode<V: NodeValue> {
    node: VecNodeInner<V>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VecNodeInner<V: NodeValue> {
    items: Vec<V>,
    ids: Vec<usize>,
    callback: Option<Callback<Vec<V>>>,
    callback_item: Option<Callback<(usize, V)>>,
    callback_push: Option<Callback<V>>,
    callback_pop: Option<Callback<()>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VecMessage<V: NodeValue> {
    Error(String),
    State(Vec<V>),
    Item((usize, V)),
    Push(V),
    Pop(()),
}

impl<V: NodeValue> VecNode<V> {
    pub fn items(&self) -> &Vec<V> { &self.node.items }
    pub fn item(&self, index: usize) -> Option<&V> { self.node.items.get(index) }
    
    pub fn callback(&self) -> &Callback<Vec<V>> { 
        match &self.node.callback {
            Some(callback) => callback,
            None => {todo!()},
        }
    }
    pub fn callback_item(&self) -> &Callback<(usize, V)> { 
        match &self.node.callback_item {
            Some(callback) => callback,
            None => {todo!()},
        }
    }
    pub fn callback_push(&self) -> &Callback<V> { 
        match &self.node.callback_push {
            Some(callback) => callback,
            None => {todo!()},
        }
    }
    pub fn callback_pop(&self) -> &Callback<()> { 
        match &self.node.callback_pop {
            Some(callback) => callback,
            None => {todo!()},
        }
    }

    pub fn emit(&self, items: Vec<V>) {
        match &self.node.callback {
            Some(callback) => callback.emit(items),
            None => {},
        }
    }

    pub fn emit_item(&self, index: usize, value: V) { 
        match &self.node.callback_item {
            Some(callback) => callback.emit((index, value)),
            None => {},
        }
    }

    pub fn emit_push(&self, value: V) { 
        match &self.node.callback_push {
            Some(callback) => callback.emit(value),
            None => {},
        }
    }

    pub fn emit_pop(&self) { 
        match &self.node.callback_pop {
            Some(callback) => callback.emit(()),
            None => {},
        }
    }

    fn apply(&mut self, value: Vec<V>) {
        self.node.items = value;
    }

    fn apply_item(&mut self, index: usize, item: V) {
        self.node.items[index] = item;
    }

    fn push(&mut self, value: V) {
        self.node.items.push(value)
    }

    fn pop(&mut self) -> Option<V> {
        self.node.items.pop()
    }

    pub fn apply_item_export<Msg: StateMessage>(&mut self, index: usize, item: V) -> Msg {
        self.node.items[index] = item.clone();
        let ids = vec_pushed(&self.node.ids, 2);
        let ids = vec_pushed(&ids, index);
        Msg::new(ids.as_slice(), 0, Box::new(item))
    }
}

impl<V: NodeValue> NodeValue for Vec<V> {}

impl<V: NodeValue> State for Vec<V> {
    type Property = VecNode<V>;
    type Message = VecMessage<V>;
}

impl<V: NodeValue> StateProperty for VecNode<V> {
    type State = Vec<V>;
    type Message = VecMessage<V>;

    fn clone_state(&self) -> Self::State {
        self.node.items.clone()
    }

    fn apply_state(&mut self, state: Self::State) {
        self.apply(state.clone());
    }

    fn apply_export<Msg: StateMessage>(&mut self, value: Vec<V>) -> Msg {
        self.node.items = value.clone();
        let ids = vec_pushed(&self.node.ids, 1);
        Msg::new(ids.as_slice(), 0, Box::new(value))
    }

    fn apply_message(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("❗ {err}"),
            Self::Message::State(items) => {
                self.apply(items);
            },
            Self::Message::Item((index, item)) => {
                self.apply_item(index, item);
            },
            Self::Message::Push(item) => {
                self.push(item);
            },
            Self::Message::Pop(()) => {
                self.pop();
            },
        }
    }

    fn export_message(&self, message: &mut Self::Message) {
        match message {
            Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
            Self::Message::State(items) => *items = self.items().clone(),
            Self::Message::Item((index, item_mut)) => if let Some(item) = self.item(*index) {
                *item_mut = item.clone()
            },
            Self::Message::Push(_) => {},
            Self::Message::Pop(_) => {},
        }
    }

    fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: &yew::Context<Comp>,
    ) -> Self    
    where
        Comp: yew::BaseComponent,
        Msg: StateMessage,
        <Comp as yew::BaseComponent>::Message: From<Msg>,
    {
        let callback_ids = vec_pushed(&ids, 1);
        let callback = context.link().callback(move |items: Vec<V>| 
            Msg::new(callback_ids.as_slice(), 0, Box::new(items))
        );

        let callback_item_ids = vec_pushed(&ids, 2);
        let callback_item = context.link().callback(move |(index, value): (usize, V)| 
            Msg::new(vec_pushed(&callback_item_ids, index).as_slice(), 0, Box::new(value))
        );


        let callback_push_ids = vec_pushed(&ids, 3);
        let callback_push = context.link().callback(move |value: V| 
            Msg::new(callback_push_ids.as_slice(), 0, Box::new(value))
        );

        let callback_pop_ids = vec_pushed(&ids, 4);
        let callback_pop = context.link().callback(move |value: ()| 
            Msg::new(callback_pop_ids.as_slice(), 0, Box::new(value))
        );

        Self { 
            node: VecNodeInner {
                items: Vec::default(),
                ids,
                callback: Some(callback),
                callback_item: Some(callback_item),
                callback_push: Some(callback_push),
                callback_pop: Some(callback_pop),
            },
        }
    }
    
    fn new_default(
        ids: Vec<usize>,
    ) -> Self {        
        Self { 
            node: VecNodeInner {
                items: Vec::default(),
                ids,
                callback: None,
                callback_item: None,
                callback_push: None,
                callback_pop: None,
            },
        }
    }
}

impl<V: NodeValue> StateMessage for VecMessage<V> {
    fn error(err: String) -> Self { Self::Error(err) }
    
    fn try_new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn std::any::Any>,
    ) -> Result<Self, Box<dyn std::any::Any>> {
        match ids[index] {
            1 => Ok(Self::State(*value.downcast()?)),
            2 => Ok(Self::Item((ids[index + 1], *value.downcast()?).into())),
            3 => Ok(Self::Push(*value.downcast()?)),
            4 => Ok(Self::Pop(())),
            _ => Err(value),
        }        
    }
}