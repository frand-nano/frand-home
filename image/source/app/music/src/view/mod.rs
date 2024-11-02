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
        <div class="horizontal_div">
            <PlaylistView 
                visible = { client.playlist_visible.clone() }
                playlist = { server.playlist.clone() }
                playlist_page = { client.musiclist.page.clone() }
            />
            <div>
                <YoutubePlayerView
                    music_id = { client.youtube_player.music_id.clone() }
                />
                <LyricsView/>
            </div>
            <div>
                <ServerPlayerView/>        
                <MusicQueueView/>    
                <MusiclistView
                    musiclist = { client.musiclist.clone() }
                    youtube_player_music_id = { client.youtube_player.music_id.clone() }
                />          
            </div>
        </div>
    }
}