use frand_home_app::state::app::App;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Default, Clone, PartialEq, Properties)]
pub struct AppProperty {
    pub socket: App::Node,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AppMessage {
    Send(App::Message),
    Receive(App::Message),
}

impl From<App::Message> for AppMessage {
    fn from(value: App::Message) -> Self {
        Self::Send(value)
    }
}