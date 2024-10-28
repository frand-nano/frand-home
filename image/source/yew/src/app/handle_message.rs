use frand_home_app::state::socket_state::{SocketStateMessage, SocketStateProperty};
use frand_home_state::StateProperty;

pub fn handle_message(
    prop: &mut SocketStateProperty,
    message: SocketStateMessage,
) {
    match message {
        SocketStateMessage::State(socket_state) => {
            prop.apply_message(
                SocketStateMessage::State(socket_state),
            );          
        },
        SocketStateMessage::Server(server_state_message) => {
            prop.server.apply_message(server_state_message);
        },
        SocketStateMessage::Client(client_state_message) => {
            prop.client.apply_message(client_state_message);                 
        },
        SocketStateMessage::Opened(_) => {},
        SocketStateMessage::Closed(_) => {},
        SocketStateMessage::Error(_) => {},
        SocketStateMessage::Alert(message) => {
            gloo_dialogs::alert(&message);
        },
    }
}