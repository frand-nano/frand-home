use yew::{BaseComponent, Callback, Context, Properties};

use crate::StateMessage;

pub trait NodeValue: 'static + Default + PartialEq {}

#[derive(Default, Clone, Properties)]
pub struct Node<V: NodeValue> {
    node: NodeInner<V>,
}

#[derive(Default, Clone)]
pub struct NodeInner<V: NodeValue> {
    value: V,
    callback: Callback<V>,
}

impl<V: NodeValue> PartialEq for Node<V> {
    fn eq(&self, other: &Self) -> bool {
        self.node.value == other.node.value 
    }
}

impl<V: NodeValue> Node<V> {
    pub fn value(&self) -> &V { &self.node.value }

    pub fn apply(&mut self, value: V) {
        self.node.value = value;
    }

    pub fn emit(&self, value: V) {
        self.node.callback.emit(value)
    }

    pub fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self 
    where
        Comp: BaseComponent,
        Msg: StateMessage,
        <Comp as BaseComponent>::Message: From<Msg>,
    {
        let callback_ids = ids.clone();
        let callback = match context {
            Some(context) => context.link().callback(move |value: V| 
                Msg::new(callback_ids.as_slice(), 0, Box::new(value))
            ),
            None => Default::default(),
        };

        Self { 
            node: NodeInner { 
                value: V::default(), 
                callback, 
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