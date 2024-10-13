use serde::{Deserialize, Serialize};
use yew::{Callback, Properties};

use crate::{node::NodeValue, state::{State, StateMessage, StateProperty}, vec_pushed};

#[derive(Default, Clone, PartialEq, Properties)]
pub struct VecNode<V: NodeValue> {
    node: VecNodeInner<V>,
}

#[derive(Default, Clone, PartialEq)]
pub struct VecNodeInner<V: NodeValue> {
    items: Vec<V>,
    ids: Vec<usize>,
    callback: Callback<Vec<V>>,
    callback_item: Callback<(usize, V)>,
    callback_push: Callback<V>,
    callback_pop: Callback<()>,
}

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn callback(&self) -> &Callback<Vec<V>> { &self.node.callback }
    pub fn callback_item(&self) -> &Callback<(usize, V)> { &self.node.callback_item }
    pub fn callback_push(&self) -> &Callback<V> { &self.node.callback_push }
    pub fn callback_pop(&self) -> &Callback<()> { &self.node.callback_pop }

    pub fn emit(&self, items: Vec<V>) {
        self.node.callback.emit(items);
    }

    pub fn emit_item(&self, index: usize, value: V) { 
        self.node.callback_item.emit((index, value));
    }

    pub fn emit_push(&self, value: V) { 
        self.node.callback_push.emit(value);
    }

    pub fn emit_pop(&self) { 
        self.node.callback_pop.emit(());
    }
}

impl<V: NodeValue> VecNodeInner<V> {
    fn apply(&mut self, value: Vec<V>) {
        self.items = value;
    }

    fn apply_item(&mut self, index: usize, item: V) {
        self.items[index] = item;
    }

    fn push(&mut self, value: V) {
        self.items.push(value)
    }

    fn pop(&mut self) -> Option<V> {
        self.items.pop()
    }
}

impl<V: NodeValue> NodeValue for Vec<V> {}

impl<V: NodeValue> State for Vec<V> {
    type Property = VecNode<V>;
    type Message = VecMessage<V>;
}

impl<V: NodeValue> StateProperty for VecNode<V> {
    type Message = VecMessage<V>;

    fn apply_message(&mut self, message: Self::Message) {
        match message {
            Self::Message::Error(err) => log::error!("{err}"),
            Self::Message::State(items) => {
                self.node.apply(items);
            },
            Self::Message::Item((index, item)) => {
                self.node.apply_item(index, item);
            },
            Self::Message::Push(item) => {
                self.node.push(item);
            },
            Self::Message::Pop(()) => {
                self.node.pop();
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
        context: Option<&yew::Context<Comp>>,
    ) -> Self    
    where
        Comp: yew::BaseComponent,
        Msg: StateMessage,
        <Comp as yew::BaseComponent>::Message: From<Msg>,
    {
        let callback_ids = vec_pushed(&ids, 1);
        let callback = match context {
            Some(context) => context.link().callback(move |items: Vec<V>| 
                Msg::new(callback_ids.as_slice(), 0, Box::new(items))
            ),
            None => Default::default(),
        };

        let callback_item_ids = vec_pushed(&ids, 2);
        let callback_item = match context {
            Some(context) => context.link().callback(move |(index, value): (usize, V)| 
                Msg::new(vec_pushed(&callback_item_ids, index).as_slice(), 0, Box::new(value))
            ),
            None => Default::default(),
        };


        let callback_push_ids = vec_pushed(&ids, 3);
        let callback_push = match context {
            Some(context) => context.link().callback(move |value: V| 
                Msg::new(callback_push_ids.as_slice(), 0, Box::new(value))
            ),
            None => Default::default(),
        };

        let callback_pop_ids = vec_pushed(&ids, 4);
        let callback_pop = match context {
            Some(context) => context.link().callback(move |value: ()| 
                Msg::new(callback_pop_ids.as_slice(), 0, Box::new(value))
            ),
            None => Default::default(),
        };

        Self { 
            node: VecNodeInner {
                items: Vec::default(),
                ids,
                callback,
                callback_item,
                callback_push,
                callback_pop,
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