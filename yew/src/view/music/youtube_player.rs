use frand_home_common::Node;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct YoutubePlayerProperty {
    pub video_id: Node<String>,
}

#[function_component]
pub fn YoutubePlayer(prop: &YoutubePlayerProperty) -> Html {
    let video_id = prop.video_id.value();

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