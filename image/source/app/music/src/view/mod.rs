use frand_home_state::State;
use lyrics::Lyrics;
use music_queue::MusicQueue;
use musiclist::Musiclist;
use playlist::Playlist;
use server_player::ServerPlayer;
use yew::{html, Html};
use youtube_player::YoutubePlayer;

use crate::state::{client::client_state::ClientState, server::server_state::ServerState};

pub mod playlist;
pub mod musiclist;
pub mod youtube_player;
pub mod lyrics;
pub mod server_player;
pub mod music_queue;

pub fn view(
    server_prop: &<ServerState as State>::Property,
    client_prop: &<ClientState as State>::Property,
) -> Html {
    html! {
        <div style="display:flex; flex-direction: row;">
            <Playlist 
                visible = { client_prop.playlist_visible.clone() }
                list_items = { server_prop.playlist.list_items.clone() }
                musiclist_playlist_id = { client_prop.musiclist.playlist_page.playlist_id.clone() }
            />
            <div>
                <YoutubePlayer
                    video_id = { client_prop.youtube_player.video_id.clone() }
                />
                <Lyrics/>
            </div>
            <div>
                <ServerPlayer/>        
                <MusicQueue/>    
                <Musiclist
                    musiclist = { client_prop.musiclist.clone() }
                    youtube_player_video_id = { client_prop.youtube_player.video_id.clone() }
                />          
            </div>
        </div>
    }
}