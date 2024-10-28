use std::collections::HashMap;
use frand_home_app::{backend::component::App, state::{client::client_state::ClientStateProperty, socket_state::{SocketStateMessage, SocketStateProperty}}};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use frand_home_state::StateProperty;
use uuid::Uuid;

use crate::{authorize::user::User, APP_CONFIG, CONFIG};

pub struct Server {
    pub app: App,
    pub receiver: UnboundedReceiver<ServerMessage>,
    pub users: HashMap<Uuid, User>,
    pub senders: HashMap<Uuid, UnboundedSender<SocketStateMessage>>,
    pub socket_prop: SocketStateProperty,
    pub client_props: HashMap<Uuid, ClientStateProperty>,
}

#[derive(Debug, Clone)]
pub struct ServerHandle {
    id: Uuid,
    user: User,
    server_sender: UnboundedSender<ServerMessage>,    
}

pub struct ServerMessage {
    pub id: Uuid,
    pub user: Option<User>,
    pub sender: Option<UnboundedSender<SocketStateMessage>>,
    pub message: SocketStateMessage,
}

impl Server {
    pub async fn new() -> anyhow::Result<(Self, UnboundedSender<ServerMessage>)> {
        let (sender, receiver) = unbounded_channel();

        let host = match CONFIG.settings.local_mode {
            true => "localhost",
            false => "frand-home-mysql",
        };
        let mysql_user = CONFIG.keys.mysql_user();
        let mysql_pass = CONFIG.keys.mysql_pass();
        let mysql_url = format!("mysql://{mysql_user}:{mysql_pass}@{host}:3306");

        let app = App::new(&APP_CONFIG, &mysql_url)?;

        let mut socket_prop = SocketStateProperty::default();
        socket_prop.server.apply_state(app.new_server_state().await?);
        socket_prop.client.apply_state(app.new_client_state().await?);

        let server = Self {
            app,
            receiver,      
            users: HashMap::new(),     
            senders: HashMap::new(),    
            socket_prop, 
            client_props: HashMap::new(),     
        };
        
        Ok((server, sender))
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Some(message) = self.receiver.recv().await { 
            self.handle_message(message).await?;
        }
        Ok(())
    }
}

impl ServerHandle {
    pub fn new(
        user: User,
        server_sender: UnboundedSender<ServerMessage>,    
        socket_sender: UnboundedSender<SocketStateMessage>,    
    ) -> anyhow::Result<Self> {
        let result = Self { 
            id: Uuid::new_v4(), 
            user, 
            server_sender: server_sender.clone(), 
        };

        result.server_sender.send(
            ServerMessage {
                id: result.id,
                user: Some(result.user.clone()),
                sender: Some(socket_sender),
                message: SocketStateMessage::Opened(()),
            }
        )?;

        Ok(result)
    }

    pub fn send(&self, message: SocketStateMessage) -> anyhow::Result<()> {
        Ok(self.server_sender.send(
            ServerMessage {
                id: self.id,
                user: None,
                sender: None,
                message,
            }
        )?)
    }
}