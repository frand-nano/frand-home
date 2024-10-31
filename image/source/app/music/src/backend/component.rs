use std::collections::HashMap;

use awc::Client;
use frand_home_node::{Item, Message, Node, StateNode, VecMessage};
use mysql::Pool;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::state::{client::{client_state::{ClientState, ClientStateMessage}, musiclist_state::MusiclistStateMessage}, server::{playlist_state::{PlaylistItemsStateMessage, PlaylistState, PlaylistStateMessage, PlaylistItemStateMessage}, server_state::{ServerState, ServerStateMessage}}};

use super::{config::Config, database::init_database, youtube::{playlist::Playlist, playlist_items::PlaylistItems}};

pub struct Music {
    pub config: &'static Config,
    pub client: Client,
    pub pool: Pool,
}

impl Music {    
    pub fn new(
        config: &'static Config,
        mysql_url: &str,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            client: Client::default(),
            pool: init_database(mysql_url)?,
        })
    }

    pub async fn new_server_state(
        &self,
    ) -> anyhow::Result<ServerState> {
        Ok(ServerState {
            playlist: PlaylistState {
                list_items: Playlist::youtube_get(self).await?.into(), 
            },
        })
    }

    pub async fn new_client_state(
        &self,
    ) -> anyhow::Result<ClientState> {              
        Ok(ClientState {
            playlist_visible: true,
            musiclist: Default::default(),
            youtube_player: Default::default(),
            lyrics: Default::default(),
        })
    }

    pub async fn handle_server_message<Msg: Message>(
        &self,
        senders: &HashMap<Uuid, UnboundedSender<Msg>>,
        prop: &mut <ServerState as Item>::Node,
        message: <<ServerState as Item>::Node as Node>::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            ServerStateMessage::Playlist(
                PlaylistStateMessage::ListItems(
                    PlaylistItemsStateMessage::Items(
                        VecMessage::Item(
                            (index, PlaylistItemStateMessage::Refresh(refresh))
                        )
                    )
                )
            ) => {
                if refresh {
                    let refresh = &mut prop.playlist.list_items.items.item_mut(index).refresh;
                    let message: Msg = refresh.apply_export(false);
                                
                    for sender in senders.values() {
                        sender.send(message.clone())?;
                    }
                }
            },
            _ => {},
        })
    }
    
    pub async fn handle_client_message<Msg: Message>(
        &self,
        sender: &UnboundedSender<Msg>,
        prop: &mut <ClientState as Item>::Node,
        message: <<ClientState as Item>::Node as Node>::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            ClientStateMessage::Musiclist(
                MusiclistStateMessage::PlaylistPage(_)        
            ) => {
                let playlist_items = PlaylistItems::youtube_get(
                    self,
                    &prop.musiclist.playlist_page.clone_state(),
                ).await?;
    
                let message: Msg = prop.musiclist.list_items.apply_export(
                    playlist_items.into(),
                );          
                                
                sender.send(message)?;  
            },
            _ => {},
        })
    }   
}