use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use yew::{BaseComponent, Context, Properties};

use crate::base::{ids_pushed, Callback, Message, RootMessage, MessageData, Node, State, ITEM_ID, STATE_ID};

#[derive(Debug, Clone, Properties)]
pub struct OptionNode<I: State> 
    where <I::Node as Node<I>>::Message : Serialize + for<'a> Deserialize<'a>
{
    ids: Vec<usize>,
    callback: Callback<Option<I>>,
    state: Option<I::Node>,
    item: I::Node,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OptionNodeMessage<I: State> 
    where <I::Node as Node<I>>::Message : Serialize + for<'a> Deserialize<'a>
{
    Error(String),
    State(Option<I>),
    Item(<I::Node as Node<I>>::Message),
}

impl<I: State> State for Option<I> 
    where <I::Node as Node<I>>::Message : Serialize + for<'a> Deserialize<'a>
{
    type Node = OptionNode<I>;
}

impl<I: State> PartialEq for OptionNode<I> 
    where <I::Node as Node<I>>::Message : Serialize + for<'a> Deserialize<'a>
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<I: State> Node<Option<I>> for OptionNode<I> 
    where <I::Node as Node<I>>::Message : Serialize + for<'a> Deserialize<'a>
{
    type Message = OptionNodeMessage<I>;
    
    fn new<Comp: BaseComponent, Msg: RootMessage>(
        ids: Vec<usize>,
        id: Option<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self where <Comp as BaseComponent>::Message: From<Msg> {
        let ids = ids_pushed(ids, id);

        Self { 
            ids: ids.clone(),
            callback: Callback::new(ids.clone(), STATE_ID, context),
            item: <I::Node as Node<I>>::new(ids, Some(ITEM_ID), context),
            state: None,
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
            item: <I::Node as Node<I>>::new_default(ids, Some(ITEM_ID)),
            state: None,
        }
    }

    fn ids(&self) -> &Vec<usize> { &self.ids }
    fn set_id(&mut self, index: usize, id: usize) { 
        self.ids[index] = id;
        self.callback.set_id(index, id);
        self.item.set_id(index, id);
        if let Some(state) = &mut self.state {
            state.set_id(index, id);
        }
    } 
    fn callback(&self) -> &Callback<Option<I>> { &self.callback }
    fn clone_state(&self) -> Option<I> { 
        self.state.as_ref().map(|state| state.clone_state()) 
    }
    fn apply_state(&mut self, state: Option<I>) { 
        match (&mut self.state, state) {
            (None, Some(state)) => {
                let mut item = self.item.clone();
                item.apply_state(state);
                self.state = Some(item);
            },
            (Some(self_state), Some(state)) => {
                self_state.apply_state(state);
            },
            (_, None) => {
                self.state = None;
            },
        }
    }
    fn apply(&mut self, message: Self::Message) {
        match message {
            OptionNodeMessage::Error(err) => {
                log::error!("❗ {}.apply_message: {err}", stringify!(OptionNode));
            },
            OptionNodeMessage::State(state) => self.apply_state(state),
            OptionNodeMessage::Item(message) => {
                if let Some(state) = &mut self.state {
                    state.apply(message);
                }
            },
        }     
    }
}

impl<I: State> Message for OptionNodeMessage<I> 
    where <I::Node as Node<I>>::Message : Serialize + for<'a> Deserialize<'a>
{
    fn try_error(err: String) -> anyhow::Result<Self> { Ok(Self::Error(err)) }
    
    fn try_new(depth: usize, data: MessageData) -> anyhow::Result<Self> {
        match data.ids[depth] {
            STATE_ID => match data.data.downcast() {
                Ok(data) => Ok(Self::State(*data)),
                Err(_) => Err(anyhow::anyhow!("ids: {:?}, depth: {}", data.ids, depth)),
            },
            ITEM_ID => Ok(Self::Item(<<I::Node as Node<I>>::Message as Message>::try_new(depth + 1, data)?)),
            _ => Err(anyhow::anyhow!("ids: {:?}, depth: {}", data.ids, depth)),
        }    
    }
}

impl<I: State> OptionNode<I> 
    where <I::Node as Node<I>>::Message : Serialize + for<'a> Deserialize<'a>
{
    pub fn item(&self) -> Option<&I::Node> { self.state.as_ref() }
}