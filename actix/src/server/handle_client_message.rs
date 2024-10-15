use anyhow::bail;
use frand_home_common::{state::{client::{client_state::ClientStateMessage, music::{client_music_state::ClientMusicStateMessage, musiclist_state::MusiclistStateMessage, playlist_state::PlaylistPageState}}, socket_state::SocketStateMessage}, StateProperty};
use uuid::Uuid;

use crate::{server::send, youtube::playlist_items::PlaylistItems};

use super::Server;
    
impl Server {
    pub async fn handle_client_message(
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

        let client_state = match self.client_states.get_mut(&id) {
            Some(client_state) => {
                client_state.apply_message(message.clone());
                Some(client_state)
            },
            None => {
                log::error!("❗ client_states has no key: {id}");
                None
            },
        };

        send(&self.senders, &id, SocketStateMessage::Client(message.clone()))?;

        if let Some(client_state) = client_state {
            match message {
                ClientStateMessage::Music(
                    ClientMusicStateMessage::Musiclist(
                        MusiclistStateMessage::PlaylistPage(_)
                    )
                ) => {
                    let playlist_items = PlaylistItems::youtube_get(
                        &self.client, 
                        &PlaylistPageState {
                            playlist_id: client_state.music.musiclist.playlist_page.playlist_id.value().clone(),
                            page_token: client_state.music.musiclist.playlist_page.page_token.value().clone(),
                        },
                    ).await?;

                    let message = client_state.music.musiclist.list_items.state.apply_export(
                        playlist_items.into(),
                    );

                    log::info!(" > {user} 🔗 Client {}",
                        serde_json::to_string_pretty(&message).unwrap_or_default(),
                    );               
                                 
                    self.send(&id, message)?;
                },
                _ => {},
            }
        }        

        Ok(())
    }
}