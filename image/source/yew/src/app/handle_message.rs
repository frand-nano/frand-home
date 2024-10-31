use frand_home_app::state::socket_state::{SocketStateMessage, SocketStateNode};
use frand_home_node::StateNode;

pub fn handle_message(
    prop: &mut SocketStateNode,
    message: SocketStateMessage,
) {
    match message {
        SocketStateMessage::State(socket_state) => {
            prop.apply(
                SocketStateMessage::State(socket_state),
            );          
        },
        SocketStateMessage::Server(server_state_message) => {
            prop.server.apply(server_state_message);
        },
        SocketStateMessage::Client(client_state_message) => {
            prop.client.apply(client_state_message);                 
        },
        SocketStateMessage::Opened(_) => {},
        SocketStateMessage::Closed(_) => {},
        SocketStateMessage::Error(_) => {},
        SocketStateMessage::Alert(message) => gloo_dialogs::alert(&message),
    }
}