use yew::{BaseComponent, Context};

use super::StateMessage;

pub trait StateProperty {
    type Message;

    fn apply(&mut self, message: Self::Message);
    fn export_to(&self, message: &mut Self::Message);

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