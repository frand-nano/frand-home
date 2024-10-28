use yew::{BaseComponent, Context};

use super::StateMessage;

pub trait StateProperty {
    type State;
    type Message;

    fn clone_state(&self) -> Self::State;
    fn apply_state(&mut self, state: Self::State);
    fn apply_export<Msg: StateMessage>(&mut self, state: Self::State) -> Msg;
    fn apply_message(&mut self, message: Self::Message);
    fn export_message(&self, message: &mut Self::Message);

    fn new<Comp, Msg>(
        ids: Vec<usize>,
        context: &Context<Comp>,
    ) -> Self    
    where
        Comp: BaseComponent,
        Msg: StateMessage,
        <Comp as BaseComponent>::Message: From<Msg>,
    ;

    fn new_default(
        ids: Vec<usize>,
    ) -> Self;
}