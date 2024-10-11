use frand_home_common::state::socket_state::SocketStateMessage;
use yew::Context;
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use crate::app::{app_property::AppMessage, App};

pub struct ClientSocket {
    task: Option<WebSocketTask>,
}

impl ClientSocket {
    pub fn new(context: &Context<App>) -> Self {
        let callback = context.link().callback(
            |message| AppMessage::Receive(message)
        );

        let notification = context.link().batch_callback(
            |status| match status {
                WebSocketStatus::Opened => Some(AppMessage::Receive(SocketStateMessage::Opened(()))),
                WebSocketStatus::Closed => Some(AppMessage::Receive(SocketStateMessage::Closed(()))),
                WebSocketStatus::Error => Some(AppMessage::Receive(SocketStateMessage::Error(format!("Error")))),
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
                log::error!("❗ ClientSocket::new connect err: {err}");
                None
            },
        };

        Self { task }
    }

    pub fn send(&mut self, message: SocketStateMessage) {
        if let Some(task) = &mut self.task {
            match serde_json::to_string_pretty(&message) {
                Ok(message) => task.send(message),
                Err(err) => task.send(format!("❗ ClientSocket::send err: {err}")),
            }  
        }              
    }
}