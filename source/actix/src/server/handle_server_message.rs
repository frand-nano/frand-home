use anyhow::bail;
use frand_home_common::{state::{server::server_state::ServerStateMessage, socket_state::SocketStateMessage}, StateProperty};
use uuid::Uuid;

use super::{broadcast, Server};

impl Server {
    pub async fn handle_server_message(
        &mut self,
        id: &Uuid,    
        message: ServerStateMessage,
    ) -> anyhow::Result<()> {
        match self.users.get(id) {
            Some(user) => if user.server_whitelist() {   
                log::info!("{user} 🔗 Client {}",
                    serde_json::to_string_pretty(&message).unwrap_or_default(),
                );          
                user  
            } else {
                return Ok(log::warn!("⛔ Unauthorized server message inbound"));  
            },
            None => bail!("❗ users not contains id:{id}"),
        };    
        
        let senders = &self.senders;
        let server_state = &mut self.socket_state.server;
        match &message {
            _ => {
                server_state.apply_message(message.clone());
                broadcast(senders, SocketStateMessage::Server(message))?;
            },
        }
        
        Ok(()) 
    }
}