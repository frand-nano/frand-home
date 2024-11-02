use frand_home_node::ValueNode;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct YoutubePlayerProperty {
    pub music_id: ValueNode<String>,
}

#[function_component]
pub fn YoutubePlayerView(prop: &YoutubePlayerProperty) -> Html {
    let video_id = prop.music_id.value();

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