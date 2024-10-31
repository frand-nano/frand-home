use frand_home_node::{Item, StateNode, ValueNode};
use yew::{function_component, html, Html, Properties};

use crate::state::client::musiclist_state::MusiclistState;

#[derive(Properties, PartialEq)]
pub struct MusiclistProperty {
    pub musiclist: <MusiclistState as Item>::Node,
    pub youtube_player_video_id: ValueNode<String>,
}

#[function_component]
pub fn Musiclist(prop: &MusiclistProperty) -> Html {    
    let pages = {
        let page_token = prop.musiclist.playlist_page.page_token.clone();
        let prev_page_token = prop.musiclist.list_items.prev_page_token.clone_state();
        let prev_page_disabled = prev_page_token.is_none();
        let onclick_prev_page = move |_| {
            page_token.emit(prev_page_token.clone());
        };

        let page_token = prop.musiclist.playlist_page.page_token.clone();
        let next_page_token = prop.musiclist.list_items.next_page_token.clone_state();
        let next_page_disabled = next_page_token.is_none();
        let onclick_next_page = move |_| {
            page_token.emit(next_page_token.clone());
        };

        html! {
            <div>
                <button disabled={prev_page_disabled} onclick={onclick_prev_page}>
                {"prev"}
                </button>
                <button disabled={next_page_disabled} onclick={onclick_next_page}>
                {"next"}
                </button>
            </div>
        }
    };

    let items: Vec<_> = prop.musiclist.list_items.items.iter()
    .map(|item| {
        let youtube_player_video_id = prop.youtube_player_video_id.clone();
        let title = item.title.clone_state();
        let video_id = item.video_id.clone_state();
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
            {pages}
            {items}
        </div>
    }
}