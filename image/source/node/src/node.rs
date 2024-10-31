use std::fmt::Debug;
use yew::{BaseComponent, Context, Properties};

use crate::{base::{ids_pushed, Callback, State, Message, RootMessage, MessageData, Node, STATE_ID}, impl_state_for};

#[derive(Debug, Clone, Properties)]
pub struct ValueNode<S: State> {
    ids: Vec<usize>,
    callback: Callback<S>,
    state: S,
}

impl_state_for!{ 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    char, bool, (),
    String,
}

impl<S: State> ValueNode<S> {
    pub fn value(&self) -> &S { &self.state }
}

impl<S: State> PartialEq for ValueNode<S> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state 
    }
}

impl<S: State + Message> Node<S> for ValueNode<S> {
    type Message = S;
    
    fn new<Comp: BaseComponent, Msg: RootMessage>(
        ids: Vec<usize>,
        id: Option<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self where <Comp as BaseComponent>::Message: From<Msg> {
        let ids = ids_pushed(ids, id);
        Self { 
            ids: ids.clone(),
            callback: Callback::new(ids, STATE_ID, context),
            state: S::default(),
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
            state: S::default(),
        }
    }

    fn ids(&self) -> &Vec<usize> { &self.ids }
    fn set_id(&mut self, index: usize, id: usize) { 
        self.ids[index] = id;
        self.callback.set_id(index, id);
    }
    fn callback(&self) -> &Callback<S> { &self.callback }
    fn clone_state(&self) -> S { self.state.clone() }
    fn apply_state(&mut self, state: S) { self.state = state }
    fn apply(&mut self, message: Self::Message) {
        self.apply_state(message);
    }
}