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
    server_message: ServerStateMessage,
) -> anyhow::Result<()> {   
    socket_state.server.apply_message(server_message.clone());
    broadcast(senders, SocketStateMessage::Server(server_message.clone()))?;

    match server_message {
        _ => {},
    }

    Ok(()) 
}