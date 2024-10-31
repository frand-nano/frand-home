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
    server_prop: &MusicServer::Node,
    client_prop: &MusicClient::Node,
) -> Html {
    html! {
        <div style="display:flex; flex-direction: row;">
            <PlaylistView 
                visible = { client_prop.playlist_visible.clone() }
                list_items = { server_prop.playlist.list_items.clone() }
                musiclist_playlist_id = { client_prop.musiclist.playlist_page.playlist_id.clone() }
            />
            <div>
                <YoutubePlayerView
                    video_id = { client_prop.youtube_player.video_id.clone() }
                />
                <LyricsView/>
            </div>
            <div>
                <ServerPlayerView/>        
                <MusicQueueView/>    
                <MusiclistView
                    musiclist = { client_prop.musiclist.clone() }
                    youtube_player_video_id = { client_prop.youtube_player.video_id.clone() }
                />          
            </div>
        </div>
    }
}