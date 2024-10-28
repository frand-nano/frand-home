use frand_home_app::state::socket_state::{SocketStateMessage, SocketStateProperty};
use frand_home_state::StateProperty;
use yew::{Component, Context, Html};

use crate::socket::client_socket::ClientSocket;

use super::{app_property::{AppMessage, AppProperty}, handle_message};

pub struct App {
    socket: ClientSocket,
    prop: AppProperty,
}

impl App {
    fn new(context: &Context<Self>) -> Self {
        Self {
            socket: ClientSocket::new(context),
            prop: AppProperty {
                socket: <SocketStateProperty as StateProperty>::new::<App, SocketStateMessage>(
                    vec![], 
                    context,
                )
            },
        }        
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = AppProperty;

    fn create(context: &Context<App>) -> Self {
        Self::new(context)
    }

    fn view(&self, _context: &Context<App>) -> Html {     
        frand_home_app::view(&self.prop.socket.server, &self.prop.socket.client)
    }

    fn update(&mut self, _context: &Context<App>, message: Self::Message) -> bool {   
        match message {       
            Self::Message::Send(socket_message) => self.socket.send(socket_message),
            Self::Message::Receive(socket_message) => handle_message(&mut self.prop.socket, socket_message),
        }
        true
    }
}
