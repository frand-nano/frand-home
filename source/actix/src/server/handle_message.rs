use anyhow::bail;
use frand_home_common::{state::socket_state::{SocketState, SocketStateMessage}, StateProperty};

use crate::{authorize::user::User, server::{handle_client_message::handle_client_message, handle_server_message::handle_server_message}};

use super::{Server, ServerMessage};

impl Server {
    pub async fn handle_message(
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

    fn init_client_socket_state(
        &self,
        user: &User,
    ) -> anyhow::Result<SocketState> {
        let mut result = self.socket_state.clone_state();    

        result.client.user = user.clone().into();
        result.client.task_bar.playlist_visible = true;

        Ok(result)
    }
}
