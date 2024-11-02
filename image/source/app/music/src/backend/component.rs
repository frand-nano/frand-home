use std::collections::HashMap;
use anyhow::anyhow;
use frand_home_node::{Node, RootMessage, VecMessage};
use mysql::Pool;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::state::{client::{music_client::MusicClient, musiclist::Musiclist, youtube_player::YoutubePlayer}, server::{music_server::MusicServer, playlist::{Playlist, PlaylistItem}}};

use super::{config::Config, database::{init_database, music::insert_update_musics, select_playlist_pages, select_musics}, youtube::{self, get_playlist_items_all}};

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
        let playlist = self.get_playlists().await?
        .items.first()
        .map(|t| t.to_owned());

        let pages = match playlist {
            Some(playlist) => select_playlist_pages(
                &self.config, 
                &mut self.pool.get_conn()?, 
                &playlist.page.id,
            )?,
            None => Default::default(),
        };

        let page = pages.first().map(|page| page.to_owned()).unwrap_or_default();
        
        let items = select_musics(
            &mut self.pool.get_conn()?, 
            &page,
        )?;

        let music_id = items.first().map(|item| item.music_id.to_owned()).unwrap_or_default();

        Ok(MusicClient::State {
            playlist_visible: true,
            musiclist: Musiclist::State { 
                page, 
                pages, 
                items, 
            },
            youtube_player: YoutubePlayer::State {
                music_id,
            },
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
                let playlist = server.playlist.items
                .get_mut(index)
                .ok_or(anyhow!("server.playlist.list_items.items has no index {index}"))?;

                let message: Msg = playlist.update.apply_export(true);
                for sender in senders.values() {
                    sender.send(message.clone())?;
                }
                            
                let playlist_page = playlist.page.clone_state();
                let mut conn = self.pool.get_conn()?;
                
                let playlist_items = get_playlist_items_all(
                    &self.client, 
                    &self.config, 
                    &playlist_page.id,
                ).await?;

                insert_update_musics(
                    &mut conn, 
                    &playlist_page.id,
                    playlist_items,
                ).await?;               

                let pages = select_playlist_pages(
                    &self.config, 
                    &mut conn, 
                    &playlist_page.id,
                )?;

                if let Some(page) = pages.first() {
                    let message: Msg = playlist.page.apply_export(page.to_owned());
                    for sender in senders.values() {
                        sender.send(message.clone())?;
                    }
                } 

                let message: Msg = playlist.update.apply_export(false);
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
                let page = client.musiclist.page.clone_state();
                let message: Msg = client.musiclist.items.apply_export(
                    select_musics(
                        &mut self.pool.get_conn()?, 
                        &page,
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

            let pages = select_playlist_pages(
                &self.config, 
                &mut conn, 
                &playlist_id,
            )?;

            Ok(pages.first().map(|page| {
                PlaylistItem::State { 
                    youtube_title: item.snippet.title, 
                    update: false, 
                    page: page.to_owned(), 
                }
            }))            
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Playlist::State {
            items: items.into_iter().filter_map(|t| t).collect(),
        })
    }

}