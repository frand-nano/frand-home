use frand_home_node::{Item, StateNode, ValueNode};
use yew::{function_component, html, Html, Properties};

use crate::state::server::playlist_state::PlaylistItemsState;

#[derive(Properties, PartialEq)]
pub struct PlaylistProperty {
    pub visible: ValueNode<bool>,
    pub list_items: <PlaylistItemsState as Item>::Node,
    pub musiclist_playlist_id: ValueNode<String>,
}

#[function_component]
pub fn Playlist(prop: &PlaylistProperty) -> Html {
    let visible = prop.visible.clone();
    let visible_value = *visible.value();
    let onclick_visible = move |_| {
        visible.emit(!visible_value)
    };

    let items: Vec<_> = prop.list_items.items.iter()
    .enumerate()
    .map(|(_, item)| {
        let musiclist_playlist_id = prop.musiclist_playlist_id.clone();
        let title = item.title.clone_state();
        let playlist_id = item.playlist_id.clone_state();
        let onclick_playlist = move |_| {
            musiclist_playlist_id.emit(playlist_id.clone())
        };

        let item_refresh = item.refresh.clone();
        let item_refresh_value = *item_refresh.value();
        let onclick_item_refresh = move |_| {
            item_refresh.emit(true);
        };

        html! {
            <div>   
                <button disabled={item_refresh_value} onclick={onclick_item_refresh}> 
                {"🔄"}
                </button>
                <button disabled={item_refresh_value} onclick={onclick_playlist}>
                {title}
                </button>
            </div>
        }
    }).collect(); 

    match visible_value {
        true => {
            html! {
                <div style="display:flex; flex-direction: column;">                    
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
