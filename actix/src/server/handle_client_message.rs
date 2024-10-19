use anyhow::bail;
use frand_home_common::{state::{client::{client_state::ClientStateMessage, music::{client_music_state::ClientMusicStateMessage, musiclist_state::MusiclistStateMessage}}, socket_state::SocketStateMessage}, StateProperty};
use uuid::Uuid;

use crate::{server::send, youtube::playlist_items::PlaylistItems};

use super::Server;
    
impl Server {
    pub async fn handle_client_message(
        &mut self,
        id: &Uuid,  
        message: ClientStateMessage,
    ) -> anyhow::Result<()> {        
        match self.users.get(id) {
            Some(user) => if user.client_whitelist() {   
                log::info!("{user} 🔗 Client {}",
                    serde_json::to_string_pretty(&message).unwrap_or_default(),
                );          
                user  
            } else {
                return Ok(log::warn!("⛔ Unauthorized client message inbound"));  
            },
            None => bail!("❗ users not contains id:{id}"),
        };    

        let senders = &self.senders;
        match self.client_states.get_mut(&id) {
            Some(client_state) => {
                match &message {
                    ClientStateMessage::Music(
                        ClientMusicStateMessage::Musiclist(
                            MusiclistStateMessage::PlaylistPage(_)
                        )
                    ) => {
                        client_state.apply_message(message.clone());
                        send(&self.senders, &id, SocketStateMessage::Client(message))?;  

                        let playlist_items = PlaylistItems::youtube_get(
                            &self.client, 
                            &client_state.music.musiclist.playlist_page.clone_state(),
                        ).await?;
    
                        let message = client_state.music.musiclist.list_items.apply_export(
                            playlist_items.into(),
                        );          
                                     
                        self.send(&id, message)?;
                    },
                    _ => {
                        client_state.apply_message(message.clone());
                        send(senders, &id, SocketStateMessage::Client(message))?;  
                    },
                }
            },
            None => {
                log::error!("❗ client_states has no key: {id}");
            },
        }

        Ok(())
    }
}