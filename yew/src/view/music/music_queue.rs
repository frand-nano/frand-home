use frand_home_common::{state::server::view::music::music_queue_state::MusicQueueState, State};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct MusicQueueProperty {
    pub state: <MusicQueueState as State>::Property,
}

#[function_component]
pub fn MusicQueue(_prop: &MusicQueueProperty) -> Html {
    html! {
        <div>

        </div>
    }
}