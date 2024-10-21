use std::collections::HashMap;
use awc::Client;
use frand_home_common::{state::{server::server_state::ServerStateMessage, socket_state::{SocketStateMessage, SocketStateProperty}}, StateProperty};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use super::broadcast;

pub async fn handle_server_message(
    _client: &Client,
    senders: &HashMap<Uuid, UnboundedSender<SocketStateMessage>>,
    socket_state: &mut SocketStateProperty,
    message: ServerStateMessage,
) -> anyhow::Result<()> {    
    match &message {
        _ => {
            socket_state.server.apply_message(message.clone());
            broadcast(senders, SocketStateMessage::Server(message))?;
        },
    }
    
    Ok(()) 
}