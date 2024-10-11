
use frand_home_common::{state::{app_state::{AppState, AppStateMessage, AppStateProperty}, socket_state::SocketStateMessage}, State, StateProperty};
use yew::{html, Component, Context, Html};

use crate::socket::client_socket::ClientSocket;

pub struct App {
    socket: ClientSocket,
    state: AppState,
    prop: AppStateProperty,
}

impl App {
    fn new(context: &Context<Self>) -> Self {
        let state = AppState::default();
        Self {
            socket: ClientSocket::new(context),
            prop: <AppStateProperty as StateProperty>::new(vec![], &state, Some(context)),
            state,
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
                
            </div>
        }
    }

    fn update(&mut self, _context: &Context<App>, message: Self::Message) -> bool {   
        match message {       
            Self::Message::Error(err) => {
                log::error!("{err}");          
            },
            Self::Message::State(app_state) => {
                self.state = app_state;                         
            },
            Self::Message::Send(socket_message) => self.socket.send(socket_message),
            Self::Message::Receive(socket_message) => {
                match socket_message {
                    SocketStateMessage::State(socket_state) => {
                        self.state.receive = socket_state;                    
                    },
                    SocketStateMessage::Client(client_state_message) => {
                        self.state.receive.client.apply(client_state_message);                 
                    },
                    SocketStateMessage::Opened(_) => {/*todo!()*/},
                    SocketStateMessage::Closed(_) => {/*todo!()*/},
                    SocketStateMessage::Error(_) => {/*todo!()*/},
                }
            },
        }
        true
    }
}
