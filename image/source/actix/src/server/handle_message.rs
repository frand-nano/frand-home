use anyhow::bail;
use frand_home_app::state::socket_state::SocketStateMessage;
use frand_home_node::StateNode;

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

                        self.socket_prop.server.apply(message.clone());
                        for sender in self.senders.values() {
                            sender.send(SocketStateMessage::Server(message.clone()))?;
                        }

                        self.app.handle_server_message(
                            &self.senders, 
                            &mut self.socket_prop.server, 
                            message,
                        ).await?;  
                    } else {
                        return Ok(log::warn!("⛔ Unauthorized server message inbound"));  
                    },
                    _ => bail!("❗ Unregistered id:{id}"),
                };    
            },
            SocketStateMessage::Client(message) => {
                match (self.users.get(&id), self.senders.get(&id), self.client_props.get_mut(&id)) {
                    (Some(user), Some(sender), Some(client_prop)) => if user.client_whitelist() {   
                        log::info!("{user} 🔗 Client {}",
                            serde_json::to_string_pretty(&message).unwrap_or_default(),
                        );    

                        client_prop.apply(message.clone());
                        sender.send(SocketStateMessage::Client(message.clone()))?; 

                        self.app.handle_client_message(
                            sender, 
                            client_prop, 
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
    
                            let mut socket_state = self.socket_prop.clone_state();
                            socket_state.client.user = user.clone().into();

                            let mut client_prop = self.socket_prop.client.clone();
                            client_prop.apply_state(socket_state.client.clone());
    
                            self.client_props.insert(id, client_prop);  
    
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
                        self.client_props.remove(&id);
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
