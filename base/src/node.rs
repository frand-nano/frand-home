use yew::{BaseComponent, Callback, Context, Properties};

use crate::StateMessage;

#[derive(Clone, Properties, PartialEq, Default)]
pub struct Node<V: PartialEq> {
    pub callback: NodeCallback<V>,
}

impl<V: Default + Clone + PartialEq + 'static> Node<V> {
    pub fn value(&self) -> &V { &self.callback.value }

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
            callback: NodeCallback::new(ids, context), 
        }
    }

    pub fn applied(&self, value: V) -> Self {
        Self {
            callback: self.callback.applied(value),        
        }
    }

    pub fn emit(&self, value: V) {
        self.callback.emit(value)
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct NodeCallback<V> {
    ids: Vec<usize>,
    callback: Callback<V>,
    value: V,
}

impl<V> NodeCallback<V> 
where V: Default + 'static
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
        let callback_ids = ids.clone();
        let callback = match context {
            Some(context) => context.link().callback(move |value: V| 
                Msg::new(callback_ids.as_slice(), 0, Box::new(value))
            ),
            None => Default::default(),
        };

        Self { 
            ids, 
            callback, 
            value: V::default(),
        }
    }

    pub fn applied(&self, value: V) -> Self {        
        Self { 
            ids: self.ids.clone(), 
            callback: self.callback.clone(), 
            value,
        }
    }

    pub fn callback(&self) -> &Callback<V> {
        &self.callback
    }

    pub fn emit(&self, value: V) {
        self.callback.emit(value)
    }
}