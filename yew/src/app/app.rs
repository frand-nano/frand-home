
use frand_home_common::{
    state::socket_state::{SocketStateMessage, SocketStateProperty}, 
    StateProperty,
};
use yew::{html, Component, Context, Html};

use crate::{socket::client_socket::ClientSocket, view::task_bar::TaskBar};

use super::app_property::{AppMessage, AppProperty};

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
                    Some(context),
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
        html! {
            <div>
                <TaskBar
                    user = { self.prop.socket.client.user.clone() }
                    number = { self.prop.socket.client.number.clone() }
                />     
            </div>
        }
    }

    fn update(&mut self, _context: &Context<App>, message: Self::Message) -> bool {   
        match message {       
            Self::Message::Send(socket_message) => self.socket.send(socket_message),
            Self::Message::Receive(socket_message) => {
                match socket_message {
                    SocketStateMessage::State(socket_state) => {
                        self.prop.socket.apply(
                            SocketStateMessage::State(socket_state),
                        );          
                    },
                    SocketStateMessage::Client(client_state_message) => {
                        self.prop.socket.client.apply(client_state_message);                 
                    },
                    SocketStateMessage::Opened(_) => {},
                    SocketStateMessage::Closed(_) => {},
                    SocketStateMessage::Error(_) => {},
                }
            },
        }
        true
    }
}
