use frand_home_app::state::app::App;
use frand_home_node::Node;

pub fn handle_message(
    prop: &mut App::Node,
    message: App::Message,
) {
    match message {
        App::Message::State(socket_state) => {
            prop.apply(
                App::Message::State(socket_state),
            );          
        },
        App::Message::Server(server_state_message) => {
            prop.server.apply(server_state_message);
        },
        App::Message::Client(client_state_message) => {
            prop.client.apply(client_state_message);                 
        },
        App::Message::Opened(_) => {},
        App::Message::Closed(_) => {},
        App::Message::Error(_) => {},
        App::Message::Alert(message) => gloo_dialogs::alert(&message),
    }
}