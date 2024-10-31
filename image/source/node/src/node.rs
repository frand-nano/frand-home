use std::fmt::Debug;
use yew::{BaseComponent, Context, Properties};

use crate::{base::{ids_pushed, Callback, Item, Message, MessageData, Node, StateNode, STATE_ID}, impl_item_for};

#[derive(Debug, Clone, Properties)]
pub struct ValueNode<I: Item> {
    ids: Vec<usize>,
    callback: Callback<I>,
    state: I,
}

impl_item_for!{ 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    char, bool, (),
    String,
}

impl<I: Item> ValueNode<I> {
    pub fn value(&self) -> &I { &self.state }
}

impl<I: Item> PartialEq for ValueNode<I> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state 
    }
}

impl<I: Item + Message> Node for ValueNode<I> {
    type Item = I;
    type Message = I;
    
    fn new<Comp: BaseComponent, Msg: Message>(
        ids: Vec<usize>,
        id: Option<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self where <Comp as BaseComponent>::Message: From<Msg> {
        let ids = ids_pushed(ids, id);
        Self { 
            ids: ids.clone(),
            callback: Callback::new(ids, STATE_ID, context),
            state: I::default(),
        }
    }    

    fn new_default(
        ids: Vec<usize>,
        id: Option<usize>,
    ) -> Self {
        let ids = ids_pushed(ids, id);
        Self { 
            ids: ids.clone(),
            callback: Callback::new_default(ids, STATE_ID),
            state: I::default(),
        }
    }

    fn ids(&self) -> &Vec<usize> { &self.ids }
    fn set_id(&mut self, index: usize, id: usize) { 
        self.ids[index] = id;
        self.callback.set_id(index, id);
    }
}

impl<I: Item + Message> StateNode<I> for ValueNode<I> {
    fn callback(&self) -> &Callback<I> { &self.callback }
    fn clone_state(&self) -> I { self.state.clone() }
    fn apply_state(&mut self, state: I) { self.state = state }
    fn apply(&mut self, message: Self::Message) {
        self.apply_state(message);
    }
}