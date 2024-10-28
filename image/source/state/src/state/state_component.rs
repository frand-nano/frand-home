use super::State;

pub trait StateComponent {
    type ServerState: State;
    type ClientState: State;
}