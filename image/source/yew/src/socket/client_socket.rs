use frand_home_app::state::app::App;
use yew::Context;
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use crate::app::{app_property::AppMessage, YewApp};

pub struct ClientSocket {
    task: Option<WebSocketTask>,
}

impl ClientSocket {
    pub fn new(context: &Context<YewApp>) -> Self {
        let callback = context.link().callback(
            |message| AppMessage::Receive(message)
        );

        let notification = context.link().batch_callback(
            |status| match status {
                WebSocketStatus::Opened => Some(AppMessage::Receive(App::Message::Opened(()))),
                WebSocketStatus::Closed => Some(AppMessage::Receive(App::Message::Closed(()))),
                WebSocketStatus::Error => Some(AppMessage::Receive(App::Message::Error(format!("Error")))),
            }
        );

        let task = WebSocketService::connect(
            "/ws/", 
            callback,
            notification,
        );

        let task = match task {
            Ok(task) => Some(task),
            Err(err) => {
                log::error!(" ClientSocket::new connect err: {err}");
                None
            },
        };

        Self { task }
    }

    pub fn send(&mut self, message: App::Message) {
        if let Some(task) = &mut self.task {
            match serde_json::to_string_pretty(&message) {
                Ok(message) => task.send(message),
                Err(err) => task.send(format!("❗ ClientSocket::send err: {err}")),
            }  
        }              
    }
}