use frand_home_node::{Node, ValueNode};
use yew::{function_component, html, Html, Properties};

use crate::state::server::playlist::{PlaylistPage, Playlist};

#[derive(Properties, PartialEq)]
pub struct PlaylistProperty {
    pub visible: ValueNode<bool>,
    pub playlist: Playlist::Node,
    pub playlist_page: PlaylistPage::Node,
}

#[function_component]
pub fn PlaylistView(prop: &PlaylistProperty) -> Html {
    let visible = prop.visible.clone();
    let visible_value = *visible.value();
    let onclick_visible = move |_| {
        visible.emit(!visible_value)
    };

    let items: Vec<_> = prop.playlist.items.iter()
    .enumerate()
    .map(|(_, item)| {
        let playlist_page = prop.playlist_page.clone();
        let youtube_title = item.youtube_title.clone_state();
        let page = item.page.clone_state();
        let page_disabled = page.id.to_string().is_empty();
        let onclick_playlist = move |_| {         
            playlist_page.emit(page.clone())          
        };

        let item_update = item.update.clone();
        let item_update_value = *item_update.value();
        let onclick_item_update = move |_| {
            item_update.emit(true);
        };

        html! {
            <div>   
                <button disabled={item_update_value} onclick={onclick_item_update}> 
                {"🔄"}
                </button>
                <button disabled={page_disabled} onclick={onclick_playlist}>
                {youtube_title}
                </button>
            </div>
        }
    }).collect(); 

    match visible_value {
        true => {
            html! {
                <div class="vertical_div">                    
                    <button onclick={onclick_visible}>{" < Playlist"}</button>
                    {items}
                </div>
            }
        },
        false => { 
            html! {                 
                <button onclick={onclick_visible}>{" > "}</button>
            } 
        },
    }
}
