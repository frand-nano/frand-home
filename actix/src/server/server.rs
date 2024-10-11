use anyhow::bail;
use frand_home_common::{state::{client::client_state::{ClientState, ClientStateMessage}, socket_state::SocketStateMessage}, State};
use std::collections::HashMap;
use awc::Client;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use crate::authorize::user::User;

pub struct Server {
    client: Client,
    receiver: UnboundedReceiver<ServerMessage>,
    users: HashMap<Uuid, User>,
    senders: HashMap<Uuid, UnboundedSender<SocketStateMessage>>,
    client_states: HashMap<Uuid, ClientState>,
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
    pub fn new() -> (Self, UnboundedSender<ServerMessage>) {
        let (sender, receiver) = unbounded_channel();
        let server = Self {
            client: Client::default(),
            receiver,      
            users: HashMap::new(),     
            senders: HashMap::new(),    
            client_states: HashMap::new(),     
        };
        (server, sender)
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Some(message) = self.receiver.recv().await { 
            self.handle_message(message).await?;
        }
        Ok(())
    }

    fn send(
        &self,
        id: &Uuid,
        message: SocketStateMessage,
    ) -> anyhow::Result<()> {
        Ok(match self.senders.get(id) {
            Some(sender) => sender.send(message)?,
            None => bail!("❗ senders not contains id:{id}"),
        })
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
                            self.client_states.insert(id, ClientState::default());  
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
    
    async fn handle_client_message(
        &mut self,
        id: &Uuid,  
        message: ClientStateMessage,
    ) -> anyhow::Result<()> {        
        let user = match self.users.get(id) {
            Some(user) => user,
            None => bail!("❗ users not contains id:{id}"),
        };   

        if !user.client_whitelist() {           
            return Ok(log::warn!("⛔ Unauthorized client message inbound"));  
        }

        log::info!("{user} 🔗 Client {}",
            serde_json::to_string_pretty(&message).unwrap_or_default(),
        );    
        match self.client_states.get_mut(&id) {
            Some(client_state) => client_state.apply(message.clone()),
            None => log::error!("❗ client_states has no key: {id}"),
        }
        self.send(&id, SocketStateMessage::Client(message.clone()))?;

        match message {
            _ => {},
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
    Ok(result)
}
