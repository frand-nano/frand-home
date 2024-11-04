use std::collections::HashMap;
use anyhow::anyhow;
use frand_home_node::{Node, RootMessage, VecMessage};
use mysql::Pool;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::state::{client::{music_client::MusicClient, musiclist::Musiclist, youtube_player::YoutubePlayer}, server::{music_server::MusicServer, playlist::{Playlist, PlaylistItem, PlaylistPage}}};

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

        let page = playlist.map(|t| t.page).unwrap_or_default();
        
        let pages = select_playlist_pages(
            &self.config, 
            &mut self.pool.get_conn()?, 
            &page.id,
        )?;

        let items = select_musics(
            &mut self.pool.get_conn()?, 
            &page,
        )?;

        let music = items.first().map(|item| item.to_owned()).unwrap_or_default();

        Ok(MusicClient::State {
            playlist_visible: true,
            musiclist: Musiclist::State { 
                visible: true,
                page, 
                pages, 
                items, 
            },
            youtube_player: YoutubePlayer::State {
                music,
            },
            lyrics: Default::default(),
        })
    }

    pub async fn handle_server_message<Msg: RootMessage>(
        &self,
        senders: &HashMap<Uuid, UnboundedSender<Msg>>,
        sender: &UnboundedSender<Msg>,
        server: &mut MusicServer::Node,
        client: &mut MusicClient::Node,
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
                            
                let page = playlist.page.clone_state();
                let mut conn = self.pool.get_conn()?;
                
                let playlist_items = get_playlist_items_all(
                    &self.client, 
                    &self.config, 
                    &page.id,
                ).await?;

                insert_update_musics(
                    &mut conn, 
                    &page.id,
                    playlist_items,
                ).await?;               

                let pages = select_playlist_pages(
                    &self.config, 
                    &mut conn, 
                    &page.id,
                )?;

                if let Some(page) = pages.first() {
                    let message: Msg = playlist.page.apply_export(page.to_owned());
                    for sender in senders.values() {
                        sender.send(message.clone())?;
                    }
                    self.client_open_music_page(sender, client, page.to_owned()).await?;
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
                self.client_open_music_page(sender, client, page).await?;
            },
            _ => {},
        })
    }   

    pub async fn client_open_music_page<Msg: RootMessage>(
        &self,
        sender: &UnboundedSender<Msg>,
        client: &mut MusicClient::Node,  
        page: PlaylistPage::State,      
    ) -> anyhow::Result<()> {
        let pages = select_playlist_pages(
            &self.config, 
            &mut self.pool.get_conn()?, 
            &page.id,
        )?;  

        let message: Msg = client.musiclist.pages.apply_export(pages);
        sender.send(message)?;  

        let message: Msg = client.musiclist.items.apply_export(
            select_musics(
                &mut self.pool.get_conn()?, 
                &page,
            )?,
        );          
                    
        sender.send(message)?;  

        Ok(())
    }
}

impl Music {
    async fn get_playlists(&self) -> anyhow::Result<Playlist::State> {
        let playlist = youtube::Playlist::youtube_get(
            &self.client, 
            &self.config,
        ).await?;

        let items = playlist.items.into_iter()
        .map(|item| {
            let playlist_id = self.config.playlist_id(&item.id)?;

            let pages = select_playlist_pages(
                &self.config, 
                &mut self.pool.get_conn()?, 
                &playlist_id,
            )?;

            let page = pages.first().map(|t| t.to_owned())
            .unwrap_or_else(|| PlaylistPage::State { 
                id: playlist_id, 
                ..Default::default() 
            });

            Ok(PlaylistItem::State { 
                youtube_title: item.snippet.title, 
                update: false, 
                page: page.to_owned(), 
            })            
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Playlist::State {
            items,
        })
    }

}