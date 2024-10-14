use anyhow::bail;
use frand_home_common::{state::{server::server_state::ServerStateMessage, socket_state::SocketStateMessage}, StateProperty};
use uuid::Uuid;

use super::Server;

impl Server {
    pub async fn handle_server_message(
        &mut self,
        id: &Uuid,    
        message: ServerStateMessage,
    ) -> anyhow::Result<()> {
        let user = match self.users.get(id) {
            Some(user) => user,
            None => bail!("❗ users not contains id:{id}"),
        };
    
        if !user.server_whitelist() {           
            return Ok(log::warn!("⛔ Unauthorized server message inbound"));  
        }
    
        log::info!("{user} 🔗 Server {}",
            serde_json::to_string_pretty(&message).unwrap_or_default(),
        );    
        
        self.socket_state.server.apply_message(message.clone());
        self.broadcast(SocketStateMessage::Server(message))?;     
        
        Ok(()) 
    }
}