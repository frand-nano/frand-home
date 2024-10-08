use frand_home_common::{state::client::view::music::youtube_player_state::YoutubePlayerState, State};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct YoutubePlayerProperty {
    pub state: <YoutubePlayerState as State>::Property,
}

#[function_component]
pub fn YoutubePlayer(prop: &YoutubePlayerProperty) -> Html {
    let video_id = prop.state.video_id.value();

    html! {
        <div>
            <p>{"YoutubePlayer"}</p>
            <object 
                type="text/html" 
                data={format!("//www.youtube.com/embed/{video_id}")}
            />
        </div>
    }
}