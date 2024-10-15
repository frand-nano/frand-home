use yew::{html, Html};

use crate::{app::app_property::AppProperty, view::{music::{lyrics::Lyrics, music_queue::MusicQueue, musiclist::Musiclist, playlist::Playlist, server_player::ServerPlayer, youtube_player::YoutubePlayer}, task_bar::TaskBar}};

pub fn view(prop: &AppProperty) -> Html {     
    html! {
        <div>
            <TaskBar
                user = { prop.socket.client.user.clone() }
                playlist_visible = { prop.socket.client.music.playlist.visible.clone() }
            />     
            <div style="display:flex; flex-direction: row;">
                <Playlist 
                    visible = { prop.socket.client.music.playlist.visible.clone() }
                    list_items = { prop.socket.client.music.playlist.list_items.clone() }
                    musiclist_playlist_id = { prop.socket.client.music.musiclist.playlist_page.playlist_id.clone() }
                />
                <div>
                    <YoutubePlayer
                        video_id = { prop.socket.client.music.youtube_player.video_id.clone() }
                    />
                    <Lyrics/>
                </div>
                <div>
                    <ServerPlayer/>        
                    <MusicQueue/>    
                    <Musiclist
                        musiclist = { prop.socket.client.music.musiclist.clone() }
                        youtube_player_video_id = { prop.socket.client.music.youtube_player.video_id.clone() }
                    />          
                </div>
            </div>
        </div>
    }
}