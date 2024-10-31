use std::{fmt::Debug, slice::{Iter, IterMut}};

use serde::{Deserialize, Serialize};
use yew::{BaseComponent, Context, Properties};

use crate::base::{ids_pushed, Callback, Item, Message, MessageData, Node, StateNode, ITEM_ID, POP_ID, PUSH_ID, STATE_ID};

#[derive(Debug, Clone, Properties)]
pub struct VecNode<I: Item> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    ids: Vec<usize>,
    callback: Callback<Vec<I>>,
    item: I::Node,
    state: Vec<I::Node>,
    push_callback: Callback<I>,
    pop_callback: Callback<()>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VecMessage<I: Item> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    Error(String),
    State(Vec<I>),
    Item((usize, <I::Node as Node>::Message)),
    Push(I),
    Pop(()),
}

impl<I: Item> VecNode<I> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    pub fn iter(&self) -> Iter<I::Node> { self.state.iter() }
    pub fn iter_mut(&mut self) -> IterMut<I::Node> { self.state.iter_mut() }
}

impl<I: Item> Item for Vec<I> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    type Node = VecNode<I>;
}

impl<I: Item> PartialEq for VecNode<I> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state 
    }
}

impl<I: Item> Node for VecNode<I> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    type Item = I;
    type Message = VecMessage<I>;
    
    fn new<Comp: BaseComponent, Msg: Message>(
        ids: Vec<usize>,
        id: Option<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self where <Comp as BaseComponent>::Message: From<Msg> {
        let ids = ids_pushed(ids, id);

        Self { 
            ids: ids.clone(),
            callback: Callback::new(ids.clone(), STATE_ID, context),
            item: <I::Node as Node>::new(ids_pushed(ids.clone(), Some(ITEM_ID)), Some(0), context),
            state: Vec::default(),
            push_callback: Callback::new(ids.clone(), PUSH_ID, context),
            pop_callback: Callback::new(ids.clone(), POP_ID, context),
        }
    }    

    fn new_default(
        ids: Vec<usize>,
        id: Option<usize>,
    ) -> Self {
        let ids = ids_pushed(ids, id);
        Self { 
            ids: ids.clone(),
            callback: Callback::new_default(ids.clone(), STATE_ID),
            item: <I::Node as Node>::new_default(ids_pushed(ids.clone(), Some(ITEM_ID)), Some(0)),
            state: Vec::default(),
            push_callback: Callback::new_default(ids.clone(), PUSH_ID),
            pop_callback: Callback::new_default(ids.clone(), POP_ID),
        }
    }

    fn ids(&self) -> &Vec<usize> { &self.ids }
    fn set_id(&mut self, index: usize, id: usize) { 
        self.ids[index] = id;
        self.callback.set_id(index, id);
        self.item.set_id(index, id);
        for item in &mut self.state {
            item.set_id(index, id);
        }
        self.push_callback.set_id(index, id);
        self.pop_callback.set_id(index, id);
    }
}

impl<I: Item> Message for VecMessage<I> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    fn error(err: String) -> Self { Self::Error(err) }
    
    fn try_new(depth: usize, data: MessageData) -> Result<Self, (Vec<usize>, usize)> {
        match data.ids[depth] {
            STATE_ID => match data.data.downcast() {
                Ok(data) => Ok(Self::State(*data)),
                Err(_) => Err((data.ids, depth)),
            },
            ITEM_ID => Ok(Self::Item((data.ids[depth+1], <I::Node as Node>::Message::new(depth+2, data)))),
            PUSH_ID => match data.data.downcast() {
                Ok(data) => Ok(Self::Push(*data)),
                Err(_) => Err((data.ids, depth)),
            },
            POP_ID => Ok(Self::Pop(())),
            _ => Err((data.ids, depth)),
        }    
    }
}

impl<I: Item> From<MessageData> for VecMessage<I> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    fn from(data: MessageData) -> Self {
        Self::new(0, data)
    }
}

impl<I: Item> StateNode<Vec<I>> for VecNode<I> 
    where 
    <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>,
    I::Node : StateNode<I>
{
    fn callback(&self) -> &Callback<Vec<I>> { &self.callback }
    fn clone_state(&self) -> Vec<I> { 
        self.state.iter()
        .map(|item| item.clone_state())
        .collect()
    }

    fn apply_state(&mut self, state: Vec<I>) { 
        if state.len() < self.state.len() {
            self.state.truncate(state.len());
        } else if self.state.len() < state.len() {
            let (start, end) = (self.state.len(), state.len());
            for index in start..end {
                let mut item = self.item.clone();                
                item.set_id(item.ids().len()-1, index);
                self.state.push(item);
            }
        }

        for (index, item) in state.into_iter().enumerate() {
            self.state[index].apply_state(item)
        }  
    }

    fn apply(&mut self, message: Self::Message) {
        match message {
            VecMessage::Error(err) => {
                log::error!("❗ {}.apply_message: {err}", stringify!(VecNode));
            },
            VecMessage::State(state) => self.apply_state(state),
            VecMessage::Item((index, message)) => {
                self.state[index].apply(message);
            },
            VecMessage::Push(state) => {
                let mut item = self.item.clone();       
                item.apply_state(state);   
                item.set_id(item.ids().len()-1, self.state.len());
                self.state.push(item);
            },
            VecMessage::Pop(()) => {
                self.state.pop();
            },
        }     
    }
}

impl<I: Item> VecNode<I> 
    where <I::Node as Node>::Message : Serialize + for<'a> Deserialize<'a>
{
    pub fn push_callback(&self) -> &Callback<I> { &self.push_callback }
    pub fn pop_callback(&self) -> &Callback<()> { &self.pop_callback }
    pub fn item(&self, index: usize) -> &I::Node { &self.state[index] }
    pub fn item_mut(&mut self, index: usize) -> &mut I::Node { &mut self.state[index] }
}