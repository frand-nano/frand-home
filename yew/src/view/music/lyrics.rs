use frand_home_common::{state::client::view::music::lyrics_state::LyricsState, State};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LyricsProperty {
    pub state: <LyricsState as State>::Property,
}

#[function_component]
pub fn Lyrics(_prop: &LyricsProperty) -> Html {
    html! {
        <div>

        </div>
    }
}