use frand_home_node::Node;
use lyrics::LyricsView;
use music_queue::MusicQueueView;
use musiclist::MusiclistView;
use playlist::PlaylistView;
use server_player::ServerPlayerView;
use yew::{html, Html};
use youtube_player::YoutubePlayerView;

use crate::state::{client::music_client::MusicClient, server::music_server::MusicServer};

pub mod playlist;
pub mod musiclist;
pub mod youtube_player;
pub mod lyrics;
pub mod server_player;
pub mod music_queue;

pub fn view(
    server: &MusicServer::Node,
    client: &MusicClient::Node,
) -> Html {
    html! {
        <div style="display:flex; flex-direction: row;">
            <PlaylistView 
                visible = { client.playlist_visible.clone() }
                playlist = { server.playlist.clone() }
                playlist_page = { client.musiclist.page.clone() }
            />
            <div>
                <YoutubePlayerView
                    video_id = { client.youtube_player.video_id.clone() }
                />
                <LyricsView/>
            </div>
            <div>
                <ServerPlayerView/>        
                <MusicQueueView/>    
                <MusiclistView
                    musiclist = { client.musiclist.clone() }
                    pages = {    
                        server.playlist.items.iter()
                        .find(|item| item.playlist_id.clone_state() == client.musiclist.page.id.clone_state())
                        .map(|playlist| playlist.pages.clone_state())
                        .unwrap_or_default()  
                    }
                    youtube_player_video_id = { client.youtube_player.video_id.clone() }
                />          
            </div>
        </div>
    }
}