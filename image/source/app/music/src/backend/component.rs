use std::collections::HashMap;

use frand_home_node::{Node, RootMessage, VecMessage};
use mysql::Pool;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::state::{client::{music_client::MusicClient, musiclist::Musiclist}, server::{playlist::{PlaylistItem, PlaylistItems, Playlist}, music_server::MusicServer}};

use super::{config::Config, database::init_database, youtube};

pub struct Music {
    pub config: &'static Config,
    pub client: awc::Client,
    pub pool: Pool,
}

impl Music {    
    pub fn new(
        config: &'static Config,
        mysql_url: &str,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            client: awc::Client::default(),
            pool: init_database(mysql_url)?,
        })
    }

    pub async fn new_server_state(
        &self,
    ) -> anyhow::Result<MusicServer::State> {
        Ok(MusicServer::State {
            playlist: Playlist::State {
                list_items: youtube::Playlist::youtube_get(self).await?.into(), 
            },
        })
    }

    pub async fn new_client_state(
        &self,
    ) -> anyhow::Result<MusicClient::State> {              
        Ok(MusicClient::State {
            playlist_visible: true,
            musiclist: Default::default(),
            youtube_player: Default::default(),
            lyrics: Default::default(),
        })
    }

    pub async fn handle_server_message<Msg: RootMessage>(
        &self,
        senders: &HashMap<Uuid, UnboundedSender<Msg>>,
        prop: &mut MusicServer::Node,
        message: MusicServer::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            MusicServer::Message::Playlist(
                Playlist::Message::ListItems(
                    PlaylistItems::Message::Items(
                        VecMessage::Item(
                            (index, PlaylistItem::Message::Refresh(refresh))
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
    
    pub async fn handle_client_message<Msg: RootMessage>(
        &self,
        sender: &UnboundedSender<Msg>,
        prop: &mut MusicClient::Node,
        message: MusicClient::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            MusicClient::Message::Musiclist(
                Musiclist::Message::PlaylistPage(_)        
            ) => {
                let playlist_items = youtube::PlaylistItems::youtube_get(
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