use frand_home_app::state::app::App;
use frand_home_node::Node;
use yew::{Component, Context, Html};

use crate::socket::client_socket::ClientSocket;

use super::{app_property::{AppMessage, AppProperty}, handle_message};

pub struct YewApp {
    socket: ClientSocket,
    prop: AppProperty,
}

impl YewApp {
    fn new(context: &Context<Self>) -> Self {
        Self {
            socket: ClientSocket::new(context),
            prop: AppProperty {
                socket: App::Node::new::<YewApp, App::Message>(
                    vec![], 
                    None,
                    Some(context),
                )
            },
        }        
    }
}

impl Component for YewApp {
    type Message = AppMessage;
    type Properties = AppProperty;

    fn create(context: &Context<YewApp>) -> Self {
        Self::new(context)
    }

    fn view(&self, _context: &Context<YewApp>) -> Html {     
        frand_home_app::view(&self.prop.socket.server, &self.prop.socket.client)
    }

    fn update(&mut self, _context: &Context<YewApp>, message: Self::Message) -> bool {   
        match message {       
            Self::Message::Send(socket_message) => self.socket.send(socket_message),
            Self::Message::Receive(socket_message) => handle_message(&mut self.prop.socket, socket_message),
        }
        true
    }
}
