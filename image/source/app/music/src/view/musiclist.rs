use frand_home_node::Node;
use yew::{function_component, html, Html, Properties};

use crate::state::client::musiclist::{Musiclist, MusiclistItem};

#[derive(Properties, PartialEq)]
pub struct MusiclistProperty {
    pub musiclist: Musiclist::Node,
    pub youtube_player_music: MusiclistItem::Node,
}

#[function_component]
pub fn MusiclistView(prop: &MusiclistProperty) -> Html {    
    let visible = prop.musiclist.visible.clone();
    let visible_value = *visible.value();
    let onclick_visible = move |_| {
        visible.emit(!visible_value)
    };

    let pages: Vec<_> = prop.musiclist.pages.clone_state().into_iter().enumerate()
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

    let items: Vec<_> = prop.musiclist.items.iter()
    .map(|item| {
        let youtube_player_music = prop.youtube_player_music.clone();
        let youtube_title = item.youtube_title.clone_state();
        let music = item.clone_state();
        
        let onclick_music = move |_| {
            youtube_player_music.emit(music.clone())
        };
        html! {
            <>
                <button onclick={onclick_music}>
                    {youtube_title}
                </button>
                <div/>
            </>
        }
    }).collect(); 

    match visible_value {
        true => {
            html! {
                <div id="musiclist" class="vertical">            
                    <button onclick={onclick_visible}>
                        {"Musiclist > "}
                    </button>
                    <div>{pages}</div>
                    <div id="musiclist_items" class="vertical">
                        {items}
                    </div>
                </div>
            }
        },
        false => { 
            html! {            
                <div id="musiclist" style="width: 2rem" class="vertical">        
                    <button onclick={onclick_visible}>
                        {" < "}
                    </button>
                </div>
            } 
        },
    }
}