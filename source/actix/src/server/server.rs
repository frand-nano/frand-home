use frand_home_common::{
    state::{
        client::client_state::ClientStateProperty, 
        server::music::playlist_state::PlaylistState, 
        socket_state::{SocketState, SocketStateMessage, SocketStateProperty},
    },
    StateProperty,
};

use std::collections::HashMap;
use awc::Client;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use crate::{authorize::user::User, mysql::Database, youtube::playlist::Playlist, CONFIG};

pub struct Server {
    pub client: Client,
    pub db: Database,
    pub receiver: UnboundedReceiver<ServerMessage>,
    pub users: HashMap<Uuid, User>,
    pub senders: HashMap<Uuid, UnboundedSender<SocketStateMessage>>,
    pub socket_state: SocketStateProperty,
    pub client_states: HashMap<Uuid, ClientStateProperty>,
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
        
        let client = Client::default();

        let db = match CONFIG.settings.local_mode {
            true => Database::new("localhost")?,
            false => Database::new("frand-home-mysql")?,
        };

        let mut socket_state = SocketStateProperty::default();
        socket_state.apply_state(Self::init_socket_state(&client).await?);

        let server = Self {
            client,
            db,
            receiver,      
            users: HashMap::new(),     
            senders: HashMap::new(),    
            socket_state, 
            client_states: HashMap::new(),     
        };
        
        Ok((server, sender))
    }

    async fn init_socket_state(
        client: &Client,
    ) -> anyhow::Result<SocketState> {
        let mut result = SocketState::default();

        result.server.music.playlist = PlaylistState {
            list_items: Playlist::youtube_get(client, &CONFIG.settings.playlists).await?.into(), 
        };

        result.client.task_bar.playlist_visible = true;

        Ok(result)
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

pub fn broadcast(
    senders: &HashMap<Uuid, UnboundedSender<SocketStateMessage>>,
    message: SocketStateMessage,
) -> anyhow::Result<()> {
    for sender in senders.values() {
        sender.send(message.clone())?;
    }
    Ok(())
}