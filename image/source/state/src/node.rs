use std::fmt::Debug;

use yew::{BaseComponent, Callback, Context, Properties};

use crate::state::StateMessage;

pub trait NodeValue: 'static + Debug + Default + Clone + PartialEq + Send + Sync {}

#[derive(Debug, Clone, Properties)]
pub struct Node<V: NodeValue> {
    node: NodeInner<V>,
}

#[derive(Debug, Clone)]
pub struct NodeInner<V: NodeValue> {
    ids: Vec<usize>,
    value: V,
    callback: Option<Callback<V>>,
}

impl<V: NodeValue> PartialEq for Node<V> {
    fn eq(&self, other: &Self) -> bool {
        self.node.value == other.node.value 
    }
}

impl<V: NodeValue> Node<V> {
    pub fn value(&self) -> &V { &self.node.value }
    pub fn callback(&self) -> &Callback<V> { 
        match &self.node.callback {
            Some(callback) => callback,
            None => {todo!()},
        }
    }

    pub fn apply(&mut self, value: V) {
        self.node.value = value.clone();
    }

    pub fn export<Msg: StateMessage>(&mut self, value: V) -> Msg {
        Msg::new(self.node.ids.as_slice(), 0, Box::new(value))
    }

    pub fn apply_export<Msg: StateMessage>(&mut self, value: V) -> Msg {
        self.node.value = value.clone();
        Msg::new(self.node.ids.as_slice(), 0, Box::new(value))
    }

    pub fn emit(&self, value: V) {        
        match &self.node.callback {
            Some(callback) => callback.emit(value),
            None => {},
        }
    }

    pub fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: &Context<Comp>,
    ) -> Self 
    where
        Comp: BaseComponent,
        Msg: StateMessage,
        <Comp as BaseComponent>::Message: From<Msg>,
    {
        let callback_ids = ids.clone();
        let callback = context.link().callback(move |value: V| 
            Msg::new(callback_ids.as_slice(), 0, Box::new(value))
        );

        Self { 
            node: NodeInner {
                ids: ids.clone(), 
                value: V::default(), 
                callback: Some(callback), 
            }, 
        }
    }

    pub fn new_default(
        ids: Vec<usize>,
    ) -> Self {
        Self { 
            node: NodeInner {
                ids: ids.clone(), 
                value: V::default(), 
                callback: None, 
            }, 
        }
    }
}

macro_rules! node_value {
    ( $head: ty $(,$tys: ty)* $(,)? ) => { 
        impl NodeValue for $head {}         
        $(impl NodeValue for $tys {})*      
    };
}

node_value!{ 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    char, bool, (),
    String,
}