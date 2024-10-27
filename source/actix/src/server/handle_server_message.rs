use std::collections::HashMap;
use awc::Client;
use frand_home_common::{state::{server::{music::{playlist_state::{PlaylistItemsStateMessage, PlaylistPageState, PlaylistStateMessage}, server_music_state::ServerMusicStateMessage}, server_state::ServerStateMessage}, socket_state::{SocketStateMessage, SocketStateProperty}}, StateProperty, VecMessage};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::youtube::playlist_items::PlaylistItems;

use super::broadcast;

pub async fn handle_server_message(
    client: &Client,
    senders: &HashMap<Uuid, UnboundedSender<SocketStateMessage>>,
    socket_state: &mut SocketStateProperty,
    server_message: ServerStateMessage,
) -> anyhow::Result<()> {   
    socket_state.server.apply_message(server_message.clone());
    broadcast(senders, SocketStateMessage::Server(server_message.clone()))?;

    match server_message {
        ServerStateMessage::Music(
            ServerMusicStateMessage::Playlist(
                PlaylistStateMessage::ListItems(
                    PlaylistItemsStateMessage::Items(
                        VecMessage::Item(
                            (index, mut item)
                        )
                    )
                )
            )
        ) => {
            if item.refresh {
                let playlist_items = PlaylistItems::youtube_get(
                    client, 
                    &PlaylistPageState { 
                        playlist_id: item.playlist_id.clone(), 
                        page_token: None,
                    },
                ).await?;
                
                item.refresh = false;
                let message = socket_state.server.music.playlist.list_items.items.apply_item_export(
                    index, 
                    item,
                );          
                            
                broadcast(senders, message)?; 
            }
        },
        _ => {},
    }

    Ok(()) 
}