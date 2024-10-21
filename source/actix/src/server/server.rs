use anyhow::bail;
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

use crate::{authorize::user::User, mysql::Database, server::{handle_client_message::handle_client_message, handle_server_message::handle_server_message}, youtube::playlist::Playlist, CONFIG};

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
            client: Client::default(),
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

        Ok(result)
    }

    fn init_client_socket_state(
        &self,
        user: &User,
    ) -> anyhow::Result<SocketState> {
        let mut result = self.socket_state.clone_state();    

        result.client.user = user.clone().into();
        result.client.task_bar.playlist_visible = true;

        Ok(result)
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Some(message) = self.receiver.recv().await { 
            self.handle_message(message).await?;
        }
        Ok(())
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
            SocketStateMessage::Server(message) => {
                match self.users.get(&id) {
                    Some(user) => if user.server_whitelist() {   
                        log::info!("{user} 🔗 Server {}",
                            serde_json::to_string_pretty(&message).unwrap_or_default(),
                        );          
                        handle_server_message(
                            &self.client, 
                            &self.senders, 
                            &mut self.socket_state, 
                            message,
                        ).await?;  
                    } else {
                        return Ok(log::warn!("⛔ Unauthorized server message inbound"));  
                    },
                    _ => bail!("❗ Unregistered id:{id}"),
                };    
            },
            SocketStateMessage::Client(message) => {
                match (self.users.get(&id), self.senders.get(&id), self.client_states.get_mut(&id)) {
                    (Some(user), Some(sender), Some(client_state)) => if user.client_whitelist() {   
                        log::info!("{user} 🔗 Client {}",
                            serde_json::to_string_pretty(&message).unwrap_or_default(),
                        );         
                        handle_client_message(
                            &self.client, 
                            sender, 
                            client_state, 
                            message,
                        ).await?;  
                    } else {
                        return Ok(log::warn!("⛔ Unauthorized client message inbound"));  
                    },
                    _ => bail!("❗ Unregistered id:{id}"),
                };   
            },
            SocketStateMessage::Opened(_) => {
                if let Some(user) = message.user {
                    log::info!("{user} 🔗 Opened {id}");   
                    if user.client_whitelist() {
                        if let Some(sender) = message.sender { 
                            self.users.insert(id, user.clone());         
                            self.senders.insert(id, sender.clone());      

                            let socket_state = self.init_client_socket_state(&user)?;
                            let mut client_state_property = self.socket_state.client.clone();
                            client_state_property.apply_state(socket_state.client.clone());

                            self.client_states.insert(id, client_state_property);  

                            sender.send(SocketStateMessage::State(socket_state))?;
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
            SocketStateMessage::Alert(_) => {},
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