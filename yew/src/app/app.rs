
use frand_home_common::{
    state::{app_state::{AppStateMessage, AppStateProperty}, socket_state::SocketStateMessage}, 
    StateProperty,
};
use yew::{html, Component, Context, Html};

use crate::{socket::client_socket::ClientSocket, view::task_bar::TaskBar};

pub struct App {
    socket: ClientSocket,
    prop: AppStateProperty,
}

impl App {
    fn new(context: &Context<Self>) -> Self {
        Self {
            socket: ClientSocket::new(context),
            prop: <AppStateProperty as StateProperty>::new(vec![], Some(context)),
        }        
    }
}

impl Component for App {
    type Message = AppStateMessage;
    type Properties = AppStateProperty;

    fn create(context: &Context<App>) -> Self {
        Self::new(context)
    }

    fn view(&self, _context: &Context<App>) -> Html {     
        html! {
            <div>
                <TaskBar
                    user = { self.prop.receive.client.user.clone() }
                    number = { self.prop.receive.client.number.clone() }
                />     
            </div>
        }
    }

    fn update(&mut self, _context: &Context<App>, message: Self::Message) -> bool {   
        match message {       
            Self::Message::Error(err) => {
                log::error!("{err}");          
            },
            Self::Message::State(app_state) => {
                self.prop.apply(Self::Message::State(app_state));              
            },
            Self::Message::Send(socket_message) => self.socket.send(socket_message),
            Self::Message::Receive(socket_message) => {
                match socket_message {
                    SocketStateMessage::State(socket_state) => {
                        self.prop.receive.apply(
                            SocketStateMessage::State(socket_state),
                        );          
                    },
                    SocketStateMessage::Client(client_state_message) => {
                        self.prop.receive.client.apply(client_state_message);                 
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
