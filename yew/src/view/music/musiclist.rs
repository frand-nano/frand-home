use frand_home_common::{state::client::view::music::musiclist_state::MusiclistState, Node, State};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct MusiclistProperty {
    pub state: <MusiclistState as State>::Property,
    pub youtube_player_video_id: Node<String>,
}

#[function_component]
pub fn Musiclist(prop: &MusiclistProperty) -> Html {
    let items: Vec<_> = prop.state.list_items.items.items().clone().into_iter()
    .map(|item| {
        let youtube_player_video_id = prop.youtube_player_video_id.clone();
        let title = item.title.clone();
        let video_id = item.video_id.clone();
        let onclick_music = move |_| {
            youtube_player_video_id.emit(video_id.clone())
        };
        html! {
            <button onclick={onclick_music}>
            {title}
            </button>
        }
    }).collect(); 

    html! {
        <div style="display:flex; flex-direction: column;">
            <p>{"Musiclist"}</p>
            {items}
        </div>
    }
}