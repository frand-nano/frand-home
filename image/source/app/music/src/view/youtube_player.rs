use yew::{function_component, html, Html, Properties};
use crate::state::client::youtube_player::YoutubePlayer;

#[derive(Properties, PartialEq)]
pub struct YoutubePlayerProperty {
    pub youtube_player: YoutubePlayer::Node,
}

#[function_component]
pub fn YoutubePlayerView(prop: &YoutubePlayerProperty) -> Html {
    let title = prop.youtube_player.music.youtube_title.value();
    let music_id = prop.youtube_player.music.music_id.value();

    html! {
        <div id="youtube_player">
            <p> {title} </p>
            <div id="youtube_player_container">
                <object 
                    type="text/html" 
                    data={format!("//www.youtube.com/embed/{music_id}")}
                    style="position:absolute; top:0; left:0; width: 100%; height: 100%;"
                />
            </div>
        </div>
    }
}