use std::collections::HashMap;

use frand_home_music::backend::component::Music;
use frand_home_state::State;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::state::{client::client_state::{ClientState, ClientStateMessage}, server::server_state::{ServerState, ServerStateMessage}};

use super::config::Config;

pub struct App {
    pub config: &'static Config,
    pub music: Music,
}

impl App {
    pub fn new(
        config: &'static Config,
        mysql_url: &str,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            music: Music::new(&config.music, mysql_url)?,
        })
    }

    pub async fn new_server_state(
        &self,
    ) -> anyhow::Result<ServerState> {
        Ok(ServerState {
            music: Music::new_server_state(&self.music).await?,
        })
    }

    pub async fn new_client_state(
        &self,
    ) -> anyhow::Result<ClientState> {       
        Ok(ClientState {
            user: Default::default(),
            task_bar: Default::default(),
            music: Music::new_client_state(&self.music).await?,
        })
    }

    pub async fn handle_server_message<Msg: frand_home_state::StateMessage>(
        &self,
        senders: &HashMap<Uuid, UnboundedSender<Msg>>,
        prop: &mut <ServerState as State>::Property,
        message: <ServerState as State>::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            ServerStateMessage::Music(message) => {
                Music::handle_server_message(&self.music, senders, &mut prop.music, message).await?
            },
            _ => {},
        })
    }
    
    pub async fn handle_client_message<Msg: frand_home_state::StateMessage>(
        &self,
        sender: &UnboundedSender<Msg>,
        prop: &mut <ClientState as State>::Property,
        message: <ClientState as State>::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            ClientStateMessage::Music(message) => {
                Music::handle_client_message(&self.music, sender, &mut prop.music, message).await?
            },
            _ => {},
        })
    }   
}