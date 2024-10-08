use std::any::Any;

pub use yew::{Properties, BaseComponent, Context, Callback};
pub use frand_home_macro::{PropertyState, JsonConvert};
pub use node::*;

pub mod node;
pub mod vec;
pub mod option;

pub trait State {
    type Message;
    type Property;
    fn apply(&mut self, message: Self::Message);
}
pub trait StateMessage {
    fn new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn Any>,    
    ) -> Self 
    where Self: Sized
    {
        match Self::try_new(ids, index, value) {
            Ok(result) => result,
            Err(err) => {
                let err = format!("❗ StateMessage::new 
                    ids:{:#?}, 
                    index:{index}, 
                    err:{:#?}, 
                ", ids, err);
                log::error!("{err}");
                Self::error(err)
            },
        }
    }

    fn try_new(
        ids: &[usize], 
        index: usize, 
        #[allow(unused_variables)] value: Box<dyn Any>,
    ) -> Result<Self, Box<dyn Any>> where Self: Sized;    

    fn error(err: String) -> Self;
}

pub trait StateProperty {
    fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: Option<&Context<Comp>>,
    ) -> Self    
    where
        Comp: BaseComponent,
        Msg: StateMessage,
        <Comp as BaseComponent>::Message: From<Msg>,
    ;
}

pub fn vec_pushed<V: Clone>(vec: &Vec<V>, value: V) -> Vec<V> {
    let mut result = vec.clone();
    result.push(value);
    result
}