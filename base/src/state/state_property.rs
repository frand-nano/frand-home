use yew::{BaseComponent, Context};

use super::StateMessage;

pub trait StateProperty {
    fn new<Comp, State, Msg>(
        ids: Vec<usize>,
        state: &State,
        context: Option<&Context<Comp>>,
    ) -> Self    
    where
        Comp: BaseComponent,
        Msg: StateMessage,
        <Comp as BaseComponent>::Message: From<Msg>,
    ;
}