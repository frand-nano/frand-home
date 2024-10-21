use anyhow::bail;
use frand_home_common::state::{client::{client_state::{ClientState, ClientStateMessage, ClientStateProperty}, music::playlist_state::PlaylistState}, socket_state::{SocketStateMessage, SocketStateProperty}};

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
    pub fn new() -> anyhow::Result<(Self, UnboundedSender<ServerMessage>)> {
        let (sender, receiver) = unbounded_channel();

        let db = match CONFIG.settings.local_mode {
            true => Database::new("localhost")?,
            false => Database::new("frand-home-mysql")?,
        };
        
        let server = Self {
            client: Client::default(),
            db,
            receiver,      
            users: HashMap::new(),     
            senders: HashMap::new(),    
            socket_state: Default::default(), 
            client_states: HashMap::new(),     
        };
        
        Ok((server, sender))
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Some(message) = self.receiver.recv().await { 
            self.handle_message(message).await?;
        }
        Ok(())
    }

    pub fn send(
        &self,
        id: &Uuid,
        message: SocketStateMessage,
    ) -> anyhow::Result<()> {
        send(&self.senders, id, message)
    }

    pub fn broadcast(
        &self,
        message: SocketStateMessage,
    ) -> anyhow::Result<()> {
        broadcast(&self.senders, message)
    }

    async fn handle_message(
        &mut self,
        message: ServerMessage,
    ) -> anyhow::Result<()> {
        let id = message.id;
        match message.message {
            SocketStateMessage::State(_) => {
                if let Some(user) = self.users.get(&id) {
                    log::info!("{user} 🔗 State");                 
                }                 
            },
            SocketStateMessage::Server(server_state_message) => {
                self.handle_server_message(&id, server_state_message).await?;  
            },
            SocketStateMessage::Client(client_state_message) => {
                self.handle_client_message(&id, client_state_message).await?;  
            },
            SocketStateMessage::Opened(_) => {
                if let Some(user) = message.user {
                    log::info!("{user} 🔗 Opened {id}");   
                    if user.client_whitelist() {
                        if let Some(sender) = message.sender { 
                            self.users.insert(id, user.clone());         
                            self.senders.insert(id, sender);      
                            self.client_states.insert(id, self.socket_state.client.clone());  
                        }
                    }                    
                }
            },
            SocketStateMessage::Closed(_) => {
                if let Some(user) = self.users.get(&id) {
                    log::info!("{user} 🔗 Closed");     
                    if user.client_whitelist() {
                        self.users.remove(&id);
                        self.senders.remove(&id);
                        self.client_states.remove(&id);
                    }
                } 
            },
            SocketStateMessage::Error(err) => {
                if let Some(user) = self.users.get(&id) {
                    log::info!("{user} 🔗 Error err: {err}");   
                }
            },
        }
        Ok(())
    }
}

impl ServerHandle {
    pub async fn new(
        client: &Client,
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

        let client_state = init_client_state(client, &result.user).await?;

        result.send(SocketStateMessage::Client(ClientStateMessage::State(client_state)))?;

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

async fn init_client_state(
    client: &Client,
    user: &User,
) -> anyhow::Result<ClientState> {
    let mut result = ClientState::default();
    result.user = user.clone().into();
    result.music.playlist = PlaylistState {
        visible: true,
        list_items: Playlist::youtube_get(client, &CONFIG.settings.playlists).await?.into(), 
    };
    Ok(result)
}

pub fn send(
    senders: &HashMap<Uuid, UnboundedSender<SocketStateMessage>>,
    id: &Uuid,
    message: SocketStateMessage,
) -> anyhow::Result<()> {
    Ok(match senders.get(id) {
        Some(sender) => sender.send(message)?,
        None => bail!("❗ senders not contains id:{id}"),
    })
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