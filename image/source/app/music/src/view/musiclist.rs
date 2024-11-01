use frand_home_node::{Node, ValueNode};
use yew::{function_component, html, Html, Properties};

use crate::state::{client::musiclist::Musiclist, server::playlist::PlaylistPage};

#[derive(Properties, PartialEq)]
pub struct MusiclistProperty {
    pub musiclist: Musiclist::Node,
    pub pages: Vec<PlaylistPage::State>,
    pub youtube_player_video_id: ValueNode<String>,
}

#[function_component]
pub fn MusiclistView(prop: &MusiclistProperty) -> Html {    
    let pages: Vec<_> = prop.pages.clone().into_iter().enumerate()
    .map(|(index, page)| {
        let musiclist_page = prop.musiclist.page.clone();            
        let onclick_page = move |_| {
            musiclist_page.emit(page.clone())
        };
        html! {        
            <button onclick={onclick_page}>
            {index}
            </button>
        }
    })
    .collect();

    let pages = {
        html! {
            <div>
                {pages}
            </div>
        }
    };

    let items: Vec<_> = prop.musiclist.items.iter()
    .map(|item| {
        let youtube_player_video_id = prop.youtube_player_video_id.clone();
        let youtube_title = item.youtube_title.clone_state();
        let music_id = item.music_id.clone_state();
        
        let onclick_music = move |_| {
            youtube_player_video_id.emit(music_id.clone())
        };
        html! {
            <button onclick={onclick_music}>
            {youtube_title}
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