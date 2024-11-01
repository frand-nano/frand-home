use std::collections::HashMap;
use anyhow::anyhow;
use frand_home_node::{Node, RootMessage, VecMessage};
use mysql::Pool;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::state::{client::{music_client::MusicClient, musiclist::Musiclist}, server::{music_server::MusicServer, playlist::{PlaylistItem, Playlist}}};

use super::{config::Config, database::{init_database, music::insert_update_musics, select_music_ranges, select_musics}, youtube};

pub struct Music {
    config: &'static Config,
    client: awc::Client,
    pool: Pool,
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
            playlist: self.get_playlists().await?,
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
        server: &mut MusicServer::Node,
        message: MusicServer::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            MusicServer::Message::Playlist(
                Playlist::Message::Items(
                    VecMessage::Item(
                        (index, PlaylistItem::Message::Update(update))
                    )
                )
            ) if update => {
                let node = server.playlist.items
                .get_mut(index).ok_or(anyhow!("server.playlist.list_items.items has no index {index}"))?;

                let message: Msg = node.update.apply_export(false);
                            
                insert_update_musics(
                    &self.client, 
                    &self.config, 
                    &mut self.pool.get_conn()?, 
                    &node.playlist_id.clone_state(),
                ).await?;               

                for sender in senders.values() {
                    sender.send(message.clone())?;
                }
            },
            _ => {},
        })
    }
    
    pub async fn handle_client_message<Msg: RootMessage>(
        &self,
        sender: &UnboundedSender<Msg>,
        _server: &MusicServer::Node,
        client: &mut MusicClient::Node,
        message: MusicClient::Message,
    ) -> anyhow::Result<()> {
        Ok(match message {
            MusicClient::Message::Musiclist(
                Musiclist::Message::Page(_)        
            ) => {    
                let message: Msg = client.musiclist.items.apply_export(
                    select_musics(
                        &mut self.pool.get_conn()?, 
                        &client.musiclist.page.clone_state(),
                    )?,
                );          
                                
                sender.send(message)?;  
            },
            _ => {},
        })
    }   
}

impl Music {
    async fn get_playlists(&self) -> anyhow::Result<Playlist::State> {
        let playlist = youtube::Playlist::youtube_get(
            &self.client, 
            &self.config,
        ).await?;

        let mut conn = self.pool.get_conn()?;

        let items = playlist.items.into_iter()
        .map(|item| {
            let playlist_id = self.config.playlist_id(&item.id)?;

            Ok(PlaylistItem::State { 
                youtube_title: item.snippet.title, 
                update: false, 
                pages: select_music_ranges(&self.config, &mut conn, &playlist_id)?, 
                playlist_id,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Playlist::State {
            items,
        })
    }

}