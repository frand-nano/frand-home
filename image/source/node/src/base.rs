use std::{any::Any, fmt::Debug};
use yew::{BaseComponent, Context};

pub const ERROR_ID: usize = 0;
pub const STATE_ID: usize = 1;
pub const ITEM_ID: usize = 2;
pub const PUSH_ID: usize = 3;
pub const POP_ID: usize = 4;

pub trait State: 'static + Debug + Clone + Default + PartialEq + Send + Sync {
    type Node: Node<Self>;    
}

pub trait Node<S: State>: 'static + Debug + Clone + PartialEq {
    type Message: Message;
    
    fn new<Comp: BaseComponent, Msg: RootMessage>(
        ids: Vec<usize>,
        id: Option<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self where <Comp as BaseComponent>::Message: From<Msg>;    

    fn new_default(
        ids: Vec<usize>,
        id: Option<usize>,
    ) -> Self;

    fn ids(&self) -> &Vec<usize>;
    fn set_id(&mut self, index: usize, id: usize);    
    fn callback(&self) -> &Callback<S>;
    fn clone_state(&self) -> S;
    fn apply_state(&mut self, state: S);
    fn apply(&mut self, message: Self::Message);
    fn emit(&self, state: S) { 
        self.callback().emit(state) 
    }
    fn apply_export<Msg: RootMessage>(&mut self, state: S) -> Msg { 
        self.apply_state(state.clone());
        Msg::new(0, self.callback().export(state)) 
    }
}

pub trait Message: 'static + Debug + Clone + Send + Sync {
    fn try_error(err: String) -> anyhow::Result<Self>;
    fn try_new(depth: usize, data: MessageData) -> anyhow::Result<Self>;
}

pub trait RootMessage: Message {
    fn error(err: String) -> Self;
    fn new(depth: usize, data: MessageData) -> Self {
        match Self::try_new(depth, data) {
            Ok(message) => message,
            Err(err) => Self::error(err.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct MessageData {
    pub ids: Vec<usize>,
    pub data: Box<dyn Any>,
}

#[derive(Debug, Clone)]
pub struct Callback<I: State> {
    ids: Vec<usize>,
    callback: Option<yew::Callback<(Vec<usize>, I)>>,
}

impl MessageData {
    pub fn new(ids: Vec<usize>, data: Box<dyn Any>) -> Self {
        Self { ids, data }
    }
}

impl<I: State> Callback<I> {
    pub fn new<Comp: BaseComponent, Msg: RootMessage>(
        ids: Vec<usize>,
        id: usize,
        context: Option<&Context<Comp>>,
    ) -> Self where <Comp as BaseComponent>::Message: From<Msg> {
        let ids = ids_pushed(ids, Some(id));
        Self { 
            ids: ids.clone(),
            callback: context.map(|context| context.link().callback(move |(ids, item): (Vec<usize>, I)| {
                Msg::new(0, MessageData::new(ids.clone(), Box::new(item)))
            })),
        }
    }

    pub fn new_default(
        ids: Vec<usize>,
        id: usize,
    ) -> Self {
        let ids = ids_pushed(ids, Some(id));
        Self { 
            ids: ids.clone(),
            callback: None,
        }
    }

    pub fn set_id(&mut self, index: usize, id: usize) { 
        self.ids[index] = id;
    }

    fn emit(&self, item: I) {  
        match &self.callback {
            Some(callback) => callback.emit((self.ids.clone(), item)),
            None => { panic!("callback is None {:#?}", self.ids) },
        }
    }

    fn export(&self, item: I) -> MessageData {  
        MessageData::new(self.ids.clone(), Box::new(item))
    }
}

#[macro_export]
macro_rules! impl_state_for {
    ( $head: ty $(,$tys: ty)* $(,)? ) => { 
        impl_state_for!{ @inner($head, $($tys,)*) }
    };
    ( @inner($($tys: ty,)*) ) => {    
        $(
            impl Message for $tys {
                fn try_error(err: String) -> anyhow::Result<Self> { 
                    anyhow::bail!("❗ {} try_error({err})", stringify!($tys))
                }
                
                fn try_new(depth: usize, data: MessageData) -> anyhow::Result<Self> {
                    match data.data.downcast() {
                        Ok(data) => Ok(*data),
                        Err(_) => Err(anyhow::anyhow!("ids: {:?}, depth: {}", data.ids, depth)),
                    }
                }
            }

            impl State for $tys {
                type Node = ValueNode<Self>;
            }
        )*      
    };
}

pub fn ids_pushed(mut ids: Vec<usize>, id: Option<usize>) -> Vec<usize> {
    if let Some(id) = id {
        ids.push(id);
    }
    ids
}