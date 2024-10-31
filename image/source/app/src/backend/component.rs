use std::collections::HashMap;

use frand_home_music::backend::component::Music;
use frand_home_node::RootMessage;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::state::{client::client::Client, server::server::Server};

use super::config::Config;

pub struct ActixApp {
    pub config: &'static Config,
    pub music: Music,
}

impl ActixApp {
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
    ) -> anyhow::Result<Server::State> {
        Ok(Server::State {
            music: Music::new_server_state(&self.music).await?,
        })
    }

    pub async fn new_client_state(
        &self,
    ) -> anyhow::Result<Client::State> {       
        Ok(Client::State {
            user: Default::default(),
            task_bar: Default::default(),
            music: Music::new_client_state(&self.music).await?,
        })
    }

    pub async fn handle_server_message<Msg: RootMessage>(
        &self,
        senders: &HashMap<Uuid, UnboundedSender<Msg>>,
        prop: &mut Server::Node,
        message: Server::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            Server::Message::Music(message) => {
                Music::handle_server_message(&self.music, senders, &mut prop.music, message).await?
            },
            _ => {},
        })
    }
    
    pub async fn handle_client_message<Msg: RootMessage>(
        &self,
        sender: &UnboundedSender<Msg>,
        prop: &mut Client::Node,
        message: Client::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            Client::Message::Music(message) => {
                Music::handle_client_message(&self.music, sender, &mut prop.music, message).await?
            },
            _ => {},
        })
    }   
}