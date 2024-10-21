use awc::Client;
use frand_home_common::{
    state::{
        client::{client_state::{ClientStateMessage, ClientStateProperty}, 
        music::{client_music_state::ClientMusicStateMessage, musiclist_state::MusiclistStateMessage}}, 
        socket_state::SocketStateMessage
    },
    StateProperty,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::youtube::playlist_items::PlaylistItems;

pub async fn handle_client_message(
    client: &Client,
    sender: &UnboundedSender<SocketStateMessage>,
    client_state: &mut ClientStateProperty,
    message: ClientStateMessage,
) -> anyhow::Result<()> {        
    match &message {
        ClientStateMessage::Music(
            ClientMusicStateMessage::Musiclist(
                MusiclistStateMessage::PlaylistPage(_)
            )
        ) => {
            client_state.apply_message(message.clone());
            sender.send(SocketStateMessage::Client(message))?;  

            let playlist_items = PlaylistItems::youtube_get(
                client, 
                &client_state.music.musiclist.playlist_page.clone_state(),
            ).await?;

            let message = client_state.music.musiclist.list_items.apply_export(
                playlist_items.into(),
            );          
                         
            sender.send(message)?;  
        },
        _ => {
            client_state.apply_message(message.clone());
            sender.send(SocketStateMessage::Client(message))?;  
        },
    }

    Ok(())
}